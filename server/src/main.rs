use std::sync::Arc;

use bytes::BytesMut;
use fulytic_logic::core::{Codec, PlayerInfo};
use tokio::io::AsyncReadExt;

pub mod client;
pub mod server;

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    println!("Starting up...");

    pretty_env_logger::init();

    tokio::spawn(async {
        let shutdown_signal = tokio::signal::ctrl_c();
        tokio::select! {
            _ = shutdown_signal => shutdown(),
        }
    });

    let default_panic = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        default_panic(info);
        shutdown();
    }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:30010")
        .await
        .expect("Failed to start TcpListener");

    let server = server::Server::new();

    loop {
        let (mut connection, address) = listener.accept().await?;

        if let Err(e) = connection.set_nodelay(true) {
            log::warn!("failed to set TCP_NODELAY {e}");
        }

        log::info!("Accepted connection from {address}");

        let server = server.clone();
        tokio::spawn(async move {
            let mut buf = BytesMut::new();

            let read = connection.read_buf(&mut buf).await;
            let client = match read {
                Ok(count) => {
                    if count == 0 {
                        log::debug!("reading 0 bytes");
                        return;
                    }

                    log::debug!("read {count} bytes");

                    let Ok((player, _)) = PlayerInfo::decode(buf.split().as_mut()) else {
                        log::error!("Failed to decode player info");
                        return;
                    };

                    log::info!("Player {player:#?} connected");

                    let client = client::Client::new(player, connection, address);
                    server.add_client(client.clone()).await;
                    client
                }
                Err(error) => {
                    log::error!("Error while reading incoming packet {}", error);
                    return;
                }
            };

            while !client.is_closed() {
                let open = client.poll_connection().await;
                if open {
                    client.process_packets(&server).await;
                };
            }

            server.remove_client(client.player_info.id).await;
        });
    }
}

fn shutdown() {
    println!("Shutting down...");
    std::process::exit(0);
}
