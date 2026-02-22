use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="text-center py-8 mt-8 border-t border-slate-800 text-slate-600 text-sm">
            <p>
                "Built with "
                <a
                    href="https://leptos.dev"
                    target="_blank"
                    rel="noopener noreferrer"
                    class="hover:text-slate-400 transition-colors"
                >
                    "Leptos"
                </a>
                " & "
                <a
                    href="https://tailwindcss.com"
                    target="_blank"
                    rel="noopener noreferrer"
                    class="hover:text-slate-400 transition-colors"
                >
                    "Tailwind CSS"
                </a>
            </p>
        </footer>
    }
}
