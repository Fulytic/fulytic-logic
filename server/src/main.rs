use std::{net::SocketAddr, sync::Arc};

use bytes::BytesMut;
use fulytic_logic::core::{Codec, PlayerInfo};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    sync::Mutex,
};

pub mod client;
pub mod server;

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    pretty_env_logger::init();

    log::debug!("Server starting up with debug log level");

    log::info!("Starting up..");

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

    let addr = SocketAddr::from(([0, 0, 0, 0], 30010));

    #[allow(clippy::expect_used)]
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to start TcpListener");

    let server = server::Server::new();

    log::info!("Server started on {:#?}", addr);

    loop {
        let (connection, address) = listener.accept().await?;

        if let Err(e) = connection.set_nodelay(true) {
            log::warn!("failed to set TCP_NODELAY {e}");
        }

        let (connection_reader, connection_writer) = connection.into_split();
        let connection_reader = Arc::new(Mutex::new(connection_reader));
        let connection_writer = Arc::new(Mutex::new(connection_writer));

        log::info!("Accepted connection from {address}");

        let server = server.clone();
        tokio::spawn(async move {
            let client = {
                let mut buf = BytesMut::with_capacity(256);
                let read = connection_reader.lock().await.read_buf(&mut buf).await;
                match read {
                    Ok(count) => {
                        if count == 0 {
                            log::debug!("reading 0 bytes");
                            return;
                        }

                        let Ok((player, _)) = PlayerInfo::decode(buf.split().as_mut()) else {
                            log::error!("Failed to decode player info");
                            return;
                        };

                        let (tx, mut rx) = tokio::sync::mpsc::channel(5);

                        let client = client::Client::new(tx, player, address);

                        server.add_client(client.clone()).await;

                        let cloned = client.clone();
                        tokio::spawn(async move {
                            while rx.recv().await.is_some() {
                                let mut s2c = cloned.s2c.lock().await.split();
                                if let Err(e) =
                                    connection_writer.lock().await.write_buf(&mut s2c).await
                                {
                                    log::warn!("Failed to s2c: {e}");
                                    cloned.close();
                                    break;
                                }
                            }
                        });

                        client
                    }
                    Err(error) => {
                        log::error!("Error while reading incoming packet {}", error);
                        return;
                    }
                }
            };

            log::info!("Client connected: {:#?}", client.player_info);

            while !client.is_closed() {
                client
                    .poll_connection(&server, connection_reader.clone())
                    .await;
            }

            log::info!("Client disconnected: {:#?}", client.player_info);

            server.remove_client(client.player_info.id).await;
        });
    }
}

fn shutdown() {
    log::info!("Shutting down...");
    std::process::exit(0);
}
