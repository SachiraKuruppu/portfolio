use leptos::*;

use crate::components::tab_container::tab_bar::TabBar;

use super::tab::Tab;

#[component]
pub fn TabContainer(tabs: Vec<Tab>) -> impl IntoView {
    let (read_selected_tab_id, write_selected_tab_id) = create_signal(0);
    
    let titles = tabs.iter().map(move |tab| {
        tab.title.to_string()
    }).collect::<Vec<_>>();

    let on_tab_select_callback = Callback::from(move |id: usize| {
        write_selected_tab_id.set(id);
    });

    let memoized_tab_content = create_memo(move |_| {
        let selected_tab_id = read_selected_tab_id.get();
        let selected_tab = &tabs[selected_tab_id];

        selected_tab.get_content()
    });

    view! {
        <TabBar titles={titles} read_selected_tab_id={read_selected_tab_id} on_tab_select={on_tab_select_callback} />
        <main class="w-full">
            {move || memoized_tab_content.get()}
        </main>
    }
}
