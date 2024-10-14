pub mod abilities;
pub mod skills;
pub mod core_attributes;
pub mod conditions;
pub mod saving_throws;

use skills::SkillsSortKey;
use saving_throws::SavingThrowsSortKey;
use core_attributes::CoreAttributesSortKey;
use abilities::AbilitiesSortKey;
use conditions::ConditionsSortKey;

#[derive(strum::Display)]
pub enum StatsSortKey {
    #[strum(to_string = "#Skills#{0}")]
    Skills(SkillsSortKey),
    #[strum(to_string = "#SavingThrows#{0}")]
    SavingThrows(SavingThrowsSortKey),
    #[strum(to_string = "#CoreAttributes#{0}")]
    CoreAttributes(CoreAttributesSortKey),
    #[strum(to_string = "#Abilities#{0}")]
    Abilities(AbilitiesSortKey),
    #[strum(to_string = "#Conditions#{0}")]
    Conditions(ConditionsSortKey),
    #[strum(to_string = "#Defenses")]
    Defenses(),
}
