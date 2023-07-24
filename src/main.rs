#![allow(non_snake_case)]

pub mod character;
pub mod rulebook;

use character::Character;
use dioxus::prelude::*;
use dioxus_desktop::{Config, WindowBuilder};
use lazy_static::lazy_static;
use native_dialog::MessageDialog;
use rulebook::Rulebook;

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

fn App(cx: Scope) -> Element {
    let sheets: &UseRef<Vec<Character>> = use_ref(cx, || vec![Character::new(&RULEBOOK)]);
    let active = use_state(cx, || 0 as usize);
    let show_advantage_search = use_state(cx, || true);
    let advantage_search = use_ref(cx, String::new);

    let rsheets = sheets.read();
    let ga = *active.get();
    let activesheet = &rsheets[ga];

    render!(
        style {
            include_str!("../static/style.css")
        }
        div {
            id: "tabs",
            for i in 0..sheets.read().len() {
                if i == ga {
                    rsx!{
                        button {
                            class: "selected",
                            onclick: move |_| {active.set(i)},
                            "{sheets.read()[i].name}"
                        }
                    }
                } else {
                    rsx!{
                        button {
                            onclick: move |_| {active.set(i)},
                            "{sheets.read()[i].name}"
                        }
                    }
                }
            }
            button {
                onclick: move |_| { sheets.with_mut(|s| {
                    s.push(Character::new(&RULEBOOK));
                    active.set(s.len() - 1)
                })},
                "+"
            }
        }
        div {
            id: "info",
            h1 {
                "Character Info"
            }
            div {class: "content",
            label{
                r#for: "name",
                "Name: "
            }
            input{
                name: "name",
                r#type: "text",
                value: "{sheets.read()[*active.get()].name}",
                size:  "{sheets.read()[*active.get()].name.len() + 1}",
                oninput: |event| { sheets.with_mut(|s| { s[*active.get()].name = event.value.clone(); s[*active.get()].name.truncate(35) })},
            }
            label{
                r#for: "identity",
                "Identity: "
            }
            input{
                name: "identity",
                r#type: "text",
                value: "{sheets.read()[*active.get()].identity}",
                size:  "{sheets.read()[*active.get()].identity.len()}",
                oninput: |event| { sheets.with_mut(|s| { s[*active.get()].identity = event.value.clone(); s[*active.get()].identity.truncate(35) })},
            }
            label{
                r#for: "secret",
                "Secret: "
            }
            input{
                name: "secret",
                r#type: "checkbox",
                value: "{sheets.read()[*active.get()].secret}",
                oninput: |event| { sheets.with_mut(|s| { s[*active.get()].secret = event.value.parse().unwrap() })},
            }
            label{
                r#for: "group",
                "Group: "
            }
            input{
                name: "group",
                r#type: "text",
                value: "{sheets.read()[*active.get()].group}",
                size:  "{sheets.read()[*active.get()].group.len()}",
                oninput: |event| { sheets.with_mut(|s| { s[*active.get()].group = event.value.clone(); s[*active.get()].group.truncate(45) })},
            }
            label{
                r#for: "base",
                "Base: "
            }
            input{
                name: "base",
                r#type: "text",
                value: "{sheets.read()[*active.get()].base}",
                size:  "{sheets.read()[*active.get()].base.len()}",
                oninput: |event| { sheets.with_mut(|s| { s[*active.get()].base = event.value.clone(); s[*active.get()].base.truncate(35) })},
            }
            label{
                r#for: "powerlevel",
                "Power Level: "
            }
            input{
                name: "powerlevel",
                r#type: "number",
                value: "{sheets.read()[*active.get()].power_level}",
                oninput: |event| { sheets.with_mut(|s| {
                    let val = event.value.clone();
                    match val.parse::<i32>() {
                        Ok(num) => s[*active.get()].power_level = num,
                        _ => ()
                    }
                })},
            }
            label{
                r#for: "exp",
                "Extra Points: "
            }
            input{
                name: "exp",
                r#type: "number",
                value: "{sheets.read()[*active.get()].exp}",
                oninput: |event| { sheets.with_mut(|s| {
                    let val = event.value.clone();
                    match val.parse::<i32>() {
                        Ok(num) => s[*active.get()].exp = num,
                        _ => ()
                    }
                })},
            }
            label{
                r#for: "heropoints",
                "Hero Points: "
            }
            input{
                name: "heropoints",
                r#type: "number",
                value: "{sheets.read()[*active.get()].hero_points}",
                oninput: |event| { sheets.with_mut(|s| {
                    let val = event.value.clone();
                    match val.parse::<i32>() {
                        Ok(num) => s[*active.get()].hero_points = num,
                        _ => ()
                    }
                })},
            }
            label {
                "Total: {sheets.read()[*active.get()].calculate_points_spent()} / {sheets.read()[*active.get()].calculate_point_max()}"
            }
        }
        }

        div {
            id: "abilityscore",
            table { tr {
            activesheet.ability_scores.iter().map(|(k, v)| {
                let k = (*k).clone();
                rsx! {
                    td{
                        label {
                            r#for: "{k}",
                            "{k}"
                        }
                        br {}
                        input {
                            name: "{k}",
                            r#type: "number",
                            value: "{v}",
                            oninput: move |event| { sheets.with_mut(|s| {
                                let val = event.value.clone();
                                match val.parse::<i32>() {
                                    Ok(num) => _ = s[*active.get()].ability_scores.insert(k, num),
                                    _ => ()
                                }
                            })},
                        }
                    }
                }
            })}}
        }

        div {
            id:"defense",
            h1 {
                "Defense"
            }
            table {
                activesheet.defenses.iter().map(|(k, v)| {
                    let k = (*k).clone();
                    let k_caps = String::from(k).to_uppercase();
                    let total = activesheet.calc_defense(k);
                    rsx! {
                        tr {
                            td {b{"{k_caps}"}}
                            td {
                                if k != "toughness" {
                                    rsx! {
                                        input {
                                        value: "{v}",
                                        r#type: "number",
                                        oninput: move |event| { sheets.with_mut(|s| {
                                            let val = event.value.clone();
                                            match val.parse::<i32>() {
                                                Ok(num) => _ = s[ga].defenses.insert(k, num),
                                                _ => ()
                                            }
                                        })}
                                    }}
                                }
                            }
                            td {
                                "{total}"
                            }
                        }
                    }
                })
            }

        }
        div {
            id:"offense",
            h1 {
                "Offense"
            }
            table {
                tr {
                    td {
                        colspan: "4",
                        style: "text-align: center; vertical-align: middle;",
                        b { "Initiative: {activesheet.calc_initiative()}" }
                    }
                }
                activesheet.offense.iter().map(|(k, v)| {
                    let k = (*k).clone();
                    rsx! {
                        tr {
                            td {"{k}"}
                            td {
                                input {
                                    value: "{v}",
                                    r#type: "number",
                                    oninput: move |event| { sheets.with_mut(|s| {
                                        let val = event.value.clone();
                                        match val.parse::<i32>() {
                                            Ok(num) => _ = s[ga].offense.insert(k, num),
                                            _ => ()
                                        }
                                    })}
                                }
                            }
                            if k == "Unarmed" {
                                rsx!{
                                    td {
                                        "Close"
                                    }
                                    td {
                                        "Damage {activesheet.ability_scores.get(\"str\").unwrap()}"
                                    }
                                }
                            }
                        }
                    }
                })
            }
        }

        div {
            id:"skills",
            h1 {
                "Skills"
            }
            table {
                activesheet.skills.iter().map(|(k, v)| {
                    let k = (*k).clone();
                    let total = activesheet.calc_skill(k);
                    rsx! {
                        tr {
                            td {"{k}"}
                            td {
                                input {
                                    value: "{v}",
                                    r#type: "number",
                                    oninput: move |event| { sheets.with_mut(|s| {
                                        let val = event.value.clone();
                                        match val.parse::<i32>() {
                                            Ok(num) => _ = s[ga].skills.insert(k, num),
                                            _ => ()
                                        }
                                    })}
                                }
                            }
                            td {"{total}"}
                        }
                    }
                })
            }
        }
        div {
            id:"advantages",
            h1 {
                "Advantages"
                button {
                    class: "add",
                    onclick: move |event| {show_advantage_search.set(false)},
                    "+"
                }
            }
            table {
                activesheet.advantages.iter().enumerate().map(|(idx, a)| {
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
                                            oninput: move |event| { sheets.with_mut( |s| {
                                                let val = event.value.clone();
                                                match val.parse::<i32>() {
                                                    Ok(num) => s[ga].set_advantage_ranks(idx, num),
                                                    _ => ()
                                                }
                                            })}
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
                                            oninput: move |event| { sheets.with_mut( |s| {
                                                let val = s[ga].set_advantage_note(idx, event.value.clone());
                                            })}
                                        }
                                    }
                                },
                                None => rsx! {td{}}
                            }
                            td {
                                button {
                                    onclick: move |_| {sheets.with_mut(|s| s[ga].delete_advantage(idx))},
                                    "x"
                                }
                            }
                        }
                    }
                })
            }
        }
        div {
            id: "advantages_popup",
            class: "modal",
            hidden: "{show_advantage_search.get()}",
            div {
                class: "modal-content",
                h1 {
                    "Add Advantages"
                    button {
                        class: "add",
                        onclick: move |event| {show_advantage_search.set(true)},
                        "x"
                    }
                }
                input {
                    r#type: "text",
                    value: "{advantage_search.read()}",
                    oninput: |event| { advantage_search.with_mut(|s| { *s = event.value.clone() })},
                }
                table {
                    tr {
                        th {"Name"}
                        th {"Type"}
                        th {colspan: "2", "Summary"}
                    }
                    RULEBOOK.advantages.iter().filter(|a| a.name.contains(advantage_search.read().as_str())).enumerate().filter(|(idx, _)| !activesheet.has_advantage(*idx)).map(|(idx, a)| {
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
                                        onclick: move |event| {
                                            sheets.with_mut(|s| {
                                                s[ga].add_advantage(idx);
                                            })
                                        },
                                        "+"
                                    }
                                }
                            }
                        }
                    })
                }
            }
        }
    )
}
