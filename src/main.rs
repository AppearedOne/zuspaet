#![windows_subsystem = "windows"]

use chrono::{Datelike, Local};
use iced::event::{self, Event};
use iced::widget::{
    button, checkbox, column, combo_box, container, horizontal_rule, horizontal_space, pick_list,
    row, scrollable, slider, stack, text, text_input, toggler, tooltip, vertical_rule,
    vertical_space,
};
use iced::window;
use iced::{alignment, Alignment, Element, Font, Length, Padding, Subscription, Task, Theme};

pub mod bootstrap;
use bootstrap::*;
pub mod absences;
pub mod toast;
use toast::*;
pub mod db;
pub mod list;
pub mod menu;
pub mod new;
pub mod settings;
pub mod stats;
pub mod themes;
pub mod time;

use db::{Class, DataBase, DataBaseError, Lesson};
use stats::{update_stats, StatState, StatsMessage};

//#[cfg(not(target_arch = "wasm32"))]
//#[tokio::main]
//async fn main() -> iced::Result {
fn main() -> iced::Result {
    iced::application(
        || {
            (
                App::new().0,
                Task::perform(
                    db::DataBase::load_file("db.json"),
                    |r: Result<DataBase, DataBaseError>| match r {
                        Ok(db) => Message::DBLoaded(db),
                        Err(_) => Message::Notify(Toast::new(
                            "Error",
                            "DB nix geladen oopsiwoopsy",
                            Status::Danger,
                        )),
                    },
                )
                .chain(Task::perform(
                    settings::load_from_file("settings.json"),
                    Message::SettingsLoaded,
                )),
            )
        },
        App::update,
        App::view,
    )
    .theme(App::theme)
    .subscription(App::subscription)
    .exit_on_close_request(false)
    .font(bootstrap::ICON_FONT_BYTES)
    .title(App::title)
    .run()
}

const ICON_FONT: Font = Font::with_name("bootstrap-icons");

async fn save_all(db: db::DataBase, sets: settings::Settings) -> Result<(), db::DataBaseError> {
    settings::save_to_file(sets, "settings.json").await;
    return db.save_file("db.json".to_string()).await;
}

#[derive(Debug, Clone)]
pub enum ViewControl {
    ADD,
    LISTVIEW,
    STATS,
    SETTINGS,
    MENU,
    ABSENCES,
}

pub struct App {
    add_entry: db::Entry,
    db: db::DataBase,
    view: ViewControl,
    view_origin: ViewControl,
    combo: combo_box::State<Class>,
    sel_pers: Option<Class>,

    combo2: combo_box::State<Lesson>,
    sel_lesson: Option<Lesson>,
    status_text: String,
    theme_state: combo_box::State<Theme>,
    selected_theme: Option<Theme>,

    stats: StatState,

    toasts: Vec<Toast>,

    abs: db::LessonAbs,

    menu: menu::MenuState,

    list: list::ListState,
}

#[derive(Debug, Clone)]
pub enum Message {
    Exit(Result<(), db::DataBaseError>),
    EventOccurred(Event),
    DBLoaded(DataBase),
    GoView(ViewControl),
    BackView,
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
    Stats(StatsMessage),
    SaveExit,
    BackupDB,
    DeleteDB,
    SaveDB,
    Notify(Toast),
    Nothing,
    CloseToast(usize),
    Abs(absences::AbsMsg),
    MainMenu(menu::MenuMsg),
    List(list::ListMsg),
}

impl App {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                add_entry: db::Entry::empty(),
                db: db::DataBase::empty(),
                view: ViewControl::MENU,
                view_origin: ViewControl::MENU,
                combo: combo_box::State::new(db::Class::all()),
                sel_pers: None,
                combo2: combo_box::State::new(db::Lesson::all()),
                sel_lesson: None,
                status_text: String::new(),
                theme_state: combo_box::State::new(Theme::ALL.to_vec()),
                selected_theme: None,
                stats: StatState::new(),
                toasts: vec![],
                abs: db::LessonAbs::new(),
                menu: menu::MenuState::new(),
                list: list::ListState::default(),
            },
            Task::none(),
        )
    }
    fn title(&self) -> String {
        String::from("Verspätungsmanager4002 Ultra Pro Max")
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
            Message::SaveExit => {
                let mut sets = settings::Settings::new();
                match self.selected_theme.clone() {
                    Some(theme) => {
                        sets.theme = theme.to_string();
                    }
                    None => (),
                }
                return Task::perform(save_all(self.db.clone(), sets), Message::Exit);
            }
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
                self.abs = db::LessonAbs::new_smart(&self.db.absences);
            }
            Message::GoView(v) => {
                self.view_origin = self.view.clone();
                self.view = v;
            }
            Message::BackView => {
                self.view = self.view_origin.clone();
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
            Message::IsFirst(_) => {
                self.add_entry.first_lesson = !self.add_entry.first_lesson;
            }
            Message::DelayE(d) => {
                self.add_entry.delay_min = d;
            }
            Message::AddEntry => {
                self.db.data.push(self.add_entry.clone());
                self.view = ViewControl::LISTVIEW;
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
            Message::Stats(t) => return update_stats(self, t),
            Message::BackupDB => {
                let name = Local::now().to_string();
                return Task::perform(
                    self.db
                        .clone()
                        .save_file(format!("{}-bak.json", name).to_string()),
                    |r| -> Message {
                        match r {
                            Ok(_) => Message::Notify(Toast::new(
                                "Success",
                                "Created Backup",
                                Status::Success,
                            )),
                            Err(_) => Message::Notify(Toast::new(
                                "Error",
                                "DatabaseError thrown",
                                Status::Danger,
                            )),
                        }
                    },
                );
            }
            Message::DeleteDB => {
                self.db = DataBase::empty();
            }
            Message::SaveDB => {
                let mut sets = settings::Settings::new();
                match self.selected_theme.clone() {
                    Some(theme) => {
                        sets.theme = theme.to_string();
                    }
                    None => (),
                }
                return Task::perform(save_all(self.db.clone(), sets), |r| match r {
                    Ok(_) => Message::Notify(Toast::new(
                        "Success",
                        "DB and Settings saved",
                        Status::Success,
                    )),
                    Err(_) => Message::Notify(Toast::new(
                        "Failed",
                        "DB and Settings not saved",
                        Status::Danger,
                    )),
                });
            }
            Message::Notify(t) => self.notify(t),
            Message::CloseToast(index) => {
                self.toasts.remove(index);
            }
            Message::Nothing => (),
            Message::Abs(msg) => return absences::handle_absences(msg, self),
            Message::MainMenu(msg) => return menu::update_menu(self, msg),
            Message::List(msg) => return list::update_list(self, msg),
        }
        Task::none()
    }

    fn view(&self) -> Element<Message> {
        let content = match self.view {
            ViewControl::ADD => new::new_entry_view(self),
            ViewControl::LISTVIEW => list::listview(self),
            ViewControl::STATS => stats::stats_view(self),
            ViewControl::SETTINGS => settings::settings_view(self),
            ViewControl::MENU => menu::menu_view(self),
            ViewControl::ABSENCES => absences::absences_view(self),
        };
        toast::Manager::new(content, &self.toasts, Message::CloseToast)
            .timeout(3)
            .into()
    }

    fn theme(&self) -> Theme {
        match &self.selected_theme {
            Some(theme) => theme.clone(),
            None => Theme::KanagawaDragon,
        }
    }

    pub fn notify(&mut self, t: Toast) {
        self.toasts.push(t);

    }
}
impl Default for App {
    fn default() -> Self {
        Self::new().0
    }
}
