use serde::{Deserialize, Serialize};

#[cfg(feature = "counter")]
use crate::features::counter::Counter;
#[cfg(feature = "instax_framer")]
use crate::features::instax_framer::InstaxFramer;
#[cfg(feature = "omni_themes")]
use crate::features::omni_themes::OmniThemes;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct OmniAppConfig {
    #[cfg(feature = "counter")]
    pub counter: Counter,
    #[cfg(feature = "omni_themes")]
    pub omni_themes: OmniThemes,
    #[cfg(feature = "instax_framer")]
    pub instax_framer: InstaxFramer,
}

impl PartialEq for OmniAppConfig {
    #[allow(unused)]
    fn eq(&self, other: &Self) -> bool {
        #[cfg(feature = "counter")]
        if self.counter != other.counter {
            return false;
        }

        #[cfg(feature = "omni_themes")]
        if self.omni_themes.dark_theme != other.omni_themes.dark_theme
            || self.omni_themes.light_theme != other.omni_themes.light_theme
            || self.omni_themes.application_theme_mode != other.omni_themes.application_theme_mode
        {
            return false;
        }

        #[cfg(feature = "instax_framer")]
        if self.instax_framer.selected_file != other.instax_framer.selected_file {
            return false;
        }

        true
    }
}

impl Default for OmniAppConfig {
    fn default() -> Self {
        Self {
            #[cfg(feature = "counter")]
            counter: Counter::init(),
            #[cfg(feature = "omni_themes")]
            omni_themes: OmniThemes::init(),
            #[cfg(feature = "instax_framer")]
            instax_framer: InstaxFramer::init(),
        }
    }
}
