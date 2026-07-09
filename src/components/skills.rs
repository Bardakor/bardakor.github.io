// src/components/skills.rs — Liam Abourousse Portfolio × Leptos 0.7

use crate::content;
use leptos::prelude::*;

#[component]
pub fn Skills() -> impl IntoView {
    let s = content::load().skills;
    let skill_items = s
        .marquee
        .split('/')
        .map(str::trim)
        .filter(|item| !item.is_empty())
        .map(String::from)
        .collect::<Vec<_>>();

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
                <div class="skills-static-grid grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-5 border">
                    {skill_items.into_iter().map(|item| {
                        view! {
                            <div class="skill-chip min-h-[72px] px-4 py-3 flex items-center justify-center text-center font-serif text-[clamp(1.2rem,2.5vw,2rem)] font-black uppercase">
                                {item}
                            </div>
                        }
                    }).collect_view()}
                </div>
            </div>
        </section>
    }
}
