use iced::Theme;

pub const DEFAULT: &str = "Default";
pub const GRUVBOX: &str = "Gruvbox";
pub const SOLARIZED: &str = "Solarized";

pub const ALL_THEMES: [&str; 3] = [DEFAULT, GRUVBOX, SOLARIZED];

pub fn theme_from(theme_name: &str, dark_theme: &bool) -> Theme {
    match (theme_name, dark_theme) {
        (GRUVBOX, true) => Theme::GruvboxDark,
        (GRUVBOX, false) => Theme::GruvboxLight,
        (SOLARIZED, true) => Theme::SolarizedDark,
        (SOLARIZED, false) => Theme::SolarizedLight,
        (_, true) => Theme::Dark,
        (_, false) => Theme::Light,
    }
}
