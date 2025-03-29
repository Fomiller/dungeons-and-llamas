use lambda_runtime::tracing::info;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BaseStats {
    pub strength: String,
    pub dexterity: String,
    pub constitution: String,
    pub intelligence: String,
    pub wisdom: String,
    pub charisma: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ClassWithBaseStats {
    pub name: Class,
    pub base_stats: BaseStats,
    pub key_ability_modifier: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, strum::Display, strum::EnumString)]
#[strum(ascii_case_insensitive)]
pub enum Class {
    Barbarian,
    Bard,
    Cleric,
    Druid,
    Fighter,
    Monk,
    Paladin,
    Ranger,
    Rogue,
    Sorcerer,
    Warlock,
    Wizard,
}

pub fn get_base_stats_by_name(class_name: &str) -> anyhow::Result<BaseStats> {
    // Search for the class by name and return its base_stats if found
    info!("Getting stats for {} class", class_name);

    let stats = BASE_STATS
        .as_array()
        .unwrap()
        .iter()
        .find(|&class| class["name"] == class_name)
        .map(|class| {
            info!("Class {}", class);
            class["base_stats"].clone()
        })
        .expect("BASE_STATS should have a base_stats key");

    info!("HELLO STATS {}", stats);

    match serde_json::from_value(stats) {
        Ok(value) => Ok(value),
        Err(e) => Err(anyhow::anyhow!(e)),
    }
}

lazy_static! {
    pub static ref BASE_STATS: serde_json::Value = {
        serde_json::json!([
            {
                "name": "barbarian",
                "key_ability_modifier": ["strength", "constitution"],
                "base_stats": {
                    "strength": "15",
                    "dexterity": "10",
                    "constitution": "14",
                    "intelligence": "8",
                    "wisdom": "12",
                    "charisma": "10"
                }
            },
            {
                "name": "bard",
                "key_ability_modifier": ["charisma", "dexterity"],
                "base_stats": {
                    "strength": "8",
                    "dexterity": "14",
                    "constitution": "12",
                    "intelligence": "10",
                    "wisdom": "10",
                    "charisma": "15"
                }
            },
            {
                "name": "cleric",
                "key_ability_modifier": ["wisdom", "constitution"],
                "base_stats": {
                    "strength": "10",
                    "dexterity": "10",
                    "constitution": "14",
                    "intelligence": "12",
                    "wisdom": "15",
                    "charisma": "8"
                }
            },
            {
                "name": "druid",
                "key_ability_modifier": ["wisdom", "constitution"],
                "base_stats": {
                    "strength": "8",
                    "dexterity": "12",
                    "constitution": "14",
                    "intelligence": "10",
                    "wisdom": "15",
                    "charisma": "10"
                }
            },
            {
                "name": "fighter",
                "key_ability_modifier": ["strength or dexterity", "constitution"],
                "base_stats": {
                    "strength": "15",
                    "dexterity": "12",
                    "constitution": "14",
                    "intelligence": "10",
                    "wisdom": "10",
                    "charisma": "8"
                }
            },
            {
                "name": "monk",
                "key_ability_modifier": ["dexterity", "wisdom"],
                "base_stats": {
                    "strength": "10",
                    "dexterity": "15",
                    "constitution": "12",
                    "intelligence": "8",
                    "wisdom": "14",
                    "charisma": "10"
                }
            },
            {
                "name": "paladin",
                "key_ability_modifier": ["strength", "charisma"],
                "base_stats": {
                    "strength": "15",
                    "dexterity": "8",
                    "constitution": "14",
                    "intelligence": "10",
                    "wisdom": "10",
                    "charisma": "12"
                }
            },
            {
                "name": "ranger",
                "key_ability_modifier": ["dexterity", "wisdom"],
                "base_stats": {
                    "strength": "10",
                    "dexterity": "15",
                    "constitution": "12",
                    "intelligence": "10",
                    "wisdom": "14",
                    "charisma": "8"
                }
            },
            {
                "name": "rogue",
                "key_ability_modifier": ["dexterity", "intelligence or constitution"],
                "base_stats": {
                    "strength": "8",
                    "dexterity": "15",
                    "constitution": "12",
                    "intelligence": "14",
                    "wisdom": "10",
                    "charisma": "10"
                }
            },
            {
                "name": "sorcerer",
                "key_ability_modifier": ["charisma", "constitution"],
                "base_stats": {
                    "strength": "8",
                    "dexterity": "10",
                    "constitution": "12",
                    "intelligence": "14",
                    "wisdom": "10",
                    "charisma": "15"
                }
            },
            {
                "name": "warlock",
                "key_ability_modifier": ["charisma", "constitution"],
                "base_stats": {
                    "strength": "8",
                    "dexterity": "10",
                    "constitution": "14",
                    "intelligence": "12",
                    "wisdom": "10",
                    "charisma": "15"
                }
            },
            {
                "name": "wizard",
                "key_ability_modifier": ["intelligence", "constitution"],
                "base_stats": {
                    "strength": "8",
                    "dexterity": "10",
                    "constitution": "12",
                    "intelligence": "15",
                    "wisdom": "14",
                    "charisma": "10"
                }
            }
        ]
        )
    };
}
