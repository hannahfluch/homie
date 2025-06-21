use std::path::PathBuf;

use clap::Parser;
use clap_num::number_range;

#[derive(Parser, Debug)]
#[command(name = "Homie")]
#[command(author = "Hannah F. <github: hannahfluch>")]
#[command(version, about, long_about = None)]
pub(crate) struct Cli {
    #[clap(
        short = 's',
        long,
        value_name = "PATH",
        help = "Initial path to directory with animation sprites. Directory must contain subdirectories for each event type."
    )]
    pub(crate) sprites_path: Option<PathBuf>,

    #[clap(
        short,
        long,
        value_name = "WIDTH",
        help = "Width of the character (px). Can be inferred as long as the height is provided."
    )]
    pub(crate) width: Option<u16>,

    #[clap(
        short = 'H',
        long,
        value_name = "HEIGHT",
        help = "Height of the character (px). Can be inferred as long as the width is provided."
    )]
    pub(crate) height: Option<u16>,

    #[clap(
        short,
        long,
        value_name = "FPS",
        help = "Frames per second to animate character."
    )]
    pub(crate) fps: Option<u32>,

    #[clap(
        short,
        long,
        value_name = "SPEED",
        help = "How often the character's position is updated per second."
    )]
    pub(crate) movement_speed: Option<u32>,
    #[clap(
        short = 'S',
        long,
        value_name = "SIGNAL-FREQ",
        help = "How often to check for signals per second. Or how often to reload sprites if automatic reload is enabled."
    )]
    pub(crate) signal_frequency: Option<u32>,

    #[clap(
        short = 'r',
        long,
        num_args = 0..=1,
        default_missing_value = "true",
        value_name = "AUTOMATIC-RELOAD",
        help = "Enables the automatic reload of sprites, the frequency should be specific using -S."
    )]
    pub(crate) automatic_reload: Option<bool>,

    #[clap(
           short,
           long,
           value_name = "ON_CLICK-PERCENT",
           value_parser = less_than_101,
           help = "Chance of on-click event occurring."
       )]
    pub(crate) onclick_event_chance: Option<u8>,

    #[clap(
        short,
        long,
        value_name = "X-START",
        help = "Starting position of homie on x-axis."
    )]
    pub(crate) x: Option<i32>,

    #[clap(
        short,
        long,
        value_name = "Y-START",
        help = "Starting position of homie on y-axis."
    )]
    pub(crate) y: Option<i32>,

    #[clap(
        short,
        long,
        value_name = "RUN-LEFT",
        num_args = 0..=1,
        default_missing_value = "true",
        help = "Make homie move to the left instead of the default: right."
    )]
    pub(crate) left: Option<bool>,

    #[clap(
        short = 'F',
        long,
        num_args = 0..=1,
        default_missing_value = "true",
        value_name = "FLIP-HORIZONTAL",
        help = "Used to flip the horizontal direction of sprites."
    )]
    pub(crate) flip_horizontal: Option<bool>,

    #[clap(
        short = 'v',
        long,
        num_args = 0..=1,
        default_missing_value = "true",
        value_name = "FLIP-VERTICAL",
        help = "Used to flip the vertical direction of sprites."
    )]
    pub(crate) flip_vertical: Option<bool>,

    #[clap(
        short,
        long,
        num_args = 0..=1,
        default_missing_value = "true",
        value_name = "DEBUG-MODE",
        help = "Used to disable out of bounds checks."
    )]
    pub(crate) debug: Option<bool>,

    #[clap(
        short = 'C',
        long,
        value_name = "PATH",
        help = "Path to homie configuration file. If none provided, default config is created at: $HOME/.config/homie/ "
    )]
    pub(crate) config_path: Option<String>,
}

fn less_than_101(s: &str) -> Result<u8, String> {
    number_range(s, 0, 100)
}
