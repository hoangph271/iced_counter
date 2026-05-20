use iced::{Alignment, Element, Length, Subscription, Task, Theme, border, theme, widget};
#[cfg(feature = "config")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "config", derive(Serialize, Deserialize))]
#[derive(Debug, PartialEq, Clone, Hash)]
pub enum OmniThemeMode {
    SystemDefault,
    Dark,
    Light,
}

#[derive(Debug, Clone)]
pub(crate) enum OmniThemesMessage {
    ChangeThemeMode(OmniThemeMode),
    ChangeSystemThemeMode(theme::Mode),
    SwitchLightTheme(SerializableTheme),
    SwitchDarkTheme(SerializableTheme),
    CriticalStateChanged,
}

#[cfg_attr(feature = "config", derive(Serialize, Deserialize))]
#[derive(PartialEq, Debug, Clone, Hash)]
pub enum SerializableTheme {
    Light,
    Dark,
    Dracula,
    Nord,
    SolarizedLight,
    SolarizedDark,
    GruvboxLight,
    GruvboxDark,
    CatppuccinLatte,
    CatppuccinFrappe,
    CatppuccinMacchiato,
    CatppuccinMocha,
    TokyoNight,
    TokyoNightStorm,
    TokyoNightLight,
    KanagawaWave,
    KanagawaDragon,
    KanagawaLotus,
    Moonfly,
    Nightfly,
    Oxocarbon,
    Ferra,
}

impl std::fmt::Display for SerializableTheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SerializableTheme::Dark => f.write_str("Dark"),
            SerializableTheme::Light => f.write_str("Light"),
            SerializableTheme::Dracula => f.write_str("Dracula"),
            SerializableTheme::Nord => f.write_str("Nord"),
            SerializableTheme::SolarizedLight => f.write_str("Solarized Light"),
            SerializableTheme::SolarizedDark => f.write_str("Solarized Dark"),
            SerializableTheme::GruvboxLight => f.write_str("Gruvbox Light"),
            SerializableTheme::GruvboxDark => f.write_str("Gruvbox Dark"),
            SerializableTheme::CatppuccinLatte => f.write_str("Catppuccin Latte"),
            SerializableTheme::CatppuccinFrappe => f.write_str("Catppuccin Frappe"),
            SerializableTheme::CatppuccinMacchiato => f.write_str("Catppuccin Macchiato"),
            SerializableTheme::CatppuccinMocha => f.write_str("Catppuccin Mocha"),
            SerializableTheme::TokyoNight => f.write_str("Tokyo Night"),
            SerializableTheme::TokyoNightStorm => f.write_str("Tokyo Night Storm"),
            SerializableTheme::TokyoNightLight => f.write_str("Tokyo Night Light"),
            SerializableTheme::KanagawaWave => f.write_str("Kanagawa Wave"),
            SerializableTheme::KanagawaDragon => f.write_str("Kanagawa Dragon"),
            SerializableTheme::KanagawaLotus => f.write_str("Kanagawa Lotus"),
            SerializableTheme::Moonfly => f.write_str("Moonfly"),
            SerializableTheme::Nightfly => f.write_str("Nightfly"),
            SerializableTheme::Oxocarbon => f.write_str("Oxocarbon"),
            SerializableTheme::Ferra => f.write_str("Ferra"),
        }
    }
}

impl From<Theme> for SerializableTheme {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::Light => Self::Light,
            Theme::Dark => Self::Dark,
            Theme::Dracula => Self::Dracula,
            Theme::Ferra => Self::Ferra,
            Theme::Nord => Self::Nord,
            Theme::SolarizedLight => Self::SolarizedLight,
            Theme::SolarizedDark => Self::SolarizedDark,
            Theme::GruvboxLight => Self::GruvboxLight,
            Theme::GruvboxDark => Self::GruvboxDark,
            Theme::CatppuccinLatte => Self::CatppuccinLatte,
            Theme::CatppuccinFrappe => Self::CatppuccinFrappe,
            Theme::CatppuccinMacchiato => Self::CatppuccinMacchiato,
            Theme::CatppuccinMocha => Self::CatppuccinMocha,
            Theme::TokyoNight => Self::TokyoNight,
            Theme::TokyoNightStorm => Self::TokyoNightStorm,
            Theme::TokyoNightLight => Self::TokyoNightLight,
            Theme::KanagawaWave => Self::KanagawaWave,
            Theme::KanagawaDragon => Self::KanagawaDragon,
            Theme::KanagawaLotus => Self::KanagawaLotus,
            Theme::Moonfly => Self::Moonfly,
            Theme::Nightfly => Self::Nightfly,
            Theme::Oxocarbon => Self::Oxocarbon,
            // TODO: Custom themes can't round-trip through SerializableTheme; fall back to Dark
            // until we either store the custom palette or expose iced's built-in theme list
            Theme::Custom(_) => {
                unimplemented!("Custom themes are not supported yet")
            }
        }
    }
}

