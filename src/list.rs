use crate::bootstrap::*;
use crate::db::Class;
use crate::themes::{self, text_fg, text_fg_succes};
use crate::{App, Message, ViewControl};
use iced::widget::{
    button, checkbox, column, combo_box, container, horizontal_rule, horizontal_space, pick_list,
    row, scrollable, slider, stack, text, text_input, toggler, tooltip, vertical_rule,
    vertical_space,
};
use iced::Element;
use iced::{alignment, Alignment, Font, Length, Padding, Subscription, Task, Theme};

#[derive(Clone, Debug, Default, PartialEq)]
pub enum ListMode {
    Total,
    Absences,
    #[default]
    Delays
}

impl ToString for ListMode {
    fn to_string(&self) -> String {
        match self {
            Self::Total => "Total".to_string(),
            Self::Delays => "Verspätungen".to_string(),
            Self::Absences => "Absenzen".to_string(),
        }
    }
}

impl ListMode {
    pub fn all() -> Vec<Self> {
        vec![ListMode::Total, ListMode::Absences, ListMode::Delays]
    }
}

#[derive(Clone, Debug)]
pub enum ListMsg {
    SetListMode(ListMode),
}

pub fn update_list(app: &mut App, msg: ListMsg) -> Task<Message> {
    match msg {
       ListMsg::SetListMode(lm) => app.list.kind = lm, 
    }
    Task::none()
}

pub struct ListState {
   kind: ListMode, 
}

impl Default for ListState {
    fn default() -> Self {
        ListState { kind: ListMode::default() }
    }
}

pub fn delays_list(app: &App) -> Element<Message> {let mut entries = column![];
    for entry in &app.db.data {
        let t = text(entry.person.to_string());
        entries = entries.push(column![
            row![
                t.size(20).style(text::success),
                horizontal_space(),
                text(entry.lesson.to_string()).style(|theme: &Theme| text::primary(theme)),
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
                    button(
                        text(icon_to_string(Bootstrap::TrashthreeFill))
                            .font(ICON_FONT)
                            .size(22)
                            .style(themes::text_fg_danger)
                    )
                    .on_press(Message::DLEntry(entry.clone()))
                    .style(button::text),
                    //button("Bearbeiten").on_press(Message::Edit)
                ]
                .align_x(Alignment::Center)
                .spacing(5)
                .padding(5),
            ]
            .spacing(5)
            .padding(20)
            .align_y(Alignment::Center),
            horizontal_rule(1),
        ]);
    }
    entries.into()
}
pub fn absences_list(app: &App) -> Element<Message> {
    let mut entries = column![];
    for entry in &app.db.absences {
        let t = text(entry.lesson.to_string());
        entries = entries.push(column![
            row![
                t.size(20).style(text::success),
                horizontal_space(),
                text(entry.lesson.to_string()).style(|theme: &Theme| text::primary(theme)),
                text(format!(" Erste Lektion: {} ", entry.first_lesson)),
                text(format!("{} Absenzen", crate::db::Class::all().len() - entry.present.len()))
                    .style(|theme: &Theme| text::primary(theme)),
                horizontal_space(),
                column![
                    text(entry.lesson_time.to_string()),
                    text(entry.date.to_string())
                ]
                .padding(3),
                column![
                    button(
                        text(icon_to_string(Bootstrap::TrashthreeFill))
                            .font(ICON_FONT)
                            .size(22)
                            .style(themes::text_fg_danger)
                    )
                    .on_press(Message::Nothing)
                    .style(button::text),
                    //button("Bearbeiten").on_press(Message::Edit)
                ]
                .align_x(Alignment::Center)
                .spacing(5)
                .padding(5),
            ]
            .spacing(5)
            .padding(20)
            .align_y(Alignment::Center),
            horizontal_rule(1),
        ]);
    }
    entries.into()
}
pub fn listview(app: &App) -> Element<Message> {
    let select_kind = pick_list(ListMode::all(), Some(&app.list.kind),|a| Message::List(ListMsg::SetListMode(a)));
    let entries: Element<Message> = match &app.list.kind {
        ListMode::Total => text("Coming soon (tm)").into(),
        ListMode::Delays => delays_list(app),
        ListMode::Absences => absences_list(app),
    };
        row![column![
        //text("Verspätungsmanager4000 Ultra Pro Max").size(20),
        //text(format!("{}", std::env::current_dir().unwrap().display())),
        text(app.status_text.clone()).style(text::danger),
        row![
            button(
                row![
                    text(icon_to_string(Bootstrap::ArrowLeftSquareFill))
                        .font(ICON_FONT)
                        .style(themes::text_fg)
                        .size(22),
                    text("Zurück").style(themes::text_fg).size(20)
                ]
                .spacing(5)
                .align_y(Alignment::Center)
            )
            .on_press(Message::BackView)
            .style(button::text),
            horizontal_space(),
            select_kind,
        ].align_y(Alignment::Center)
        .padding(5)
        .spacing(5),
        horizontal_rule(1),
        scrollable(entries).style(themes::scrollbar_invis),
    ]
    .width(Length::Fill)
    .align_x(alignment::Alignment::Center),]
    .align_y(Alignment::Center)
    .width(Length::Fill)
    .into()
}
