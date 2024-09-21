use leptos::*;

pub struct Tab {
    pub title: &'static str,
    pub content: View,
}

impl Tab {
    pub fn get_content(&self) -> View {
        view! {
            <div class="w-fit mx-auto">{self.content.clone()}</div>
        }
        .into_view()
    }
}
