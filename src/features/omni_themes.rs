use iced::{theme, widget, Alignment, Element, Length, Subscription, Task, Theme};
use std::fmt::Display;

pub const DEFAULT: &str = "Default";
pub const GRUVBOX: &str = "Gruvbox";
pub const SOLARIZED: &str = "Solarized";

pub const ALL_THEMES: [&str; 3] = [DEFAULT, GRUVBOX, SOLARIZED];

#[derive(Debug, PartialEq, Clone)]
pub enum OmniThemeMode {
    SystemDefault,
    Dark,
    Light,
}

impl Display for OmniThemeMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                OmniThemeMode::SystemDefault => "System default",
                OmniThemeMode::Dark => "Dark theme",
                OmniThemeMode::Light => "Light theme",
            }
        )
    }
}

#[derive(Debug, Clone)]
pub(crate) enum OmniThemesMessage {
    ChangeThemeMode(OmniThemeMode),
    ChangeSystemThemeMode(theme::Mode),
    SwitchTheme(String),
    NoOp,
}

#[derive(Debug)]
pub(crate) struct OmniThemes {
    pub application_theme_mode: OmniThemeMode,
    pub system_theme_mode: theme::Mode,
    pub theme_name: String,
}

impl OmniThemes {
    pub fn theme_from_state(state: &Self) -> Theme {
        let OmniThemes {
            theme_name,
            application_theme_mode: theme_mode,
            system_theme_mode,
            ..
        } = state;

        let theme_mode = match theme_mode {
            OmniThemeMode::SystemDefault => match system_theme_mode {
                theme::Mode::Light => &OmniThemeMode::Light,
                theme::Mode::Dark => &OmniThemeMode::Dark,
                theme::Mode::None => &OmniThemeMode::SystemDefault,
            },
            OmniThemeMode::Dark => &OmniThemeMode::Dark,
            OmniThemeMode::Light => &OmniThemeMode::Light,
        };

        match (theme_name.as_ref(), theme_mode) {
            (GRUVBOX, OmniThemeMode::Dark) => Theme::GruvboxDark,
            (GRUVBOX, OmniThemeMode::Light) => Theme::GruvboxLight,
            (SOLARIZED, OmniThemeMode::Dark) => Theme::SolarizedDark,
            (SOLARIZED, OmniThemeMode::Light) => Theme::SolarizedLight,
            (_, OmniThemeMode::Dark) => Theme::Dark,
            (_, OmniThemeMode::Light) => Theme::Light,
            (_, OmniThemeMode::SystemDefault) => Theme::GruvboxLight,
        }
    }

    pub(crate) fn init() -> OmniThemes {
        Self {
            application_theme_mode: OmniThemeMode::SystemDefault,
            system_theme_mode: theme::Mode::None,
            theme_name: GRUVBOX.to_owned(),
        }
    }

    pub(crate) fn update(&mut self, message: OmniThemesMessage) -> Task<OmniThemesMessage> {
        match message {
            OmniThemesMessage::ChangeThemeMode(theme_mode) => {
                self.application_theme_mode = theme_mode;
            }
            OmniThemesMessage::ChangeSystemThemeMode(theme_mode) => {
                self.system_theme_mode = theme_mode;
            }
            OmniThemesMessage::SwitchTheme(theme_name) => self.theme_name = theme_name,
            OmniThemesMessage::NoOp => {}
        };

        Task::none()
    }

    pub(crate) fn view(&self) -> Element<'_, OmniThemesMessage> {
        widget::row![
            widget::PickList::new(
                ALL_THEME_MODES,
                Some(&self.application_theme_mode),
                OmniThemesMessage::ChangeThemeMode
            )
            .width(Length::Shrink),
            widget::PickList::new(ALL_THEMES, Some(self.theme_name.as_str()), |theme_name| {
                match theme_name {
                    DEFAULT | GRUVBOX | SOLARIZED => {
                        OmniThemesMessage::SwitchTheme(theme_name.to_string())
                    }
                    _ => OmniThemesMessage::NoOp,
                }
            }),
        ]
        .spacing(16)
        .align_y(Alignment::Center)
        .into()
    }

    pub(crate) fn start_up_tasks(&self) -> Task<OmniThemesMessage> {
        iced::system::theme().map(OmniThemesMessage::ChangeSystemThemeMode)
    }

    pub(crate) fn subscription(&self) -> Subscription<OmniThemesMessage> {
        iced::system::theme_changes().map(OmniThemesMessage::ChangeSystemThemeMode)
    }
}

pub const ALL_THEME_MODES: [OmniThemeMode; 3] = [
    OmniThemeMode::SystemDefault,
    OmniThemeMode::Dark,
    OmniThemeMode::Light,
];
