use dioxus::prelude::*;

fn main() {
    env_logger::init();
    launch(charts_demo::demo_element);
}
