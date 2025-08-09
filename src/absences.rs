use crate::bootstrap::ICON_FONT;
use crate::db::{Class, Lesson, LessonAbs};
use crate::themes::{self, styled_menu_button, text_fg, text_fg_succes, ColorType};
use crate::time;
use crate::toast::Toast;
use crate::{absences, bootstrap::*};
use crate::{App, Message, ViewControl};
use chrono::Datelike;
use iced::widget::{
    button, checkbox, column, combo_box, container, horizontal_rule, horizontal_space, mouse_area,
    pick_list, row, scrollable, slider, stack, text, text_input, toggler, tooltip, vertical_rule,
    vertical_space,
};
use iced::{alignment, Alignment, Font, Length, Padding, Subscription, Task, Theme};
use iced::{Element, Function};

async fn nothing() {}

#[derive(Debug, Clone)]
pub enum AbsMsg {
    SelectClass(Lesson),
    NextTime,
    LastTime,
    NextDate,
    LastDate,
    Add,
    TogglePerson(Class),
    FirstLessonToggle,
    SmartNew,
}

pub fn handle_absences(msg: AbsMsg, app: &mut App) -> Task<Message> {
    match msg {
        AbsMsg::SelectClass(lesson) => app.abs.lesson = lesson,
        AbsMsg::NextTime => app.abs.lesson_time = time::get_next_lesson(app.abs.lesson_time),
        AbsMsg::LastTime => app.abs.lesson_time = time::get_prev_lesson(app.abs.lesson_time),
        AbsMsg::NextDate => {
            app.abs.date = app.abs.date.succ_opt().expect("Theres no tommorow?");
        }
        AbsMsg::LastDate => {
            app.abs.date = app.abs.date.pred_opt().expect("There was no yesterday?")
        }
        AbsMsg::Add => {
            app.db.absences.push(app.abs.clone());
            app.notify(Toast::new("Absenz erfasst", "idk was da anemuen? Treffen sich zwei jäger", crate::toast::Status::Success));
            return Task::perform(nothing(), |_| Message::BackView);
        },
        AbsMsg::TogglePerson(p) => app.abs.toggle_person(p),
        AbsMsg::FirstLessonToggle => app.abs.first_lesson = !app.abs.first_lesson,
        AbsMsg::SmartNew => app.abs = LessonAbs::new_smart(&app.db.absences),
    }
    Task::none()
}

pub fn absences_view(app: &App) -> Element<Message> {
    let mut grid = column![].padding(10).spacing(5);

    for (i, absence) in app.db.absences.iter().enumerate() {
        grid = grid.push(
            container(row![text(absence.lesson.to_string())].padding(10)).style(
                move |a: &Theme| {
                    if i % 2 == 0 {
                        container::primary(a)
                    } else {
                        container::dark(a)
                    }
                },
            ),
        );
    }

    let mut person_grid = column![];
    for (i, p) in Class::all().into_iter().enumerate() {
        let p_c = p.clone();
        let is_checked = app.abs.present.contains(&p);
        person_grid = person_grid.push(
            mouse_area(
                container(
                    row![
                        text(p.to_string())
                            .style(match is_checked {
                                true => text::success,
                                false => text::danger,
                            })
                            .size(18),
                        horizontal_space(),
                        toggler(is_checked)
                            .on_toggle(move |_| Message::Abs(AbsMsg::TogglePerson(p.clone())))
                    ]
                    .align_y(Alignment::Center)
                    .padding(10),
                )
                .style(move |a| {
                    if i % 2 == 0 {
                        container::transparent(a)
                    } else {
                        container::secondary(a)
                    }
                }),
            )
            .on_press(Message::Abs(AbsMsg::TogglePerson(p_c))),
        )
    }

    let date_picker = container(
        row![
            button(
                text(icon_to_string(Bootstrap::DashCircleFill))
                    .font(ICON_FONT)
                    .style(text_fg)
            )
            .on_press(Message::Abs(AbsMsg::LastDate))
            .style(button::text),
            text(format!("{}, ", app.abs.date)).style(text_fg),
            text(format!("{}", app.abs.date.weekday())).style(text_fg),
            button(
                text(icon_to_string(Bootstrap::PlusCircleFill))
                    .font(ICON_FONT)
                    .style(text_fg)
            )
            .on_press(Message::Abs(AbsMsg::NextDate))
            .style(button::text),
        ]
        .spacing(1)
        .align_y(Alignment::Center),
    )
    .style(container::bordered_box);

    let time_picker = container(
        row![
            button(
                text(icon_to_string(Bootstrap::DashCircleFill))
                    .font(ICON_FONT)
                    .style(text_fg)
            )
            .on_press(Message::Abs(AbsMsg::LastTime))
            .style(button::text),
            text(app.abs.lesson_time.to_string()).style(text_fg),
            button(
                text(icon_to_string(Bootstrap::PlusCircleFill))
                    .font(ICON_FONT)
                    .style(text_fg)
            )
            .on_press(Message::Abs(AbsMsg::NextTime))
            .style(button::text),
        ]
        .spacing(2)
        .align_y(Alignment::Center),
    )
    .style(container::bordered_box);

    let fl = toggler(app.abs.first_lesson)
        .on_toggle(|_| Message::Abs(AbsMsg::FirstLessonToggle))
        .label("Erste Lektion".to_string());

    let reload_b = button(text(icon_to_string(Bootstrap::ArrowClockwise)).font(ICON_FONT))
        .on_press(Message::Abs(AbsMsg::SmartNew))
        .style(button::secondary);

    let add = styled_menu_button(
        Bootstrap::CheckSquareFill,
        "Hinzufügen",
        Message::Abs(AbsMsg::Add),
        ColorType::Succes,
    );

    let add_abs = container(column![
        row![
            reload_b,
            pick_list(Lesson::all(), Some(app.abs.lesson.clone()), |l| {
                Message::Abs(AbsMsg::SelectClass(l))
            }),
            date_picker,
            time_picker,
            fl,
            horizontal_space(),
            add
        ]
        .padding(5)
        .spacing(10)
        .align_y(Alignment::Center),
        person_grid,
    ]);

    container(
        column![
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
                horizontal_space()
            ],
            horizontal_rule(1),
            vertical_space().height(5),
            scrollable(add_abs).style(themes::scrollbar_invis),
            scrollable(grid),
        ]
        .spacing(5),
    )
    .into()
}
