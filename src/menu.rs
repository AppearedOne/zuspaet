use crate::bootstrap::ICON_FONT;
use crate::bootstrap::*;
use crate::db::Class;
use crate::themes::{self, text_fg, text_fg_succes};
use crate::{App, Message, ViewControl};
use iced::widget::text::Alignment;
use iced::widget::{
    button, checkbox, column, combo_box, container, horizontal_rule, horizontal_space, pick_list,
    row, scrollable, slider, stack, text, text_input, toggler, tooltip, vertical_rule,
    vertical_space, mouse_area
};
use iced::{alignment, Font, Length, Padding, Subscription, Task, Theme};
use iced::{Element, Function};

#[derive(Debug, Clone)]
enum MenuStyle {
    Danger,
    Default,
}

pub struct MenuState {
    title: String 
}

impl MenuState {
    pub fn new() -> Self {
        MenuState { title: "".to_string() }
    }
}
impl Default for MenuState {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug)]
pub enum MenuMsg {
    SetText(String),
}

fn main_menu_button<'a>(msg: Message, icon: Bootstrap, style: MenuStyle, info: &str) -> Element<'a, Message> {
    mouse_area(button(
        text(icon_to_string(icon))
            .align_y(alignment::Vertical::Center)
            .align_x(alignment::Horizontal::Center)
            .font(ICON_FONT)
            .style(match style {
                MenuStyle::Danger => text::danger,
                MenuStyle::Default => text::primary,
            })
            .size(30),
    )
    .on_press(msg)
    .style(themes::round_button_border)).on_exit(Message::MainMenu(MenuMsg::SetText("".to_string()))).on_enter(Message::MainMenu(MenuMsg::SetText(info.to_string())))
    .into()
}

pub fn menu_view(app: &App) -> Element<Message> {
    let list = main_menu_button(
        Message::GoView(ViewControl::LISTVIEW),
        Bootstrap::ListTask,
        MenuStyle::Default,
        "Verspätungen und Absenzenliste"
    );

    let stats = main_menu_button(
        Message::GoView(ViewControl::STATS),
        Bootstrap::BarChartFill,
        MenuStyle::Default,
        "Statistiken"
    );

    let new = main_menu_button(
        Message::GoView(ViewControl::ADD),
        Bootstrap::Plus,
        MenuStyle::Default,
        "Neue Verspätung"
    );

    let settings = main_menu_button(
        Message::GoView(ViewControl::SETTINGS),
        Bootstrap::GearFill,
        MenuStyle::Default,
        "Einstellungen"
    );

    let absences = main_menu_button(
        Message::GoView(ViewControl::ABSENCES),
        Bootstrap::ClipboardPlusFill,
        MenuStyle::Default,
        "Absenzen erfassen"
    );

    let exit = button(
        text(icon_to_string(Bootstrap::BoxArrowRight))
            .align_y(alignment::Vertical::Center)
            .align_x(alignment::Horizontal::Center)
            .font(ICON_FONT)
            .style(text::danger)
            .size(30),
    )
    .on_press(Message::SaveExit)
    .style(|a, b| {
        let mut t = themes::round_button_border(a, b);
        t.border.color = a.extended_palette().danger.base.color;
        t.shadow.color = a.extended_palette().danger.strong.color;
        match b {
            button::Status::Active => (),
            button::Status::Hovered => t.border.color = a.extended_palette().danger.strong.color,
            button::Status::Pressed => t.border.color = a.extended_palette().danger.strong.color,
            button::Status::Disabled => (),
        }

        t
    });

    column![
        vertical_space(),
        text("Verspätungsmanager 4002").size(30).style(text::primary),
        row![new, absences, list, stats, settings, exit]
            .spacing(10)
            .padding(5),
        text(&app.menu.title),
        vertical_space(),
    ]
    .spacing(10)
    .width(Length::Fill)
    .align_x(Alignment::Center)
    .into()
}

pub fn update_menu(app: &mut App, msg: MenuMsg) -> Task<Message> {
    match msg {
        MenuMsg::SetText(s) => app.menu.title = s,
    }
    Task::none()
}
