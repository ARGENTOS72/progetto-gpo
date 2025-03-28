use dioxus::prelude::*;

use crate::{backend::glossary::Chapter, Route};

fn check_collapsed<'a>(chapter: &'a str, current_chapter: &'a str) -> &'a str {
    if chapter != current_chapter {
        "collapsed"
    } else {
        ""
    }
}

fn check_show<'a>(chapter: &'a str, current_chapter: &'a str) -> &'a str {
    if chapter == current_chapter {
        "show"
    } else {
        ""
    }
}

fn check_active<'a>(chapter: &'a str, current_chapter: &'a str) -> &'a str {
    if chapter == current_chapter {
        "active"
    } else {
        ""
    }
}

#[component]
pub fn GlossaryNavigation(chapters: Vec<Chapter>, current_chapter: Signal<String>) -> Element {
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

                        for chapter in chapters {
                            h2 {
                                class: "accordion-header",
                                id: "heading-{chapter.get_title()}",
                                class: "accordion-button {check_collapsed(chapter.get_title(), &current_chapter())}",
                                "type": "button",
                                "data-bs-toggle": "collapse",
                                "data-bs-target": "#collapse-{chapter.get_title()}",
                                "aria-expanded": "true",
                                "aria-controls": "collapse-{chapter.get_title()}",
                                onclick: {
                                    let clone_chapter = chapter.clone();

                                    move |_| {
                                        if current_chapter() != clone_chapter.get_title() {
                                            current_chapter.with_mut(|value| *value = clone_chapter.get_title().to_string());
                                        }
                                    }
                                },
                                "{chapter.get_title()}"
                            }
                            div {
                                id: "collapse-{chapter.get_title()}",
                                class: "accordion-collapse collapse {check_show(chapter.get_title(), &current_chapter())}",
                                "aria-labelledby": "#heading-{chapter.get_title()}",
                                "data-bs-parent": "#chaptersAccordion"
                            }
                            div {
                                class: "accordion-body",

                                ul {
                                    class: "list-unstyled ms-3",

                                    for sub_chapters in chapter.get_sub_chapters() {
                                        li {
                                            // class: "{ active: chapterSelected.subChapter == indexSubChapter && chapterSelected.chapter == index }",
                                            // @click="chapterSelected = { chapter: chapterSelected.chapter, subChapter: indexSubChapter }",
                                            // v-for="(subChapter, indexSubChapter) in subChaptersLoaded[index]\">{{ subChapter.title }}"
                                            // onclick:
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
