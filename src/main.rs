extern crate anyhow;
extern crate env_logger;
extern crate gl;
extern crate glfw;
extern crate glm;
extern crate image;
extern crate log;
extern crate thiserror;

use glfw::Context;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to create window")]
    WindowFailure,
}

fn main() -> anyhow::Result<()> {
    env_logger::init();
    log::info!("Window initialisation ...");

    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS)?;
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));
    #[cfg(target_os = "macos")]
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

    let (mut window, events) = glfw
        .create_window(1920, 1080, "Rust Renderer", glfw::WindowMode::Windowed)
        .ok_or(Error::WindowFailure)?;

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    gl::load_with(|symbol| window.get_proc_address(symbol));

    log::info!("Window initialisation ... complete");

    while !window.should_close() {
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        window.swap_buffers();
        glfw.poll_events();

        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::FramebufferSize(width, height) => unsafe {
                    gl::Viewport(0, 0, width, height)
                },
                glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Release, _) => {
                    window.set_should_close(true)
                }
                _ => {}
            }
        }
    }

    log::info!("Goodbye");
    Ok(())
}
