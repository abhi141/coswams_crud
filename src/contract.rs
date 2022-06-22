
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{InfoResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{InfoState, INFO_STATE};

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
        age: msg.age
    };
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
            } => update_data(deps,info,name,age)
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

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetData {} => to_binary(&query_count(deps)?),
    }
}

fn query_count(deps: Deps) -> StdResult<InfoResponse> {
    let state = INFO_STATE.load(deps.storage)?;
    Ok(InfoResponse { name:state.name,age:state.age })
}


