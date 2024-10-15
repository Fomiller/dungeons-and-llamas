use items::ItemSortKey;

pub mod items;

#[derive(strum::Display)]
pub enum InventorySortKey {
    #[strum(to_string = "Item#")]
    Item,
}

#[derive(Default)]
pub struct InventorySortKeyBuilder {
    item: Option<ItemSortKey>,
}
