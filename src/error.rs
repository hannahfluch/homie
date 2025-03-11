use confy::ConfyError;
use image::ImageError;
use thiserror::Error;

#[derive(Debug, Error)]
pub(crate) enum HomieError {
    #[error("Configuration Failed: {0}")]
    InvalidConfig(#[from] ConfyError),
    #[error("No sprites path specidied")]
    NoSprites,
    #[error("No width/height was provided.")]
    NoDimensions,
    #[error("Graphical Failure: {0}")]
    Glib(#[from] gtk4::glib::Error),
    #[error("I/O Error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Coordinates out of bounds: x: {0}px, y: {1}px for screen width: {2}px, screen height: {3}px, character width: {4}px, character height: {5}px  - Use debug flag to disable bounds-checking")]
    CoordinatesOutOfBounds(i32, i32, i32, i32, i32, i32),
    #[error("Unable to get screen resolution!")]
    NoScreenResolution,
    #[error("Gif Processing Error: {0}")]
    GifDecodingFailed(#[from] ImageError),
}
