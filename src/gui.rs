#![allow(non_snake_case)]

use dioxus::prelude::*;

use crate::{character::Character, RULEBOOK};

pub fn CharacterTabs(cx: Scope) -> Element {
    let sheets = use_shared_state::<Vec<Character>>(cx).unwrap();
    let active_idx = use_shared_state::<usize>(cx).unwrap();

    let aidx_val = *active_idx.read();
    render! {
        style {
            include_str!("../static/style.css")
        }
        div {
            id: "tabs",
            for i in 0..sheets.read().len() {
                if i == aidx_val {
                    rsx!{
                        button {
                            class: "selected",
                            onclick: move |_| {*active_idx.write() = i},
                            "{sheets.read()[aidx_val].name}"
                        }
                    }
                } else {
                    rsx!{
                        button {
                            onclick: move |_| {*active_idx.write() = i},
                            "{sheets.read()[i].name}"
                        }
                    }
                }
            }
            button {
                onclick: move |_| {
                    sheets.write().push(Character::new(&RULEBOOK));
                    *active_idx.write() = sheets.read().len() - 1;
                },
                "+"
            }
        }
    }
}

pub fn CharacterInfo(cx: Scope) -> Element {
    let sheets = use_shared_state::<Vec<Character>>(cx).unwrap();
    let active_idx = use_shared_state::<usize>(cx).unwrap();

    let aidx_val = *active_idx.read();

    render! {
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
                value: "{sheets.read()[aidx_val].name}",
                size:  "25",
                oninput: move |event| { sheets.write()[aidx_val].name = event.value.clone() },
            }
            label{
                r#for: "identity",
                "Identity: "
            }
            input{
                name: "identity",
                r#type: "text",
                value: "{sheets.read()[aidx_val].identity}",
                size:  "25",
                oninput: move |event| { sheets.write()[aidx_val].identity = event.value.clone() },
            }
            label{
                r#for: "secret",
                "Secret: "
            }
            input{
                name: "secret",
                r#type: "checkbox",
                value: "{sheets.read()[aidx_val].secret}",
                oninput: move |event| { sheets.write()[aidx_val].secret = event.value.clone().parse().unwrap_or_default() },
            }
            label{
                r#for: "group",
                "Group: "
            }
            input{
                name: "group",
                r#type: "text",
                value: "{sheets.read()[aidx_val].group}",
                size:  "25",
                oninput: move |event| { sheets.write()[aidx_val].group = event.value.clone() },
            }
            label{
                r#for: "base",
                "Base: "
            }
            input{
                name: "base",
                r#type: "text",
                value: "{sheets.read()[aidx_val].base}",
                size:  "25",
                oninput: move |event| { sheets.write()[aidx_val].base = event.value.clone() },
            }
            label{
                r#for: "powerlevel",
                "Power Level: "
            }
            input{
                name: "powerlevel",
                r#type: "number",
                value: "{sheets.read()[aidx_val].power_level}",
                oninput: move |event| {
                    let val = event.value.clone();
                    match val.parse::<i32>() {
                        Ok(num) => sheets.write()[aidx_val].power_level = num,
                        _ => ()
                    }
                },
            }
            label{
                r#for: "exp",
                "Extra Points: "
            }
            input{
                name: "exp",
                r#type: "number",
                value: "{sheets.read()[aidx_val].exp}",
                oninput: move |event| {
                    let val = event.value.clone();
                    match val.parse::<i32>() {
                        Ok(num) => sheets.write()[aidx_val].exp = num,
                        _ => ()
                    }
                },
            }
            label{
                r#for: "heropoints",
                "Hero Points: "
            }
            input{
                name: "heropoints",
                r#type: "number",
                value: "{sheets.read()[aidx_val].hero_points}",
                oninput: move |event| {
                    let val = event.value.clone();
                    match val.parse::<i32>() {
                        Ok(num) => sheets.write()[aidx_val].hero_points = num,
                        _ => ()
                    }
                },
            }
            label {
                "Total: {sheets.read()[aidx_val].calculate_points_spent()} / {sheets.read()[aidx_val].calculate_point_max()}"
            }
        }
        }
    }
}

pub fn AbilityScores(cx: Scope) -> Element {
    let sheets = use_shared_state::<Vec<Character>>(cx).unwrap();
    let active_idx = use_shared_state::<usize>(cx).unwrap();

    let aidx_val = *active_idx.read();

    render! {
        div {
            id: "abilityscore",
            table { tr {
            sheets.read()[aidx_val].ability_scores.iter().map(|(k, v)| {
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
                            oninput: move |event| {
                                let val = event.value.clone();
                                match val.parse::<i32>() {
                                    Ok(num) => _ = sheets.write()[aidx_val].ability_scores.insert(k, num),
                                    _ => ()
                                }
                            },
                        }
                    }
                }
            })}}
        }
    }
}

