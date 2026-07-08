// src/components/pdf_modal.rs — macOS-style in-app PDF preview

use leptos::prelude::*;
use wasm_bindgen::JsCast;

#[derive(Clone, Copy)]
pub struct PdfPreviewHandle {
    pub open: ReadSignal<bool>,
    set_open: WriteSignal<bool>,
    pub src: ReadSignal<String>,
    set_src: WriteSignal<String>,
    pub title: ReadSignal<String>,
    set_title: WriteSignal<String>,
}

impl PdfPreviewHandle {
    pub fn show(&self, src: impl Into<String>, title: impl Into<String>) {
        self.set_src.set(src.into());
        self.set_title.set(title.into());
        self.set_open.set(true);
    }

    pub fn close(&self) {
        self.set_open.set(false);
        self.set_src.set(String::new());
    }
}

pub fn provide_pdf_preview() -> PdfPreviewHandle {
    let (open, set_open) = signal(false);
    let (src, set_src) = signal(String::new());
    let (title, set_title) = signal(String::new());

    let handle = PdfPreviewHandle {
        open,
        set_open,
        src,
        set_src,
        title,
        set_title,
    };
    provide_context(handle);
    handle
}

#[component]
pub fn PdfPreviewModal(handle: PdfPreviewHandle) -> impl IntoView {
    let open = handle.open;
    let src = handle.src;
    let title = handle.title;

    Effect::new(move |_| {
        if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
            if let Some(body) = doc.body() {
                let el: &web_sys::HtmlElement = body.unchecked_ref();
                if open.get() {
                    let _ = el.class_list().add_1("pdf-modal-open");
                } else {
                    let _ = el.class_list().remove_1("pdf-modal-open");
                }
            }
        }
    });

    let close = move |_| handle.close();

    view! {
        <div
            class="pdf-modal-overlay"
            class:hidden=move || !open.get()
            aria-hidden=move || (!open.get()).to_string()
            on:click=close
        >
            <div
                class="pdf-window"
                role="dialog"
                aria-modal="true"
                aria-label=move || title.get()
                on:click=|ev| ev.stop_propagation()
            >
                <div class="pdf-window-chrome">
                    <div class="pdf-traffic-lights">
                        <button type="button" class="traffic-light close" aria-label="Close preview" on:click=close></button>
                        <span class="traffic-light minimize" aria-hidden="true"></span>
                        <span class="traffic-light maximize" aria-hidden="true"></span>
                    </div>
                    <p class="pdf-window-title">{move || title.get()}</p>
                </div>
                <div class="pdf-window-body">
                    {move || {
                        if open.get() && !src.get().is_empty() {
                            view! {
                                <iframe
                                    class="pdf-frame"
                                    src=src.get()
                                    title=title.get()
                                ></iframe>
                            }.into_any()
                        } else {
                            view! { <div></div> }.into_any()
                        }
                    }}
                </div>
            </div>
        </div>
    }
}

pub fn is_pdf_href(href: &str) -> bool {
    href.to_ascii_lowercase().ends_with(".pdf")
}
