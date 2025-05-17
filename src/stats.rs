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
        let percent: f32 = (p.1 as f32 / app.db.data.len() as f32 * 100.0).round();
        ranking = ranking.push(
            row![
                text(format!("{}.", i + 1)),
                text(p.0.to_string()),
                text(p.1.to_string()),
                text(format!("- {}%", percent))
            ]
            .spacing(5)
            .align_y(Alignment::Start),
        );
    }
    let mut ranking_l =
        column![text("Verspätungen pro Lektion").style(text::danger)].align_x(Alignment::Start);
    for (i, p) in app.db.ranking_vec_lesson().into_iter().enumerate() {
        let percent: f32 = (p.1 as f32 / app.db.data.len() as f32 * 100.0).round();
        ranking_l = ranking_l.push(
            row![
                text(format!("{}.", i + 1)).style(text::success),
                text(p.0.to_string()).style(text::success),
                text(p.1.to_string()).style(text::success),
                text(format!(" {}%", percent))
            ]
            .spacing(5)
            .align_y(Alignment::Start),
        );
    }
    let avg = text(format!("Durchschnitt: {}", app.db.average_delay()));
    let min = text(format!(
        "Minimum: {}",
        app.db.data.iter().map(|x| x.delay_min).min().unwrap()
    ));
    let sum_min = text(format!("Summe (min): {}", app.db.sum_min()));
    let max = text(format!(
        "Maximum: {}",
        app.db.data.iter().map(|x| x.delay_min).max().unwrap()
    ));
    let total = text(format!("Total: {}", app.db.data.len()));
    let first_percent = (text(format!(
        "Erste Lektion des Tages: {}%",
        app.db.get_percent_first_lesson()
    )));
    row![column![
        button("Zurück").on_press(Message::GoView(ViewControl::MAIN)),
        row![
            ranking.align_x(Alignment::Start).spacing(5),
            ranking_l.align_x(Alignment::Start).spacing(5),
            column![avg, min, max, total, first_percent, sum_min].spacing(10)
        ]
        .spacing(30)
        .align_y(Alignment::Start)
    ]
    .padding(5)
    .width(Length::Fill)
    .align_x(Alignment::Center)]
    .height(Length::Fill)
    .into()
}
