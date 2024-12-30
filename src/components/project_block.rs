use leptos::prelude::*;

/// A parameterized incrementing button
#[component]
pub fn Project_block(title: String, img: String, description: String) -> impl IntoView {
    view! {
        <a href="https://travel.stevenguido.com/">
            <div class="box bg-white border border-gray-300 rounded-lg shadow-lg overflow-hidden w-80
            hover:shadow-xl hover:scale-105 transform transition">
                <div class="box-title dark:bg-blue-500 text-white text-center text-lg font-bold py-2">
                    {title}
                </div>
                <div class="box-image flex justify-center items-center p-4">
                    <img src=img alt="Placeholder Image" class="rounded-lg"/>
                </div>
                <div class="box-description p-4 text-sm text-gray-600 text-center">
                    {description}
                </div>

            </div>
        </a>
    }
}
