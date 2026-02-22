use crate::components::footer::Footer;
use crate::components::hero::Hero;
use crate::components::project_card::ProjectCard;
use crate::models::PROJECTS;
use leptos::prelude::*;

const ACTIVE_TAG: &str =
    "px-4 py-1.5 rounded-full text-sm font-medium bg-blue-600 text-white transition-colors";
const INACTIVE_TAG: &str =
    "px-4 py-1.5 rounded-full text-sm font-medium bg-slate-700 text-slate-300 hover:bg-slate-600 transition-colors";

#[component]
pub fn Home() -> impl IntoView {
    let mut all_tags: Vec<&'static str> = PROJECTS
        .iter()
        .flat_map(|p| p.tags.iter().copied())
        .collect();
    all_tags.sort_unstable();
    all_tags.dedup();

    let (selected_tag, set_selected_tag) = signal::<Option<&'static str>>(None);

    let filtered = Memo::new(move |_| match selected_tag() {
        None => PROJECTS.to_vec(),
        Some(tag) => PROJECTS
            .iter()
            .filter(|p| p.tags.contains(&tag))
            .cloned()
            .collect(),
    });

    view! {
        <div class="min-h-screen bg-gradient-to-br from-slate-900 via-slate-800 to-slate-900 text-slate-100 flex flex-col">
            <Hero/>

            // Tag filter bar
            <div class="flex justify-center gap-2 py-4 px-8 flex-wrap">
                <button
                    class=move || if selected_tag().is_none() { ACTIVE_TAG } else { INACTIVE_TAG }
                    on:click=move |_| set_selected_tag(None)
                >
                    "All"
                </button>
                {all_tags.iter().copied().map(|tag| view! {
                    <button
                        class=move || if selected_tag() == Some(tag) { ACTIVE_TAG } else { INACTIVE_TAG }
                        on:click=move |_| set_selected_tag(Some(tag))
                    >
                        {tag}
                    </button>
                }).collect_view()}
            </div>

            // Project grid
            <main class="flex-1 grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6 p-8 max-w-6xl mx-auto w-full items-start">
                <For
                    each=move || filtered()
                    key=|p| p.title
                    children=|p| view! {
                        <ProjectCard
                            title=p.title
                            href=p.href
                            src=p.src
                            description=p.description
                            coming_soon=p.coming_soon
                        />
                    }
                />
            </main>

            <Footer/>
        </div>
    }
}
