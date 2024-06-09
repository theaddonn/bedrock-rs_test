use std::io::Cursor;
use std::net::SocketAddrV4;
use std::str::FromStr;

use bedrock_rs::proto::compression::Compression;
use bedrock_rs::proto::listener::ListenerConfig;
use bedrock_rs::proto::login::{handle_login_server_side, LoginServerSideOptions};
use tokio::main;

use bedrock_rs::core::*;
use bedrock_rs::proto::gamepacket::GamePacket;
use bedrock_rs::proto::packets::disconnect::DisconnectPacket;
//use bedrock_rs::world::World;

#[main]
async fn main() {


    //let world = World::open("C:/Users/adria/AppData/Local/Packages/Microsoft.MinecraftUWP_8wekyb3d8bbwe/LocalState/games/com.mojang/minecraftWorlds/cBFbZglOAQA=").unwrap();

    //println!("WORLD: \n{:#?}", world);

    let mut listener = bedrock_rs::proto::listener::Listener::new_raknet(
        ListenerConfig {
            name: String::from("My Server"),
            sub_name: String::from("bedrock-rs"),
            player_count_max: i32::MAX as u32,
            player_count_current: i32::MAX as u32 -1,
            nintendo_limited: false,
        },
        SocketAddrV4::from_str("127.0.0.1:19132").unwrap(),
    ).await.unwrap();

    listener.start().await.unwrap();

    loop {
        let mut conn = listener.accept().await.unwrap();

        tokio::spawn(async move {
            println!("===== START =====");

            let res_message = match handle_login_server_side(&mut conn, LoginServerSideOptions {
                compression: Compression::None,
                encryption: false,
                authentication_enabled: false,
                allowed_proto_versions: vec![671, 662],
            }).await {
                Ok(_) => { "success!".to_string() }
                Err(e) => { format!("ERR({e:?})") }
            };

            println!("LOGIN RESULT: {:?}", res_message);

            conn.send_gamepackets(vec![GamePacket::Disconnect(
                DisconnectPacket {
                    reason: VAR::new(0),
                    message: Some(res_message),
                }
            )]).await.unwrap();

            loop {

            }
        });
    }
}

