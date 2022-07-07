
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{InfoResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{InfoState, INFO_STATE,MAP_INFO};
//wasm1xufjsng0zjskcajhdlawdfe5sdn3ch4kwg9mugfv3wgt96zw4ptqg9yy8f -- 1510- increment age
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:coswasm-crud";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {

    let state = InfoState {
        name: msg.name,
        age: msg.age,
        uuid:msg.uuid
       
    };
    MAP_INFO.save(deps.storage,state.uuid.clone() , &state)?;
    
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    INFO_STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("count", msg.age.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Update {name,age
            } => update_data(deps,info,name,age),

        ExecuteMsg::UpdateMapInfo { uuid }=>increment_age(deps, info, uuid)
    }
}


pub fn update_data(deps: DepsMut, _info: MessageInfo, name: String,age:i32) -> Result<Response, ContractError> {
    INFO_STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
       
        state.name = name;
        state.age=age;
        Ok(state)
    })?;
    Ok(Response::new().add_attribute("method", "reset"))
}

// pub fn increment_age(deps: DepsMut, _info: MessageInfo, uuid: String) -> Result<Response, ContractError> {
//     MAP_INFO.update(deps.storage,uuid,| info:Option<InfoState>|->Result<Response>{
//         info.unwrap().age= info.unwrap().age+1;
//         info.unwrap().name= info.unwrap().name;
//         info.unwrap().uuid= uuid;
//         Ok(..info)

//         // Ok(InfoState {name:"".to_string(),age:2,uuid:"".to_string()})
//         // Ok(info.unwrap_or_default().age.checked_add(1)?)
//     });
//     Ok(Response::new().add_attribute("method", "increment")
//     .add_attribute("incremented_value", "1"))
// }


pub fn increment_age(deps: DepsMut, _info: MessageInfo, uuid: String) -> Result<Response, ContractError> {
    let state = MAP_INFO.load(deps.storage,uuid.clone())?;
    let updated_entry = InfoState {name:state.name,age:state.age+2,uuid:state.uuid};
    MAP_INFO.save(deps.storage, uuid.clone(), &updated_entry)? ;
    // MAP_INFO.update(deps.storage,uuid,| info:Option<InfoState>|->Result<Response>{
    //     info.unwrap().age= info.unwrap().age+1;
    //     info.unwrap().name= info.unwrap().name;
    //     info.unwrap().uuid= uuid;
    //     Ok(..info)

    //     // Ok(InfoState {name:"".to_string(),age:2,uuid:"".to_string()})
    //     // Ok(info.unwrap_or_default().age.checked_add(1)?)
    // });
    Ok(Response::new().add_attribute("method", "increment")
    .add_attribute("incremented_value", "2")
    .add_attribute("incremented_key", uuid.clone()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetData {} => to_binary(&query_count(deps)?),
        QueryMsg::GetMapData {rand_string} =>to_binary(&query_map(deps,rand_string)?)
    }
}

fn query_count(deps: Deps) -> StdResult<InfoResponse> {
    let state = INFO_STATE.load(deps.storage)?;
    Ok(InfoResponse { name:state.name,age:state.age,uuid:state.uuid.to_string() })
}

fn query_map(deps: Deps,rand:String) -> StdResult<Response> {
    let state = MAP_INFO.load(deps.storage,rand)?;
    let res = Response::new()
        // .add_attribute("action", "mint")
        .add_attribute("age", state.age.to_string())
        .add_attribute("name", state.name)
        .add_attribute("uuid",state.uuid);

    Ok(res)
}






