use dioxus::prelude::*;

use crate::backend::glossary::{get_sub_chapters, SubChapter};

#[component]
pub fn GlossaryNavigation(chapters: Vec<String>, sub_chapters: Vec<String>) -> Element {
    let sub_chapters = use_signal(|| {
        let mut sub_chapters = Vec::new();

        for chapter in &chapters {
            sub_chapters.push(get_sub_chapters(chapter));
        }

        sub_chapters
    });

    rsx! {
        div {
            class: "d-flex", style: "height: 100%;",

            div {
                class: "bg-light border-end p-3 flex-shrink-0",
                style: "width: 25%; overflow: auto;",

                div {
                    class: "accordion",
                    id: "chaptersAccordion",

                    div {
                        class: "accordion-item",

                        // "v-for": "(chapterTitle, index) in chapters",
                        // for(chapterTitle, index) in chapters {//TODO for
                            h2 {
                                class: "accordion-header",
                                id: "'heading' + index",
                                // onclick: |_| getSubChapter(chapterTitle, index),
                                class: "accordion-button",
                                // :class="{ collapsed: index != 0 }",
                                // type: "button",
                                // data-bs-toggle: "collapse",
                                // v-bind:data-bs-target="'#collapse' + index",
                                // aria-expanded="true",
                                // v-bind:aria-controls="'collapse' + index"
                                // {{ chapterTitle }}
                            }
                            div {
                                // v-bind:id="'collapse' + index",
                                class: "accordion-collapse collapse",
                                // :class="{ show: index == 0 }",
                                // v-bind:aria-labelledby="'#heading' + index",
                                // data-bs-parent: "#chaptersAccordion"
                            }
                            div {
                                class: "accordion-body",

                                ul {
                                    class: "list-unstyled ms-3",

                                    li {
                                        // class="{ active: chapterSelected.subChapter == indexSubChapter && chapterSelected.chapter == index }",
                                        // @click="chapterSelected = { chapter: chapterSelected.chapter, subChapter: indexSubChapter }",
                                        // v-for="(subChapter, indexSubChapter) in subChaptersLoaded[index]\">{{ subChapter.title }}"
                                    }
                                }
                            }
                        // }
                    }
                }
            }
        }
    }
}
