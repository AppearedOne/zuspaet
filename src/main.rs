use chrono::Datelike;
use iced::event::{self, Event};
use iced::widget::{
    button, checkbox, column, combo_box, container, horizontal_rule, horizontal_space, pick_list,
    row, scrollable, slider, stack, text, text_input, toggler, tooltip, vertical_rule,
    vertical_space,
};
use iced::window;
use iced::{alignment, Alignment, Element, Font, Length, Padding, Subscription, Task, Theme};

pub mod db;
pub mod settings;
pub mod stats;
pub mod time;

use db::{Class, DataBase, Lesson};

//#[cfg(not(target_arch = "wasm32"))]
//#[tokio::main]
//async fn main() -> iced::Result {
fn main() -> iced::Result {
    /*std::env::set_current_dir(
       std::env::current_exe()
            .expect("Couldnt get exec dir")
            .parent()
            .unwrap(),
    )
    .expect("didnt work");*/
    iced::application(App::title, App::update, App::view)
        .theme(App::theme)
        .subscription(App::subscription)
        .exit_on_close_request(false)
        .run_with(|| {
            (
                App::new().0,
                Task::perform(db::DataBase::load_file("db.json"), Message::DBLoaded).chain(
                    Task::perform(
                        settings::load_from_file("settings.json"),
                        Message::SettingsLoaded,
                    ),
                ),
            )
        })
}
async fn save_all(db: db::DataBase, sets: settings::Settings) -> Result<(), db::DataBaseError> {
    settings::save_to_file(sets, "settings.json").await;
    return db.save_file("db.json".to_string()).await;
}

#[derive(Debug, Clone)]
pub enum ViewControl {
    ADD,
    MAIN,
    STATS,
    SETTINGS,
}

pub struct App {
    add_entry: db::Entry,
    db: db::DataBase,
    view: ViewControl,
    combo: combo_box::State<Class>,
    sel_pers: Option<Class>,

    combo2: combo_box::State<Lesson>,
    sel_lesson: Option<Lesson>,
    status_text: String,
    theme_state: combo_box::State<Theme>,
    selected_theme: Option<Theme>,
}

#[derive(Debug, Clone)]
pub enum Message {
    Exit(Result<(), db::DataBaseError>),
    EventOccurred(Event),
    DBLoaded(DataBase),
    GoView(ViewControl),
    Add,
    AddEntry,
    SelectPerson(Class),
    SelectLesson(Lesson),
    IsFirst(bool),
    DelayE(u32),
    RemDay,
    AddDay,
    DLEntry(db::Entry),
    LastLessonTime,
    NextLessonTime,
    Edit,
    ThemeSelected(Theme),
    SettingsLoaded(settings::Settings),
}

