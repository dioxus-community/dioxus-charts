mod demo;

use dioxus_desktop::{Config, WindowBuilder};

fn main() {
    dioxus_desktop::launch_cfg(
        demo::demo_element,
        Config::new()
            .with_custom_head(format!(
                "<style>{}</style>",
                include_str!("./public/tailwind.css")
            ))
            .with_window(WindowBuilder::new().with_title("Dioxus Charts Examples")),
    );
}
