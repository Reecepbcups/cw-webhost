#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Order, Response, StdError, StdResult,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Config, ExpireTimes, Website, CONFIG, EXPIRE_TIMES, WEBSITES};

const CONTRACT_NAME: &str = "crates.io:cw-webhost";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    CONFIG.save(
        deps.storage,
        &Config {
            manager: msg.managers,
            cost: msg.cost,
            period: msg.period.unwrap_or_default(),
        },
    )?;

    Ok(Response::new().add_attribute("method", "instantiate"))
}

fn is_manager(deps: Deps, info: MessageInfo) -> bool {
    let config = match CONFIG.load(deps.storage) {
        Ok(config) => config,
        Err(_) => return false,
    };

    match config.manager {
        Some(managers) => managers.contains(&info.sender.to_string()),
        None => false,
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::NewWebsite { name, source } => {
            if WEBSITES.may_load(deps.storage, &name)?.is_some() {
                return Err(ContractError::AlreadyExists { name: name });
            }

            let config = CONFIG.load(deps.storage)?;

            if let Some(cost) = config.cost {
                if !info.funds.contains(&cost) {
                    return Err(ContractError::NotEnoughFunds {
                        required: cost.to_string(),
                    });
                }
            }

            let website = Website {
                creator: info.sender.to_string(),
                source: source,
                created_height: env.block.height,
                in_grace_period: false,
            };

            WEBSITES.save(deps.storage, &name, &website)?;
            Ok(Response::new())
        }

        ExecuteMsg::ExpireOld {} => {
            // TODO: This would be put into x/clock module
            let config = CONFIG.load(deps.storage)?;

            // if there are no expirations, then it does not matter
            if config.period == 0 {
                return Err(ContractError::Std(StdError::GenericErr {
                    msg: "".to_string(),
                }));
            }

            check_expirations(deps, env, &config, String::new())?;
            Ok(Response::new())
        }

        // manager only
        ExecuteMsg::Remove { name } => {
            if !is_manager(deps.as_ref(), info) {
                return Err(ContractError::Unauthorized {});
            }

            WEBSITES.remove(deps.storage, &name);
            Ok(Response::new())
        }
    }
}

fn check_expirations(deps: DepsMut, env: Env, config: &Config, name: String) -> StdResult<()> {
    // see if there is a cooldown in the config, if so then set it in the EXPIRE_TIMES map
    let expire_height = env.block.height + config.period;
    let expire_times = EXPIRE_TIMES.may_load(deps.storage, expire_height)?;

    if let Some(mut expire_times) = expire_times {
        expire_times.names.push(name.clone());
        EXPIRE_TIMES.save(deps.storage, expire_height, &expire_times)?;
    } else {
        let expire_times = ExpireTimes {
            names: vec![name.clone()],
        };
        EXPIRE_TIMES.save(deps.storage, expire_height, &expire_times)?;
    }

    let mut grace_periods: Vec<String> = Vec::new();
    let mut del_keys: Vec<u64> = Vec::new();

    for k in EXPIRE_TIMES.range(deps.storage, None, None, Order::Ascending) {
        let (key, expire_times) = k?;

        if key < env.block.height {
            for name in expire_times.names {
                grace_periods.push(name);
            }

            del_keys.push(key);
        }
    }

    for key in del_keys {
        // i assume it is not safe to remove in range yea?
        EXPIRE_TIMES.remove(deps.storage, key);
    }

    for name in grace_periods {
        WEBSITES.update(deps.storage, &name, |website| match website {
            Some(mut website) => {
                website.in_grace_period = true;
                Ok(website)
            }
            None => Err(StdError::generic_err("No website found")),
        })?;
    }

    Ok(())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetWebsite { name } => to_binary(&query_website(deps, name)?),
    }
}

fn query_website(deps: Deps, name: String) -> StdResult<Website> {
    let site = WEBSITES.may_load(deps.storage, &name)?;

    match site {
        Some(site) => Ok(site),
        None => Err(StdError::generic_err("No website found")),
    }
}