impl App {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                add_entry: db::Entry::empty(),
                db: db::DataBase::empty(),
                view: ViewControl::MAIN,
                combo: combo_box::State::new(db::Class::all()),
                sel_pers: None,
                combo2: combo_box::State::new(db::Lesson::all()),
                sel_lesson: None,
                status_text: String::new(),
                theme_state: combo_box::State::new(Theme::ALL.to_vec()),
                selected_theme: None,
            },
            Task::none(),
        )
    }
    fn title(&self) -> String {
        String::from("Verspätungsmanager4001 Ultra Pro Max")
    }
    fn subscription(&self) -> Subscription<Message> {
        event::listen().map(Message::EventOccurred)
    }
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Exit(res) => match res {
                Ok(_) => {
                    println!("EXITING???");
                    return window::get_latest().and_then(window::close);
                }
                Err(_) => self.status_text = "Couldn't save data, not exiting".to_string(),
            },
            Message::EventOccurred(event) => {
                if let Event::Window(window::Event::CloseRequested) = event {
                    let mut sets = settings::Settings::new();
                    match self.selected_theme.clone() {
                        Some(theme) => {
                            sets.theme = theme.to_string();
                        }
                        None => (),
                    }
                    /*return Task::perform(
                        self.db.clone().save_file("db.json".to_string()),
                        Message::Exit,
                    );*/
                    return Task::perform(save_all(self.db.clone(), sets), Message::Exit);
                }
                return Task::none();
            }
            Message::DBLoaded(d) => {
                self.db = d;
            }
            Message::GoView(v) => {
                self.view = v;
            }
            Message::Add => {
                self.add_entry.lesson_time = time::get_last_lesson();
                self.add_entry.date = time::get_today();
                self.view = ViewControl::ADD;
            }
            Message::SelectPerson(p) => {
                self.add_entry.person = p.clone();
                self.sel_pers = Some(p);
            }
            Message::SelectLesson(l) => {
                self.add_entry.lesson = l.clone();
                self.sel_lesson = Some(l);
                println!("{}", self.add_entry.lesson_time);
            }
            Message::IsFirst(b) => {
                self.add_entry.first_lesson = !self.add_entry.first_lesson;
            }
            Message::DelayE(d) => {
                self.add_entry.delay_min = d;
            }
            Message::AddEntry => {
                self.db.data.push(self.add_entry.clone());
                self.view = ViewControl::MAIN;
            }
            Message::AddDay => {
                self.add_entry.date = self.add_entry.date.succ_opt().expect("Theres no tommorow?");
            }
            Message::RemDay => {
                self.add_entry.date = self
                    .add_entry
                    .date
                    .pred_opt()
                    .expect("There was no yesterday?");
            }
            Message::DLEntry(e) => {
                for i in 0..self.db.data.len() {
                    if self.db.data[i] == e {
                        self.db.data.remove(i);
                        break;
                    }
                }
            }
            Message::LastLessonTime => {
                self.add_entry.lesson_time = time::get_prev_lesson(self.add_entry.lesson_time);
            }
            Message::NextLessonTime => {
                self.add_entry.lesson_time = time::get_next_lesson(self.add_entry.lesson_time);
            }
            Message::Edit => {}
            Message::ThemeSelected(t) => self.selected_theme = Some(t),
            Message::SettingsLoaded(sets) => {
                self.selected_theme = settings::string_to_theme(&sets.theme);
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<Message> {
        let s = "Erste Lektion des Tages?".to_string();
        match self.view {
            ViewControl::ADD => row![column![
                text("Neuer EINTRAG!!! (wooowww)").size(30),
                horizontal_rule(0),
                combo_box(
                    &self.combo,
                    "Niemand?",
                    self.sel_pers.as_ref(),
                    Message::SelectPerson
                ),
                combo_box(
                    &self.combo2,
                    "Keine?",
                    self.sel_lesson.as_ref(),
                    Message::SelectLesson
                ),
                row![
                    text(format!("Lektionsstart: {}", self.add_entry.lesson_time)),
                    button("-").on_press(Message::LastLessonTime),
                    button("+").on_press(Message::NextLessonTime)
                ]
                .spacing(5)
                .align_y(Alignment::Center),
                toggler(self.add_entry.first_lesson)
                    .on_toggle(Message::IsFirst)
                    .label(s),
                row![
                    text(format!("Verspätung: {}", self.add_entry.delay_min)),
                    slider(
                        std::ops::RangeInclusive::new(0, 45),
                        self.add_entry.delay_min,
                        Message::DelayE
                    )
                ]
                .spacing(5)
                .align_y(Alignment::Center),
                row![
                    text(format!("{},", self.add_entry.date)),
                    text(format!("{}", self.add_entry.date.weekday())),
                    button("-").on_press(Message::RemDay),
                    button("+").on_press(Message::AddDay)
                ]
                .spacing(5)
                .align_y(Alignment::Center),
                row![
                    button("Hinzufügen").on_press(Message::AddEntry),
                    button("Doch nicht").on_press(Message::GoView(ViewControl::MAIN))
                ]
                .spacing(5)
                .align_y(Alignment::Center),
            ]
            .width(Length::Fill)
            .spacing(5)
            .align_x(Alignment::Center),]
            .align_y(Alignment::Center)
            .width(Length::Fill)
            .spacing(5)
            .into(),
            ViewControl::MAIN => {
                let mut lates = column![];
                for entry in &self.db.data {
                    let t = text(entry.person.to_string());
                    lates = lates.push(column![
                        row![
                            t.size(20).style(text::success),
                            horizontal_space(),
                            text(entry.lesson.to_string())
                                .style(|theme: &Theme| text::primary(theme)),
                            text(format!(" Erste Lektion: {} ", entry.first_lesson)),
                            text(format!("{} Min", entry.delay_min))
                                .style(|theme: &Theme| text::primary(theme)),
                            horizontal_space(),
                            column![
                                text(entry.lesson_time.to_string()),
                                text(entry.date.to_string())
                            ]
                            .padding(3),
                            column![
                                button("Löschen").on_press(Message::DLEntry(entry.clone())),
                                button("Bearbeiten").on_press(Message::Edit)
                            ]
                            .align_x(Alignment::Center)
                            .spacing(5)
                            .padding(5),
                        ]
                        .padding(20)
                        .align_y(Alignment::Center),
                        horizontal_rule(0),
                    ]);
                }
                row![column![
                    text("Verspätungsmanager4000 Ultra Pro Max").size(20),
                    //text(format!("{}", std::env::current_dir().unwrap().display())),
                    text(self.status_text.clone()).style(text::danger),
                    row![
                        button("Neuer Eintrag").on_press(Message::Add),
                        button("Statistik").on_press(Message::GoView(ViewControl::STATS)),
                        button("Einstellungen").on_press(Message::GoView(ViewControl::SETTINGS))
                    ]
                    .padding(5)
                    .spacing(5),
                    horizontal_rule(0),
                    scrollable(lates),
                ]
                .width(Length::Fill)
                .align_x(alignment::Alignment::Center),]
                .align_y(Alignment::Center)
                .width(Length::Fill)
                .into()
            }
            ViewControl::STATS => stats::stats_view(self),
            ViewControl::SETTINGS => settings::settings_view(self),
        }
    }

    fn theme(&self) -> Theme {
        match &self.selected_theme {
            Some(theme) => theme.clone(),
            None => Theme::KanagawaDragon,
        }
    }
}
impl Default for App {
    fn default() -> Self {
        Self::new().0
    }
}
