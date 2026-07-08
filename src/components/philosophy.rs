// src/components/philosophy.rs — Liam Abourousse Portfolio × Leptos 0.7

use crate::content;
use leptos::prelude::*;

#[component]
pub fn Philosophy() -> impl IntoView {
    let p = content::load().philosophy;
    let steps = p.steps.clone();

    view! {
        <section class="philosophy-section grid grid-cols-1 md:grid-cols-12 border-b" id="method">
            <div class="md:col-span-8 p-[clamp(32px,5vw,80px)] flex flex-col justify-center border-r bg-red text-white">
                <p class="philosophy-eyebrow font-serif text-[clamp(1.1rem,2.2vw,1.75rem)] font-bold tracking-normal uppercase text-white mb-7 leading-tight">{p.eyebrow}</p>
                <h2 class="hero-title font-serif text-[clamp(3rem,8vw,7.5rem)] font-black leading-[0.95] tracking-normal text-center text-white">
                    {p.headline_1}<br /><span class="italic">{p.headline_2}</span>
                </h2>
            </div>

            <div class="md:col-span-4 p-[clamp(32px,5vw,80px)] flex flex-col justify-center bg-offwhite text-text">
                <div class="geometric-deco grid grid-cols-3 gap-3 mb-8">
                    {steps.into_iter().enumerate().map(|(i, step)| view! {
                        <div class="border p-3 min-h-[72px] flex flex-col justify-between">
                            <span class="font-serif text-[1.1rem] font-bold text-red">{format!("{:02}", i + 1)}</span>
                            <span class="text-[0.55rem] tracking-normal uppercase text-muted">{step}</span>
                        </div>
                    }).collect_view()}
                </div>
                <p class="text-[0.9rem] leading-[1.8] text-text max-w-[360px]">
                    {p.body}
                </p>
            </div>
        </section>
    }
}
