use crate::infra::log_source::LogSource;
use bevy::prelude::Resource;

#[derive(Resource, Debug, Clone, PartialEq)]
pub struct LogSourceConfiguration {
    pub enabled_sources: Vec<LogSource>,
    pub disabled_sources: Vec<LogSource>,
    pub debug_mode: bool,
}

impl Default for LogSourceConfiguration {
    fn default() -> Self {
        LogSourceConfiguration {
            enabled_sources: vec![
                LogSource::Inventory,
                LogSource::Input,
                LogSource::Player,
                LogSource::Creature,
                LogSource::Companion,
                LogSource::NPC,
                LogSource::Interface,
                LogSource::State,
                LogSource::Quest,
                LogSource::Time,
                LogSource::Farm,
                LogSource::World,
                LogSource::Enemy,
                LogSource::Ldtk,
            ],
            disabled_sources: vec![],
            debug_mode: false,
        }
    }
}

impl LogSourceConfiguration {
    pub fn new(enabled_sources: Vec<LogSource>, disabled_sources: Vec<LogSource>) -> Self {
        LogSourceConfiguration {
            enabled_sources,
            disabled_sources,
            debug_mode: false,
        }
    }

    pub fn is_enabled(&self, source: &LogSource) -> bool {
        self.enabled_sources.contains(source) && !self.disabled_sources.contains(source)
    }

    pub fn set_debug_mode(&mut self, debug_mode: bool) {
        self.debug_mode = debug_mode;
    }

    pub fn enable_source(&mut self, source: LogSource) {
        // push the source to the list
        if !self.enabled_sources.contains(&source) {
            self.enabled_sources.push(source);
        }

        // filter out the source from disabled sources
        if self.disabled_sources.contains(&source) {
            self.disabled_sources.retain(|&s| s != source);
        }
    }

    pub fn disable_source(&mut self, source: LogSource) {
        if !self.disabled_sources.contains(&source) {
            self.disabled_sources.push(source);
        }

        if self.enabled_sources.contains(&source) {
            self.enabled_sources.retain(|&s| s != source);
        }
    }
}
