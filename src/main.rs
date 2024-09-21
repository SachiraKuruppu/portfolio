mod components;

use leptos::*;
use console_log;
use log::Level;
use components::tab_container::{Tab, TabContainer};

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(Level::Debug).expect("error initializing log");

    let tabs = vec! [
        Tab {
            title: "Tab 1",
            content: view! {
                <h1>"Tab 1"</h1>
            }
            .into_view(),
        },
        Tab {
            title: "Tab 2",
            content: view! {
                <h1>"Tab 2"</h1>
            }
            .into_view(),
        },
    ];

    mount_to_body(move || view! {
        <TabContainer tabs={tabs} />
    });
}
