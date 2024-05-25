use std::net::SocketAddrV4;
use std::str::FromStr;

use bedrock_rs::proto::compression::CompressionMethods;
use bedrock_rs::proto::compression::zlib::ZlibCompression;
use bedrock_rs::proto::listener::ListenerConfig;
use bedrock_rs::proto::login::{handle_login_server_side, LoginServerSideOptions};
use tokio::main;

use miniz_oxide::deflate::{compress_to_vec, compress_to_vec_zlib};
use miniz_oxide::inflate::{decompress_to_vec, decompress_to_vec_zlib};
use rusty_leveldb::compressor::NoneCompressor;
use rusty_leveldb::{Compressor, CompressorList, Options, DB, LdbIterator};
use std::rc::Rc;
use bedrock_rs::core::*;
use bedrock_rs::proto::gamepacket::GamePacket;
use bedrock_rs::proto::packets::disconnect::DisconnectPacket;

/// A zlib compressor that with zlib wrapper
struct ZlibCompressor(u8);

impl ZlibCompressor {
    /// level 0-10
    pub fn new(level: u8) -> Self {
        assert!(level <= 10);
        Self(level)
    }
}

impl Compressor for ZlibCompressor {
    fn encode(&self, block: Vec<u8>) -> rusty_leveldb::Result<Vec<u8>> {
        Ok(compress_to_vec_zlib(&block, self.0))
    }

    fn decode(&self, block: Vec<u8>) -> rusty_leveldb::Result<Vec<u8>> {
        decompress_to_vec_zlib(&block).map_err(|e| rusty_leveldb::Status {
            code: rusty_leveldb::StatusCode::CompressionError,
            err: e.to_string(),
        })
    }
}

/// A zlib compressor that without zlib wrapper
///
/// > windowBits can also be –8..–15 for raw deflate. In this case, -windowBits determines the window size. deflate() will then generate raw deflate data with no zlib header or trailer, and will not compute a check value.
/// >
/// > From [zlib manual](https://zlib.net/manual.html)
///
/// It seems like mojang use this most
///
/// A copy of mojang's implementation can be find [here](https://github.com/reedacartwright/rbedrock/blob/fb32a899da4e15c1aaa0d6de2b459e914e183516/src/leveldb-mcpe/db/zlib_compressor.cc#L119).
struct RawZlibCompressor(u8);

impl RawZlibCompressor {
    /// level 0-10
    pub fn new(level: u8) -> Self {
        assert!(level <= 10);
        Self(level)
    }
}

impl Compressor for RawZlibCompressor {
    fn encode(&self, block: Vec<u8>) -> rusty_leveldb::Result<Vec<u8>> {
        Ok(compress_to_vec(&block, self.0))
    }

    fn decode(&self, block: Vec<u8>) -> rusty_leveldb::Result<Vec<u8>> {
        decompress_to_vec(&block).map_err(|e| rusty_leveldb::Status {
            code: rusty_leveldb::StatusCode::CompressionError,
            err: e.to_string(),
        })
    }
}

pub fn mcpe_options(compression_level: u8) -> Options {
    let mut opt = Options::default();

    // compressor id list can be find in [mojang's implementation](https://github.com/reedacartwright/rbedrock/blob/fb32a899da4e15c1aaa0d6de2b459e914e183516/src/leveldb-mcpe/include/leveldb/c.h#L194-L200)
    // And it seems like mojang don't use slippy or zstd of minecraft bedrock
    //
    // If you need to open old world, old world fallback compressor can be find [here](https://github.com/reedacartwright/rbedrock/blob/fb32a899da4e15c1aaa0d6de2b459e914e183516/src/bedrock_leveldb.c#L152-L153)
    let mut list = CompressorList::new();
    list.set_with_id(0, NoneCompressor::default());
    list.set_with_id(2, ZlibCompressor::new(compression_level));
    list.set_with_id(4, RawZlibCompressor::new(compression_level));
    opt.compressor_list = Rc::new(list);

    // Set compressor
    // Minecraft bedrock may use other id than 4
    //
    // There is a bug in this library that you have to open a database with the same compression type as it was written to.
    opt.compressor = 4;

    opt
}

#[main]
async fn main() {
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

