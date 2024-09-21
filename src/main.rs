mod components;

use leptos::*;
use std::sync::Arc;
use components::tab_container::TabBar;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! {
        <TabBar
            titles={vec!["Tab 1".to_string(), "Tab 2".to_string(), "Tab 3".to_string()]}
            selected_tab_id={0}
            on_tab_select={Arc::new(|id| log::info!("Selected tab: {}", id))}
        />
    });
}
