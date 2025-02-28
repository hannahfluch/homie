use std::{
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
    rc::Rc,
    time::Duration,
};

use frame::{transform, Frame};
use gtk4::{gdk, glib, prelude::*, subclass::prelude::*};
use image::{codecs::gif::GifDecoder, AnimationDecoder};
use paintable::Sprites;

use super::{HomieError, InternalConfig, State};

mod frame;
mod paintable;

gtk4::glib::wrapper! {
    pub(crate) struct GifPaintable(ObjectSubclass<paintable::GifPaintable>) @implements gdk::Paintable;
}

impl Default for GifPaintable {
    fn default() -> Self {
        glib::Object::new()
    }
}

impl GifPaintable {
    /// Loads the bytes of a GIF into the paintable.
    pub(crate) fn load_animations(
        &self,
        path: &Rc<PathBuf>,
        config: &InternalConfig,
    ) -> Result<(), HomieError> {
        let imp = self.imp();
        imp.current_idx.set(0);

        if let Some(source_id) = imp.timeout_source_id.take() {
            source_id.remove();
        }

        imp.sprites.replace(Sprites {
            idle: Some(Self::load_gif(path.join("idle.gif"), config)?),
            click: Some(Self::load_gif(path.join("click.gif"), config)?),
            running: Some(Self::load_gif(path.join("run.gif"), config)?),
        });

        // make sure the first frame is queued to play
        self.setup_next_frame();

        Ok(())
    }

    pub(crate) fn state(&self) -> State {
        let imp = self.imp();
        imp.state.get()
    }

    fn load_gif<P: AsRef<Path>>(
        path: P,
        config: &InternalConfig,
    ) -> Result<Vec<Frame>, HomieError> {
        let decoder = GifDecoder::new(BufReader::new(File::open(path)?))?;
        let mut frames = decoder
            .into_frames()
            .collect_frames()?
            .into_iter()
            .map(|f| transform(f, config))
            .map(Frame::from)
            .collect::<Vec<Frame>>();

        if let Some(fps) = config.fps {
            frames.iter_mut().for_each(|f| {
                f.frame_duration = Duration::from_millis(1000 / fps as u64);
            });
        }

        Ok(frames)
    }

    pub(crate) fn switch_animation(&self, new_state: State) {
        let imp = self.imp();

        imp.state.set(new_state);
        imp.current_idx.set(0); // start from first frame of new animation
    }

    fn setup_next_frame(&self) {
        let imp = self.imp();
        let idx = imp.current_idx.get();
        let state = imp.state.get();

        // Get the correct animation based on the state
        let frames_ref = match state {
            State::Idle => &imp.sprites.borrow().idle,
            State::Click => &imp.sprites.borrow().click,
            State::Running => &imp.sprites.borrow().running,
        };

        // if we have stored no frames then we early return early
        // and instead render a default frame in `paintable::GifPaintable::snapshot`
        let frames = match frames_ref {
            Some(frames) => frames,
            None => return,
        };

        let next_frame = frames.get(idx).unwrap();
        imp.next_frame.replace(Some(next_frame.texture.clone()));

        // invalidate the contents so that the new frame will be rendered
        self.invalidate_contents();

        // setup a callback to this function once the frame has finished so that
        // we can play the next frame
        let update_next_frame_callback = glib::clone!(
            #[weak(rename_to = paintable)]
            self,
            move || {
                paintable.imp().timeout_source_id.take();
                paintable.setup_next_frame();
            }
        );

        let source_id =
            glib::timeout_add_local_once(next_frame.frame_duration, update_next_frame_callback);
        imp.timeout_source_id.replace(Some(source_id));

        // setup the index for the next call to setup_next_frame
        let mut new_idx = idx + 1;
        if new_idx >= frames.len() {
            if state == State::Click {
                self.switch_animation(State::Idle);
            }

            new_idx = 0; // loop
        }
        imp.current_idx.set(new_idx);
    }
}
