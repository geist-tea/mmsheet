use std::ops::IndexMut;

use dioxus::prelude::Props;
use indexmap::IndexMap;

use crate::rulebook::{AdvantageInfo, Rulebook};

#[derive(PartialEq, Props)]
pub struct Character<'a> {
    pub name: String,
    pub identity: String,
    pub secret: bool,
    pub group: String,
    pub base: String,
    pub power_level: i32,
    pub exp: i32,
    pub hero_points: i32,
    pub ability_scores: IndexMap<&'a str, i32>,
    pub defenses: IndexMap<&'a str, i32>,
    pub skills: IndexMap<&'a str, i32>,
    pub offense: IndexMap<&'a str, i32>,
    pub advantages: Vec<Advantage>,
    pub powers: Vec<Power>,
    pub conditions: Vec<Condition>,
    pub notes: String,
    pub rulebook: &'a Rulebook<'a>,
}

#[derive(PartialEq, Props)]
pub struct DerivedStat<'a> {
    pub derived_from: &'a str,
    pub invested: i32,
    pub total: i32,
}

#[derive(PartialEq)]
pub struct Advantage {
    pub id: usize,
    pub ranks: Option<i32>,
    pub notes: Option<String>,
}

pub enum AdvantageType {
    Combat,
    Fortune,
    General,
    Skill,
}

pub struct AdvantageRanks {
    ranks: i32,
    max: Option<i32>,
}

#[derive(PartialEq, Props)]
pub struct Power {
    name: String,
    effect: i32,
    extras: Vec<i32>,
    flaws: Vec<i32>,
    ranks: i32,
    descriptors: String,
    alt_effects: Vec<Power>,
    add_effects: Vec<Power>,
}

pub struct PowerData {
    name: String,
    cost: i32,
    power_type: PowerType,
    action: Action,
    range: Range,
    duration: Duration,
    applicable_extras: i32,
    applicable_flaws: i32,
}

pub enum Action {
    Standard,
    Move,
    Free,
    Reaction,
    None,
}

pub enum Range {
    Personal,
    Close,
    Ranged,
    Perception,
    Rank,
}

pub enum Duration {
    Instant,
    Concentration,
    Sustained,
    Continuous,
    Permanent,
}

pub enum PowerType {
    Attack,
    Control,
    Defense,
    General,
    Movement,
    Sensory,
}

#[derive(PartialEq, Props)]
pub struct Condition {
    name: String,
    stat_changes: Vec<fn(String, i32) -> i32>,
}

impl<'a> Character<'a> {
    pub fn new(rules: &'a Rulebook) -> Self {
        Self {
            name: String::from("New Hero"),
            identity: String::from(""),
            secret: false,
            group: String::from(""),
            base: String::from(""),
            power_level: 10,
            exp: 0,
            hero_points: 1,
            ability_scores: IndexMap::from([
                ("str", 0),
                ("sta", 0),
                ("agl", 0),
                ("dex", 0),
                ("fgt", 0),
                ("int", 0),
                ("awe", 0),
                ("pre", 0),
            ]),
            defenses: IndexMap::from([
                ("dodge", 0),
                ("parry", 0),
                ("will", 0),
                ("fortitude", 0),
                ("toughness", 0),
            ]),
            skills: IndexMap::from([
                ("Acrobatics", 0),
                ("Athletics", 0),
                ("Deception", 0),
                ("Insight", 0),
                ("Intimidation", 0),
                ("Investigation", 0),
                ("Perception", 0),
                ("Persuasion", 0),
                ("Sleight of Hand", 0),
                ("Stealth", 0),
                ("Technology", 0),
                ("Treatment", 0),
                ("Vehicles", 0),
            ]),
            offense: IndexMap::from([("Unarmed", 0)]),
            advantages: Vec::new(),
            powers: Vec::new(),
            conditions: Vec::new(),
            notes: String::new(),
            rulebook: rules,
        }
    }

    pub fn calculate_points_spent(&self) -> i32 {
        0
    }

    pub fn calculate_point_max(&self) -> i32 {
        (self.power_level * 15) + self.exp
    }

    pub fn calc_initiative(&self) -> String {
        match self.ability_scores.get("agility") {
            Some(x) => {
                if *x >= 0 {
                    format!("+{x}")
                } else {
                    format!("{x}")
                }
            }
            None => String::from("+0"),
        }
    }

    pub fn calc_defense(&self, def: &str) -> String {
        let k = *self.rulebook.stat_derivations.get(def).unwrap();
        let a = *self.ability_scores.get(k).unwrap();
        let b = *self.defenses.get(def).unwrap();
        let t = a + b;
        if t < 0 {
            format!("{t}")
        } else {
            format!("+{t}")
        }
    }

    pub fn calc_skill(&self, skill: &str) -> String {
        let k = *self.rulebook.stat_derivations.get(skill).unwrap_or(&"int");
        let a = *self.ability_scores.get(k).unwrap();
        let b = *self.skills.get(skill).unwrap();
        let t = a + b;
        if t < 0 {
            format!("{t}")
        } else {
            format!("+{t}")
        }
    }

    pub fn add_advantage(&mut self, id: usize) {
        self.advantages
            .push(Advantage::new(id, &self.rulebook.advantages[id]))
    }

    pub fn has_advantage(&self, id: usize) -> bool {
        self.advantages
            .iter()
            .map(|a| a.id == id)
            .reduce(|l, r| l || r)
            .unwrap_or(false)
    }

    pub fn set_advantage_ranks(&mut self, idx: usize, new_rank: i32) {
        let max_rank = self.rulebook.advantages[self.advantages[idx].id]
            .max_ranks
            .unwrap_or(999);

        match self.advantages[idx].ranks {
            Some(_) => {
                self.advantages[idx].ranks = Some(if new_rank <= max_rank {
                    new_rank
                } else {
                    max_rank
                })
            }
            None => (),
        }
    }

    pub fn set_advantage_note(&mut self, idx: usize, new_note: String) {
        self.advantages[idx].notes = Some(new_note);
    }

    pub fn delete_advantage(&mut self, idx: usize) {
        self.advantages.remove(idx);
    }
}

impl Advantage {
    pub fn new(id: usize, from: &AdvantageInfo) -> Self {
        Self {
            id: id,
            ranks: if from.ranked { Some(0) } else { None },
            notes: if from.notes {
                Some(String::new())
            } else {
                None
            },
        }
    }
}
