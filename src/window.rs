extern crate sdl2;
extern crate thiserror;

use sdl2::event::WindowEvent;
use sdl2::video::GLContext;
use sdl2::{Sdl, VideoSubsystem};
type VideoWindow = sdl2::video::Window;
type SdlEvent = sdl2::event::Event;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("SDL Failure: {0}")]
    SDL(String),
    #[error("Failed to create GL context: {0}")]
    GL(String),
}

pub struct Window {
    sdl: Sdl,
    window: VideoWindow,
    _video: VideoSubsystem,
    _gl_context: GLContext,
}

pub use sdl2::keyboard::Keycode;
pub enum Event {
    KeyUp(Keycode),
    KeyDown(Keycode),
}

impl Window {
    pub fn new(title: &str, width: u32, height: u32) -> anyhow::Result<Self> {
        let sdl = sdl2::init().map_err(Error::SDL)?;
        let video = sdl.video().map_err(Error::SDL)?;
        {
            let gl_attr = video.gl_attr();
            gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
            gl_attr.set_context_version(3, 3);
        }
        let window = video
            .window(title, width, height)
            .opengl()
            .resizable()
            .build()?;

        let gl_context = window.gl_create_context().map_err(Error::GL)?;
        gl::load_with(|s| video.gl_get_proc_address(s) as *const std::os::raw::c_void);
        unsafe { gl::Enable(gl::DEPTH_TEST) };

        Ok(Window {
            sdl,
            window,
            _video: video,
            _gl_context: gl_context,
        })
    }

    pub fn run<F>(self, game_logic: F) -> anyhow::Result<()>
    where
        F: Fn(f32, (u32, u32), &[Event]),
    {
        let mut events = Vec::with_capacity(50);
        let mut event_pump = self.sdl.event_pump().map_err(Error::SDL)?;

        let start_time = std::time::Instant::now();
        'main: loop {
            let passed_time = std::time::Instant::now()
                .duration_since(start_time)
                .as_secs_f32();

            events.clear();
            for event in event_pump.poll_iter() {
                match event {
                    SdlEvent::Quit { .. }
                    | SdlEvent::KeyUp {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'main,
                    SdlEvent::Window {
                        win_event: WindowEvent::Resized(width, height),
                        ..
                    } => unsafe {
                        gl::Viewport(0, 0, width, height);
                    },
                    SdlEvent::KeyUp {
                        keycode: Some(keycode),
                        ..
                    } => events.push(Event::KeyUp(keycode)),
                    SdlEvent::KeyDown {
                        keycode: Some(keycode),
                        ..
                    } => events.push(Event::KeyDown(keycode)),
                    _ => {}
                }
            }

            game_logic(passed_time, self.window.size(), &events);

            self.window.gl_swap_window();
        }
        Ok(())
    }
}
