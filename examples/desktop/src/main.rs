use dioxus_desktop::{Config, WindowBuilder};

fn main() {
    env_logger::init();
    dioxus_desktop::launch_cfg(
        charts_demo::demo_element,
        Config::new()
            .with_custom_head(format!("<style>{}</style>", include_str!("./tailwind.css")))
            .with_window(WindowBuilder::new().with_title("Dioxus Charts Examples")),
    );
}
