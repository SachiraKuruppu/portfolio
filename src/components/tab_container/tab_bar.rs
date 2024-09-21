use leptos::*;

#[component]
fn TabTitle(title: String, selected: bool, on_click: impl Fn() + 'static) -> impl IntoView {
    let selected_styles = "inline-block p-4 text-blue-600 border-b-2 border-blue-600 rounded-t-lg active dark:text-blue-500 dark:border-blue-500";
    let unselected_styles = "inline-block p-4 border-b-2 border-transparent rounded-t-lg hover:text-gray-600 hover:border-gray-300 dark:hover:text-gray-300";

    let class = if selected { selected_styles } else { unselected_styles };

    view! {
        <li className="me-2">
            <button
                on:click={move |_| on_click()}
                class={class}
            >
                {title}
            </button>
        </li>
    }
}

#[component]
pub fn TabBar(titles: Vec<String>, selected_tab_id: usize, on_tab_select: Callback<usize>) -> impl IntoView {
    view! {
        <div class="text-sm font-medium text-center text-gray-500 border-b border-gray-200 dark:text-gray-400 dark:border-gray-700">
            <ul class="flex flex-wrap -mb-px">
                {
                    titles.iter().enumerate().map(|(index, title)| {
                        let selected = index == selected_tab_id;

                        view! {
                            <TabTitle title={title.clone()} selected={selected} on_click={move || on_tab_select.call(index)} />
                        }
                    }).collect::<Vec<_>>()
                }
            </ul>
        </div>
    }
}
