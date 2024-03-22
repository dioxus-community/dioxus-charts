use dioxus::{
    desktop::{Config, WindowBuilder},
    prelude::*,
};

fn main() {
    env_logger::init();

    let config = Config::new()
        .with_custom_head(format!("<style>{}</style>", include_str!("./tailwind.css")))
        .with_window(WindowBuilder::new().with_title("Dioxus Charts Examples"));

    let builder = LaunchBuilder::desktop().with_cfg(config);
    builder.launch(charts_demo::demo_element);
}
