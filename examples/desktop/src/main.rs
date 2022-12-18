fn main() {
    env_logger::init();
    dioxus::desktop::launch_cfg(charts_demo::demo_element, |c| {
        c.with_custom_head(format!("<style>{}</style>", include_str!("./tailwind.css")))
            .with_window(|w| w.with_title("Dioxus Charts Examples"))
    });
}
