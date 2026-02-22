use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, StaticSegment};

mod components;
mod pages;

use crate::pages::home::Home;
use crate::pages::not_found::NotFound;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Html attr:lang="en" attr:dir="ltr" attr:data-theme="dark"/>
        <Title text="Steven Guido"/>

        <Router>
            <Routes fallback=|| view! { <NotFound/> }>
                <Route path=StaticSegment("") view=Home/>
            </Routes>
        </Router>
    }
}
