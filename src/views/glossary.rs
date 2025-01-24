use dioxus::prelude::*;

// const GLOSSARY_CSS: Asset = asset!("/assets/styling/[...]");

#[component]
pub fn Glossary() -> Element {
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
