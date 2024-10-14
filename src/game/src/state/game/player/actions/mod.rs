pub mod spells;

use spells::SpellSortKey;

#[derive(strum::Display)]
pub enum ActionsSortKey {
    #[strum(to_string = "Spells#{0}")]
    Spells(SpellSortKey),
    #[strum(to_string = "#Action")]
    Action,
    #[strum(to_string = "#BonusAction")]
    BonusAction,
    #[strum(to_string = "#Action")]
    Reaction,
}
// 345345345345#GameState#Player#Actions#Spells#Cantrip
// 345345345345#GameState#Player#Actions#Spells#Spell#Concentration
// 345345345345#GameState#Player#Actions#Spells#Spell#Instant
// 345345345345#GameState#Player#Actions#Action
// 345345345345#GameState#Player#Actions#BonusAction
// 345345345345#GameState#Player#Actions#Reaction
