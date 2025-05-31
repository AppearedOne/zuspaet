use crate::bootstrap::*;
use crate::db::Class;
use crate::themes;
use crate::{App, Message, ViewControl};
use iced::widget::{
    button, checkbox, column, combo_box, container, horizontal_rule, horizontal_space, pick_list,
    row, scrollable, slider, stack, text, text_input, toggler, tooltip, vertical_rule,
    vertical_space,
};
use iced::{alignment, Alignment, Element, Font, Length, Padding, Subscription, Task, Theme};
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct StatState {
    person: Option<Ranking>,
    subject: Option<Ranking>,
    detailPerson: Option<Class>,
}

impl StatState {
    pub fn new() -> Self {
        StatState {
            person: Some(Ranking::Number),
            subject: Some(Ranking::Number),
            detailPerson: Some(Class::Anina),
        }
    }
}

#[derive(Debug, Clone)]
pub enum StatsMessage {
    PersonSelected(Class),
    PersonRankingType(Ranking),
    SubjectRankingType(Ranking),
}

pub fn update_stats(app: &mut App, msg: StatsMessage) -> Task<Message> {
    match msg {
        StatsMessage::PersonSelected(class) => (),
        StatsMessage::PersonRankingType(ranking) => app.stats.person = Some(ranking),
        StatsMessage::SubjectRankingType(ranking) => app.stats.subject = Some(ranking),
    }
    Task::none()
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Ranking {
    Number,
    Sum,
}

impl Ranking {
    pub fn all() -> Vec<Self> {
        vec![Ranking::Number, Ranking::Sum]
    }
}

impl std::fmt::Display for Ranking {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Ranking::Number => write!(f, "Anzahl"),
            Ranking::Sum => write!(f, "Minuten"),
        }
    }
}

pub fn stats_view(app: &App) -> Element<Message> {
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
    let first_percent = text(format!(
        "Erste Lektion des Tages: {}%",
        app.db.get_percent_first_lesson()
    ));
    row![column![
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
            .on_press(Message::GoView(ViewControl::MAIN))
            .style(button::text),
            horizontal_space()
        ],
        horizontal_rule(1),
        horizontal_space().height(5),
        row![
            ranking_person(app),
            //vertical_rule(1),
            ranking_lesson(app),
            //vertical_rule(1),
            column![avg, min, max, total, first_percent, sum_min].spacing(10)
        ]
        .spacing(5)
        .align_y(Alignment::Start)
    ]
    .padding(5)
    .width(Length::Fill)
    .align_x(Alignment::Center)]
    .height(Length::Fill)
    .into()
}

fn ranking_lesson(app: &App) -> Element<Message> {
    let mut ranking = column![];
    for (i, p) in app
        .db
        .ranking_vec_lesson(app.stats.subject.clone())
        .into_iter()
        .enumerate()
    {
        ranking = ranking.push(
            row![
                text(format!("{}.", i + 1)).size(20),
                text(p.0.to_string()).size(20).style(themes::text_fg_succes),
                horizontal_space(),
                text(format!("{}min", p.2.to_string()))
                    .size(18)
                    .style(themes::text_fg_sec),
                text(format!("{}%", p.3.to_string()))
                    .size(18)
                    .style(themes::text_fg_sec),
                horizontal_space(),
                text(p.1.to_string()).size(18),
                horizontal_space().width(5),
            ]
            .spacing(10)
            .align_y(Alignment::Start)
            .padding(5),
        );
    }
    let ranking_subject: Element<Message> =
        pick_list(Ranking::all(), app.stats.subject.clone(), |x| {
            Message::Stats(StatsMessage::SubjectRankingType(x))
        })
        .placeholder("None")
        .into();
    container(column![
        row![
            text("Nach Fach").size(22).style(themes::text_fg),
            horizontal_space(),
            ranking_subject
        ]
        .spacing(5)
        .align_y(Alignment::Center),
        scrollable(ranking).style(themes::scrollbar_invis)
    ])
    .into()
}

fn ranking_person(app: &App) -> Element<Message> {
    let mut ranking = column![];
    for (i, p) in app
        .db
        .ranking_vec(app.stats.person.clone())
        .into_iter()
        .enumerate()
    {
        ranking = ranking.push(
            row![
                text(format!("{}.", i + 1)).size(20),
                text(p.0.to_string()).size(20).style(themes::text_fg_succes),
                horizontal_space(),
                text(format!("{}min", p.2.to_string()))
                    .size(18)
                    .style(themes::text_fg_sec),
                text(format!("{}%", p.3.to_string()))
                    .size(18)
                    .style(themes::text_fg_sec),
                horizontal_space(),
                text(p.1.to_string()).size(18),
                horizontal_space().width(5),
            ]
            .spacing(10)
            .align_y(Alignment::Start)
            .padding(5),
        );
    }
    let ranking_person: Element<Message> =
        pick_list(Ranking::all(), app.stats.person.clone(), |x| {
            Message::Stats(StatsMessage::PersonRankingType(x))
        })
        .placeholder("None")
        .into();
    container(column![
        row![
            text("Nach Person").size(22).style(themes::text_fg),
            horizontal_space(),
            ranking_person
        ]
        .spacing(5)
        .align_y(Alignment::Center),
        scrollable(ranking).style(themes::scrollbar_invis)
    ])
    .into()
}
