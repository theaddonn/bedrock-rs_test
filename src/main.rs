use std::io::Cursor;
use std::net::SocketAddrV4;
use std::str::FromStr;

use bedrock_rs::proto::compression::CompressionMethods;
use bedrock_rs::proto::compression::zlib::ZlibCompression;
use bedrock_rs::proto::listener::ListenerConfig;
use bedrock_rs::proto::login::{handle_login_server_side, LoginServerSideOptions};
use tokio::main;

use bedrock_rs::core::*;
use bedrock_rs::nbt::little_endian::NbtLittleEndian;
use bedrock_rs::nbt::NbtTag;
use bedrock_rs::proto::gamepacket::GamePacket;
use bedrock_rs::proto::packets::disconnect::DisconnectPacket;
use byteorder::{LittleEndian, ReadBytesExt};

#[main]
async fn main() {

    let mut cur = Cursor::new(std::fs::read("Hoffnung/level.dat").unwrap());

    let ver = cur.read_i32::<LittleEndian>().unwrap();
    let len = cur.read_i32::<LittleEndian>().unwrap();

    println!("VER: {ver} | LEN: {len}");

    let (str, tag) = NbtTag::nbt_deserialize::<NbtLittleEndian>(&mut cur).unwrap();

    println!("STR: {str:?}");
    println!("TAG: {tag:#?}");

    let mut listener = bedrock_rs::proto::listener::Listener::new(
        ListenerConfig {
            name: String::from("My Server"),
            sub_name: String::from("bedrock-rs"),
            player_count_max: 10,
            player_count_current: 0,
            nintendo_limited: false,
        },
        SocketAddrV4::from_str("127.0.0.1:19132").unwrap(),
    ).await.unwrap();

    listener.start().await.unwrap();

    loop {
        let mut conn = listener.accept().await.unwrap();

        tokio::spawn(async move {
            println!("started!");

            let res_message = match handle_login_server_side(&mut conn, LoginServerSideOptions {
                compression: CompressionMethods::Zlib(ZlibCompression { threshold: 1024, compression_level: 9 }),
                encryption: false,
                authentication_enabled: false,
                allowed_proto_versions: vec![671, 662],
            }).await {
                Ok(_) => { "success!".to_string() }
                Err(e) => { format!("ERR({e:?})") }
            };

            conn.send_gamepackets(vec![GamePacket::Disconnect(
                DisconnectPacket {
                    reason: ivar32(0),
                    message: Some(res_message),
                }
            )]).await.unwrap();

            loop {

            }
        });
    }
}

