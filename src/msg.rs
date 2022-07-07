use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
// wasm168cahegeag4f4e7ed0ax9f4nj2tqjupz48vdww3hwzwuyvnn7qrqpt6cn9 -map_info

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub name: String,
    pub age:i32,
    pub uuid:String
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Update {name:String,age:i32},
    UpdateMapInfo{uuid:String}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    GetData {},
    GetMapData{rand_string:String}
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InfoResponse {
    pub name: String,
    pub age:i32,
    pub uuid:String
}

pub struct MapInfoResponse {
    pub name: String,
    pub age:i32,
    pub uuid:String
}
