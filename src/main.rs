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
    // let json = BehaviorPack::import("addon/bob/BP").unwrap();
    //
    // fs::write("bao_parsed_bp.txt", format!("{:#?}", json)).unwrap();
    //
    //
    // let json = ResourcePack::import("addon/bob/RP").unwrap();
    //
    // fs::write("bao_parsed_rp.txt", format!("{:#?}", json)).unwrap();

    // let start_game = StartGamePacket {
    //     target_actor_id: ActorUniqueID(888),
    //     target_runtime_id: ActorRuntimeID(9999),
    //     actor_game_type: Gamemode::Survival,
    //     position: Vec3 {
    //         x: LE::new(2.4),
    //         y: LE::new(5.67),
    //         z: LE::new(230.665),
    //     },
    //     rotation: Vec2 {
    //         x: LE::new(45.009),
    //         y: LE::new(1.900),
    //     },
    //     settings: LevelSettings {
    //         seed: LE::new(80085),
    //         spawn_settings: SpawnSettings {
    //             biome_type: SpawnBiomeType::Default,
    //             user_defined_biome_name: String::from("user_defined_biome_name"),
    //             dimension: Dimension::Overworld,
    //         },
    //         generator_type: GeneratorType::Overworld,
    //         game_type: Gamemode::Survival,
    //         hardcore: true,
    //         difficulty: Difficulty::Peaceful,
    //         default_spawn_block: NetworkBlockPos {
    //             x: VAR::new(2),
    //             y: VAR::new(6),
    //             z: VAR::new(7),
    //         },
    //         achievements_disabled: true,
    //         editor_world_type: EditorWorldType::NotEditor,
    //         created_in_editor: false,
    //         exported_from_editor: true,
    //         day_cycle_stop_time: VAR::new(3),
    //         education_edition_offer: VAR::new(0),
    //         education_features: false,
    //         education_product_id: String::from("ADE"),
    //         rain_level: LE::new(0.12),
    //         lightning_level: LE::new(0.43),
    //         platform_locked_content: false,
    //         multiplayer_intended: true,
    //         lan_broadcasting_intended: true,
    //         broadcasting_settings_xbox_live: VAR::new(4),
    //         broadcasting_settings_platform: VAR::new(4),
    //         commands_enabled: true,
    //         texture_pack_required: false,
    //         gamerules: vec![],
    //         experiments: Experiments {
    //             experiments: vec![],
    //             ever_toggled: true,
    //         },
    //         bonus_chest: false,
    //         start_with_map: true,
    //         player_permission: VAR::new(0),
    //         server_chunk_tick_radius: LE::new(4),
    //         locked_behavior_packs: false,
    //         locked_resource_packs: true,
    //         from_locked_template: false,
    //         msa_gamertags_only: true,
    //         from_template: false,
    //         is_template_locked_settings: false,
    //         only_spawn_v1_villagers: true,
    //         persona_disabled: false,
    //         custom_skins_disabled: true,
    //         emote_chat_muted: false,
    //         base_game_version: BaseGameVersion(String::from("*")),
    //         limited_world_width: LE::new(16),
    //         limited_world_depth: LE::new(16),
    //         new_nether: true,
    //         edu_shared_uri_resource: EduSharedResourceUri {
    //             button_name: String::from(""),
    //             link_uri: String::from(""),
    //         },
    //         force_experimental_gameplay: Some(false),
    //         chat_restriction_level: ChatRestrictionLevel::None,
    //         disable_player_interactions: false,
    //         server_id: String::from("2333"),
    //         world_id: String::from("23232524234"),
    //         scenario_id: String::from("34563456345"),
    //     },
    //     level_id: String::from("3456345"),
    //     level_name: String::from("34563456"),
    //     template_content_identity: String::from("34563456"),
    //     trial: false,
    //     movement_settings: PlayerMovementSettings {
    //         authority_mode: PlayerMovementMode::Client,
    //         rewind_history_size: VAR::new(56),
    //         server_authoritative_block_breaking: false,
    //     },
    //     current_level_time: LE::new(12000),
    //     enchantment_seed: VAR::new(80085),
    //     block_properties: vec![],
    //     items: vec![],
    //     multiplayer_correlation_id: String::from("543635634363456"),
    //     enable_item_stack_net_manager: false,
    //     server_version: String::from("1.21.0"),
    //     player_property_data: NbtTag::Compound(HashMap::new()),
    //     block_type_registry_checksum: LE::new(5),
    //     world_template_id: Uuid::new_v4(),
    //     enable_clientside_world_generation: false,
    //     use_block_network_id_hashes: false,
    //     network_permission: NetworkPermissions {
    //         server_auth_sound_enabled: true,
    //     },
    // };

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

