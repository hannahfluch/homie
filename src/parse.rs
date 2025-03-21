use std::env;
use std::path::PathBuf;

use crate::config::{cli::Cli, Config};
use crate::error::HomieError;
use clap::Parser;
use regex_lite::Regex;

/// Parse cli args and match against config file
macro_rules! parse_args {
    ($config:expr, $cli:expr, $($field:ident),*) => {{
        $(
            if let Some(value) = $cli.$field {
                $config.$field = value.into();
            }
        )*
    }};
}

/// Parse [Cli] and config arguments. Returns [Config] structure and sprites path. [HomieError] is returned in case of failure (invalid config, invalid sprites path).
///
/// Note: sprites path in config structure remains None.
pub(crate) fn run() -> Result<(Config, PathBuf), HomieError> {
    let cli = Cli::parse();

    // load specific config file or default path.
    let mut config: Config = match cli.config_path {
        Some(config_path) => confy::load_path(config_path),
        None => confy::load("homie", Option::from("config")),
    }
    .map_err(HomieError::from)?;

    parse_args!(
        config,
        cli,
        width,
        height,
        fps,
        movement_speed,
        signal_frequency,
        automatic_reload,
        onclick_event_chance,
        x,
        y,
        left,
        flip_horizontal,
        flip_vertical,
        debug
    );

    if config.width.is_none() && config.height.is_none() {
        return Err(HomieError::NoDimensions);
    }

    // check for existing sprites path
    let sprites_path = cli
        .sprites_path
        .or(config
            .sprites_path
            .take()
            .and_then(|c| expand_env(c.replace("~", "$HOME"))))
        .ok_or(HomieError::NoSprites)?;

    Ok((config, sprites_path))
}

/// Expand environment variables in paths of config file.
fn expand_env(input: String) -> Option<PathBuf> {
    Regex::new(r"\$([A-Za-z_][A-Za-z0-9_]*)").ok().map(|re| {
        re.replace_all(&input, |caps: &regex_lite::Captures| {
            env::var(&caps[1]).unwrap_or_else(|_| format!("${}", &caps[1]))
        })
        .to_string()
        .into()
    })
}
