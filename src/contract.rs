#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Addr, CosmosMsg, DepsMut, Env, MessageInfo, Response, Timestamp, SubMsg};
use kujira::msg::{KujiraMsg, AuthMsg};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    mut _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<KujiraMsg>, ContractError> {
    match msg {   
        ExecuteMsg::TokensTransferVest { to_address, vest_end_time } => {
            execute_tokens_vest(deps, env, info, to_address, vest_end_time)
        }
    }
}

pub fn execute_tokens_vest(
    _deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    to_address: Addr,
    vest_end_time: Timestamp,
) -> Result<Response<KujiraMsg>, ContractError> {
    Ok(Response::new()
        .add_submessage(SubMsg::new(CosmosMsg::Custom(KujiraMsg::Auth(
            AuthMsg::CreateVestingAccount {
                to_address,
                amount: info.funds,
                end_time: Some(vest_end_time),
                delayed: Some(false),
            },
        ))))
        .add_attribute("action", "tokens_transfer_vest")
        // .add_attribute("amount", info.funds.)
        .add_attribute("sender", info.sender))
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{DepsMut, Addr, coins, Timestamp};
    use cosmwasm_std::testing::{mock_info, mock_env, mock_dependencies};

    use crate::{msg::{InstantiateMsg, ExecuteMsg}, contract::execute};

    use crate::contract::{instantiate};

    fn default_instantiate(deps: DepsMut) {
        let msg = InstantiateMsg {};
        let info = mock_info("creator", &[]);
        instantiate(deps, mock_env(), info, msg).unwrap();
    }
    
    #[test]
    fn test_transfer_to_vest_account() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        default_instantiate(deps.as_mut());

        let to_address = Addr::unchecked("to_address1234567890");
        let vest_end_time = Timestamp::from_seconds(1_000_000_000_u64);

        let msg = ExecuteMsg::TokensTransferVest{ to_address, vest_end_time };
        let info = mock_info("from_address1223456", &coins(1000_u128, "ukuji"));
        let res = execute(deps.as_mut().branch(), env, info, msg).unwrap();        
        assert_eq!(res.messages.len(), 1);
    }
}