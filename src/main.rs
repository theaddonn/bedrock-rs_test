use std::collections::HashMap;
use bedrock_rs::core::int::{LE, VAR};
use std::fs;
use std::io::Cursor;
use std::net::{SocketAddr, SocketAddrV4};
use std::str::FromStr;
use std::thread::sleep;
use std::time::Duration;
use bedrock_rs::addons::Addon;
use bedrock_rs::addons::behavior::BehaviorPack;
use bedrock_rs::addons::resource::ResourcePack;
use bedrock_rs::core::*;
use bedrock_rs::core::actor_runtime_id::ActorRuntimeID;
use bedrock_rs::core::actor_unique_id::ActorUniqueID;
use bedrock_rs::core::world::difficulty::Difficulty;
use bedrock_rs::core::world::dimension::Dimension;
use bedrock_rs::core::world::editor_world_type::EditorWorldType;
use bedrock_rs::core::world::gamemode::Gamemode;
use bedrock_rs::core::world::generator_type::GeneratorType;
use bedrock_rs::nbt::NbtTag;
use bedrock_rs::proto::codec::ProtoCodec;
use bedrock_rs::proto::gamepacket::GamePacket;
use bedrock_rs::proto::gamepacket::GamePacket::StartGame;
use bedrock_rs::proto::login::handle::login_to_server;
use bedrock_rs::proto::login::provider::DefaultLoginProvider;
use bedrock_rs::proto::packets::disconnect::DisconnectPacket;
use bedrock_rs::proto::packets::start_game::StartGamePacket;
use bedrock_rs::proto::types::base_game_version::BaseGameVersion;
use bedrock_rs::proto::types::chat_restriction_level::ChatRestrictionLevel;
use bedrock_rs::proto::types::edu_shared_uri_resource::EduSharedResourceUri;
use bedrock_rs::proto::types::experiments::Experiments;
use bedrock_rs::proto::types::level_settings::LevelSettings;
use bedrock_rs::proto::types::network_block_pos::NetworkBlockPos;
use bedrock_rs::proto::types::network_permissions::NetworkPermissions;
use bedrock_rs::proto::types::player_movement_mode::PlayerMovementMode;
use bedrock_rs::proto::types::player_movement_settings::PlayerMovementSettings;
use bedrock_rs::proto::types::spawn_biome_type::SpawnBiomeType;
use bedrock_rs::proto::types::spawn_settings::SpawnSettings;
use tokio::main;
use uuid::Uuid;
//use bedrock_rs::world::World;

#[main]
async fn main() {
    let json = BehaviorPack::import("addon/bob/BP").unwrap();

    fs::write("bao_parsed_bp.txt", format!("{:#?}", json)).unwrap();


    let json = ResourcePack::import("addon/bob/RP").unwrap();

    fs::write("bao_parsed_rp.txt", format!("{:#?}", json)).unwrap();

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
                Err(e) => { format!("ERR({e:#?})") }
            };

            println!("LOGIN RESULT: {}", res_message);

            loop {
                println!("{:?}", conn.recv().await.unwrap());
            }

            conn.send(GamePacket::Disconnect(
                DisconnectPacket {
                    reason: VAR::new(0),
                    message: Some(res_message),
                }
            )).await.unwrap();

            conn.close().await.expect("TODO: panic message");
        });
    }
}

