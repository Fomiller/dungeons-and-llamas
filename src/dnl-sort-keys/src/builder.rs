use crate::prelude::*;
use std::any::Any;

#[derive(Default, Clone, Debug)]
pub struct RootSortKeyBuilder {
    id: String,
    game: Option<GameSortKeyBuilder>,
    user: Option<UserSortKey>,
    message: Option<MessageSortKey>,
    state: Option<StateSortKey>,
    settings: Option<bool>,
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
    pub fn state(mut self, state: StateSortKey) -> Self {
        self.state = Some(state);
        self
    }
    pub fn settings(mut self, settings: bool) -> Self {
        self.settings = Some(settings);
        self
    }

    pub fn id(mut self, id: &str) -> Self {
        self.id = id.to_string();
        self
    }

    pub fn create_entity_sk(id: String, entity: EntitySortKeyBuilder) -> RootSortKeyBuilder {
        RootSortKeyBuilder::new()
            .id(&id)
            .game(GameSortKeyBuilder::new().entity(entity))
    }

    pub fn create_state_sk(id: &str) -> RootSortKeyBuilder {
        RootSortKeyBuilder::new()
            .id(id)
            .state(StateSortKey::GameState)
    }

    pub fn create_map_sk(id: &str) -> RootSortKeyBuilder {
        RootSortKeyBuilder::new()
            .id(id)
            .state(StateSortKey::GameMap)
    }

    pub fn create_inventory_sk(
        id: String,
        inventory: InventorySortKeyBuilder,
        entity: Entity,
    ) -> RootSortKeyBuilder {
        let entity = EntitySortKeyBuilder::new(entity).inventory(inventory);
        let game = GameSortKeyBuilder::new().entity(entity);
        RootSortKeyBuilder::new().id(&id).game(game)
    }

    pub fn create_item_sk(
        id: String,
        item: ItemSortKeyBuilder,
        entity: Entity,
    ) -> RootSortKeyBuilder {
        let inventory = InventorySortKeyBuilder::new().item(item);
        let entity = EntitySortKeyBuilder::new(entity).inventory(inventory);
        let game = GameSortKeyBuilder::new().entity(entity);
        RootSortKeyBuilder::new().id(&id).game(game)
    }

    pub fn create_weapon_sk(
        id: String,
        weapon: WeaponSortKey,
        equipped: EquippedStateSortKey,
        entity: Entity,
    ) -> RootSortKeyBuilder {
        let weapons = WeaponSortKeyBuilder::new()
            .weapon(weapon)
            .equipped(equipped);
        let item = ItemSortKeyBuilder::new().weapons(weapons);
        let inventory = InventorySortKeyBuilder::new().item(item);
        let entity = EntitySortKeyBuilder::new(entity).inventory(inventory);
        let game = GameSortKeyBuilder::new().entity(entity);
        RootSortKeyBuilder::new().id(&id).game(game)
    }

    pub fn create_armor_sk(
        id: String,
        armor: ArmorSortKey,
        equipped: EquippedStateSortKey,
        entity: Entity,
    ) -> RootSortKeyBuilder {
        let armor = ArmorSortKeyBuilder::new().armor(armor).equipped(equipped);
        let item = ItemSortKeyBuilder::new().armor(armor);
        let inventory = InventorySortKeyBuilder::new().item(item);
        let entity = EntitySortKeyBuilder::new(entity).inventory(inventory);
        let game = GameSortKeyBuilder::new().entity(entity);
        RootSortKeyBuilder::new().id(&id).game(game)
    }

    pub fn create_books_and_scrolls_sk(
        id: String,
        books_and_scrolls: BookAndScrollSortKey,
        entity: Entity,
    ) -> RootSortKeyBuilder {
        let item = ItemSortKeyBuilder::new().books_and_scrolls(books_and_scrolls);
        let inventory = InventorySortKeyBuilder::new().item(item);
        let entity = EntitySortKeyBuilder::new(entity).inventory(inventory);
        let game = GameSortKeyBuilder::new().entity(entity);
        RootSortKeyBuilder::new().id(&id).game(game)
    }

