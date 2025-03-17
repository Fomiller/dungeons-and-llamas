use super::MockData;
use crate::traits::DiscordMsg;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShopToolOutput {
    pub items: Vec<ShopToolItem>,
    pub merchant: ShopToolMerchant,
    pub name: String,
    pub summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShopToolMerchant {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShopToolItem {
    pub name: String,
    pub description: String,
    pub stats: String,
    pub price: String,
}

impl MockData for ShopToolOutput {
    fn mock() -> Self {
        let sword = ShopToolItem {
            name: "Magic Sword".to_string(),
            description: "A glowing sword.".to_string(),
            stats: "1d8 + 1".to_string(),
            price: "3 gold".to_string(),
        };

        let bow = ShopToolItem {
            name: "Short Bow".to_string(),
            description: "A short bow, it seems to have some intricate carvings".to_string(),
            stats: "1d8".to_string(),
            price: "2 gold".to_string(),
        };

        let potion = ShopToolItem {
            name: "Health Potion".to_string(),
            description: "Restores Health".to_string(),
            stats: "4d8".to_string(),
            price: "5 gold".to_string(),
        };

        let items = vec![sword, bow, potion];

        let merchant = ShopToolMerchant { name: "Dirty Dan".to_string(), description: "A dirty old man who is missing teeth, but you can see a cart full of treasure behind him".to_string()};

        let name = "Dirty wares".to_string();

        let summary = "This is summary Text".to_string();

        Self {
            name,
            merchant,
            items,
            summary,
        }
    }
}

impl DiscordMsg for ShopToolOutput {
    fn to_message(&self) -> String {
        let mut item_descriptions = String::new();

        for item in &self.items {
            let description = format!(
                "**{}**\n{}\n - Stats: {}\n - Price: {}\n\n",
                item.name, item.description, item.stats, item.price
            );
            item_descriptions.push_str(&description);
        }

        format!(
            "*{}*\n# {}\n*{}*\n## Items:\n{}",
            self.summary, self.merchant.name, self.merchant.description, item_descriptions
        )
    }
}
