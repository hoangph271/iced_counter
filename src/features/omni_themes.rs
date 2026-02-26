use iced::{border, theme, widget, Alignment, Element, Length, Subscription, Task, Theme};

#[derive(Debug, PartialEq, Clone)]
pub enum OmniThemeMode {
    SystemDefault,
    Dark,
    Light,
}

#[derive(Debug, Clone)]
pub(crate) enum OmniThemesMessage {
    ChangeThemeMode(OmniThemeMode),
    ChangeSystemThemeMode(theme::Mode),
    SwitchLightTheme(Theme),
    SwitchDarkTheme(Theme),
}

#[derive(Debug)]
pub(crate) struct OmniThemes {
    pub application_theme_mode: OmniThemeMode,
    pub system_theme_mode: theme::Mode,
    pub light_theme: Theme,
    pub dark_theme: Theme,
}

pub static LIGHT_THEMES: &[Theme] = &[
    Theme::Light,
    Theme::SolarizedLight,
    Theme::GruvboxLight,
    Theme::CatppuccinLatte,
    Theme::TokyoNightLight,
    Theme::KanagawaLotus,
];

pub static DARK_THEMES: &[Theme] = &[
    Theme::Dark,
    Theme::Dracula,
    Theme::Nord,
    Theme::SolarizedDark,
    Theme::GruvboxDark,
    Theme::CatppuccinFrappe,
    Theme::CatppuccinMacchiato,
    Theme::CatppuccinMocha,
    Theme::TokyoNight,
    Theme::TokyoNightStorm,
    Theme::KanagawaWave,
    Theme::KanagawaDragon,
    Theme::Moonfly,
    Theme::Nightfly,
    Theme::Oxocarbon,
    Theme::Ferra,
];

impl OmniThemes {
    pub fn theme_from_state(state: &Self) -> Theme {
        let OmniThemes {
            application_theme_mode: mode,
            system_theme_mode,
            light_theme,
            dark_theme,
        } = state;

        match mode {
            OmniThemeMode::Light => light_theme.clone(),
            OmniThemeMode::Dark => dark_theme.clone(),
            OmniThemeMode::SystemDefault => match system_theme_mode {
                theme::Mode::Light => light_theme.clone(),
                theme::Mode::Dark | theme::Mode::None => dark_theme.clone(),
            },
        }
    }

    pub(crate) fn init() -> OmniThemes {
        Self {
            application_theme_mode: OmniThemeMode::SystemDefault,
            system_theme_mode: theme::Mode::None,
            light_theme: Theme::GruvboxLight,
            dark_theme: Theme::GruvboxDark,
        }
    }

    pub(crate) fn update(&mut self, message: OmniThemesMessage) -> Task<OmniThemesMessage> {
        match message {
            OmniThemesMessage::ChangeThemeMode(mode) => {
                self.application_theme_mode = mode;
            }
            OmniThemesMessage::ChangeSystemThemeMode(mode) => {
                self.system_theme_mode = mode;
            }
            OmniThemesMessage::SwitchLightTheme(theme) => self.light_theme = theme,
            OmniThemesMessage::SwitchDarkTheme(theme) => self.dark_theme = theme,
        };

        Task::none()
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
}
