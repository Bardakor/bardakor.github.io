// src/components/experience.rs — Liam Abourousse Portfolio × Leptos 0.7

use crate::content;
use leptos::prelude::*;

#[component]
pub fn Experience() -> impl IntoView {
    let entries = content::load().experience;
    view! {
        <section class="experience-section" id="about">
            <div class="grid grid-cols-12 border-b">
                <div class="col-span-12 p-[clamp(16px,2.5vw,32px)] flex flex-col justify-center border-b">
                    <h2 class="section-heading font-sans text-[0.7rem] font-semibold tracking-normal uppercase text-muted">"CURRICULUM VITAE"</h2>
                </div>
            </div>
            {entries.into_iter().map(|e| {
                let has_sub = !e.subtitle.is_empty();
                view! {
                    <div class="cv-row grid grid-cols-1 md:grid-cols-12 border-b hover-target">
                        <div class="md:col-span-2 p-[clamp(16px,2.5vw,32px)] flex flex-col justify-center md:border-r font-sans text-[0.7rem] font-semibold tracking-normal text-red">{e.years}</div>
                        <div class="md:col-span-4 p-[clamp(16px,2.5vw,32px)] flex flex-col justify-center md:border-r gap-1">
                            <h4 class="font-serif text-[clamp(1rem,1.5vw,1.3rem)] font-bold">{e.role}</h4>
                            <p class="text-[0.72rem] font-semibold tracking-normal text-text uppercase">{e.org}</p>
                            {has_sub.then(|| view! {
                                <p class="text-[0.62rem] tracking-normal text-red/80 uppercase leading-relaxed">{e.subtitle}</p>
                            })}
                        </div>
                        <div class="md:col-span-6 p-[clamp(16px,2.5vw,32px)] flex flex-col justify-center">
                            <p class="text-[0.8rem] text-muted leading-[1.7]">{e.desc}</p>
                        </div>
                    </div>
                }
            }).collect_view()}
        </section>
    }
}
