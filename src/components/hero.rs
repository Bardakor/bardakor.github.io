// src/components/hero.rs — Liam Abourousse Portfolio × Leptos 0.7
//
// ── UI COMPONENT ─────────────────────────────────────────────────────────────
// Fully encapsulated component. Data is injected statically at compile time.
// ─────────────────────────────────────────────────────────────────────────────

use crate::content;
use leptos::prelude::*;

#[component]
pub fn Hero() -> impl IntoView {
    let c = content::load();
    let h = c.hero;
    let stats = h.stats.clone();
    view! {
        <section class="hero-section grid grid-cols-1 md:grid-cols-12 border-b">
            <div class="hidden md:flex md:col-span-2 p-[clamp(16px,2.5vw,32px)] flex-col justify-center border-r">
                <p class="font-sans text-[0.6rem] font-medium tracking-normal uppercase text-muted [writing-mode:vertical-rl] [text-orientation:mixed]">{c.meta.edition}</p>
            </div>
            <div class="col-span-full md:col-span-6 p-[clamp(32px,5vw,80px)] flex flex-col justify-center border-r">
                <h1 class="hero-title font-serif text-[clamp(3rem,8vw,7.5rem)] font-black leading-[0.95] tracking-normal">
                    <span class="line">{h.headline_1}</span>
                    <span class="line italic">{h.headline_2}</span>
                </h1>
                <p class="hero-body text-[clamp(0.85rem,1.1vw,1rem)] leading-[1.7] text-muted max-w-[560px] mt-7">{h.body}</p>
            </div>
            <div class="col-span-full md:col-span-4 flex flex-col">
                <figure class="portrait-panel relative min-h-[360px] md:min-h-[520px] border-b overflow-hidden bg-bg-dark">
                    <img
                        src=h.portrait
                        alt=h.portrait_alt
                        loading="eager"
                        decoding="async"
                        class="absolute inset-0 h-full w-full object-cover object-[50%_38%] mix-blend-luminosity"
                    />
                    <figcaption class="absolute left-0 bottom-0 right-0 border-t bg-bg/90 px-[clamp(16px,2.5vw,32px)] py-3 text-[0.58rem] font-semibold tracking-normal text-text uppercase">
                        "Paris // Software Engineer // Maritime data"
                    </figcaption>
                </figure>
                <div class="grid grid-cols-3 md:grid-cols-1 flex-1">
                    {stats.into_iter().map(|s| {
                        view! {
                            <div class="stat-block flex flex-col justify-center p-[clamp(16px,2.5vw,32px)] gap-1 border-r last:border-r-0 md:border-r-0 md:border-b md:last:border-b-0">
                                <span class="font-serif text-[clamp(1.5rem,3vw,2.8rem)] font-black text-red">{s.num}</span>
                                <span class="text-[0.58rem] tracking-normal text-muted uppercase">{s.label}</span>
                            </div>
                        }
                    }).collect_view()}
                </div>
            </div>
        </section>
    }
}
