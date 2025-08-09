use crate::bootstrap::*;
use crate::themes;
use crate::themes::styled_button;
use crate::themes::ColorType;
use crate::{db, App, Message, ViewControl};
use chrono::prelude::*;
use iced::event::{self, Event};
use iced::widget::{
    button, checkbox, column, combo_box, container, horizontal_rule, horizontal_space, pick_list,
    row, scrollable, slider, stack, text, text_input, toggler, tooltip, vertical_rule,
    vertical_space,
};
use std::fs;
use std::path::Path;

use iced::{Alignment, Element, Font, Length, Padding, Subscription, Task, Theme};
use serde_derive::*;

pub fn settings_view(app: &App) -> Element<Message> {
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
            row![
                text("Theme:"),
                combo_box(
                    &app.theme_state,
                    "No theme selected",
                    app.selected_theme.as_ref(),
                    Message::ThemeSelected
                ),
            ]
            .spacing(10)
            .align_y(Alignment::Center),
            text(format!(
                "Working dir:  {}",
                std::env::current_dir().unwrap().display()
            )),
            styled_button(
                Bootstrap::FileEarmarkArrowDownFill,
                "Neues Backup",
                Message::BackupDB,
                ColorType::Succes
            ),
            styled_button(
                Bootstrap::DatabaseFillExclamation,
                "Verspätungen Zurücksetzen",
                Message::DeleteDB,
                ColorType::Danger
            ),
            styled_button(
                Bootstrap::DatabaseFillCheck,
                "DB und Einstellungen Speichern",
                Message::SaveDB,
                ColorType::Succes,
            )
        ]
        .spacing(5)
        .padding(20),
    )
    .into()
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Settings {
    pub theme: String,
}

impl Settings {
    pub fn new() -> Self {
        Settings {
            theme: "Dark".to_string(),
        }
    }
}

pub async fn load_from_file(path: &str) -> Settings {
    if !Path::new(path).exists() {
        return Settings::new();
    }
    let filecontent = fs::read_to_string(path).expect("Couldn't read file");
    let settings: Settings = serde_json::from_str(&filecontent).expect("Couldnt parse file");
    settings
}

pub async fn save_to_file(settings: Settings, path: &str) {
    let content = serde_json::to_string_pretty(&settings).unwrap();
    let _ = fs::write(path, content);
}

pub fn string_to_theme(theme_str: &str) -> Option<Theme> {
    for theme_type in Theme::ALL {
        if theme_type.to_string() == theme_str {
            return Some(theme_type.clone());
        }
    }
    None
}
