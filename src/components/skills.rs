// src/components/skills.rs — Liam Abourousse Portfolio × Leptos 0.7

use crate::content;
use leptos::prelude::*;

#[component]
pub fn Skills() -> impl IntoView {
    let s = content::load().skills;
    let marquee = s.marquee.clone();
    let m2 = marquee.clone();
    view! {
        <section class="skills-section border-b">
            <div class="grid grid-cols-12 border-b">
                <div class="col-span-12 p-[clamp(16px,2.5vw,32px)] flex flex-col justify-center">
                    <h2 class="section-heading font-sans text-[0.7rem] font-semibold tracking-normal uppercase text-muted">"APPENDIX A: COMPETENCIES"</h2>
                </div>
            </div>
            <div class="skills-grid grid grid-cols-1 sm:grid-cols-2 xl:grid-cols-4 border-b">
            {s.buckets.into_iter().map(|b| {
                let cls = match b.color.as_str() {
                    "blue"   => "skill-bucket p-[clamp(16px,2.5vw,32px)] flex flex-col justify-center bg-blue text-white min-h-[140px]",
                    "yellow" => "skill-bucket p-[clamp(16px,2.5vw,32px)] flex flex-col justify-center bg-yellow text-black min-h-[140px]",
                    _        => "skill-bucket p-[clamp(16px,2.5vw,32px)] flex flex-col justify-center min-h-[140px]",
                };
                view! {
                    <div class=cls>
                        <h3 class="font-sans text-[0.65rem] font-bold tracking-normal mb-3">{b.title}</h3>
                        <p class="text-[0.85rem] leading-[1.7]">{b.list}</p>
                    </div>
                }
            }).collect_view()}
            </div>
            <div class="p-[clamp(16px,2.5vw,32px)] flex flex-col justify-center">
                <div class="overflow-hidden py-5 flex">
                    <div class="ticker-track flex w-max items-center pr-4">
                        <span class="font-serif text-[clamp(2rem,4vw,3.5rem)] font-black whitespace-nowrap pl-4">{marquee.clone()}</span>
                        <span class="marquee-outline font-serif text-[clamp(2rem,4vw,3.5rem)] font-black whitespace-nowrap pl-4">{m2.clone()}</span>
                    </div>
                    <div class="ticker-track flex w-max items-center pr-4" aria-hidden="true">
                        <span class="font-serif text-[clamp(2rem,4vw,3.5rem)] font-black whitespace-nowrap pl-4">{marquee.clone()}</span>
                        <span class="marquee-outline font-serif text-[clamp(2rem,4vw,3.5rem)] font-black whitespace-nowrap pl-4">{m2}</span>
                    </div>
                </div>
            </div>
        </section>
    }
}
