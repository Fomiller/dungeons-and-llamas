use crate::scenarios::battle::BattleScenario;
use crate::scenarios::rest::RestScenario;
use crate::scenarios::shop::ShopScenario;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct SettingsResponse {}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewGameResponse {
    pub game_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ApiScenarioResponse {
    pub data: ApiScenarioResponseData,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApiScenarioResponseData {
    Battle(BattleScenario),
    Shop(ShopScenario),
    Rest(RestScenario),
}
