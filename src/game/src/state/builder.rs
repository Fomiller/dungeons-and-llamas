use super::buildable::SortKeyBuildable;
use super::game::GameSortKeyBuilder;
use super::message::MessageSortKey;
use super::user::UserSortKey;

use std::any::Any;

use crate::{
    client::{
        EquippedStateSortKey, InventorySortKeyBuilder, ItemSortKeyBuilder, PlayerSortKeyBuilder,
        WeaponSortKey, WeaponSortKeyBuilder,
    },
    state::game::player::{
        inventory::items::{
            armor::{ArmorSortKey, ArmorSortKeyBuilder},
            books_and_scrolls::BookAndScrollSortKey,
            clothing::ClothingSortKey,
            magic::MagicItemSortKey,
            tools::ToolSortKey,
        },
        stats::{
            abilities::AbilitiesSortKey, conditions::ConditionsSortKey,
            core_attributes::CoreAttributesSortKey, saving_throws::SavingThrowsSortKey,
            skills::SkillsSortKey, StatsSortKeyBuilder,
        },
    },
};

#[derive(Default)]
pub struct RootSortKeyBuilder {
    id: String,
    game: Option<GameSortKeyBuilder>,
    user: Option<UserSortKey>,
    message: Option<MessageSortKey>,
}

