use crate::bootstrap::*;
use crate::db::Class;
use crate::themes::{self, text_fg, text_fg_succes};
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
    detail_person: Option<Class>,
    detail_view: bool,
}

impl StatState {
    pub fn new() -> Self {
        StatState {
            person: Some(Ranking::Number),
            subject: Some(Ranking::Number),
            detail_person: Some(Class::Anina),
            detail_view: false,
        }
    }
}

#[derive(Debug, Clone)]
pub enum StatsMessage {
    PersonSelected(Class),
    PersonSelectedCV(Class),
    PersonRankingType(Ranking),
    SubjectRankingType(Ranking),
    OverView,
}

pub fn update_stats(app: &mut App, msg: StatsMessage) -> Task<Message> {
    match msg {
        StatsMessage::PersonSelected(class) => app.stats.detail_person = Some(class),
        StatsMessage::PersonSelectedCV(class) => {
            app.stats.detail_person = Some(class);
            app.stats.detail_view = true;
        }
        StatsMessage::OverView => app.stats.detail_view = false,
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

pub fn profile_stats(app: &App) -> Element<Message> {
    let person_text = match &app.stats.detail_person {
        None => "Niemandem".to_string(),
        Some(p) => p.to_string(),
    };
    let lates: Element<Message> = match &app.stats.detail_person {
        Some(p) => lates_person(app, p.clone()),
        None => text("Keine Verspätungen").into(),
    };
    let facts: Element<Message> = match &app.stats.detail_person {
        Some(p) => funfacts_person(app, p.clone()),
        None => text("Keine Verspätungen").into(),
    };

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
            .on_press(Message::Stats(StatsMessage::OverView))
            .style(button::text),
            horizontal_space()
        ],
        horizontal_rule(1),
        horizontal_space().height(5),
        column![
            row![
                text(format!("Statistik von {}", person_text))
                    .size(20)
                    .style(text_fg),
                horizontal_space(),
                pick_list(Class::all(), app.stats.detail_person.clone(), |p| {
                    Message::Stats(StatsMessage::PersonSelected(p))
                }),
            ]
            .align_y(Alignment::Center)
            .spacing(5),
            row![lates, facts],
        ]
    ]
    .padding(5)
    .width(Length::Fill)]
    .height(Length::Fill)
    .into()
}

fn funfacts_person(app: &App, person: Class) -> Element<Message> {
    let stats = app.db.get_profile_stats(person);
    let total = row![
        text("Verspätungen").size(18),
        horizontal_space(),
        text(format!("{:.1}", stats.num))
            .size(18)
            .align_y(Alignment::Center)
    ];
    let avg = row![
        text("Durchschnittliche Verspätung").size(18),
        horizontal_space(),
        text(format!("{:.1}min", stats.avg_min)).size(18)
    ]
    .align_y(Alignment::Center);
    let min = row![
        text("Minimum").size(18),
        horizontal_space(),
        text(format!("{:.1}min", stats.min)).size(18)
    ]
    .align_y(Alignment::Center);
    let max = row![
        text("Maximum").size(18),
        horizontal_space(),
        text(format!("{:.1}min", stats.max)).size(18)
    ]
    .align_y(Alignment::Center);
    let sum_min = row![
        text("Summe").size(18),
        horizontal_space(),
        text(format!("{:.1}min", stats.sum)).size(18)
    ]
    .align_y(Alignment::Center);

    let first_percent = row![
        text("Erste Lektion").size(18),
        horizontal_space(),
        text(format!("{:.1}%", stats.first_lesson_percent)).size(18)
    ]
    .align_y(Alignment::Center);
    let penalties = row![
        text("Strafstunden").size(18),
        horizontal_space(),
        text(format!("{:.1}", stats.theo_penalties)).size(18)
    ]
    .align_y(Alignment::Center);
    let percent = row![
        text("Prozent von allen").size(18),
        horizontal_space(),
        text(format!("{:.1}%", stats.percent)).size(18)
    ]
    .align_y(Alignment::Center);
    column![
        row![
            text("Andere Zahlen").style(themes::text_fg_sec).size(20),
            horizontal_space()
        ],
        total,
        percent,
        sum_min,
        avg,
        min,
        max,
        first_percent,
        penalties,
    ]
    .spacing(10)
    .padding(5)
    .width(Length::FillPortion(2))
    .into()
}

fn lates_person(app: &App, person: Class) -> Element<Message> {
    let mut lates = column![row![
        text("Verspätungen").style(themes::text_fg_sec).size(20),
        horizontal_space()
    ]];
    for entry in &app.db.data {
        if entry.person != person {
            continue;
        }
        let t = text(entry.person.to_string());
        lates = lates.push(column![
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
    scrollable(lates)
        .style(themes::scrollbar_invis)
        .width(Length::FillPortion(3))
        .into()
}

pub fn stats_view(app: &App) -> Element<Message> {
    if !app.stats.detail_view {
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
                funfacts(app),
            ]
            .spacing(5)
            .align_y(Alignment::Start)
        ]
        .padding(5)
        .width(Length::Fill)
        .align_x(Alignment::Center)]
        .height(Length::Fill)
        .into()
    } else {
        profile_stats(app)
    }
}

fn funfacts(app: &App) -> Element<Message> {
    let avg = text(format!("Durchschnitt: {}", app.db.average_delay()));
    let min = text(format!(
        "Minimum: {}",
        app.db.data.iter().map(|x| x.delay_min).min().unwrap()
    ));
    let sum_min = text(format!("Summe: {}min", app.db.sum_min()));
    let max = text(format!(
        "Maximum: {}",
        app.db.data.iter().map(|x| x.delay_min).max().unwrap()
    ));
    let total = text(format!("Total: {}", app.db.data.len()));
    let first_percent = text(format!(
        "Erste Lektion des Tages: {}%",
        app.db.get_percent_first_lesson()
    ));
    let penalties = text(format!(
        "Theoretische Strafstunden: {}",
        app.db.total_penalties()
    ));
    column![
        row![
            text("Andere Zahlen").style(themes::text_fg).size(22),
            horizontal_space()
        ],
        avg,
        min,
        max,
        total,
        first_percent,
        sum_min,
        penalties
    ]
    .spacing(10)
    .padding(5)
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
            text("Ranking nach Fach").size(22).style(themes::text_fg),
            horizontal_space(),
            ranking_subject
        ]
        .spacing(5)
        .align_y(Alignment::Center),
        scrollable(ranking)
            .style(themes::scrollbar_invis)
            .style(scrollable::default)
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
                button(
                    text(icon_to_string(Bootstrap::PersonLinesFill))
                        .style(text_fg_succes)
                        .font(ICON_FONT)
                )
                .on_press(Message::Stats(StatsMessage::PersonSelectedCV(p.0)))
                .style(button::text),
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
            .align_y(Alignment::Center)
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
            text("Ranking Nach Person").size(22).style(themes::text_fg),
            horizontal_space(),
            ranking_person
        ]
        .spacing(5)
        .align_y(Alignment::Center),
        scrollable(ranking).style(themes::scrollbar_invis)
    ])
    .into()
}
