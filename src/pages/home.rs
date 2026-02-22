use crate::components::project_block::ProjectBlock;
use leptos::prelude::*;

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="flex justify-center text-slate-400 items-center h-screen bg-slate-900">
            <ProjectBlock
                title="Travel Tracker"
                href="https://travel.stevenguido.com/"
                src="/static/travel.png"
                description="This is a description of the content inside the box. Add more text here to see how it adjusts."
            />
        </div>
    }
}