impl RootSortKeyBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn game(mut self, game: GameSortKeyBuilder) -> Self {
        self.game = Some(game);
        self
    }
    pub fn user(mut self, user: UserSortKey) -> Self {
        self.user = Some(user);
        self
    }
    pub fn message(mut self, message: MessageSortKey) -> Self {
        self.message = Some(message);
        self
    }

    pub fn id(mut self, id: &str) -> Self {
        self.id = id.to_string();
        self
    }

    pub fn create_player_sk(id: String, player: PlayerSortKeyBuilder) -> RootSortKeyBuilder {
        RootSortKeyBuilder::new()
            .id(&id)
            .game(GameSortKeyBuilder::new().player(player))
    }

    pub fn create_inventory_sk(
        id: String,
        inventory: InventorySortKeyBuilder,
    ) -> RootSortKeyBuilder {
        let player = PlayerSortKeyBuilder::new().inventory(inventory);
        let game = GameSortKeyBuilder::new().player(player);
        RootSortKeyBuilder::new().id(&id).game(game)
    }

    pub fn create_item_sk(id: String, item: ItemSortKeyBuilder) -> RootSortKeyBuilder {
        let inventory = InventorySortKeyBuilder::new().item(item);
        let player = PlayerSortKeyBuilder::new().inventory(inventory);
        let game = GameSortKeyBuilder::new().player(player);
        RootSortKeyBuilder::new().id(&id).game(game)
    }

    pub fn create_weapon_sk(
        id: String,
        weapon: WeaponSortKey,
        equipped: EquippedStateSortKey,
    ) -> RootSortKeyBuilder {
        let weapons = WeaponSortKeyBuilder::new()
            .weapon(weapon)
            .equipped(equipped);
        let item = ItemSortKeyBuilder::new().weapons(weapons);
        let inventory = InventorySortKeyBuilder::new().item(item);
        let player = PlayerSortKeyBuilder::new().inventory(inventory);
        let game = GameSortKeyBuilder::new().player(player);
        RootSortKeyBuilder::new().id(&id).game(game)
    }

    pub fn create_armor_sk(
        id: String,
        armor: ArmorSortKey,
        equipped: EquippedStateSortKey,
    ) -> RootSortKeyBuilder {
        let armor = ArmorSortKeyBuilder::new().armor(armor).equipped(equipped);
        let item = ItemSortKeyBuilder::new().armor(armor);
        let inventory = InventorySortKeyBuilder::new().item(item);
        let player = PlayerSortKeyBuilder::new().inventory(inventory);
        let game = GameSortKeyBuilder::new().player(player);
        RootSortKeyBuilder::new().id(&id).game(game)
    }

    pub fn create_books_and_scrolls_sk(
        id: String,
        books_and_scrolls: BookAndScrollSortKey,
    ) -> RootSortKeyBuilder {
        let item = ItemSortKeyBuilder::new().books_and_scrolls(books_and_scrolls);
        let inventory = InventorySortKeyBuilder::new().item(item);
        let player = PlayerSortKeyBuilder::new().inventory(inventory);
        let game = GameSortKeyBuilder::new().player(player);
        RootSortKeyBuilder::new().id(&id).game(game)
    }

    pub fn create_magic_sk(id: String, magic: MagicItemSortKey) -> RootSortKeyBuilder {
        let item = ItemSortKeyBuilder::new().magical(magic);
        let inventory = InventorySortKeyBuilder::new().item(item);
        let player = PlayerSortKeyBuilder::new().inventory(inventory);
        let game = GameSortKeyBuilder::new().player(player);
        RootSortKeyBuilder::new().id(&id).game(game)
    }

    pub fn create_tool_sk(id: String, tool: ToolSortKey) -> RootSortKeyBuilder {
        let item = ItemSortKeyBuilder::new().tools(tool);
        let inventory = InventorySortKeyBuilder::new().item(item);
        let player = PlayerSortKeyBuilder::new().inventory(inventory);
        let game = GameSortKeyBuilder::new().player(player);
        RootSortKeyBuilder::new().id(&id).game(game)
    }

    pub fn create_clothing_sk(id: String, clothing: ClothingSortKey) -> RootSortKeyBuilder {
        let item = ItemSortKeyBuilder::new().clothing(clothing);
        let inventory = InventorySortKeyBuilder::new().item(item);
        let player = PlayerSortKeyBuilder::new().inventory(inventory);
        let game = GameSortKeyBuilder::new().player(player);
        RootSortKeyBuilder::new().id(&id).game(game)
    }

    pub fn create_skills_sk(id: String, skills: SkillsSortKey) -> RootSortKeyBuilder {
        let stats = StatsSortKeyBuilder::new().skills(skills);
        let player = PlayerSortKeyBuilder::new().stats(stats);
        let game = GameSortKeyBuilder::new().player(player);
        RootSortKeyBuilder::new().id(&id).game(game)
    }

    pub fn create_abilities_sk(id: String, abilities: AbilitiesSortKey) -> RootSortKeyBuilder {
        let stats = StatsSortKeyBuilder::new().abilities(abilities);
        let player = PlayerSortKeyBuilder::new().stats(stats);
        let game = GameSortKeyBuilder::new().player(player);
        RootSortKeyBuilder::new().id(&id).game(game)
    }

    pub fn create_conditions_sk(id: String, conditions: ConditionsSortKey) -> RootSortKeyBuilder {
        let stats = StatsSortKeyBuilder::new().conditions(conditions);
        let player = PlayerSortKeyBuilder::new().stats(stats);
        let game = GameSortKeyBuilder::new().player(player);
        RootSortKeyBuilder::new().id(&id).game(game)
    }

    pub fn create_core_attributes_sk(
        id: String,
        core_attributes: CoreAttributesSortKey,
    ) -> RootSortKeyBuilder {
        let stats = StatsSortKeyBuilder::new().core_attributes(core_attributes);
        let player = PlayerSortKeyBuilder::new().stats(stats);
        let game = GameSortKeyBuilder::new().player(player);
        RootSortKeyBuilder::new().id(&id).game(game)
    }

    pub fn create_savings_throw_sk(
        id: String,
        saving_throws: SavingThrowsSortKey,
    ) -> RootSortKeyBuilder {
        let stats = StatsSortKeyBuilder::new().saving_throws(saving_throws);
        let player = PlayerSortKeyBuilder::new().stats(stats);
        let game = GameSortKeyBuilder::new().player(player);
        RootSortKeyBuilder::new().id(&id).game(game)
    }
}

impl SortKeyBuildable for RootSortKeyBuilder {
    fn build(&self) -> String {
        let mut result = String::from(format!("{}#", self.id));

        if let Some(game) = self.game {
            result.push_str(&format!("{}", game.build().to_string()));
        }
        if let Some(user) = self.user {
            result.push_str(&format!("{}", user.to_string()));
        }
        if let Some(message) = self.message {
            result.push_str(&format!("{}", message.to_string()));
        }

        result
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
