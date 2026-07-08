// src/main.rs — Liam Abourousse Portfolio × Leptos 0.7
//
// ── HOW TO EDIT CONTENT ───────────────────────────────────────────────────────
// Edit /content.json — every section of the portfolio reads from that file.
// After saving, run `trunk serve` and the page will rebuild automatically.
// ─────────────────────────────────────────────────────────────────────────────

mod components;
mod content;
mod gsap;

use leptos::prelude::*;
use leptos_meta::*;
use wasm_bindgen::JsCast;

use components::{
    contact::Contact, experience::Experience, header::Header, hero::Hero, insights::Insights,
    pdf_modal::{provide_pdf_preview, PdfPreviewModal}, philosophy::Philosophy, projects::Projects,
    recognition::Recognition, skills::Skills, splash::Splash, ticker::Ticker,
};

#[component]
fn App() -> impl IntoView {
    // Provide Meta Context for dynamic SEO updates
    provide_meta_context();

    // Boot animations after first render (~= DOMContentLoaded)
    Effect::new(|_| {
        let cb = wasm_bindgen::closure::Closure::once(move || {
            gsap::play_splash();
        });
        web_sys::window()
            .unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(cb.as_ref().unchecked_ref(), 200)
            .unwrap();
        cb.forget();
    });

    let pdf_preview = provide_pdf_preview();

    view! {
        <Title text="Liam Abourousse — Software / Data Engineer" />
        <Meta name="description" content="Liam Abourousse — Paris-based Software / Data Engineer building Python backends, data products, and B2B SaaS tooling for maritime operations." />

        <Splash />
        <PdfPreviewModal handle=pdf_preview />

        <main id="smooth-wrapper">
            <Header />

            <Ticker />

            <Hero />

            <Projects />

            <Ticker reverse=true />

            <Philosophy />

            <Experience />

            <Skills />

            <Recognition />

            <Insights />

            <Contact />
        </main>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}
