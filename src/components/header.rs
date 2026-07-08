// src/components/header.rs — Liam Abourousse Portfolio × Leptos 0.7
//
// ── UI COMPONENT ─────────────────────────────────────────────────────────────
// Fully encapsulated component. Data is injected statically at compile time.
// ─────────────────────────────────────────────────────────────────────────────

use crate::content;
use leptos::prelude::*;

#[component]
pub fn Header() -> impl IntoView {
    let meta = content::load().meta;
    view! {
        <header class="ed-header sticky top-0 z-100 bg-bg grid grid-cols-1 md:grid-cols-12">
            <div class="hidden md:flex md:col-span-3 p-[clamp(16px,2.5vw,32px)] flex-col justify-center border-r border-b">
                <span class="font-sans text-[0.6rem] font-medium tracking-normal uppercase text-muted">{meta.issue}</span>
            </div>
            <div class="col-span-1 md:col-span-5 p-[clamp(16px,2.5vw,32px)] flex justify-center items-center border-r border-b">
                <span class="logo font-serif text-[clamp(0.8rem,1.2vw,1rem)] font-bold tracking-normal hover-target">"LIAM ABOUROUSSE"</span>
            </div>
            <nav class="col-span-1 md:col-span-4 p-[clamp(16px,2.5vw,32px)] flex flex-row flex-wrap gap-4 items-center justify-center border-b" aria-label="Main navigation">
                <a class="nav-link text-[0.65rem] font-semibold tracking-normal uppercase hover-target" href="#work">"Work"</a>
                <a class="nav-link text-[0.65rem] font-semibold tracking-normal uppercase hover-target" href="#method">"Method"</a>
                <a class="nav-link text-[0.65rem] font-semibold tracking-normal uppercase hover-target" href="#about">"CV"</a>
                <a class="nav-link text-[0.65rem] font-semibold tracking-normal uppercase hover-target" href="#contact">"Contact"</a>
            </nav>
        </header>
    }
}
