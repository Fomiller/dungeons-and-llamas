use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

// definitions provided from https://transform.tools/json-to-rust-serde

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DndBeyondCharacter {
    pub id: i64,
    pub success: bool,
    pub message: String,
    pub data: Data,
    pub pagination: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub id: i64,
    pub user_id: i64,
    pub username: String,
    pub is_assigned_to_player: bool,
    pub readonly_url: String,
    pub decorations: Decorations,
    pub name: String,
    pub social_name: Value,
    pub gender: Value,
    pub faith: Value,
    pub age: Value,
    pub hair: Value,
    pub eyes: Value,
    pub skin: Value,
    pub height: Value,
    pub weight: Value,
    pub inspiration: bool,
    pub base_hit_points: i64,
    pub bonus_hit_points: Value,
    pub override_hit_points: Value,
    pub removed_hit_points: i64,
    pub temporary_hit_points: i64,
    pub current_xp: i64,
    pub alignment_id: Value,
    pub lifestyle_id: Value,
    pub stats: Vec<Stat>,
    pub bonus_stats: Vec<BonusStat>,
    pub override_stats: Vec<OverrideStat>,
    pub background: Background,
    pub race: Race,
    pub race_definition_id: Value,
    pub race_definition_type_id: Value,
    pub notes: Notes,
    pub traits: Traits,
    pub preferences: Preferences,
    pub configuration: Configuration,
    pub lifestyle: Value,
    pub inventory: Vec<Inventory>,
    pub currencies: Currencies,
    pub classes: Vec<Class>,
    pub feats: Vec<Feat>,
    pub features: Value,
    pub custom_defense_adjustments: Vec<Value>,
    pub custom_senses: Vec<Value>,
    pub custom_speeds: Vec<Value>,
    pub custom_proficiencies: Vec<Value>,
    pub custom_actions: Vec<Value>,
    pub character_values: Vec<Value>,
    pub conditions: Vec<Value>,
    pub death_saves: DeathSaves,
    pub adjustment_xp: i64,
    pub spell_slots: Vec<SpellSlot>,
    pub pact_magic: Vec<PactMagic>,
    pub active_source_categories: Vec<i64>,
    pub spells: Spells,
    pub options: Options,
    pub choices: Choices,
    pub actions: Actions,
    pub modifiers: Modifiers,
    pub class_spells: Vec<ClassSpell>,
    pub custom_items: Vec<Value>,
    pub campaign: Value,
    pub creatures: Vec<Value>,
    pub optional_origins: Vec<Value>,
    pub optional_class_features: Vec<Value>,
    pub date_modified: String,
    pub provided_from: String,
    pub can_edit: bool,
    pub status: i64,
    pub status_slug: Value,
    pub campaign_setting: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Decorations {
    pub avatar_url: String,
    pub frame_avatar_url: Value,
    pub backdrop_avatar_url: Value,
    pub small_backdrop_avatar_url: Value,
    pub large_backdrop_avatar_url: Value,
    pub thumbnail_backdrop_avatar_url: Value,
    pub default_backdrop: DefaultBackdrop,
    pub avatar_id: i64,
    pub portrait_decoration_key: Value,
    pub frame_avatar_decoration_key: Value,
    pub frame_avatar_id: Value,
    pub backdrop_avatar_decoration_key: Value,
    pub backdrop_avatar_id: Value,
    pub small_backdrop_avatar_decoration_key: String,
    pub small_backdrop_avatar_id: Value,
    pub large_backdrop_avatar_decoration_key: String,
    pub large_backdrop_avatar_id: Value,
    pub thumbnail_backdrop_avatar_decoration_key: String,
    pub thumbnail_backdrop_avatar_id: Value,
    pub theme_color: ThemeColor,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DefaultBackdrop {
    pub backdrop_avatar_url: String,
    pub small_backdrop_avatar_url: String,
    pub large_backdrop_avatar_url: String,
    pub thumbnail_backdrop_avatar_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThemeColor {
    pub theme_color_id: i64,
    pub theme_color: String,
    pub background_color: String,
    pub name: String,
    pub race_id: Value,
    pub sub_race_id: Value,
    pub class_id: i64,
    pub tags: Vec<String>,
    pub decoration_key: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stat {
    pub id: i64,
    pub name: Value,
    pub value: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BonusStat {
    pub id: i64,
    pub name: Value,
    pub value: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OverrideStat {
    pub id: i64,
    pub name: Value,
    pub value: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Background {
    pub has_custom_background: bool,
    pub definition: Definition,
    pub definition_id: Value,
    pub custom_background: CustomBackground,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Definition {
    pub id: i64,
    pub entity_type_id: i64,
    pub definition_key: String,
    pub name: String,
    pub description: String,
    pub snippet: String,
    pub short_description: String,
    pub skill_proficiencies_description: String,
    pub tool_proficiencies_description: String,
    pub languages_description: String,
    pub equipment_description: String,
    pub feature_name: String,
    pub feature_description: String,
    pub avatar_url: Value,
    pub large_avatar_url: Value,
    pub suggested_characteristics_description: String,
    pub suggested_proficiencies: Value,
    pub suggested_languages: Value,
    pub organization: Value,
    pub contracts_description: String,
    pub spells_pre_description: String,
    pub spells_post_description: String,
    pub personality_traits: Vec<Value>,
    pub ideals: Vec<Value>,
    pub bonds: Vec<Value>,
    pub flaws: Vec<Value>,
    pub is_homebrew: bool,
    pub sources: Vec<Source>,
    pub spell_list_ids: Vec<Value>,
    pub feat_list: FeatList,
    pub granted_feats: Vec<GrantedFeat>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Source {
    pub source_id: i64,
    pub page_number: i64,
    pub source_type: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeatList {
    pub id: i64,
    pub name: String,
    pub feat_ids: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GrantedFeat {
    pub id: i64,
    pub name: String,
    pub feat_ids: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomBackground {
    pub id: i64,
    pub entity_type_id: i64,
    pub name: Value,
    pub description: Value,
    pub features_background: Value,
    pub characteristics_background: Value,
    pub features_background_definition_id: Value,
    pub characteristics_background_definition_id: Value,
    pub background_type: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Race {
    pub is_sub_race: bool,
    pub base_race_name: String,
    pub entity_race_id: i64,
    pub entity_race_type_id: i64,
    pub definition_key: String,
    pub full_name: String,
    pub base_race_id: i64,
    pub base_race_type_id: i64,
    pub description: String,
    pub avatar_url: String,
    pub large_avatar_url: String,
    pub portrait_avatar_url: String,
    pub more_details_url: String,
    pub is_homebrew: bool,
    pub is_legacy: bool,
    pub group_ids: Vec<Value>,
    #[serde(rename = "type")]
    pub type_field: i64,
    pub supports_subrace: Value,
    pub sub_race_short_name: Value,
    pub base_name: String,
    pub racial_traits: Vec<RacialTrait>,
    pub weight_speeds: WeightSpeeds,
    pub feat_ids: Vec<Value>,
    pub size: Value,
    pub size_id: i64,
    pub sources: Vec<Source3>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RacialTrait {
    pub definition: Definition2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Definition2 {
    pub id: i64,
    pub definition_key: String,
    pub entity_type_id: i64,
    pub display_order: Option<i64>,
    pub name: String,
    pub description: String,
    pub snippet: String,
    pub hide_in_builder: bool,
    pub hide_in_sheet: bool,
    pub activation: Value,
    pub source_id: i64,
    pub source_page_number: Option<i64>,
    pub creature_rules: Vec<Value>,
    pub spell_list_ids: Vec<Value>,
    pub feature_type: i64,
    pub sources: Vec<Source2>,
    pub affected_feature_definition_keys: Vec<Value>,
    pub is_called_out: bool,
    pub entity_type: String,
    #[serde(rename = "entityID")]
    pub entity_id: String,
    pub entity_race_id: i64,
    pub entity_race_type_id: i64,
    pub display_configuration: DisplayConfiguration,
    pub required_level: Value,
    pub categories: Vec<Category>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Source2 {
    pub source_id: i64,
    pub page_number: Option<i64>,
    pub source_type: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DisplayConfiguration {
    #[serde(rename = "RACIALTRAIT")]
    pub racialtrait: i64,
    #[serde(rename = "ABILITYSCORE")]
    pub abilityscore: i64,
    #[serde(rename = "LANGUAGE")]
    pub language: i64,
    #[serde(rename = "CLASSFEATURE")]
    pub classfeature: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Category {
    pub id: i64,
    pub entity_type_id: i64,
    pub entity_id: i64,
    pub definition_key: String,
    pub entity_tag_id: i64,
    pub tag_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeightSpeeds {
    pub normal: Normal,
    pub encumbered: Value,
    pub heavily_encumbered: Value,
    pub push_drag_lift: Value,
    #[serde(rename = "override")]
    pub override_field: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Normal {
    pub walk: i64,
    pub fly: i64,
    pub burrow: i64,
    pub swim: i64,
    pub climb: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Source3 {
    pub source_id: i64,
    pub page_number: i64,
    pub source_type: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Notes {
    pub allies: Value,
    pub personal_possessions: Value,
    pub other_holdings: Value,
    pub organizations: Value,
    pub enemies: Value,
    pub backstory: Value,
    pub other_notes: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Traits {
    pub personality_traits: Value,
    pub ideals: Value,
    pub bonds: Value,
    pub flaws: Value,
    pub appearance: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Preferences {
    pub use_homebrew_content: bool,
    pub progression_type: i64,
    pub encumbrance_type: i64,
    pub ignore_coin_weight: bool,
    pub hit_point_type: i64,
    pub show_unarmed_strike: bool,
    pub show_scaled_spells: bool,
    pub primary_sense: i64,
    pub primary_movement: i64,
    pub privacy_type: i64,
    pub sharing_type: i64,
    pub ability_score_display_type: i64,
    pub enforce_feat_rules: bool,
    pub enforce_multiclass_rules: bool,
    pub enable_optional_class_features: bool,
    pub enable_optional_origins: bool,
    pub enable_dark_mode: bool,
    pub enable_container_currency: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Configuration {
    pub starting_equipment_type: Value,
    pub ability_score_type: i64,
    pub show_help_text: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Inventory {
    pub id: i64,
    pub entity_type_id: i64,
    pub definition: Definition3,
    pub definition_id: i64,
    pub definition_type_id: i64,
    pub display_as_attack: Value,
    pub quantity: i64,
    pub is_attuned: bool,
    pub equipped: bool,
    pub equipped_entity_type_id: Option<i64>,
    pub equipped_entity_id: Option<i64>,
    pub charges_used: i64,
    pub limited_use: Option<LimitedUse>,
    pub container_entity_id: i64,
    pub container_entity_type_id: i64,
    pub container_definition_key: String,
    pub currency: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Definition3 {
    pub id: i64,
    pub base_type_id: i64,
    pub entity_type_id: i64,
    pub definition_key: String,
    pub can_equip: bool,
    pub magic: bool,
    pub name: String,
    pub snippet: Option<String>,
    pub weight: f64,
    pub weight_multiplier: f64,
    pub capacity: Option<String>,
    pub capacity_weight: f64,
    #[serde(rename = "type")]
    pub type_field: String,
    pub description: String,
    pub can_attune: bool,
    pub attunement_description: Option<String>,
    pub rarity: String,
    pub is_homebrew: bool,
    pub version: Value,
    pub source_id: Value,
    pub source_page_number: Value,
    pub stackable: bool,
    pub bundle_size: i64,
    pub avatar_url: Value,
    pub large_avatar_url: Value,
    pub filter_type: String,
    pub cost: Option<f64>,
    pub is_pack: bool,
    pub tags: Vec<String>,
    pub granted_modifiers: Vec<GrantedModifier>,
    pub sub_type: Option<String>,
    pub is_consumable: bool,
    pub weapon_behaviors: Vec<Value>,
    pub base_item_id: Option<i64>,
    pub base_armor_name: Option<String>,
    pub strength_requirement: Value,
    pub armor_class: Option<i64>,
    pub stealth_check: Option<i64>,
    pub damage: Option<Damage>,
    pub damage_type: Option<String>,
    pub fixed_damage: Value,
    #[serde(default)]
    pub properties: Vec<Property>,
    pub attack_type: Option<i64>,
    pub category_id: Option<i64>,
    pub range: Option<i64>,
    pub long_range: Option<i64>,
    pub is_monk_weapon: bool,
    pub level_infusion_granted: Value,
    pub sources: Vec<Source4>,
    pub armor_type_id: Option<i64>,
    pub gear_type_id: Option<i64>,
    pub grouped_id: Option<i64>,
    pub can_be_added_to_inventory: bool,
    pub is_container: bool,
    pub is_custom_item: bool,
    pub is_legacy: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GrantedModifier {
    pub fixed_value: i64,
    pub id: String,
    pub entity_id: Value,
    pub entity_type_id: Value,
    #[serde(rename = "type")]
    pub type_field: String,
    pub sub_type: String,
    pub dice: Dice,
    pub restriction: String,
    pub stat_id: Value,
    pub requires_attunement: bool,
    pub duration: Value,
    pub friendly_type_name: String,
    pub friendly_subtype_name: String,
    pub is_granted: bool,
    pub bonus_types: Vec<Value>,
    pub value: Value,
    pub available_to_multiclass: bool,
    pub modifier_type_id: i64,
    pub modifier_sub_type_id: i64,
    pub component_id: i64,
    pub component_type_id: i64,
    pub tag_constraints: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dice {
    pub dice_count: i64,
    pub dice_value: i64,
    pub dice_multiplier: Value,
    pub fixed_value: i64,
    pub dice_string: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Damage {
    pub dice_count: i64,
    pub dice_value: i64,
    pub dice_multiplier: Value,
    pub fixed_value: Value,
    pub dice_string: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Property {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub notes: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Source4 {
    pub source_id: i64,
    pub page_number: Option<i64>,
    pub source_type: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LimitedUse {
    pub max_uses: i64,
    pub number_used: i64,
    pub reset_type: String,
    pub reset_type_description: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Currencies {
    pub cp: i64,
    pub sp: i64,
    pub gp: i64,
    pub ep: i64,
    pub pp: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Class {
    pub id: i64,
    pub entity_type_id: i64,
    pub level: i64,
    pub is_starting_class: bool,
    pub hit_dice_used: i64,
    pub definition_id: i64,
    pub subclass_definition_id: Value,
    pub definition: Definition4,
    pub subclass_definition: Value,
    pub class_features: Vec<ClassFeature2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Definition4 {
    pub id: i64,
    pub definition_key: String,
    pub name: String,
    pub description: String,
    pub equipment_description: String,
    pub parent_class_id: Value,
    pub avatar_url: String,
    pub large_avatar_url: String,
    pub portrait_avatar_url: String,
    pub more_details_url: String,
    pub spell_casting_ability_id: Value,
    pub sources: Vec<Source5>,
    pub class_features: Vec<ClassFeature>,
    pub hit_dice: i64,
    pub wealth_dice: Value,
    pub can_cast_spells: bool,
    pub knows_all_spells: Value,
    pub spell_prepare_type: Value,
    pub spell_container_name: Value,
    pub source_page_number: i64,
    pub subclass_definition: Value,
    pub is_homebrew: bool,
    pub primary_abilities: Vec<Value>,
    pub spell_rules: SpellRules,
    pub prerequisites: Vec<Prerequisite>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Source5 {
    pub source_id: i64,
    pub page_number: i64,
    pub source_type: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClassFeature {
    pub id: i64,
    pub name: String,
    pub prerequisite: Value,
    pub description: String,
    pub required_level: i64,
    pub display_order: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpellRules {
    pub multi_class_spell_slot_divisor: i64,
    pub is_ritual_spell_caster: bool,
    pub level_cantrips_known_maxes: Vec<i64>,
    pub level_spell_known_maxes: Vec<i64>,
    pub level_spell_slots: Vec<Vec<i64>>,
    pub multi_class_spell_slot_rounding: i64,
    pub level_prepared_spell_maxes: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Prerequisite {
    pub description: String,
    pub prerequisite_mappings: Vec<PrerequisiteMapping>,
    pub hide_prerequisite: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrerequisiteMapping {
    pub id: i64,
    pub entity_id: i64,
    pub entity_type_id: i64,
    #[serde(rename = "type")]
    pub type_field: String,
    pub sub_type: String,
    pub value: i64,
    pub friendly_type_name: String,
    pub friendly_sub_type_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClassFeature2 {
    pub definition: Definition5,
    pub level_scale: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Definition5 {
    pub id: i64,
    pub definition_key: String,
    pub entity_type_id: i64,
    pub display_order: i64,
    pub name: String,
    pub description: String,
    pub snippet: String,
    pub activation: Value,
    pub multi_class_description: String,
    pub required_level: i64,
    pub is_sub_class_feature: bool,
    pub limited_use: Vec<LimitedUse2>,
    pub hide_in_builder: bool,
    pub hide_in_sheet: bool,
    pub source_id: i64,
    pub source_page_number: i64,
    pub creature_rules: Vec<Value>,
    pub level_scales: Vec<Value>,
    pub infusion_rules: Vec<Value>,
    pub spell_list_ids: Vec<Value>,
    pub class_id: i64,
    pub feature_type: i64,
    pub sources: Vec<Source6>,
    pub affected_feature_definition_keys: Vec<Value>,
    pub entity_type: String,
    #[serde(rename = "entityID")]
    pub entity_id: String,
    pub granted_feats: Vec<GrantedFeat2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LimitedUse2 {
    pub level: Value,
    pub uses: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Source6 {
    pub source_id: i64,
    pub page_number: i64,
    pub source_type: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GrantedFeat2 {
    pub id: i64,
    pub name: String,
    pub feat_ids: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Feat {
    pub component_type_id: i64,
    pub component_id: i64,
    pub definition: Definition6,
    pub definition_id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Definition6 {
    pub id: i64,
    pub entity_type_id: i64,
    pub definition_key: String,
    pub name: String,
    pub description: String,
    pub snippet: String,
    pub activation: Activation,
    pub source_id: Value,
    pub source_page_number: Value,
    pub creature_rules: Vec<Value>,
    pub prerequisites: Vec<Value>,
    pub is_homebrew: bool,
    pub sources: Vec<Source7>,
    pub spell_list_ids: Vec<Value>,
    pub is_repeatable: bool,
    pub repeatable_parent_id: Value,
    pub categories: Vec<Category2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Activation {
    pub activation_time: Value,
    pub activation_type: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Source7 {
    pub source_id: i64,
    pub page_number: i64,
    pub source_type: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Category2 {
    pub id: i64,
    pub entity_type_id: i64,
    pub entity_id: i64,
    pub definition_key: String,
    pub entity_tag_id: i64,
    pub tag_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeathSaves {
    pub fail_count: i64,
    pub success_count: i64,
    pub is_stabilized: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpellSlot {
    pub level: i64,
    pub used: i64,
    pub available: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PactMagic {
    pub level: i64,
    pub used: i64,
    pub available: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Spells {
    pub race: Vec<Value>,
    pub class: Vec<Value>,
    pub background: Value,
    pub item: Vec<Value>,
    pub feat: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Options {
    pub race: Vec<Value>,
    pub class: Vec<Value>,
    pub background: Value,
    pub item: Value,
    pub feat: Vec<Feat2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Feat2 {
    pub component_id: i64,
    pub component_type_id: i64,
    pub definition: Definition7,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Definition7 {
    pub id: i64,
    pub entity_type_id: i64,
    pub name: String,
    pub description: String,
    pub snippet: String,
    pub activation: Value,
    pub source_id: Value,
    pub source_page_number: Value,
    pub creature_rules: Vec<Value>,
    pub spell_list_ids: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Choices {
    pub race: Vec<Race2>,
    pub class: Vec<Class2>,
    pub background: Vec<Background2>,
    pub item: Value,
    pub feat: Vec<Feat3>,
    pub choice_definitions: Vec<ChoiceDefinition>,
    pub definition_key_name_map: DefinitionKeyNameMap,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Race2 {
    pub component_id: i64,
    pub component_type_id: i64,
    pub id: String,
    pub parent_choice_id: Value,
    #[serde(rename = "type")]
    pub type_field: i64,
    pub sub_type: Option<i64>,
    pub option_value: Option<i64>,
    pub label: Option<String>,
    pub is_optional: bool,
    pub is_infinite: bool,
    pub default_subtypes: Vec<Value>,
    pub display_order: Value,
    pub options: Vec<Value>,
    pub option_ids: Vec<i64>,
    pub tag_constraints: Vec<TagConstraint>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TagConstraint {
    pub id: i64,
    pub parent_component_type_id: i64,
    pub parent_component_id: i64,
    pub entity_modifier_id: i64,
    pub entity_tag_id: i64,
    pub definition_key: String,
    pub tag_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Class2 {
    pub component_id: i64,
    pub component_type_id: i64,
    pub id: String,
    pub parent_choice_id: Value,
    #[serde(rename = "type")]
    pub type_field: i64,
    pub sub_type: Option<i64>,
    pub option_value: Option<i64>,
    pub label: Option<String>,
    pub is_optional: bool,
    pub is_infinite: bool,
    pub default_subtypes: Vec<Value>,
    pub display_order: Value,
    pub options: Vec<Value>,
    pub option_ids: Vec<i64>,
    pub tag_constraints: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Background2 {
    pub component_id: i64,
    pub component_type_id: i64,
    pub id: String,
    pub parent_choice_id: Value,
    #[serde(rename = "type")]
    pub type_field: i64,
    pub sub_type: i64,
    pub option_value: i64,
    pub label: String,
    pub is_optional: bool,
    pub is_infinite: bool,
    pub default_subtypes: Vec<String>,
    pub display_order: Value,
    pub options: Vec<Value>,
    pub option_ids: Vec<i64>,
    pub tag_constraints: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Feat3 {
    pub component_id: i64,
    pub component_type_id: i64,
    pub id: String,
    pub parent_choice_id: Value,
    #[serde(rename = "type")]
    pub type_field: i64,
    pub sub_type: Value,
    pub option_value: i64,
    pub label: Value,
    pub is_optional: bool,
    pub is_infinite: bool,
    pub default_subtypes: Vec<Value>,
    pub display_order: Value,
    pub options: Vec<Value>,
    pub option_ids: Vec<i64>,
    pub tag_constraints: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChoiceDefinition {
    pub id: String,
    pub options: Vec<Option>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Option {
    pub id: i64,
    pub label: String,
    pub description: Option<String>,
    pub source_id: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DefinitionKeyNameMap {
    #[serde(rename = "feat:1789140")]
    pub feat_1789140: String,
    #[serde(rename = "feat:1789214")]
    pub feat_1789214: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Actions {
    pub race: Vec<Value>,
    pub class: Vec<Class3>,
    pub background: Value,
    pub item: Value,
    pub feat: Vec<Feat4>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Class3 {
    pub component_id: i64,
    pub component_type_id: i64,
    pub id: String,
    pub entity_type_id: String,
    pub limited_use: Value,
    pub name: String,
    pub description: String,
    pub snippet: String,
    pub ability_modifier_stat_id: Value,
    pub on_miss_description: String,
    pub save_fail_description: String,
    pub save_success_description: String,
    pub save_stat_id: Value,
    pub fixed_save_dc: Value,
    pub attack_type_range: Value,
    pub action_type: i64,
    pub attack_subtype: Value,
    pub dice: Dice2,
    pub value: Value,
    pub damage_type_id: Value,
    pub is_martial_arts: bool,
    pub is_proficient: bool,
    pub spell_range_type: Value,
    pub display_as_attack: bool,
    pub range: Range,
    pub activation: Activation2,
    pub number_of_targets: Value,
    pub fixed_to_hit: Value,
    pub ammunition: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dice2 {
    pub dice_count: i64,
    pub dice_value: i64,
    pub dice_multiplier: Value,
    pub fixed_value: Value,
    pub dice_string: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Range {
    pub range: Value,
    pub long_range: Value,
    pub aoe_type: Value,
    pub aoe_size: Value,
    pub has_aoe_special_description: bool,
    pub minimum_range: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Activation2 {
    pub activation_time: Value,
    pub activation_type: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Feat4 {
    pub component_id: i64,
    pub component_type_id: i64,
    pub id: String,
    pub entity_type_id: String,
    pub limited_use: Option<LimitedUse3>,
    pub name: String,
    pub description: String,
    pub snippet: String,
    pub ability_modifier_stat_id: Value,
    pub on_miss_description: String,
    pub save_fail_description: String,
    pub save_success_description: String,
    pub save_stat_id: Value,
    pub fixed_save_dc: Value,
    pub attack_type_range: Value,
    pub action_type: i64,
    pub attack_subtype: Value,
    pub dice: Value,
    pub value: Value,
    pub damage_type_id: Value,
    pub is_martial_arts: bool,
    pub is_proficient: bool,
    pub spell_range_type: Value,
    pub display_as_attack: Value,
    pub range: Range2,
    pub activation: Activation3,
    pub number_of_targets: Value,
    pub fixed_to_hit: Value,
    pub ammunition: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LimitedUse3 {
    pub name: Value,
    pub stat_modifier_uses_id: Value,
    pub reset_type: i64,
    pub number_used: i64,
    pub min_number_consumed: i64,
    pub max_number_consumed: i64,
    pub max_uses: i64,
    pub operator: i64,
    pub use_proficiency_bonus: bool,
    pub proficiency_bonus_operator: i64,
    pub reset_dice: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Range2 {
    pub range: Value,
    pub long_range: Value,
    pub aoe_type: Value,
    pub aoe_size: Value,
    pub has_aoe_special_description: bool,
    pub minimum_range: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Activation3 {
    pub activation_time: Value,
    pub activation_type: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Modifiers {
    pub race: Vec<Race3>,
    pub class: Vec<Class4>,
    pub background: Vec<Background3>,
    pub item: Vec<Item>,
    pub feat: Vec<Feat5>,
    pub condition: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Race3 {
    pub fixed_value: Value,
    pub id: String,
    pub entity_id: i64,
    pub entity_type_id: i64,
    #[serde(rename = "type")]
    pub type_field: String,
    pub sub_type: String,
    pub dice: Value,
    pub restriction: String,
    pub stat_id: Value,
    pub requires_attunement: bool,
    pub duration: Value,
    pub friendly_type_name: String,
    pub friendly_subtype_name: String,
    pub is_granted: bool,
    pub bonus_types: Vec<Value>,
    pub value: Value,
    pub available_to_multiclass: bool,
    pub modifier_type_id: i64,
    pub modifier_sub_type_id: i64,
    pub component_id: i64,
    pub component_type_id: i64,
    pub tag_constraints: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Class4 {
    pub fixed_value: Value,
    pub id: String,
    pub entity_id: Option<i64>,
    pub entity_type_id: Option<i64>,
    #[serde(rename = "type")]
    pub type_field: String,
    pub sub_type: String,
    pub dice: Value,
    pub restriction: String,
    pub stat_id: Value,
    pub requires_attunement: bool,
    pub duration: Value,
    pub friendly_type_name: String,
    pub friendly_subtype_name: String,
    pub is_granted: bool,
    pub bonus_types: Vec<Value>,
    pub value: Value,
    pub available_to_multiclass: bool,
    pub modifier_type_id: i64,
    pub modifier_sub_type_id: i64,
    pub component_id: i64,
    pub component_type_id: i64,
    pub tag_constraints: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Background3 {
    pub fixed_value: Value,
    pub id: String,
    pub entity_id: i64,
    pub entity_type_id: i64,
    #[serde(rename = "type")]
    pub type_field: String,
    pub sub_type: String,
    pub dice: Value,
    pub restriction: String,
    pub stat_id: Value,
    pub requires_attunement: bool,
    pub duration: Value,
    pub friendly_type_name: String,
    pub friendly_subtype_name: String,
    pub is_granted: bool,
    pub bonus_types: Vec<Value>,
    pub value: Value,
    pub available_to_multiclass: bool,
    pub modifier_type_id: i64,
    pub modifier_sub_type_id: i64,
    pub component_id: i64,
    pub component_type_id: i64,
    pub tag_constraints: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub fixed_value: i64,
    pub id: String,
    pub entity_id: Value,
    pub entity_type_id: Value,
    #[serde(rename = "type")]
    pub type_field: String,
    pub sub_type: String,
    pub dice: Dice3,
    pub restriction: String,
    pub stat_id: Value,
    pub requires_attunement: bool,
    pub duration: Value,
    pub friendly_type_name: String,
    pub friendly_subtype_name: String,
    pub is_granted: bool,
    pub bonus_types: Vec<Value>,
    pub value: Value,
    pub available_to_multiclass: bool,
    pub modifier_type_id: i64,
    pub modifier_sub_type_id: i64,
    pub component_id: i64,
    pub component_type_id: i64,
    pub tag_constraints: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dice3 {
    pub dice_count: i64,
    pub dice_value: i64,
    pub dice_multiplier: Value,
    pub fixed_value: i64,
    pub dice_string: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Feat5 {
    pub fixed_value: Option<i64>,
    pub id: String,
    pub entity_id: Option<i64>,
    pub entity_type_id: Option<i64>,
    #[serde(rename = "type")]
    pub type_field: String,
    pub sub_type: String,
    pub dice: Value,
    pub restriction: String,
    pub stat_id: Value,
    pub requires_attunement: bool,
    pub duration: Value,
    pub friendly_type_name: String,
    pub friendly_subtype_name: String,
    pub is_granted: bool,
    pub bonus_types: Vec<Value>,
    pub value: Option<i64>,
    pub available_to_multiclass: bool,
    pub modifier_type_id: i64,
    pub modifier_sub_type_id: i64,
    pub component_id: i64,
    pub component_type_id: i64,
    pub tag_constraints: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClassSpell {
    pub entity_type_id: i64,
    pub character_class_id: i64,
    pub spells: Vec<Value>,
}

