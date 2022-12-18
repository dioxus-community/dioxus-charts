use dioxus::prelude::*;

fn main() {
    //env_logger::init();
    dioxus::web::launch(charts_demo::demo_element);
}