    pub fn create_magic_sk(
        id: String,
        magic: MagicItemSortKey,
        entity: Entity,
    ) -> RootSortKeyBuilder {
        let item = ItemSortKeyBuilder::new().magical(magic);
        let inventory = InventorySortKeyBuilder::new().item(item);
        let entity = EntitySortKeyBuilder::new(entity).inventory(inventory);
        let game = GameSortKeyBuilder::new().entity(entity);
        RootSortKeyBuilder::new().id(&id).game(game)
    }

    pub fn create_tool_sk(id: String, tool: ToolSortKey, entity: Entity) -> RootSortKeyBuilder {
        let item = ItemSortKeyBuilder::new().tools(tool);
        let inventory = InventorySortKeyBuilder::new().item(item);
        let entity = EntitySortKeyBuilder::new(entity).inventory(inventory);
        let game = GameSortKeyBuilder::new().entity(entity);
        RootSortKeyBuilder::new().id(&id).game(game)
    }

    pub fn create_clothing_sk(
        id: String,
        clothing: ClothingSortKey,
        entity: Entity,
    ) -> RootSortKeyBuilder {
        let item = ItemSortKeyBuilder::new().clothing(clothing);
        let inventory = InventorySortKeyBuilder::new().item(item);
        let entity = EntitySortKeyBuilder::new(entity).inventory(inventory);
        let game = GameSortKeyBuilder::new().entity(entity);
        RootSortKeyBuilder::new().id(&id).game(game)
    }

    pub fn create_skills_sk(
        id: String,
        skills: SkillsSortKey,
        entity: Entity,
    ) -> RootSortKeyBuilder {
        let stats = StatsSortKeyBuilder::new().skills(skills);
        let entity = EntitySortKeyBuilder::new(entity).stats(stats);
        let game = GameSortKeyBuilder::new().entity(entity);
        RootSortKeyBuilder::new().id(&id).game(game)
    }

    pub fn create_abilities_sk(
        id: &str,
        abilities: AbilitiesSortKey,
        entity: Entity,
    ) -> RootSortKeyBuilder {
        let stats = StatsSortKeyBuilder::new().abilities(abilities);
        let entity = EntitySortKeyBuilder::new(entity).stats(stats);
        let game = GameSortKeyBuilder::new().entity(entity);
        RootSortKeyBuilder::new().id(id).game(game)
    }

    pub fn create_conditions_sk(
        id: String,
        conditions: ConditionsSortKey,
        entity: Entity,
    ) -> RootSortKeyBuilder {
        let stats = StatsSortKeyBuilder::new().conditions(conditions);
        let entity = EntitySortKeyBuilder::new(entity).stats(stats);
        let game = GameSortKeyBuilder::new().entity(entity);
        RootSortKeyBuilder::new().id(&id).game(game)
    }

    pub fn create_core_attributes_sk(
        id: String,
        core_attributes: CoreAttributesSortKey,
        entity: Entity,
    ) -> RootSortKeyBuilder {
        let stats = StatsSortKeyBuilder::new().core_attributes(core_attributes);
        let entity = EntitySortKeyBuilder::new(entity).stats(stats);
        let game = GameSortKeyBuilder::new().entity(entity);
        RootSortKeyBuilder::new().id(&id).game(game)
    }

    pub fn create_savings_throw_sk(
        id: String,
        saving_throws: SavingThrowsSortKey,
        entity: Entity,
    ) -> RootSortKeyBuilder {
        let stats = StatsSortKeyBuilder::new().saving_throws(saving_throws);
        let entity = EntitySortKeyBuilder::new(entity).stats(stats);
        let game = GameSortKeyBuilder::new().entity(entity);
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
        if let Some(state) = self.state {
            result.push_str(&format!("{}", state.to_string()));
        }
        if let Some(_) = self.settings {
            result.push_str(&format!("{}", RootSortKey::Settings))
        }

        result
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
