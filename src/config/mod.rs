use serde_derive::{Deserialize, Serialize};

pub(crate) mod cli;
pub(crate) mod default;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct Config {
    pub(crate) fps: Option<u32>,
    pub(crate) width: Option<u16>,
    pub(crate) height: Option<u16>,
    pub(crate) movement_speed: u32,
    pub(crate) onclick_event_chance: u8,
    pub(crate) x: i32,
    pub(crate) y: i32,
    pub(crate) sprites_path: Option<String>,
    pub(crate) left: bool,
    pub(crate) flip_horizontal: bool,
    pub(crate) flip_vertical: bool,
    pub(crate) debug: bool,
    pub(crate) signal_frequency: u32,
    pub(crate) automatic_reload: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            movement_speed: default::MOVEMENT_SPEED,
            onclick_event_chance: default::ON_CLICK_CHANCE,
            x: default::X,
            y: default::Y,
            left: default::RUN_LEFT,
            flip_horizontal: default::FLIP_HORIZONTAL,
            flip_vertical: default::FLIP_VERTICAL,
            debug: default::DEBUG,
            signal_frequency: default::SIGNAL_FREQUENCY,
            automatic_reload: default::AUTOMATIC_RELOAD,
            sprites_path: None,
            fps: None,
            width: None,
            height: None,
        }
    }
}
#[derive(Copy, Clone, Debug)]
pub(crate) struct InternalConfig {
    pub(crate) fps: Option<u32>,
    pub(crate) width: Option<u16>,
    pub(crate) height: Option<u16>,
    pub(crate) movement_speed: u32,
    pub(crate) onclick_event_chance: u8,
    pub(crate) x: i32,
    pub(crate) y: i32,
    pub(crate) left: bool,
    pub(crate) flip_horizontal: bool,
    pub(crate) flip_vertical: bool,
    pub(crate) debug: bool,
    pub(crate) signal_frequency: u32,
    pub(crate) automatic_reload: bool,
}

impl From<Config> for InternalConfig {
    fn from(value: Config) -> Self {
        InternalConfig {
            fps: value.fps,
            width: value.width,
            height: value.height,
            movement_speed: value.movement_speed,
            onclick_event_chance: value.onclick_event_chance,
            x: value.x,
            y: value.y,
            left: value.left,
            flip_vertical: value.flip_vertical,
            flip_horizontal: value.flip_horizontal,
            debug: value.debug,
            signal_frequency: value.signal_frequency,
            automatic_reload: value.automatic_reload,
        }
    }
}
