use items::ItemSortKey;

pub mod items;

#[derive(strum::Display)]
pub enum InventorySortKey {
    #[strum(to_string = "#Item#{0}")]
    Item(ItemSortKey),
}
