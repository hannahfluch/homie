use std::path::PathBuf;
use std::rc::Rc;
use std::time::Duration;

use gdk4::prelude::PaintableExt;
use gif::GifPaintable;
use gtk4::glib::{timeout_add_local, unix_signal_add_local, ControlFlow};
use gtk4::prelude::FixedExt;
use gtk4::prelude::{ApplicationExt, ApplicationExtManual};
use gtk4::prelude::{GtkWindowExt, WidgetExt};
use gtk4::Fixed;
use gtk4::Picture;
use gtk4::{ApplicationWindow, GestureClick};
use gtk4_layer_shell::{Edge, Layer, LayerShell};

use helpers::load_css;
use helpers::screen_resolution;
use helpers::update_input_region;
use state::State;

use crate::config::{Config, InternalConfig};
use crate::error::BuddyError;

mod gif;
mod helpers;
mod state;

const SIGUSR1: i32 = 10;
const SIGUSR2: i32 = 12;

/// Prepare and render character.
pub(crate) fn render_character(config: Config, sprites_path: PathBuf) {
    let app_id = format!("hannahfluch.buddy.instance{}", std::process::id());

    let application = gtk4::Application::new(Some(app_id.as_str()), Default::default());

    application.connect_startup(|_| load_css());

    let sprites_path = Rc::new(sprites_path);
    let config = config.into();

    application.connect_activate(move |app| {
        let result = activate(app, config, &sprites_path);

        if let Err(err) = result {
            eprintln!("An error occurred: {}", err);
            std::process::exit(1);
        }
    });
    application.run_with_args::<&str>(&[]);
}

/// Active GTK app. May fail and return [BuddyError].
fn activate(
    application: &gtk4::Application,
    config: InternalConfig,
    sprites_path: &Rc<PathBuf>,
) -> Result<(), BuddyError> {
    let InternalConfig {
        movement_speed,
        onclick_event_chance,
        x,
        y,
        left,
        debug,
        signal_frequency,
        automatic_reload,
        ..
    } = config;

    let window = ApplicationWindow::new(application);

    window.init_layer_shell();

    // Display above normal windows
    window.set_layer(Layer::Overlay);

    for (anchor, state) in [
        (Edge::Left, true),
        (Edge::Right, true),
        (Edge::Top, false),
        (Edge::Bottom, true),
    ] {
        window.set_anchor(anchor, state);
    }

    window.present(); // present prematurely to be able to get screen resolution

    let (screen_width, screen_height) =
        screen_resolution(&window).ok_or(BuddyError::NoScreenResolution)?;
    let sprites = GifPaintable::default();
    let sprites_path_clone = Rc::clone(sprites_path);
    sprites.load_animations(&sprites_path_clone, &config)?;

    let (width, height) = helpers::infer_size(&config, sprites.intrinsic_aspect_ratio());

    // check for valid starting coordinates
    if !debug && ((x + width) >= screen_width || x < 0 || (y + height) >= screen_height || y < 0) {
        return Err(BuddyError::CoordinatesOutOfBounds(
            x,
            y,
            screen_width,
            screen_height,
            width,
            height,
        ));
    }

    // start with idle sprites
    let character = Picture::for_paintable(&sprites);
    // default position
    character.set_hexpand(true);
    character.set_vexpand(true);
    character.set_halign(gtk4::Align::Fill);
    character.set_valign(gtk4::Align::Fill);
    character.set_size_request(width, height);

    let fixed = Fixed::new();
    fixed.put(&character, x as f64, y as f64);
    window.set_child(Some(&fixed));
    window.set_size_request(screen_width, height);
    window.set_resizable(false);

    // default input region
    update_input_region(&window, width, height, x, 0);

    // signal handling for reloading sprites

    let sprites_clone = sprites.clone();
    let sprites_path_clone = Rc::clone(sprites_path);
    unix_signal_add_local(SIGUSR1, move || {
        if let Err(err) = sprites_clone.load_animations(&sprites_path_clone, &config) {
            println!("Warning: Could not update sprites: {}", err)
        }

        ControlFlow::from(true)
    });

    let sprites_clone = sprites.clone();
    let sprites_path_clone = Rc::clone(sprites_path);
    unix_signal_add_local(SIGUSR2, move || {
        if let Err(err) = sprites_clone.load_animations(&sprites_path_clone, &config) {
            println!("Warning: Could not update sprites: {}", err)
        }

        ControlFlow::from(true)
    });

    if automatic_reload {
        let sprites_clone = sprites.clone();
        let sprites_path_clone = Rc::clone(sprites_path);

        timeout_add_local(
            Duration::from_millis(1000 / signal_frequency as u64),
            move || {
                if let Err(err) = sprites_clone.load_animations(&sprites_path_clone, &config) {
                    println!("Warning: Could not update sprites: {}", err)
                }

                ControlFlow::from(true)
            },
        );
    }

    let character_clone = character.clone();
    let sprites_clone = sprites.clone();
    // move character
    timeout_add_local(
        Duration::from_millis(1000 / movement_speed as u64),
        move || {
            if sprites_clone.state() == State::Running {
                let (x, y) = fixed.child_position(&character_clone);
                // update position
                let (x, y) = if left {
                    let x = if x - 10.0 <= -width as f64 {
                        screen_width as f64
                    } else {
                        x - 10.0
                    };

                    (x, y)
                } else {
                    let x = if x + 10.0 >= screen_width as f64 {
                        -width as f64
                    } else {
                        x + 10.0
                    };

                    (x, y)
                };
                // move along screen
                fixed.move_(&character_clone, x, y);
                update_input_region(&window, width, height, x as i32, 0);
            }
            ControlFlow::from(true)
        },
    );

    // change state of character (idle/initiating run)
    let gesture = GestureClick::new();

    gesture.connect_pressed(
        move |_gesture: &GestureClick, _n_press: i32, _x: f64, _y: f64| {
            let state = sprites.state();
            if state != State::Click {
                if state == State::Idle && fastrand::u8(0..=100) <= onclick_event_chance {
                    // play click event and continue
                    sprites.switch_animation(State::Click);
                } else {
                    sprites.switch_animation(!state);
                }
            }
        },
    );

    character.add_controller(gesture);

    Ok(())
}
