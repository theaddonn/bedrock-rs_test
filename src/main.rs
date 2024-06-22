use std::net::{SocketAddr, SocketAddrV4};
use std::str::FromStr;
use std::time::Duration;

use bedrock_rs::core::*;
use bedrock_rs::proto::gamepacket::GamePacket;
use bedrock_rs::proto::login::handle::login_to_server;
use bedrock_rs::proto::login::provider::DefaultLoginProvider;
use bedrock_rs::proto::packets::disconnect::DisconnectPacket;
use bedrock_rs::text::color;
use tokio::main;

//use bedrock_rs::world::World;

#[main]
async fn main() {
    //let world = World::open("C:/Users/adria/AppData/Local/Packages/Microsoft.MinecraftUWP_8wekyb3d8bbwe/LocalState/games/com.mojang/minecraftWorlds/cBFbZglOAQA=").unwrap();

    let y = 6;
    let x = color!("<red>Hello red, <bold>red bold</bold> Just red again</red> and now nothing lol");

    println!("{x}");

    //println!("WORLD: \n{:#?}", world);

    let mut listener = bedrock_rs::proto::listener::Listener::new_raknet(
        String::from("My Server"),
        String::from("bedrock-rs"),
        10,
        2,
        SocketAddr::V4(SocketAddrV4::from_str("127.0.0.1:19132").unwrap()),
        false,
    ).await.unwrap();

    listener.start().await.unwrap();

    loop {
        let mut conn = listener.accept().await.unwrap();

        tokio::spawn(async move {
            println!("===== START =====");

            let mut conn = conn.into_shard(Duration::from_millis((Duration::from_secs(1).as_millis() / 20) as u64), 256).await;

            let res_message = match login_to_server(&mut conn, DefaultLoginProvider::new()).await {
                Ok(_) => { "success!".to_string() }
                Err(e) => { format!("ERR({e:?})") }
            };

            println!("LOGIN RESULT: {:?}", res_message);

            conn.send(GamePacket::Disconnect(
                DisconnectPacket {
                    reason: VAR::new(0),
                    message: Some(res_message),
                }
            )).await.unwrap();

            conn.close();
        });
    }
}

