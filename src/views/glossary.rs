use dioxus::prelude::*;
use crate::components::GlossaryNavigation;

// const GLOSSARY_CSS: Asset = asset!("/assets/styling/[...]");

#[component]
pub fn Glossary() -> Element {
    rsx! {
        GlossaryNavigation {}
        div {
            class: "p-4",
            style: "width: 75%; overflow: auto;",

            h1 {
                class: "mb-3",

                // "{{ (subChaptersLoaded === undefined || subChaptersLoaded === null || Object.keys(subChaptersLoaded).length === 0) ? \"Loading...\" : subChaptersLoaded[chapterSelected.chapter][chapterSelected.subChapter] }}"
            }
        }
    }
}
