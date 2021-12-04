#[cfg(feature = "std")]

use cargo_contract::cmd::CallCommand;
use cargo_contract::ExtrinsicOpts;
use std::str::FromStr;
use anyhow::Result;

const WS_ENDPOINT: &str = "ws://127.0.0.1:9944";

const MARKET_CONTRACT_ADDR: &str = "5DVJQ7rK6L5fgRvDQpjQ7CMweyXAaYXwRBKMQY7yKhGo5hqk";
const MARKET_ABI_PATH: &str = "./release/services_market.json";

const STAT_CONTRACT_ADDR: &str = "5CGW7GKo13RdxwMFuYuUEw5eAq3eTM7G9fGk16p3EzKuGi3r";
const STAT_ABI_PATH: &str = "./release/services_statistics.json";

const SURI: &str = "0xe40891ed4fa2eb6b8b89b1d641ae72e8c1ba383d809eeba64131b37bf0aa3898";
const GAS_LIMIT: u64 = 50000000000;

fn main() {
    // query query_service_by_index
    let result = call(
        MARKET_CONTRACT_ADDR.to_string(),
        MARKET_ABI_PATH.to_string(),
        String::from("query_service_by_index"),
        vec![
            String::from("0")
        ]);
    match result {
        Ok(r) => {
            println!("call result: {}", r)
        }
        Err(e) => {
            println!("call err: {}", e)
        }
    }

    // // exe allowed_provider
    // let result = exec("allowed_provider".to_string(),
    //                   vec!["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY".to_string()]);
    // match result {
    //     Ok(r) => {
    //         println!("exec result: {}", r)
    //     }
    //     Err(e) => {
    //         println!("exec err: {}", e)
    //     }
    // }

    // exe add_service
    const uuid: &'static str = "1";
    let result = exec(
        MARKET_CONTRACT_ADDR.to_string(),
        MARKET_ABI_PATH.to_string(),
        String::from("add_service"),
                      vec![
                          format!("\"{}\"", uuid), //uuid
                          format!("\"{}\"", "test1"), //name
                          format!("\"{}\"", "test1"), //desc
                          format!("\"{}\"", "test1"), //logo
                          String::from("12345678"), //createTime
                          format!("\"{}\"", "test1"), //providerName
                          format!("\"{}\"", "5F7Xv7RaJe8BBNULSuRTXWtfn68njP1NqQL5LLf41piRcEJJ"), //providerOwner
                          format!("\"{}\"", "test1"), //usage
                          format!("\"{}\"", "test1"), //schema
                          format!("\"{}\"", "test1"), //pricePlan
                          format!("\"{}\"", "test1"), //declaimer
                      ]
        );
    match result {
        Ok(r) => {
            println!("exec result: {}", r)
        }
        Err(e) => {
            println!("exec err: {}", e)
        }
    }

    // exe submit_usage
    let result = exec(
        STAT_CONTRACT_ADDR.to_string(),
        STAT_ABI_PATH.to_string(),
        String::from("submit_usage"),
                      vec![
                          format!("\"{}\"", uuid), //service_uuid
                          String::from("0"), //nonce
                          format!("\"{}\"", "test1"), //user_key
                          String::from("12345678"), //start_time
                          String::from("12345678"), //end_time
                          String::from("12345678"), //usage
                          format!("\"{}\"", "test1"), //price_plan
                          String::from("12345678"), //cost
                      ]
        );
    match result {
        Ok(r) => {
            println!("exec result: {}", r)
        }
        Err(e) => {
            println!("exec err: {}", e)
        }
    }
}

fn call(contract_addr: String, abi_path: String, name: String, args: Vec<String>) -> Result<String> {
    let url = url::Url::from_str(WS_ENDPOINT).unwrap();
    let call = CallCommand {
        name,
        args,
        extrinsic_opts: ExtrinsicOpts{
            url,
            suri: SURI.to_string(),
            password: None,
            verbosity: Default::default()
        },
        gas_limit: GAS_LIMIT,
        value: 0,
        contract: contract_addr,
        rpc: true,
        path: abi_path
    };
    call.run()
}

fn exec(contract_addr: String, abi_path: String, name: String, args: Vec<String>) -> Result<String> {
    let url = url::Url::from_str(WS_ENDPOINT).unwrap();
    let call = CallCommand {
        name,
        args,
        extrinsic_opts: ExtrinsicOpts{
            url,
            suri: SURI.to_string(),
            password: None,
            verbosity: Default::default()
        },
        gas_limit: GAS_LIMIT,
        value: 0,
        contract: contract_addr,
        rpc: false,
        path: abi_path
    };
    call.run()
}
