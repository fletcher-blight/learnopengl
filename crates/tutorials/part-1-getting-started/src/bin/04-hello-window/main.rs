fn main() -> anyhow::Result<()> {
    let window = winman::Window::new("1.4 Hello, Window", 1920, 1080)?;
    window.run(|_, _, _| {})
}
