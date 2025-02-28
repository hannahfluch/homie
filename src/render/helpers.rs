use gtk4::prelude::NativeExt;
use gtk4::ApplicationWindow;
use gtk4::CssProvider;

use gdk4::cairo::{RectangleInt, Region};
use gdk4::prelude::{DisplayExt, MonitorExt, SurfaceExt};
use gdk4::Display;

use super::InternalConfig;

/// Update click-able section of buddy on screen.
pub(super) fn update_input_region(
    window: &ApplicationWindow,
    width: i32,
    height: i32,
    x: i32,
    y: i32,
) {
    let region = Region::create_rectangle(&RectangleInt::new(x, y, width, height));
    window.surface().unwrap().set_input_region(&region);
}

/// Returns the screen resolution (width, height). May fail and return None.
pub(super) fn screen_resolution(window: &ApplicationWindow) -> Option<(i32, i32)> {
    let display = Display::default()?;

    let monitor = display.monitor_at_surface(&window.surface()?)?;
    Some((monitor.geometry().width(), monitor.geometry().height()))
}

/// Make buddy's background transparent.
pub(super) fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_string(
        r#"* {
        background-color: transparent;
    }"#,
    );

    gtk4::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    )
}

/// Infer the width/height if only one of the is provided.
pub(super) fn infer_size(config: &InternalConfig, aspect_ratio: f64) -> (i32, i32) {
    assert!(aspect_ratio > 0.0);

    match (config.width, config.height) {
        (Some(width), None) => (width as i32, (width as f64 / aspect_ratio) as i32),
        (Some(width), Some(height)) => (width as i32, height as i32),
        (None, Some(height)) => (((height as f64 * aspect_ratio) as i32), height as i32),
        (None, None) => unreachable!(),
    }
}
