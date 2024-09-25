use crate::{db, App, Message, ViewControl};
use iced::event::{self, Event};
use iced::widget::{
    button, checkbox, column, combo_box, container, horizontal_rule, horizontal_space, pick_list,
    row, scrollable, slider, stack, text, text_input, toggler, tooltip, vertical_rule,
    vertical_space,
};
use iced::{alignment, Alignment, Element, Font, Length, Padding, Subscription, Task, Theme};

pub fn stats_view(app: &App) -> Element<Message> {
    let mut ranking =
        column![text("Verspätungen pro Person").style(text::danger)].align_x(Alignment::Start);
    for (i, p) in app.db.ranking_vec().into_iter().enumerate() {
        ranking = ranking.push(
            row![
                text(format!("{}.", i + 1)),
                text(p.0.to_string()),
                text(p.1.to_string())
            ]
            .spacing(5)
            .align_y(Alignment::Start),
        );
    }
    let mut ranking_l =
        column![text("Verspätungen pro Lektion").style(text::danger)].align_x(Alignment::Start);
    for (i, p) in app.db.ranking_vec_lesson().into_iter().enumerate() {
        ranking_l = ranking_l.push(
            row![
                text(format!("{}.", i + 1)).style(text::success),
                text(p.0.to_string()).style(text::success),
                text(p.1.to_string()).style(text::success)
            ]
            .spacing(5)
            .align_y(Alignment::Start),
        );
    }
    row![column![
        button("Zurück").on_press(Message::GoView(ViewControl::MAIN)),
        row![
            ranking.align_x(Alignment::Start),
            ranking_l.align_x(Alignment::Start)
        ]
        .spacing(10)
        .align_y(Alignment::Center)
    ]
    .width(Length::Fill)
    .align_x(Alignment::Center)]
    .height(Length::Fill)
    .into()
}
