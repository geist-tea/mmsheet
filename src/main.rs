#![allow(non_snake_case)]

pub mod character;
pub mod gui;
pub mod rulebook;

use character::Character;
use dioxus::prelude::*;
use dioxus_desktop::{Config, WindowBuilder};
use lazy_static::lazy_static;
use native_dialog::MessageDialog;
use rulebook::Rulebook;

use crate::gui::*;

lazy_static! {
    static ref RULEBOOK: Rulebook<'static> = match Rulebook::new() {
        Ok(rb) => {
            println!("Loaded {} advantages", rb.advantages.len());
            println!("Loaded {} powers", rb.powers.len());
            rb
        }
        Err(e) => {
            let _ = MessageDialog::new()
                .set_title("Error")
                .set_type(native_dialog::MessageType::Error)
                .set_text(e.as_str())
                .show_alert();
            panic!("{}", e);
        }
    };
}

fn main() {
    dioxus_desktop::launch_cfg(
        App,
        Config::default().with_window(WindowBuilder::new().with_title("mmsheet")),
    )
}

enum Modal {
    Hidden,
    AdvantageSearch,
}

fn App(cx: Scope) -> Element {
    use_shared_state_provider(cx, || vec![Character::new(&RULEBOOK)]);
    use_shared_state_provider(cx, || 0 as usize);
    let modal_state = use_state(cx, || Modal::Hidden);
    let sheets = use_shared_state::<Vec<Character>>(cx).unwrap();
    let active_idx = use_shared_state::<usize>(cx).unwrap();
    let search = use_ref(cx, String::new);

    let aidx_val = *active_idx.read();

    render! {
        CharacterTabs {}
        CharacterInfo {}
        AbilityScores {}
        Defense {}
        Offense {}
        Skills {}
        div {
            id:"advantages",
            h1 {
                "Advantages"
                button {
                    class: "add",
                    onclick: move |_| modal_state.set(Modal::AdvantageSearch),
                    "+"
                }
            }
            table {
                sheets.read()[aidx_val].advantages.iter().enumerate().map(|(idx, a)| {
                    let notes = a.notes.clone();
                    rsx! {
                        tr {
                            td {"{RULEBOOK.advantages[a.id].name}" }
                            match a.ranks {
                                Some(val) => rsx! {
                                    td {
                                        input {
                                            r#type: "number",
                                            value: "{val}",
                                            oninput: move |event| {
                                                let val = event.value.clone();
                                                match val.parse::<i32>() {
                                                    Ok(num) => sheets.write()[aidx_val].set_advantage_ranks(idx, num),
                                                    _ => ()
                                                }
                                            }
                                        }
                                    }
                                },
                                None => rsx! {td{}}
                            }
                            match notes {
                                Some(val) => rsx! {
                                    td {
                                        input {
                                            r#type: "text",
                                            value: "{val}",
                                            size: "40",
                                            oninput: move |event| {
                                                let val = sheets.write()[aidx_val].set_advantage_note(idx, event.value.clone());
                                            }
                                        }
                                    }
                                },
                                None => rsx! {td{}}
                            }
                            td {
                                button {
                                    onclick: move |_| { sheets.write()[aidx_val].delete_advantage(idx) },
                                    "x"
                                }
                            }
                        }
                    }
                })
            }
        }
        match modal_state.get() {
            Modal::Hidden => rsx! {""},
            Modal::AdvantageSearch => rsx! {
                div {
                    id: "advantages_popup",
                    class: "modal",
                    div {
                        class: "modal-content",
                        h1 {
                            "Add Advantages"
                            button {
                                class: "add",
                                onclick: move |event| modal_state.set(Modal::Hidden),
                                "x"
                            }
                        }
                        input {
                            r#type: "text",
                            value: "{search.read()}",
                            oninput: |event| { search.with_mut(|s| { *s = event.value.clone() })},
                        }
                        table {
                            tr {
                                th {"Name"}
                                th {"Type"}
                                th {colspan: "2", "Summary"}
                            }
                            RULEBOOK.advantages.iter().enumerate().filter(|(_, a)| a.name.contains(search.read().as_str())).filter(|(idx, _)| !sheets.read()[aidx_val].has_advantage(*idx)).map(|(idx, a)| {
                                rsx! {
                                    tr {
                                        td {
                                            "{a.name}"
                                        }
                                        td {
                                            "{a.r#type}"
                                        }
                                        td {
                                            "{a.summary}"
                                        }
                                        td {
                                            button {
                                                class: "modal_add",
                                                onclick: move |event| { sheets.write()[aidx_val].add_advantage(idx) },
                                                "+"
                                            }
                                        }
                                    }
                                }
                            })
                        }
                    }
                }
            }
        }
    }
}
