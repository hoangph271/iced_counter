use std::fmt::Display;

use iced::Theme;

pub const DEFAULT: &str = "Default";
pub const GRUVBOX: &str = "Gruvbox";
pub const SOLARIZED: &str = "Solarized";

pub const ALL_THEMES: [&str; 3] = [DEFAULT, GRUVBOX, SOLARIZED];

#[derive(Debug, PartialEq, Clone)]
pub enum ThemeMode {
    SystemDefault,
    DarkTheme,
    LightTheme,
}

impl Display for ThemeMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                ThemeMode::SystemDefault => "System default",
                ThemeMode::DarkTheme => "Dark theme",
                ThemeMode::LightTheme => "Light theme",
            }
        )
    }
}

pub const ALL_THEME_MODES: [ThemeMode; 3] = [
    ThemeMode::SystemDefault,
    ThemeMode::DarkTheme,
    ThemeMode::LightTheme,
];

pub fn theme_from(theme_name: &str, theme_mode: &ThemeMode) -> Theme {
    let theme_mode = match theme_mode {
        ThemeMode::SystemDefault => match dark_light::detect() {
            dark_light::Mode::Dark => &ThemeMode::DarkTheme,
            dark_light::Mode::Light | dark_light::Mode::Default => &ThemeMode::LightTheme,
        },
        ThemeMode::DarkTheme => &ThemeMode::DarkTheme,
        ThemeMode::LightTheme => &ThemeMode::LightTheme,
    };

    match (theme_name, theme_mode) {
        (GRUVBOX, ThemeMode::DarkTheme) => Theme::GruvboxDark,
        (GRUVBOX, ThemeMode::LightTheme) => Theme::GruvboxLight,
        (SOLARIZED, ThemeMode::DarkTheme) => Theme::SolarizedDark,
        (SOLARIZED, ThemeMode::LightTheme) => Theme::SolarizedLight,
        (_, ThemeMode::DarkTheme) => Theme::Dark,
        (_, ThemeMode::LightTheme) => Theme::Light,
        (_, ThemeMode::SystemDefault) => panic!("ThemeMode should NOT be SystemDefault by now"),
    }
}
