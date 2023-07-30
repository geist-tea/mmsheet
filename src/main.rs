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

use crate::{character::PowerEntry, gui::*};

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
    NewPowerType,
    NewPowerEffect(usize, Option<usize>),
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
                                                sheets.write()[aidx_val].set_advantage_note(idx, event.value.clone());
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
        div {
            id: "powers",
            h1 {
                "Powers"
                button {
                    class: "add",
                    onclick: move |_| modal_state.set(Modal::NewPowerType),
                    "+"
                }
            }
            sheets.read()[aidx_val].powers.iter().enumerate().map(|(idx, power)| {
                match power {
                    PowerEntry::Power(p) => rsx! {
                        h2 {
                            "{p.name}"
                        }
                        button {
                            onclick: move |_| modal_state.set(Modal::NewPowerEffect(idx, None)),
                            "+"
                        }
                        table {
                            p.effect.iter().map(|e| {
                                rsx! {
                                    tr {
                                        td { "{RULEBOOK.powers[e.id].name}" }
                                        td {
                                            input {
                                                r#type: "number",
                                                value: "{e.ranks}",
                                            }
                                        }
                                        match &e.notes {
                                            Some(notes) => rsx! {
                                                td {
                                                    input {
                                                        r#type: "text",
                                                        value: "{notes}",
                                                    }
                                                }
                                            },
                                            None => rsx! {td{""}},
                                        }
                                        match &RULEBOOK.powers[e.id].resisted_by {
                                            Some(res) => rsx! {
                                                "DC {&RULEBOOK.powers[e.id].dc.unwrap_or(0) + &e.ranks}"
                                                select {
                                                    res.iter().map(|r| {
                                                        rsx! {
                                                            option {
                                                                "{r}"
                                                            }
                                                        }
                                                    })
                                                }
                                            },
                                            None => rsx! {""},
                                        }
                                        td {
                                            button {"+"}
                                        }
                                        td {
                                            button {"+"}
                                        }
                                    }
                                }
                            })
                        }
                    },
                    PowerEntry::Array(pa) => rsx! {
                        h2 {
                            "{pa.name}: Array"
                        }
                    }
                }
            })
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
                                onclick: move |_| modal_state.set(Modal::Hidden),
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
                                                onclick: move |_| { sheets.write()[aidx_val].add_advantage(idx) },
                                                "+"
                                            }
                                        }
                                    }
                                }
                            })
                        }
                    }
                }
            },
            Modal::NewPowerType => rsx!{
                div {
                    class: "modal",
                    div {
                        class: "modal-content",
                        h1 {
                            "Create new power"
                            button {
                                class: "add",
                                onclick: move |_| modal_state.set(Modal::Hidden),
                                "x"
                            }
                        }
                        button {
                            onclick: move |_| {
                                sheets.write()[aidx_val].create_power();
                                modal_state.set(Modal::Hidden);
                            },
                            "New Power"
                        }
                        button {
                            onclick: move |_| {
                                sheets.write()[aidx_val].create_power_array();
                                modal_state.set(Modal::Hidden);
                            },
                            "New Power Array"
                        }
                    }
                }
            },
            Modal::NewPowerEffect(p_idx, a_idx) => rsx!{
                div {
                    class: "modal",
                    div {
                        class: "modal-content",
                        h1 {
                            "Add Power Effect"
                            button {
                                class: "add",
                                onclick: move |_| modal_state.set(Modal::Hidden),
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
                                th {"Action"}
                                th {"Range"}
                                th {"Duration"}
                                th {"Resistance"}
                                th {"Cost"}
                            }
                            RULEBOOK.powers.iter().enumerate().filter(|(_, p)| p.name.contains(search.read().as_str())).map(|(idx, p)| {
                                rsx! {
                                    tr {
                                        td {
                                            "{p.name}"
                                        }
                                        td {
                                           "{p.r#type}"
                                        }
                                        td {
                                            "{p.action}"
                                        }
                                        td {
                                            "{p.range}"
                                        }
                                        td {
                                            "{p.duration}"
                                        }
                                        td {
                                            match &p.resisted_by {
                                                Some(v) => rsx!{ "{v.join(\", \")}" },
                                                None => rsx!{"—"},
                                            }
                                        }
                                        td {
                                            "{p.cost} per rank"
                                        }
                                        td {
                                            button {
                                                class: "modal_add",
                                                onclick: move |_| { sheets.write()[aidx_val].add_effect_to_power(*p_idx, idx, *a_idx) },
                                                "+"
                                            }
                                        }
                                    }
                                }
                            })
                        }
                    }
                }
            },
        }
    }
}