impl From<SerializableTheme> for Theme {
    fn from(theme: SerializableTheme) -> Self {
        match theme {
            SerializableTheme::Light => Theme::Light,
            SerializableTheme::Dark => Theme::Dark,
            SerializableTheme::Dracula => Theme::Dracula,
            SerializableTheme::Nord => Theme::Nord,
            SerializableTheme::SolarizedLight => Theme::SolarizedLight,
            SerializableTheme::SolarizedDark => Theme::SolarizedDark,
            SerializableTheme::GruvboxLight => Theme::GruvboxLight,
            SerializableTheme::GruvboxDark => Theme::GruvboxDark,
            SerializableTheme::CatppuccinLatte => Theme::CatppuccinLatte,
            SerializableTheme::CatppuccinFrappe => Theme::CatppuccinFrappe,
            SerializableTheme::CatppuccinMacchiato => Theme::CatppuccinMacchiato,
            SerializableTheme::CatppuccinMocha => Theme::CatppuccinMocha,
            SerializableTheme::TokyoNight => Theme::TokyoNight,
            SerializableTheme::TokyoNightStorm => Theme::TokyoNightStorm,
            SerializableTheme::TokyoNightLight => Theme::TokyoNightLight,
            SerializableTheme::KanagawaWave => Theme::KanagawaWave,
            SerializableTheme::KanagawaDragon => Theme::KanagawaDragon,
            SerializableTheme::KanagawaLotus => Theme::KanagawaLotus,
            SerializableTheme::Moonfly => Theme::Moonfly,
            SerializableTheme::Nightfly => Theme::Nightfly,
            SerializableTheme::Oxocarbon => Theme::Oxocarbon,
            SerializableTheme::Ferra => Theme::Ferra,
        }
    }
}

#[cfg_attr(feature = "config", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub(crate) struct OmniThemes {
    pub application_theme_mode: OmniThemeMode,
    #[cfg_attr(feature = "config", serde(skip))]
    system_theme_mode: theme::Mode,
    pub light_theme: SerializableTheme,
    pub dark_theme: SerializableTheme,
}

pub static LIGHT_THEMES: &[SerializableTheme] = &[
    SerializableTheme::Light,
    SerializableTheme::SolarizedLight,
    SerializableTheme::GruvboxLight,
    SerializableTheme::CatppuccinLatte,
    SerializableTheme::TokyoNightLight,
    SerializableTheme::KanagawaLotus,
];

pub static DARK_THEMES: &[SerializableTheme] = &[
    SerializableTheme::Dark,
    SerializableTheme::Dracula,
    SerializableTheme::Nord,
    SerializableTheme::SolarizedDark,
    SerializableTheme::GruvboxDark,
    SerializableTheme::CatppuccinFrappe,
    SerializableTheme::CatppuccinMacchiato,
    SerializableTheme::CatppuccinMocha,
    SerializableTheme::TokyoNight,
    SerializableTheme::TokyoNightStorm,
    SerializableTheme::KanagawaWave,
    SerializableTheme::KanagawaDragon,
    SerializableTheme::Moonfly,
    SerializableTheme::Nightfly,
    SerializableTheme::Oxocarbon,
    SerializableTheme::Ferra,
];

