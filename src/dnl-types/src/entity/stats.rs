use anyhow::Context;
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
                    "strength": "+2",
                    "dexterity": "+0",
                    "constitution": "+2",
                    "intelligence": "+0",
                    "wisdom": "+0",
                    "charisma": "+0"
                }
            },
            {
                "name": "bard",
                "key_ability_modifier": ["charisma", "dexterity"],
                "base_stats": {
                    "strength": "+0",
                    "dexterity": "+2",
                    "constitution": "+0",
                    "intelligence": "+0",
                    "wisdom": "+0",
                    "charisma": "+2"
                }
            },
            {
                "name": "cleric",
                "key_ability_modifier": ["wisdom", "constitution"],
                "base_stats": {
                    "strength": "+0",
                    "dexterity": "+0",
                    "constitution": "+1",
                    "intelligence": "+1",
                    "wisdom": "+2",
                    "charisma": "+0"
                }
            },
            {
                "name": "druid",
                "key_ability_modifier": ["wisdom", "constitution"],
                "base_stats": {
                    "strength": "+0",
                    "dexterity": "+0",
                    "constitution": "+2",
                    "intelligence": "+0",
                    "wisdom": "+2",
                    "charisma": "+0"
                }
            },
            {
                "name": "fighter",
                "key_ability_modifier": ["strength or dexterity", "constitution"],
                "base_stats": {
                    "strength": "+1",
                    "dexterity": "+1",
                    "constitution": "+2",
                    "intelligence": "+0",
                    "wisdom": "+0",
                    "charisma": "+0"
                }
            },
            {
                "name": "monk",
                "key_ability_modifier": ["dexterity or wisdom", "constitution"],
                "base_stats": {
                    "strength": "+0",
                    "dexterity": "+2",
                    "constitution": "+0",
                    "intelligence": "+0",
                    "wisdom": "+2",
                    "charisma": "+0"
                }
            },
            {
                "name": "paladin",
                "key_ability_modifier": ["strength", "charisma"],
                "base_stats": {
                    "strength": "+2",
                    "dexterity": "+0",
                    "constitution": "+0",
                    "intelligence": "+0",
                    "wisdom": "+0",
                    "charisma": "+2"
                }
            },
            {
                "name": "ranger",
                "key_ability_modifier": ["dexterity", "wisdom"],
                "base_stats": {
                    "strength": "+0",
                    "dexterity": "+2",
                    "constitution": "+0",
                    "intelligence": "+0",
                    "wisdom": "+2",
                    "charisma": "+0"
                }
            },
            {
                "name": "rogue",
                "key_ability_modifier": ["dexterity", "intelligence or constitution"],
                "base_stats": {
                    "strength": "+0",
                    "dexterity": "+2",
                    "constitution": "+0",
                    "intelligence": "+2",
                    "wisdom": "+0",
                    "charisma": "+0"
                }
            },
            {
                "name": "sorcerer",
                "key_ability_modifier": ["charisma", "constitution"],
                "base_stats": {
                    "strength": "+0",
                    "dexterity": "+0",
                    "constitution": "+0",
                    "intelligence": "+2",
                    "wisdom": "+2",
                    "charisma": "+0"
                }
            },
            {
                "name": "warlock",
                "key_ability_modifier": ["charisma", "constitution"],
                "base_stats": {
                    "strength": "+0",
                    "dexterity": "+0",
                    "constitution": "+0",
                    "intelligence": "+0",
                    "wisdom": "+2",
                    "charisma": "+2"
                }
            },
            {
                "name": "wizard",
                "key_ability_modifier": ["intelligence", "constitution"],
                "base_stats": {
                    "strength": "+0",
                    "dexterity": "+0",
                    "constitution": "+1",
                    "intelligence": "+1",
                    "wisdom": "+1",
                    "charisma": "+1"
                }
            }
        ])
    };
}
