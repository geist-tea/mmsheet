use std::{collections::HashMap, fmt, fs::File, io::BufReader};

#[derive(PartialEq, serde::Deserialize)]
pub struct AdvantageInfo {
    pub name: String,
    pub summary: String,
    pub description: String,
    pub r#type: AdvantageType,
    pub ranked: bool,
    pub max_ranks: Option<i32>,
    pub notes: bool,
}

#[derive(PartialEq, serde::Deserialize)]
pub enum AdvantageType {
    Combat,
    Fortune,
    General,
    Skill,
}

impl fmt::Display for AdvantageType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AdvantageType::Combat => write!(f, "Combat"),
            AdvantageType::Fortune => write!(f, "Fortune"),
            AdvantageType::General => write!(f, "General"),
            AdvantageType::Skill => write!(f, "Skill"),
        }
    }
}

#[derive(PartialEq, serde::Deserialize)]
enum AttackType {
    Close,
    Ranged,
}

#[derive(PartialEq, serde::Deserialize)]
pub struct PowerInfo {
    name: String,
    description: String,
    action: Action,
    range: Range,
    duration: PowerDuration,
    cost: i32,
    attack: Option<AttackType>,
    dc: Option<i32>,
    resisted_by: Option<Vec<String>>,
    notes: bool,
    applicable_extras: Vec<usize>,
    applicable_flaws: Vec<usize>,
}

#[derive(PartialEq, serde::Deserialize)]
pub enum PowerType {
    Attack,
    Control,
    Defense,
    General,
    Movement,
    Sensory,
}

#[derive(PartialEq, serde::Deserialize)]
pub enum PowerDuration {
    Instant,
    Concentration,
    Sustained,
    Continuous,
    Permanent,
}

#[derive(PartialEq, serde::Deserialize)]
pub enum Range {
    Personal,
    Close,
    Ranged,
    Perception,
    Rank,
}

#[derive(PartialEq, serde::Deserialize)]
pub enum Action {
    Standard,
    Move,
    Free,
    Reaction,
    None,
}

#[derive(PartialEq)]
pub struct Rulebook<'a> {
    pub stat_derivations: HashMap<&'a str, &'a str>,
    pub advantages: Vec<AdvantageInfo>,
    pub powers: Vec<PowerInfo>,
    pub mass: [&'a str; 36],
    pub time: [&'a str; 36],
    pub distance: [&'a str; 36],
    pub volume: [&'a str; 36],
}

impl Rulebook<'_> {
    /// Creates a new Rulebook instance. Looks in ./data/ for all of the necessary files, and returns an error if any are missing.
    pub fn new() -> Result<Self, String> {
        let adv_reader = match File::open("./data/advantages.json").map(BufReader::new) {
            Ok(r) => r,
            Err(_) => return Err(String::from("Couldn't find ./data/advantages.json")),
        };

        let advantages: Vec<AdvantageInfo> = match serde_json::from_reader(adv_reader) {
            Ok(data) => data,
            Err(e) => return Err(e.to_string()),
        };

        let pow_reader = match File::open("./data/powers.json").map(BufReader::new) {
            Ok(r) => r,
            Err(_) => return Err(String::from("Couldn't find ./data/powers.json")),
        };

        let powers: Vec<PowerInfo> = match serde_json::from_reader(pow_reader) {
            Ok(data) => data,
            Err(e) => return Err(e.to_string()),
        };

        Ok(Self {
            stat_derivations: HashMap::from([
                ("dodge", "agl"),
                ("fortitude", "sta"),
                ("parry", "fgt"),
                ("will", "awe"),
                ("toughness", "sta"),
                ("Acrobatics", "agl"),
                ("Athletics", "str"),
                ("Deception", "pre"),
                ("Insight", "awe"),
                ("Intimidation", "pre"),
                ("Investigation", "int"),
                ("Perception", "awe"),
                ("Persuasion", "pre"),
                ("Sleight of Hand", "dex"),
                ("Stealth", "agl"),
                ("Technology", "int"),
                ("Treatment", "int"),
                ("Vehicles", "dex"),
            ]),
            advantages: advantages,
            powers: powers,
            mass: [
                "750 grams",
                "1.5 kg",
                "3 kg",
                "6 kg",
                "12 kg",
                "24 kg",
                "50 kg",
                "100 kg",
                "200 kg",
                "400 kg",
                "800 kg",
                "1600 kg",
                "3.2 tons",
                "6 tons",
                "12 tons",
                "25 tons",
                "50 tons",
                "100 tons",
                "200 tons",
                "400 tons",
                "800 tons",
                "1,600 tons",
                "3.2 ktons",
                "6 ktons",
                "12 ktons",
                "25 ktons",
                "50 ktons",
                "100 ktons",
                "200 ktons",
                "400 ktons",
                "800 ktons",
                "1,600 ktons",
                "3,200 ktons",
                "6,400 ktons",
                "12,500 ktons",
                "25,000 ktons",
            ],
            time: [
                "1/8 second",
                "1/4 second",
                "1/2 second",
                "1 second",
                "3 seconds",
                "6 seconds",
                "12 seconds",
                "30 seconds",
                "1 minute",
                "2 minutes",
                "4 minutes",
                "8 minutes",
                "15 minutes",
                "30 minutes",
                "1 hour",
                "2 hours",
                "4 hours",
                "8 hours",
                "16 hours",
                "1 day",
                "2 days",
                "4 days",
                "1 week",
                "2 weeks",
                "1 month",
                "2 months",
                "4 months",
                "8 months",
                "1.5 years",
                "3 years",
                "6 years",
                "12 years",
                "25 years",
                "50 years",
                "100 years",
                "200 years",
            ],
            distance: [
                "15 cm",
                "50 cm",
                "1 m",
                "2 m",
                "4 m",
                "8 m",
                "16 m",
                "32 m",
                "64 m",
                "125 m",
                "250 m",
                "500 m",
                "1 km",
                "2 km",
                "4 km",
                "8 km",
                "16 km",
                "32 km",
                "64 km",
                "125 km",
                "250 km",
                "500 km",
                "1,000km",
                "2,000 km",
                "4,000 km",
                "8,000 km",
                "16,000 km",
                "32,000 km",
                "64,000 km",
                "125,000 km",
                "250.000 km",
                "500.000 km",
                "1 million km",
                "2 million km",
                "4 million km",
                "8 million km",
            ],
            volume: [
                ".0008 m³",
                ".0017 m³",
                ".0035 m³",
                ".007 m³",
                ".014 m³",
                ".025 m",
                ".05 m³",
                ".1 m³",
                "2 m³",
                "4 m³",
                "8 m³",
                "1.7 m³",
                "3.5 m³",
                "7 m³",
                "15 m³",
                "30 m³",
                "60 m³",
                "120 m³",
                "250 m³",
                "500 m³",
                "1,000 m³",
                "2,000 m³",
                "4,000 m³",
                "8,000 m³",
                "15,000 m³",
                "30.000 m³",
                "60,000 m³",
                "120,000 m³",
                "250,000 m³",
                "500,000 m³",
                "1 million m³",
                "2 million m³",
                "4 million m³",
                "8 million m³",
                "15 million m³",
                "30 million m³",
            ],
        })
    }
}
