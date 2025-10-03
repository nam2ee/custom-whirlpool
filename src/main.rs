// for see the current problem, erase line 3 and uncomment line 4 then execute!

use fixed_orca_whirlpools::{fetch_positions_in_whirlpool , fetch_whirlpools_by_token_pair, set_whirlpools_config_address, PoolInfo, WhirlpoolsConfigInput};
//use original_whirlpools::{fetch_positions_in_whirlpool, fetch_whirlpools_by_token_pair, set_whirlpools_config_address, PoolInfo, WhirlpoolsConfigInput};
use solana_rpc_client::nonblocking::rpc_client::RpcClient;
use solana_pubkey::Pubkey;
use std::str::FromStr;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    set_whirlpools_config_address(WhirlpoolsConfigInput::SolanaDevnet).unwrap();
    let rpc = Arc::new(RpcClient::new("https://api.devnet.solana.com".to_string()));
    let whirlpool_address = Pubkey::from_str("3KBZiL2g8C7tiJ32hTv5v3KM7aK9htpqTw4cTXz1HvPt").unwrap();


    let positions = fetch_positions_in_whirlpool(&rpc, whirlpool_address)
        .await
        .unwrap();

    let token_a = Pubkey::from_str("So11111111111111111111111111111111111111112").unwrap();
    let token_b = Pubkey::from_str("BRjpCHtyQLNCo8gqRUr8jtdAj5AjPYQaoqbvcZiHok1k").unwrap(); 

    let handle = tokio::spawn(async move  {

        let pool_infos = fetch_whirlpools_by_token_pair(&rpc , token_a, token_b).await.unwrap();
        for pool_info in pool_infos {
            match pool_info {
                PoolInfo::Initialized(pool) => println!("Pool is initialized: {:?}", pool),
                PoolInfo::Uninitialized(pool) => println!("Pool is not initialized: {:?}", pool),
            }
        }


    });

    handle.await.unwrap();

}
