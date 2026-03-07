// SPDX-License-Identifier: MPL-2.0

use cosmic::cosmic_config::{self, CosmicConfigEntry, cosmic_config_derive::CosmicConfigEntry};

#[derive(Debug, Clone, CosmicConfigEntry, Eq, PartialEq)]
#[version = 1]
pub struct Config {
    /// Last-used duration in minutes. 0 means indefinite.
    pub duration_mins: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self { duration_mins: 30 }
    }
}
