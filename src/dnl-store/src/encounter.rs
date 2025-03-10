use serde::*;
use dnl_types::tools::battle::BattleToolOutput;
use dnl_types::tools::shop::ShopToolOutput;
use dnl_types::tools::rest::RestToolOutput;

#[derive(Debug, Serialize, Deserialize)]
pub struct EncounterQuery {
    pub text: String,
    pub name: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EncounterState {
    pub text: String,
    pub data: BattleToolOutput
}
