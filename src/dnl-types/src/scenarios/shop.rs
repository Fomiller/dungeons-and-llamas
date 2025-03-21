use crate::dice::DiceExpression;
use crate::tools::shop::*;
use crate::traits::DiscordMsg;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShopScenario {
    pub items: Vec<ShopScenarioItem>,
    pub merchant: ShopScenarioMerchant,
    pub summary: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ShopScenarioMerchant {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShopScenarioItem {
    pub name: String,
    pub description: String,
    pub stats: DiceExpression,
    pub price: String,
}

impl DiscordMsg for ShopScenario {
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

impl From<ShopToolItem> for ShopScenarioItem {
    fn from(source: ShopToolItem) -> Self {
        Self {
            name: source.item_name,
            description: source.item_description,
            stats: source.item_stats,
            price: source.item_price,
        }
    }
}

impl From<ShopToolMerchant> for ShopScenarioMerchant {
    fn from(source: ShopToolMerchant) -> Self {
        Self {
            name: source.merchant_name,
            description: source.merchant_description,
        }
    }
}

impl From<ShopToolOutput> for ShopScenario {
    fn from(source: ShopToolOutput) -> Self {
        Self {
            items: source.items.into_iter().map(Into::into).collect(),
            merchant: ShopScenarioMerchant::from(source.merchant),
            summary: source.summary,
        }
    }
}
