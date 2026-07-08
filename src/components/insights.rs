// src/components/insights.rs — Liam Abourousse Portfolio × Leptos 0.7

use crate::components::pdf_modal::{is_pdf_href, PdfPreviewHandle};
use crate::content;
use leptos::prelude::*;

#[component]
pub fn Insights() -> impl IntoView {
    let articles = content::load().articles;
    let preview = use_context::<PdfPreviewHandle>().expect("PdfPreviewHandle");

    view! {
        <section class="insights-section">
            <div class="grid grid-cols-12 border-b">
                <div class="col-span-12 p-[clamp(16px,2.5vw,32px)] flex flex-col justify-center border-b">
                    <h2 class="section-heading font-sans text-[0.7rem] font-semibold tracking-normal uppercase text-muted">"DOCUMENTS & WRITING"</h2>
                </div>
            </div>
            {articles.into_iter().map(|a| {
                let href = a.href.clone();
                let title = a.title.clone();
                let kind = a.kind.clone();
                let preview_copy = preview;
                let title_for_click = title.clone();

                if is_pdf_href(&href) {
                    view! {
                        <button
                            type="button"
                            class="article-row grid grid-cols-[auto_1fr_auto] md:grid-cols-12 border-b hover-target w-full text-left"
                            on:click=move |_| preview_copy.show(href.clone(), title_for_click.clone())
                        >
                            <div class="col-span-1 md:col-span-2 p-[clamp(16px,2.5vw,32px)] border-r text-[0.6rem] font-semibold tracking-normal text-red flex items-center">{kind}</div>
                            <div class="col-span-1 md:col-span-9 p-[clamp(16px,2.5vw,32px)] border-r font-serif text-[clamp(0.9rem,1.5vw,1.3rem)] font-bold flex items-center">{title}</div>
                            <div class="col-span-1 md:col-span-1 p-[clamp(16px,2.5vw,32px)] text-[1.2rem] flex items-center justify-center art-arrow">"↗"</div>
                        </button>
                    }.into_any()
                } else {
                    view! {
                        <a href=href target="_blank" rel="noreferrer"
                           class="article-row grid grid-cols-[auto_1fr_auto] md:grid-cols-12 border-b hover-target">
                            <div class="col-span-1 md:col-span-2 p-[clamp(16px,2.5vw,32px)] border-r text-[0.6rem] font-semibold tracking-normal text-red flex items-center">{kind}</div>
                            <div class="col-span-1 md:col-span-9 p-[clamp(16px,2.5vw,32px)] border-r font-serif text-[clamp(0.9rem,1.5vw,1.3rem)] font-bold flex items-center">{title}</div>
                            <div class="col-span-1 md:col-span-1 p-[clamp(16px,2.5vw,32px)] text-[1.2rem] flex items-center justify-center art-arrow">"↗"</div>
                        </a>
                    }.into_any()
                }
            }).collect_view()}
        </section>
    }
}
