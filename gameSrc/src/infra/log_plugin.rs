use crate::{
    game::state::GameState,
    infra::{
        constants,
        log_source::LogSource,
        log_source_configuration::LogSourceConfiguration
    },
};
use bevy::prelude::*;

pub struct LoggingPlugin;
impl Plugin for LoggingPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<LogEvent>()
            .insert_resource(LogSourceConfiguration::default())
            .add_systems(
                Update,
                log_event_reader.run_if(in_state(GameState::Running)),
            );
    }
}

/// Method used to standardize sending an event to the log event writer
/// Note: If this turns out to be a really bad idea this is where we can easily change the logging
/// to be a simple easy peasy println!() or something
pub fn send_log_to_event(
    mut log_event_writer: MessageWriter<LogEvent>,
    message: String,
    source: LogSource,
    level: LogLevel,
) {
    let event = LogEvent {
        message,
        source,
        level,
    };
    log_event_writer.write(event);
}

pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
}

#[derive(Message)]
pub struct LogEvent {
    pub message: String,
    pub source: LogSource,
    pub level: LogLevel,
}

fn log_event_reader(
    mut log_event_reader: MessageReader<LogEvent>,
    log_source_config: Res<LogSourceConfiguration>,
) {
    if log_event_reader.is_empty() {
        return;
    }

    for event in log_event_reader.read() {
        // if the log source is not enabled skip the event
        if !log_source_config.is_enabled(&event.source) {
            debug(
                &format!("Log source is not enabled: {:?}", event.source),
                event.source,
            );
            continue;
        }
        match event.level {
            LogLevel::Debug => {
                if log_source_config.debug_mode {
                    debug(&event.message, event.source);
                }
            }
            LogLevel::Info => info(&event.message, event.source),
            LogLevel::Warning => warn(&event.message, event.source),
            LogLevel::Error => error(&event.message, event.source),
        }
    }
}

pub fn debug(msg: &str, source: LogSource) {
    if constants::DEBUG_LOG {
        let message = format!("[DEBUG][{:?}] {}", source, msg);
        println!("{}", message);
    }
}

pub fn info(msg: &str, source: LogSource) {
    let message = format!("[INFO][{:?}] {}", source, msg);
    println!("{}", message);
}

pub fn warn(msg: &str, source: LogSource) {
    let message = format!("[WARN][{:?}] {}", source, msg);
    println!("{}", message);
}

pub fn error(msg: &str, source: LogSource) {
    let message = format!("[ERROR][{:?}] {}", source, msg);
    println!("{}", message);
}

//TODO: Eventually come back and figure out some way to verify we've got stuff coming out std
//out correctly but idk this is good enough for me for now
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_debug_logging() {
        debug("test message", LogSource::Interface);
    }

    #[test]
    fn test_info_logging() {
        info("test message", LogSource::Player);
    }

    #[test]
    fn test_warn_logging() {
        warn("test message", LogSource::World);
    }

    #[test]
    fn test_error_logging() {
        error("test message", LogSource::Inventory);
    }
}
