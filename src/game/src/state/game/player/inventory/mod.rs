use items::ItemSortKeyBuilder;

pub mod items;

#[derive(strum::Display)]
pub enum InventorySortKey {
    #[strum(to_string = "Item#")]
    Item,
}

#[derive(Default)]
pub struct InventorySortKeyBuilder {
    item: Option<ItemSortKeyBuilder>,
}

impl InventorySortKeyBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn item(mut self, item: ItemSortKeyBuilder) -> Self {
        self.item = Some(item);
        self
    }

    pub fn build(self) -> String {
        let mut result = String::from("Inventory#");
        if let Some(item) = self.item {
            result.push_str(&format!("{}", item.build().to_string()));
        }
        result
    }
}
