use super::equipped::EquippedStateSortKey;
use crate::state::SortKeyBuildable;
use std::any::Any;

#[derive(Debug, Clone, Copy, strum::Display, strum::EnumIter)]
pub enum ArmorSortKey {
    #[strum(to_string = "Light")]
    Light,
    #[strum(to_string = "Medium")]
    Medium,
    #[strum(to_string = "Heavy")]
    Heavy,
    #[strum(to_string = "Shield")]
    Shield,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ArmorSortKeyBuilder {
    armor: Option<ArmorSortKey>,
    equipped: EquippedStateSortKey,
}

impl SortKeyBuildable for ArmorSortKeyBuilder {
    fn build(&self) -> String {
        let mut result = String::from(format!("Armor#{}", self.equipped.to_string()));
        if let Some(weapon) = self.armor {
            result.push_str(&format!("{}", weapon.to_string()));
        }
        result
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl From<Box<dyn SortKeyBuildable>> for ArmorSortKeyBuilder {
    fn from(skb: Box<dyn SortKeyBuildable>) -> Self {
        if let Some(armor) = skb.as_any().downcast_ref::<ArmorSortKeyBuilder>() {
            return *armor;
        }
        Self::default()
    }
}

impl ArmorSortKeyBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn armor(mut self, armor: ArmorSortKey) -> Self {
        self.armor = Some(armor);
        self
    }
    pub fn equipped(mut self, equipped: EquippedStateSortKey) -> Self {
        self.equipped = equipped;
        self
    }
}
