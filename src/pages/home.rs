// use crate::components::counter_btn::Button;
use crate::components::project_block::Project_block;
use crate::components::*;
use leptos::prelude::*;

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    view! {
        <ErrorBoundary fallback=|errors| {
            view! {
                <h1>"Uh oh! Something went wrong!"</h1>

                <p>"Errors: "</p>
                // Render a list of errors as strings - good for development purposes
                <ul>
                    {move || {
                        errors
                            .get()
                            .into_iter()
                            .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                            .collect_view()
                    }}

                </ul>
            }
        }>

            <div class="flex justify-center text-slate-400 items-center h-screen bg-slate-900">

                <Project_block
                    title=String::from("Travel Tracker")
                    img=String::from("/static/travel.png")
                    description=String::from(
                        "This is a description of the content inside the box. Add more text here to see how it adjusts.",
                    )
                />

            </div>
        </ErrorBoundary>
    }
}
