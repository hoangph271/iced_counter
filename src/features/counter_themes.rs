use std::fmt::Display;

use iced::Theme;

use crate::omni_app::OmniApp;

pub const DEFAULT: &str = "Default";
pub const GRUVBOX: &str = "Gruvbox";
pub const SOLARIZED: &str = "Solarized";

pub const ALL_THEMES: [&str; 3] = [DEFAULT, GRUVBOX, SOLARIZED];

#[derive(Debug, PartialEq, Clone)]
pub enum ThemeMode {
    SystemDefault,
    Dark,
    Light,
}

impl Display for ThemeMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                ThemeMode::SystemDefault => "System default",
                ThemeMode::Dark => "Dark theme",
                ThemeMode::Light => "Light theme",
            }
        )
    }
}

pub const ALL_THEME_MODES: [ThemeMode; 3] =
    [ThemeMode::SystemDefault, ThemeMode::Dark, ThemeMode::Light];

pub fn get_system_theme_mode() -> ThemeMode {
    match dark_light::detect() {
        Ok(dark_light::Mode::Dark) => ThemeMode::Dark,
        Ok(dark_light::Mode::Light) => ThemeMode::Light,
        Ok(dark_light::Mode::Unspecified) => ThemeMode::SystemDefault,
        Err(e) => {
            eprintln!("{e}");
            ThemeMode::SystemDefault
        }
    }
}

pub fn theme_from_state(state: &OmniApp) -> Theme {
    let OmniApp {
        theme_name,
        application_theme_mode: theme_mode,
        system_theme_mode,
        ..
    } = state;

    let theme_mode = match theme_mode {
        ThemeMode::SystemDefault => system_theme_mode,
        ThemeMode::Dark => &ThemeMode::Dark,
        ThemeMode::Light => &ThemeMode::Light,
    };

    match (theme_name.as_ref(), theme_mode) {
        (GRUVBOX, ThemeMode::Dark) => Theme::GruvboxDark,
        (GRUVBOX, ThemeMode::Light) => Theme::GruvboxLight,
        (SOLARIZED, ThemeMode::Dark) => Theme::SolarizedDark,
        (SOLARIZED, ThemeMode::Light) => Theme::SolarizedLight,
        (_, ThemeMode::Dark) => Theme::Dark,
        (_, ThemeMode::Light) => Theme::Light,
        (_, ThemeMode::SystemDefault) => Theme::GruvboxLight,
    }
}

// ? dark_light::subscribe is removed on dark-light@2.0.0, see https://github.com/rust-dark-light/dark-light/pull/60
// fn create_theme_mode_stream() -> impl Stream<Item = CounterMessage> {
//     stream::once(dark_light::subscribe()).flat_map(|it| {
//         if let Ok(stream) = it {
//             stream
//                 .map(|theme_mode| match theme_mode {
//                     dark_light::Mode::Dark => {
//                         CounterMessage::ChangeSystemThemeMode(ThemeMode::Dark)
//                     }
//                     dark_light::Mode::Light => {
//                         CounterMessage::ChangeSystemThemeMode(ThemeMode::Light)
//                     }
//                     dark_light::Mode::Default => {
//                         CounterMessage::ChangeSystemThemeMode(ThemeMode::SystemDefault)
//                     }
//                 })
//                 .left_stream()
//         } else {
//             stream::once(async { CounterMessage::NoOp }).right_stream()
//         }
//     })
// }
