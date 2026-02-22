use leptos::prelude::*;

/// A project card linking to an external project
#[component]
pub fn ProjectCard(
    #[prop(into)] title: String,
    #[prop(into)] href: String,
    #[prop(into)] src: String,
    #[prop(into)] description: String,
    #[prop(default = false)] coming_soon: bool,
) -> impl IntoView {
    let alt_text = title.clone();

    let image_section = if src.is_empty() {
        view! {
            <div class="w-full h-full flex items-center justify-center bg-gradient-to-br from-slate-700 to-slate-600">
                <span class="text-slate-500 text-5xl">"+"</span>
            </div>
        }
        .into_any()
    } else {
        view! {
            <img
                src=src
                alt=alt_text
                class="w-full h-full object-cover group-hover:scale-105 transition-transform duration-200"
            />
        }
        .into_any()
    };

    view! {
        <a
            href=href
            target="_blank"
            rel="noopener noreferrer"
            class="group flex flex-col bg-slate-800 border border-slate-700 rounded-xl overflow-hidden \
                   hover:border-blue-500 hover:shadow-lg hover:shadow-blue-900/30 \
                   hover:scale-[1.02] transition-all duration-200"
        >
            <div class="relative h-44 bg-slate-700 overflow-hidden">
                {image_section}
                {coming_soon.then(|| view! {
                    <div class="absolute top-2 right-2 bg-blue-600 text-white text-xs font-semibold px-2.5 py-1 rounded-full">
                        "Coming Soon"
                    </div>
                })}
            </div>
            <div class="p-4 flex flex-col flex-1">
                <h3 class="text-white font-semibold text-lg mb-1">{title}</h3>
                <p class="text-slate-400 text-sm leading-relaxed">{description}</p>
            </div>
        </a>
    }
}
