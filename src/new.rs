use crate::bootstrap::*;
use crate::db::Class;
use crate::themes::*;
use crate::ICON_FONT;
use crate::{App, Message, ViewControl};
use chrono::Datelike;
use iced::widget::{
    button, checkbox, column, combo_box, container, horizontal_rule, horizontal_space, pick_list,
    row, scrollable, slider, stack, text, text_input, toggler, tooltip, vertical_rule,
    vertical_space,
};
use iced::{alignment, Alignment, Element, Font, Length, Padding, Subscription, Task, Theme};
use serde_derive::{Deserialize, Serialize};

pub fn new_entry_view(app: &App) -> Element<Message> {
    let s = "Erste Lektion".to_string();
    row![
        horizontal_space(),
        column![
            vertical_space(),
            text("Neue Verspätung").size(30).style(text_fg_sec),
            horizontal_rule(1),
            combo_box(
                &app.combo,
                "Person",
                app.sel_pers.as_ref(),
                Message::SelectPerson
            )
            .padding(5),
            combo_box(
                &app.combo2,
                "Fach",
                app.sel_lesson.as_ref(),
                Message::SelectLesson
            )
            .padding(5),
            row![
                text(format!("Verspätung: {}", app.add_entry.delay_min)).size(20),
                slider(
                    std::ops::RangeInclusive::new(1, 45),
                    app.add_entry.delay_min,
                    Message::DelayE
                )
            ]
            .spacing(5)
            .padding(5)
            .align_y(Alignment::Center),
            row![
                text("Lektion").size(20),
                button(
                    text(icon_to_string(Bootstrap::DashCircleFill))
                        .size(22)
                        .font(ICON_FONT)
                        .style(text_fg)
                )
                .on_press(Message::LastLessonTime)
                .style(button::text),
                text(app.add_entry.lesson_time.to_string())
                    .size(20)
                    .style(text_fg),
                button(
                    text(icon_to_string(Bootstrap::PlusCircleFill))
                        .size(22)
                        .font(ICON_FONT)
                        .style(text_fg)
                )
                .on_press(Message::NextLessonTime)
                .style(button::text),
            ]
            .spacing(2)
            .align_y(Alignment::Center),
            row![
                button(
                    text(icon_to_string(Bootstrap::DashCircleFill))
                        .size(22)
                        .font(ICON_FONT)
                        .style(text_fg)
                )
                .on_press(Message::RemDay)
                .style(button::text),
                text(format!("{}, ", app.add_entry.date))
                    .size(20)
                    .style(text_fg),
                text(format!("{}", app.add_entry.date.weekday()))
                    .size(20)
                    .style(text_fg),
                button(
                    text(icon_to_string(Bootstrap::PlusCircleFill))
                        .size(22)
                        .font(ICON_FONT)
                        .style(text_fg)
                )
                .on_press(Message::AddDay)
                .style(button::text),
            ]
            .spacing(1)
            .align_y(Alignment::Center),
            toggler(app.add_entry.first_lesson)
                .on_toggle(Message::IsFirst)
                .label(s),
            row![
                styled_menu_button(
                    Bootstrap::CheckSquareFill,
                    "Hinzufügen",
                    Message::AddEntry,
                    ColorType::Succes
                ),
                styled_menu_button(
                    Bootstrap::XSquareFill,
                    "Kündigung",
                    Message::BackView,
                    ColorType::Danger
                ),
            ]
            .align_y(Alignment::Center),
            vertical_space(),
        ]
        .width(Length::Fill)
        .align_x(Alignment::Center)
        .spacing(5),
        horizontal_space(),
    ]
    .into()
}
