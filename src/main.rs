mod components;

use leptos::*;
use console_log;
use log::Level;
use components::tab_container::TabBar;

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(Level::Debug).expect("error initializing log");

    let on_tab_select_callback = Callback::from(move |id: usize| {
        log::info!("Selected tab: {}", id);
    });

    mount_to_body(move || view! {
        <TabBar
            titles={vec!["Tab 1".to_string(), "Tab 2".to_string(), "Tab 3".to_string()]}
            selected_tab_id={0}
            on_tab_select={on_tab_select_callback}
        />
    });
}
