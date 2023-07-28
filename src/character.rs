use indexmap::IndexMap;

use crate::{
    rulebook::{AdvantageInfo, Rulebook},
    RULEBOOK,
};

#[derive(PartialEq)]
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
    pub powers: Vec<PowerEntry>,
    // pub conditions: Vec<Condition>,
    pub notes: String,
    pub rulebook: &'a Rulebook<'a>,
}

#[derive(PartialEq)]
pub struct Advantage {
    pub id: usize,
    pub ranks: Option<i32>,
    pub notes: Option<String>,
}

#[derive(PartialEq)]
pub enum PowerEntry {
    Power(Power),
    Array(PowerArray),
}

#[derive(PartialEq)]
pub struct PowerArray {
    pub name: String,
    pub powers: Vec<Power>,
}

#[derive(PartialEq)]
pub struct Power {
    pub name: String,
    pub effect: Vec<PowerEffect>,
}

#[derive(PartialEq)]
pub struct PowerEffect {
    id: usize,
    ranks: i32,
    extras: Vec<Extra>,
    flaws: Vec<Flaw>,
    descriptors: String,
    notes: Option<String>,
}

#[derive(PartialEq)]
pub struct Extra {
    id: usize,
    ranks: Option<i32>,
    notes: Option<String>,
}

#[derive(PartialEq)]
pub struct Flaw {
    id: usize,
    ranks: Option<i32>,
    notes: Option<String>,
}

// #[derive(PartialEq, Props)]
// pub struct Condition {
//     name: String,
//     stat_changes: Vec<fn(String, i32) -> i32>,
// }

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
            // conditions: Vec::new(),
            notes: String::new(),
            rulebook: rules,
        }
    }

    //TODO
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

    pub fn create_power(&mut self) {
        self.powers.push(PowerEntry::Power(Power::new()))
    }

    pub fn create_power_array(&mut self) {
        self.powers.push(PowerEntry::Array(PowerArray {
            name: String::from("array"),
            powers: Vec::new(),
        }))
    }

    pub fn add_effect_to_power(
        &mut self,
        power_idx: usize,
        effect_id: usize,
        array_idx: Option<usize>,
    ) {
        match &mut self.powers[power_idx] {
            PowerEntry::Power(ref mut p) => p.add_effect(effect_id),
            PowerEntry::Array(ref mut pa) => match array_idx {
                Some(i) => pa.powers[i].add_effect(effect_id),
                None => (),
            },
        }
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

impl Power {
    pub fn new() -> Self {
        Self {
            name: String::from("New Power"),
            effect: Vec::new(),
        }
    }

    pub fn add_effect(&mut self, id: usize) {
        let has_notes = RULEBOOK.powers[id].notes;
        self.effect.push(PowerEffect {
            id: id,
            ranks: 0,
            extras: Vec::new(),
            flaws: Vec::new(),
            descriptors: String::new(),
            notes: if has_notes { Some(String::new()) } else { None },
        })
    }
}
