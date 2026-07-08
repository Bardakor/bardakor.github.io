// src/components/projects.rs — Liam Abourousse Portfolio × Leptos 0.7

use crate::content;
use leptos::prelude::*;

fn project_row(p: content::Project) -> impl IntoView {
    let inner = view! {
        <div class="col-span-1 p-[clamp(16px,2.5vw,32px)] border-r font-serif text-[0.9rem] font-bold text-red flex items-center justify-center">{p.num}</div>
        <div class="col-span-1 md:col-span-5 p-[clamp(16px,2.5vw,32px)] flex flex-col justify-center gap-1.5 border-r">
            <h3 class="serif-huge font-serif text-[clamp(2.5rem,5vw,4.5rem)] font-black leading-none tracking-normal">{p.title}</h3>
            <span class="text-[0.6rem] tracking-normal text-muted">{p.subtitle}</span>
        </div>
        <div class="hidden md:flex md:col-span-3 p-[clamp(16px,2.5vw,32px)] flex-col justify-center gap-2 border-r">
            {p.tags.into_iter().map(|t| view! {
                <span class="inline-block text-[0.6rem] font-semibold tracking-normal bg-bg-dark text-text-inv py-1 px-2.5 rounded-sm mr-1.5">{t}</span>
            }).collect_view()}
        </div>
        <div class="hidden md:flex md:col-span-3 p-[clamp(16px,2.5vw,32px)] flex-col justify-center">
            <p class="text-[0.8rem] text-muted leading-[1.7]">{p.desc}</p>
        </div>
    };

    if p.href.is_empty() {
        view! {
            <div
                class="project-row project-row-static invisible grid grid-cols-[auto_1fr] md:grid-cols-12 border-b"
                data-accent=p.accent
            >
                {inner}
            </div>
        }.into_any()
    } else {
        let opens_new = p.href.starts_with("http") || p.href.ends_with(".pdf");
        let target = if opens_new { "_blank" } else { "_self" };
        let rel = if opens_new { "noreferrer" } else { "" };

        view! {
            <a
                href=p.href
                target=target
                rel=rel
                class="project-row invisible grid grid-cols-[auto_1fr] md:grid-cols-12 border-b hover-target"
                data-accent=p.accent
                style="text-decoration:none;color:inherit;"
            >
                {inner}
            </a>
        }.into_any()
    }
}

#[component]
pub fn Projects() -> impl IntoView {
    let projects = content::load().projects;
    view! {
        <section class="work-section" id="work">
            <div class="grid grid-cols-12 border-b">
                <div class="col-span-12 p-[clamp(16px,2.5vw,32px)] flex flex-col justify-center border-b">
                    <h2 class="section-heading font-sans text-[0.7rem] font-semibold tracking-normal uppercase text-muted">"SELECTED ARCHIVES"</h2>
                </div>
            </div>
            {projects.into_iter().map(project_row).collect_view()}
        </section>
    }
}
