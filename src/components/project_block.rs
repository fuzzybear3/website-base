use leptos::prelude::*;

/// A project card linking to an external project
#[component]
pub fn ProjectBlock(
    #[prop(into)] title: String,
    #[prop(into)] href: String,
    #[prop(into)] src: String,
    #[prop(into)] description: String,
) -> impl IntoView {
    view! {
        <a href=href>
            <div class="bg-white border border-gray-300 rounded-lg shadow-lg overflow-hidden w-80
            hover:shadow-xl hover:scale-105 transform transition">
                <div class="dark:bg-blue-500 text-white text-center text-lg font-bold py-2">
                    {title.clone()}
                </div>
                <div class="flex justify-center items-center p-4">
                    <img src=src alt=title class="rounded-lg"/>
                </div>
                <div class="p-4 text-sm text-gray-600 text-center">
                    {description}
                </div>
            </div>
        </a>
    }
}
