use std::time::Duration;

use gtk4::{gdk, glib, prelude::*};
use image::{DynamicImage, ImageBuffer, Rgba};

pub(crate) struct Frame {
    pub(crate) texture: gdk::Texture,
    pub(crate) frame_duration: Duration,
}

impl From<image::Frame> for Frame {
    fn from(f: image::Frame) -> Self {
        let frame_duration = Duration::from(f.delay());

        let samples = f.into_buffer().into_flat_samples();

        let bytes = glib::Bytes::from(samples.as_slice());
        let layout = samples.layout;

        let texture = gdk::MemoryTexture::new(
            layout.width as i32,
            layout.height as i32,
            gdk::MemoryFormat::R8g8b8a8,
            &bytes,
            layout.height_stride,
        );

        Frame {
            texture: texture.upcast(),
            frame_duration,
        }
    }
}

pub(super) fn flip(f: image::Frame, vertical: bool, horizontal: bool) -> image::Frame {
    if !vertical && !horizontal {
        f
    } else {
        let left = f.left();
        let top = f.top();
        let delay = f.delay();
        let buffer = f.into_buffer();

        let mut dynamic_image: DynamicImage = buffer.into();
        if vertical {
            dynamic_image = dynamic_image.flipv();
        }

        if horizontal {
            dynamic_image = dynamic_image.fliph();
        }

        let buffer = ImageBuffer::<Rgba<u8>, Vec<u8>>::from(dynamic_image);
        image::Frame::from_parts(buffer, left, top, delay)
    }
}
