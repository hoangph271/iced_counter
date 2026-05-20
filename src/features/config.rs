use std::hash::{DefaultHasher, Hash, Hasher};

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

impl Hash for OmniAppConfig {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let OmniAppConfig {
            #[cfg(feature = "counter")]
            counter,
            #[cfg(feature = "omni_themes")]
            omni_themes,
            #[cfg(feature = "instax_framer")]
            instax_framer,
        } = self;

        #[cfg(feature = "counter")]
        counter.hash(state);

        #[cfg(feature = "omni_themes")]
        {
            omni_themes.application_theme_mode.hash(state);
            omni_themes.light_theme.hash(state);
            omni_themes.dark_theme.hash(state);
        }

        #[cfg(feature = "instax_framer")]
        {
            instax_framer.selected_file.hash(state);
        }
    }
}

impl OmniAppConfig {
    pub fn get_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();

        self.hash(&mut hasher);

        hasher.finish()
    }
}
