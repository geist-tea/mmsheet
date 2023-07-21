#![allow(non_snake_case)]

use dioxus::prelude::*;

use crate::character::Character;

#[inline_props]
fn CharacterInfo(cx: Scope, chara: Character) -> Element<'a> {
    cx.render(rsx!(
        div {
            id: "container",
            h1 {
                input {
                    value: chara.name.as_str(),
                }
            }
            label {
                r#for: "heroName",
                "Hero: "
            }
        }
    ))
}
