use leptos::prelude::*;

#[component]
pub fn Hero() -> impl IntoView {
    view! {
        <header class="flex flex-col items-center justify-center pt-20 pb-12 px-4 text-center">
            <h1 class="text-5xl font-bold text-white mb-3 tracking-tight">"Steven Guido"</h1>
            <p class="text-xl text-slate-400 mb-8">"Robotics Gachizei"</p>
            <div class="flex items-center gap-6">
                <a
                    href="https://github.com/fuzzybear3"
                    target="_blank"
                    rel="noopener noreferrer"
                    class="text-slate-400 hover:text-white transition-colors font-medium"
                >
                    "GitHub"
                </a>
                <span class="text-slate-700">"·"</span>
                <a
                    href="mailto:stevenrguido@gmail.com"
                    class="text-slate-400 hover:text-white transition-colors font-medium"
                >
                    "Email"
                </a>
            </div>
        </header>
    }
}
