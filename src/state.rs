use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cw_storage_plus::{Item,Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InfoState {
    pub name: String,
    pub age:i32,
    pub uuid:String
}

pub const INFO_STATE: Item<InfoState> = Item::new("info_state");
pub const MAP_INFO :Map<String,InfoState>=Map::new("map_info");
