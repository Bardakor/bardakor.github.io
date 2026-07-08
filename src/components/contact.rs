// src/components/contact.rs — Liam Abourousse Portfolio × Leptos 0.7

use crate::components::pdf_modal::PdfPreviewHandle;
use crate::content;
use leptos::prelude::*;

#[component]
pub fn Contact() -> impl IntoView {
    let c = content::load();
    let ct = c.contact;
    let preview = use_context::<PdfPreviewHandle>().expect("PdfPreviewHandle");
    let cv_en = ct.cv_en.clone();
    let cv_title = "Software Engineer — CV EN".to_string();

    view! {
        <footer class="contact-section" id="contact">
            <div class="grid grid-cols-1 md:grid-cols-12 border-b">
                <div class="col-span-full p-[clamp(32px,5vw,80px)] flex flex-col items-center justify-center bg-bg-dark text-white contact-hero">
                    <h2 class="hero-title font-serif text-[clamp(3rem,8vw,7.5rem)] font-black leading-[0.95] tracking-normal text-center text-white">
                        "SHIP DATA"<br /><span class="italic">"THAT WORKS."</span>
                    </h2>
                </div>
            </div>
            <div class="grid grid-cols-1 md:grid-cols-10 border-b">
                <a href=format!("mailto:{}", ct.email)
                   class="col-span-1 md:col-span-2 contact-link flex items-center justify-center font-sans text-[0.75rem] font-bold tracking-normal py-5 md:border-r border-b md:border-b-0 hover-target">"EMAIL"</a>
                <a href=format!("tel:{}", ct.phone)
                   class="col-span-1 md:col-span-2 contact-link flex items-center justify-center font-sans text-[0.75rem] font-bold tracking-normal py-5 md:border-r border-b md:border-b-0 hover-target">"PHONE"</a>
                <a href=ct.github target="_blank" rel="noreferrer"
                   class="col-span-1 md:col-span-2 contact-link flex items-center justify-center font-sans text-[0.75rem] font-bold tracking-normal py-5 md:border-r border-b md:border-b-0 hover-target">"GITHUB"</a>
                <a href=ct.linkedin target="_blank" rel="noreferrer"
                   class="col-span-1 md:col-span-2 contact-link flex items-center justify-center font-sans text-[0.75rem] font-bold tracking-normal py-5 md:border-r border-b md:border-b-0 hover-target">"LINKEDIN"</a>
                <button type="button"
                   class="col-span-1 md:col-span-2 contact-link flex items-center justify-center font-sans text-[0.75rem] font-bold tracking-normal py-5 hover-target"
                   on:click=move |_| preview.show(cv_en.clone(), cv_title.clone())>"CV EN"</button>
            </div>
            <div class="grid grid-cols-1 md:grid-cols-12">
                <div class="col-span-1 md:col-span-6 text-[0.55rem] tracking-normal text-muted py-3.5 px-[clamp(16px,2.5vw,32px)] md:border-r">{c.footer.left}</div>
                <div class="col-span-1 md:col-span-6 text-[0.55rem] tracking-normal text-muted py-3.5 px-[clamp(16px,2.5vw,32px)] text-right">{c.footer.right}</div>
            </div>
        </footer>
    }
}
