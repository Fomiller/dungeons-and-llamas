use crate::tools::battle::BattleToolOutput;
use crate::tools::shop::ShopToolOutput;
use crate::tools::rest::RestToolOutput;

use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ApiScenarioResponse {
    pub data: ApiScenarioResponseData,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApiScenarioResponseData {
    Battle(BattleToolOutput),
    Shop(ShopToolOutput),
    Rest(RestToolOutput),
}
