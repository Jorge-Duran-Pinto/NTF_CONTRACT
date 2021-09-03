
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, BlockInfo, Deps, DepsMut, Env, MessageInfo, Order, Pair, Response, StdError,
    StdResult,
};

use cw0::maybe_addr;
use cw2::set_contract_version;
use cw721::{
    AllNftInfoResponse, ApprovedForAllResponse, ContractInfoResponse, Cw721ReceiveMsg, Expiration,
    NftInfoResponse, NumTokensResponse, OwnerOfResponse, TokensResponse,
};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, MintMsg, MinterResponse, QueryMsg};
use crate::state::{
    increment_tokens, num_tokens, tokens, Approval, TokenInfo, CONTRACT_INFO, MINTER, OPERATORS,
};
use cw_storage_plus::Bound;


//todo
const CONTRACT_NAME: &str = "crates.io:gaming-nft";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    // we need in a new smart contarct: contractInfoResponse, contract_info, minter, response::default()
    let info = ContractInfoResponse {
        name: msg.name,
        symbol: msg.symbol,
    };

    CONTRACT_INFO.save(deps.storage, &info)?;

    let minter = deps.api.addr_validate(&msg.minter);
    MINTER.save(deps.storage, &minter.unwrap())?;

    Ok(Response::default())

}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Mint(msg) => execute_mint(deps, _env, info,msg),
        ExecuteMsg::TransferNft => (),
        ExecuteMsg::SendNft => (),
        ExecuteMsg::Approve => (),
        ExecuteMsg::Revoke => (),
        ExecuteMsg::ApproveAll => (),
        ExecuteMsg::RevokeAll => (),
    }
    Ok(Response::default())
}

pub fn execute_mint(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    // get the minter from storage todo
    // check the sender is the minter (retrun ContractError::Unauthorized) todo
    // create a new token todo
    // update token list from storage todo
    // call increment token count todo
    // return a Response
    Ok(Response::default())
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies(&[]);

        let msg = InstantiateMsg { 
            name: "NTF_CONTRACT".to_string(),
            symbol: "NTF".to_string(),
            minter: "Admin".to_string() 
        };
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

    }
    
    fn minting() {
        // create a ntf contract
        let mut deps = mock_dependencies(&[]);

        let msg = InstantiateMsg { 
            name: "NTF_CONTRACT".to_string(),
            symbol: "NTF".to_string(),
            minter: "Admin".to_string() 
        };
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // create a nft
        // create a ExecuteMsg::Mint(MintMsg)
        let token_id = "petrify".to_string();
        let name = "Petrify with Gaze".to_string();
        let description = "Allows the owner to petrify anyone looking at him or her".to_string();

        let mint_msg = ExecuteMsg::Mint(MintMsg {
            token_id: token_id.clone(),
            owner: String::from("user"),
            name: name.clone(),
            description: Some(description.clone()),
            image: None,
        });

        let res = execute(deps.as_mut(), mock_env(), info, mint_msg).unwrap();
    }

}