pub fn Defense(cx: Scope) -> Element {
    let sheets = use_shared_state::<Vec<Character>>(cx).unwrap();
    let active_idx = use_shared_state::<usize>(cx).unwrap();

    let aidx_val = *active_idx.read();

    render! {
        div {
            id: "defense",
            h1 {
                "Defense"
            }
            table {
                sheets.read()[aidx_val].defenses.iter().map(|(k, v)| {
                    let k = (*k).clone();
                    let k_caps = String::from(k).to_uppercase();
                    let total = sheets.read()[aidx_val].calc_defense(k);
                    rsx! {
                        tr {
                            td {b{"{k_caps}"}}
                            td {
                                if k != "toughness" {
                                    rsx! {
                                        input {
                                        value: "{v}",
                                        r#type: "number",
                                        oninput: move |event| {
                                            let val = event.value.clone();
                                            match val.parse::<i32>() {
                                                Ok(num) => _ = sheets.write()[aidx_val].defenses.insert(k, num),
                                                _ => ()
                                            }
                                        }
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
    }
}

pub fn Offense(cx: Scope) -> Element {
    let sheets = use_shared_state::<Vec<Character>>(cx).unwrap();
    let active_idx = use_shared_state::<usize>(cx).unwrap();

    let aidx_val = *active_idx.read();

    render! {
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
                        b { "Initiative: {sheets.read()[aidx_val].calc_initiative()}" }
                    }
                }
                sheets.read()[aidx_val].offense.iter().map(|(k, v)| {
                    let k = (*k).clone();
                    rsx! {
                        tr {
                            td {"{k}"}
                            td {
                                input {
                                    value: "{v}",
                                    r#type: "number",
                                    oninput: move |event| {
                                        let val = event.value.clone();
                                        match val.parse::<i32>() {
                                            Ok(num) => _ = sheets.write()[aidx_val].offense.insert(k, num),
                                            _ => ()
                                        }
                                    }
                                }
                            }
                            if k == "Unarmed" {
                                rsx!{
                                    td {
                                        "Close"
                                    }
                                    td {
                                        "Damage {sheets.read()[aidx_val].ability_scores.get(\"str\").unwrap()}"
                                    }
                                }
                            }
                        }
                    }
                })
            }
        }
    }
}

pub fn Skills(cx: Scope) -> Element {
    let sheets = use_shared_state::<Vec<Character>>(cx).unwrap();
    let active_idx = use_shared_state::<usize>(cx).unwrap();

    let aidx_val = *active_idx.read();

    render! {
        div {
            id:"skills",
            h1 {
                "Skills"
            }
            table {
                sheets.read()[aidx_val].skills.iter().map(|(k, v)| {
                    let k = (*k).clone();
                    let total = sheets.read()[aidx_val].calc_skill(k);
                    rsx! {
                        tr {
                            td {"{k}"}
                            td {
                                input {
                                    value: "{v}",
                                    r#type: "number",
                                    oninput: move |event| {
                                        let val = event.value.clone();
                                        match val.parse::<i32>() {
                                            Ok(num) => _ = sheets.write()[aidx_val].skills.insert(k, num),
                                            _ => ()
                                        }
                                    }
                                }
                            }
                            td {"{total}"}
                        }
                    }
                })
            }
        }
    }
}

pub fn Advantages(cx: Scope) -> Element {
    let sheets = use_shared_state::<Vec<Character>>(cx).unwrap();
    let active_idx = use_shared_state::<usize>(cx).unwrap();

    let aidx_val = *active_idx.read();

    render! {
        div {
            id:"advantages",
            h1 {
                "Advantages"
                button {
                    class: "add",
                    onclick: move |_| {},
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
    }
}

pub fn AdvantageSearchModal(cx: Scope) -> Element {
    let sheets = use_shared_state::<Vec<Character>>(cx).unwrap();
    let active_idx = use_shared_state::<usize>(cx).unwrap();
    //let modal_state = use_shared_state::<String>(cx).unwrap();
    let search = use_ref(cx, String::new);

    let aidx_val = *active_idx.read();

    render! {
        div {
            id: "advantages_popup",
            class: "modal",
            div {
                class: "modal-content",
                h1 {
                    "Add Advantages"
                    button {
                        class: "add",
                        onclick: move |_| {},
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
                    RULEBOOK.advantages.iter().filter(|a| a.name.contains(search.read().as_str())).enumerate().filter(|(idx, _)| !sheets.read()[aidx_val].has_advantage(*idx)).map(|(idx, a)| {
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
    }
}

pub fn Notes(cx: Scope) -> Element {
    let sheets = use_shared_state::<Vec<Character>>(cx).unwrap();
    let active_idx = use_shared_state::<usize>(cx).unwrap();

    let aidx_val = *active_idx.read();

    render! {
        div {
            id:"notes",
            h1 {
                "Campaign Notes"
            }
            textarea {
                rows: "10",
                oninput: move |event| { sheets.write()[aidx_val].notes = event.value.clone() }
            }
        }
    }
}
