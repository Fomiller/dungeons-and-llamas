use super::MockData;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShopToolOutput { 
    pub items: Vec<ShopToolItem>,
    pub merchant: ShopToolMerchant,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShopToolMerchant { 
    pub name:  String,
    pub description: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShopToolItem { 
    pub name:  String,
    pub description: String,
    pub stats: String,
    pub price: String
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

        Self {
            name,
            merchant,
            items
        }
    }
}

