use dioxus::{html::mo::rspace, prelude::*};
use log::debug;

use crate::{
    backend::glossary::{Chapter, SubChapter, get_glossary_body_rsxed},
    Route,
};

fn check_collapsed<'a>(chapter: &'a str, current_chapter: &'a str) -> &'a str {
    debug!("{} vs {}", chapter, current_chapter);

    if chapter != current_chapter {
        "collapsed"
    } else {
        ""
    }
}

fn check_show<'a>(chapter: &'a str, current_chapter: &'a str) -> &'a str {
    debug!("{} vs {}", chapter, current_chapter);

    if chapter == current_chapter {
        "show"
    } else {
        ""
    }
}

fn check_active<'a>(chapter: &'a str, current_chapter: &'a str) -> &'a str {
    debug!("{} vs {}", chapter, current_chapter);
    
    if chapter == current_chapter {
        "active"
    } else {
        ""
    }
}

fn check_unit<'a>(current_subch: &'a str, chapter: &'a Chapter) -> bool {
    for subch in chapter.get_sub_chapters() {
        if current_subch == subch.get_id() {
            return true;
        }
    }
    false
}


#[component]
pub fn GlossaryNavigation(chapters: Vec<Chapter>, current_subch: Signal<String>) -> Element {
    let unit: usize = current_subch
        .read()
        .split("-")
        .next()
        .unwrap_or("00")
        .parse()
        .unwrap_or(0);
    let mut current_chapter: Signal<String> = use_signal(|| format!("{:02}-00", &unit));
    let chapters_rc = std::rc::Rc::new(chapters); // Wrap in Rc
    // println!("{}", current_subch());

    rsx! {
        div {
            class: "bg-light border-end p-3 flex-shrink-0 text-break",
            style: "width: 25%; overflow: auto;",
            div {
                class: "accordion position-sticky",
                id: "chaptersAccordion",
                for chapter in chapters_rc.iter() { // Usa iter() su Rc
                    div {
                        class: "accordion-item",
                        h2 {
                            class: "accordion-header text-break",
                            id: "heading-{chapter.get_title()}",
                            class: "accordion-button {check_collapsed(chapter.get_id(), &current_chapter())}",
                            "type": "button",
                            "data-bs-toggle": "collapse",
                            "data-bs-target": "#collapse-{chapter.get_title()}",
                            "aria-expanded": "true",
                            "aria-controls": "collapse-{chapter.get_title()}",
                            onclick: {
                                let clone_chapter = chapter.clone();

                                move |_| {
                                    if current_chapter() != clone_chapter.get_id() {
                                        current_chapter.set(clone_chapter.get_id().to_string());
                                        current_subch.set(clone_chapter.get_id().to_string());
                                    }
                                }
                            },
                            "{chapter.get_title()}"
                        }
                        div {
                            id: "collapse-{chapter.get_title()}",
                            class: "accordion-collapse collapse text-break {check_show(chapter.get_id(), &current_chapter())}",
                            "aria-labelledby": "#heading-{chapter.get_title()}",
                            "data-bs-parent": "#chaptersAccordion",
                            div {
                                class: "accordion-body",

                                div {
                                    class: "list-group list-group-flush list-group-item-action",

                                    for sub_chapter in chapter.get_sub_chapters() {
                                        button {
                                            class: "list-group-item-action list-group-item text-break",
                                            "type": "button",
                                            onclick: {
                                                let chapters = chapters_rc.clone(); // Clone Rc
                                                let sub_chapter = sub_chapter.clone();
                                                move |_| {
                                                    let unit: usize = current_chapter.read()
                                                        .split("-")
                                                        .next()
                                                        .unwrap_or("00")
                                                        .parse()
                                                        .unwrap_or(0);
                                                    if let Some(ch) = chapters.get(unit) {
                                                        if !check_unit(&sub_chapter.get_id(), ch){
                                                            let new_unit: usize = sub_chapter.get_id()
                                                                .split("-")
                                                                .next()
                                                                .unwrap_or("00")
                                                                .parse()
                                                                .unwrap_or(0);
                                                            current_chapter.set(format!("{:02}-00", &new_unit));
                                                        }
                                                        if current_subch() != sub_chapter.get_id().to_string(){
                                                            current_subch.set(sub_chapter.get_id().to_string());
                                                        }
                                                    }
                                                }
                                            },
                                            "{sub_chapter.get_title()}"
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

#[component]
pub fn GlossaryContent(chapters: Vec<Chapter>, chapter_id: Signal<String>) -> Element {
    let tmp_ch = chapter_id.read();

    let ch: Vec<&str> = tmp_ch.split("-").collect();
    let unit_num: usize = ch.get(0).unwrap_or(&"00").parse().unwrap_or(0);
    let subch_num: usize = ch.get(1).unwrap_or(&"00").parse().unwrap_or(0);

    let unit = chapters.get(unit_num).unwrap().clone();
    let mut chapter = (unit.get_title(), unit.get_content());

    let tmp_ch_list = unit.get_sub_chapters();

    println!("gappytolo{:?}", ch);
    if subch_num != 0 {
        let subch = tmp_ch_list.get(subch_num-1).expect("Coglione non cancellare i capitoli, testa di cazzo!");
        chapter = (subch.get_title(), subch.get_content());
    }

    let content_rsx = get_glossary_body_rsxed(&mut chapter.1.to_string()).unwrap();

    rsx! {
        div {
            class: "p-4",
            style: "width: 73%; overflow: auto;",
            h1 {
                "{chapter.0}"
            }
            {content_rsx}
        }
    }
}