impl OmniThemes {
    pub(crate) fn init() -> OmniThemes {
        Self {
            application_theme_mode: OmniThemeMode::SystemDefault,
            system_theme_mode: theme::Mode::None,
            light_theme: SerializableTheme::GruvboxLight,
            dark_theme: SerializableTheme::GruvboxDark,
        }
    }

    pub(crate) fn update(&mut self, message: OmniThemesMessage) -> Task<OmniThemesMessage> {
        let is_critical_state_changed = matches!(
            message,
            OmniThemesMessage::ChangeSystemThemeMode(_)
                | OmniThemesMessage::SwitchLightTheme(_)
                | OmniThemesMessage::SwitchDarkTheme(_)
        );

        match message {
            OmniThemesMessage::ChangeThemeMode(mode) => {
                self.application_theme_mode = mode;
            }
            OmniThemesMessage::ChangeSystemThemeMode(mode) => {
                self.system_theme_mode = mode;
            }
            OmniThemesMessage::SwitchLightTheme(theme) => self.light_theme = theme,
            OmniThemesMessage::SwitchDarkTheme(theme) => self.dark_theme = theme,
            OmniThemesMessage::CriticalStateChanged => {}
        };

        if is_critical_state_changed {
            Task::done(OmniThemesMessage::CriticalStateChanged)
        } else {
            Task::none()
        }
    }

    pub(crate) fn view(&self) -> Element<'_, OmniThemesMessage> {
        const R: f32 = 6.0;

        let left_radius = border::Radius {
            top_left: R,
            top_right: 0.0,
            bottom_right: 0.0,
            bottom_left: R,
        };
        let mid_radius = border::Radius::default();
        let right_radius = border::Radius {
            top_left: 0.0,
            top_right: R,
            bottom_right: R,
            bottom_left: 0.0,
        };

        let current_mode = &self.application_theme_mode;
        let toggle_button = |label: &'static str, mode: OmniThemeMode, radius: border::Radius| {
            let is_active = &mode == current_mode;
            widget::button(label)
                .on_press(OmniThemesMessage::ChangeThemeMode(mode))
                .style(move |theme, status| {
                    let mut style = if is_active {
                        widget::button::primary(theme, status)
                    } else {
                        widget::button::secondary(theme, status)
                    };
                    style.border.radius = radius;
                    style
                })
        };

        widget::column![
            widget::container(
                widget::row![
                    toggle_button("☽ Dark", OmniThemeMode::Dark, left_radius),
                    toggle_button("⚙ System", OmniThemeMode::SystemDefault, mid_radius),
                    toggle_button("☀ Light", OmniThemeMode::Light, right_radius),
                ]
                .spacing(0),
            )
            .center_x(Length::Fill),
            widget::container(
                widget::row![
                    widget::text("Light:"),
                    widget::PickList::new(
                        LIGHT_THEMES,
                        Some(&self.light_theme),
                        OmniThemesMessage::SwitchLightTheme
                    )
                    .width(Length::Shrink),
                    widget::text("Dark:"),
                    widget::PickList::new(
                        DARK_THEMES,
                        Some(&self.dark_theme),
                        OmniThemesMessage::SwitchDarkTheme
                    )
                    .width(Length::Shrink),
                ]
                .spacing(8)
                .align_y(Alignment::Center),
            )
            .center_x(Length::Fill),
        ]
        .spacing(8)
        .into()
    }

    pub(crate) fn start_up_tasks(&self) -> Task<OmniThemesMessage> {
        iced::system::theme().map(OmniThemesMessage::ChangeSystemThemeMode)
    }

    pub(crate) fn subscription(&self) -> Subscription<OmniThemesMessage> {
        iced::system::theme_changes().map(OmniThemesMessage::ChangeSystemThemeMode)
    }

    pub(crate) fn theme(&self) -> Theme {
        match &self.application_theme_mode {
            OmniThemeMode::Light => self.light_theme.clone().into(),
            OmniThemeMode::Dark => self.dark_theme.clone().into(),
            OmniThemeMode::SystemDefault => match self.system_theme_mode {
                theme::Mode::Light => self.light_theme.clone().into(),
                theme::Mode::Dark | theme::Mode::None => self.dark_theme.clone().into(),
            },
        }
    }
}
