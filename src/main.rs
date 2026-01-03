use dashmap::DashMap;
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::thread;
use tracing::{debug, error, info, instrument};

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_thread_ids(true)
        .init();

    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();

    info!("Server listening on 127.0.0.1:8000");

    let shared_map: Arc<DashMap<Vec<u8>, Vec<u8>>> = Arc::new(DashMap::new());

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let db_clone = shared_map.clone();
                let peer_addr = stream.peer_addr().ok();

                thread::spawn(move || {
                    handle_client(stream, db_clone, peer_addr);
                });
            }
            Err(e) => error!("Connection failed: {}", e),
        }
    }
}

#[instrument(skip(stream, shared_map), fields(client_ip = ?peer_addr))]
fn handle_client(
    mut stream: TcpStream,
    shared_map: Arc<DashMap<Vec<u8>, Vec<u8>>>,
    peer_addr: Option<std::net::SocketAddr>,
) {
    loop {
        let mut opcode = [0; 1];

        match stream.read(&mut opcode) {
            Ok(0) => {
                info!("Client disconnected");
                break;
            }
            Ok(_) => match opcode[0] {
                1 => {
                    if let Err(e) = set_key_value(&mut stream, &shared_map) {
                        error!("Error processing set request: {}", e);
                        break;
                    }
                }
                2 => {
                    if let Err(e) = get_value(&mut stream, &shared_map) {
                        error!("Error processing get request: {}", e);
                        break;
                    }
                }
                3 => {
                    if let Err(e) = delete_key_value(&mut stream, &shared_map) {
                        error!("Error processing delete request: {}", e);
                        break;
                    }
                }
                _ => {
                    error!("Unknown opcode: {}", opcode[0]);
                    break;
                }
            },
            Err(e) => {
                error!("Stream error: {}", e);
                break;
            }
        }
    }
}

#[instrument(skip(stream, shared_map))]
fn set_key_value(stream: &mut TcpStream, shared_map: &DashMap<Vec<u8>, Vec<u8>>) -> io::Result<()> {
    let mut request_key_value_length: [u8; 4] = [0, 0, 0, 0];
    stream.read_exact(&mut request_key_value_length)?;

    let request_key_length =
        u16::from_be_bytes([request_key_value_length[0], request_key_value_length[1]]);
    let request_value_length =
        u16::from_be_bytes([request_key_value_length[2], request_key_value_length[3]]);

    let request_body_length: usize =
        (request_key_length as usize) + (request_value_length as usize);

    let mut request_body: Vec<u8> = vec![0; request_body_length];
    stream.read_exact(&mut request_body)?;

    let request_value: Vec<u8> = request_body.split_off(request_key_length as usize);
    let request_key: Vec<u8> = request_body;

    debug!(?shared_map, "Shared map before insert");

    info!(key = ?request_key, value_len = request_value.len(), "Setting Key-Value");

    shared_map.insert(request_key, request_value);

    debug!(?shared_map, "Shared map after insert");

    let response: Vec<u8> = vec![1];
    stream.write_all(&response)?;
    stream.flush()?;

    Ok(())
}

#[instrument(skip(stream, shared_map))]
fn get_value(stream: &mut TcpStream, shared_map: &DashMap<Vec<u8>, Vec<u8>>) -> io::Result<()> {
    let mut request_key_length: [u8; 2] = [0, 0];
    stream.read_exact(&mut request_key_length)?;

    let request_key_length = u16::from_be_bytes([request_key_length[0], request_key_length[1]]);

    let mut request_key: Vec<u8> = vec![0; request_key_length as usize];
    stream.read_exact(&mut request_key)?;

    info!(key = ?request_key, "Getting Value");

    let _response_value = shared_map.get(&request_key).unwrap();

    let response: Vec<u8> = vec![1];
    stream.write_all(&response)?;
    stream.flush()?;

    Ok(())
}

#[instrument(skip(stream, shared_map))]
fn delete_key_value(
    stream: &mut TcpStream,
    shared_map: &DashMap<Vec<u8>, Vec<u8>>,
) -> io::Result<()> {
    let mut request_key_length: [u8; 2] = [0, 0];
    stream.read_exact(&mut request_key_length)?;

    let request_key_length = u16::from_be_bytes([request_key_length[0], request_key_length[1]]);

    let mut request_key: Vec<u8> = vec![0; request_key_length as usize];
    stream.read_exact(&mut request_key)?;

    info!(key = ?request_key, "Deleting Key");

    shared_map.remove(&request_key).unwrap();

    let response: Vec<u8> = vec![1];
    stream.write_all(&response)?;
    stream.flush()?;

    Ok(())
}
