
mod client;
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::{thread, time::Duration};

use jwt::{token, Header, SignWithKey, Token};
use websocket::{header::Protocol, sync::Server, ClientBuilder, OwnedMessage};
#[derive(Default, Deserialize, Serialize)]
struct Custom {
    sub: String,
    rhino: bool,
}

fn new_token(user_id: &str, password: &str) -> Result<String, &'static str> {
    // Dummy auth
    if password != "password" {
        return Err("Wrong password");
    }

    let header: Header = Default::default();
    let claims = Custom {
        sub: user_id.into(),
        rhino: true,
    };
    let unsigned_token = Token::new(header, claims);

    let key: Hmac<Sha256> = Hmac::new_from_slice(b"secret_key").map_err(|_e| "Invalid key")?;

    let signed_token = unsigned_token
        .sign_with_key(&key)
        .map_err(|_e| "Sign error")?;
    Ok(signed_token.into())
}
fn websocket_server() {
	let server = Server::bind("127.0.0.1:2794").unwrap();

	for request in server.filter_map(Result::ok) {
		// Spawn a new thread for each connection.
		thread::spawn(|| {
			if !request.protocols().contains(&"rust-websocket".to_string()) {
				request.reject().unwrap();
				return;
			}

			let mut client = request.use_protocol("rust-websocket").accept().unwrap();

			let ip = client.peer_addr().unwrap();

			println!("Connection from {}", ip);

			let message = OwnedMessage::Text("Hello".to_string());
			client.send_message(&message).unwrap();

			let (mut receiver, mut sender) = client.split().unwrap();

			for message in receiver.incoming_messages() {
				let message = message.unwrap();

				match message {
					OwnedMessage::Close(_) => {
						let message = OwnedMessage::Close(None);
						sender.send_message(&message).unwrap();
						println!("Client {} disconnected", ip);
						return;
					}
					OwnedMessage::Ping(ping) => {
						let message = OwnedMessage::Pong(ping);
						sender.send_message(&message).unwrap();
					}
					_ => sender.send_message(&message).unwrap(),
				}
			}
		});
	}
}



fn communicate_with_websocket(ip: &str, message: &str) {
    print!("sdasda");
        let server_addr = format!("ws://{}", ip);
        let client = ClientBuilder::new(&server_addr)
            .unwrap()
            .add_protocol("rust-websocket")
            .connect_insecure()
            .unwrap();
    
        let (mut receiver, mut sender) = client.split().unwrap();
    
        // Send the initial message
        let msg = OwnedMessage::Text(message.to_string());
        sender.send_message(&msg).unwrap();
    
        // Spawn a thread to handle incoming messages
        thread::spawn(move || {
            for message in receiver.incoming_messages() {
                let message = message.unwrap();
    
                match message {
                    OwnedMessage::Close(_) => {
                        let message = OwnedMessage::Close(None);
                        sender.send_message(&message).unwrap();
                        println!("Server disconnected");
                        return;
                    }
                    OwnedMessage::Ping(ping) => {
                        let message = OwnedMessage::Pong(ping);
                        sender.send_message(&message).unwrap();
                    }
                    _ => println!("Received: {:?}", message),
                }
            }
        });
    
        // Keep the main thread alive to continue receiving messages
        loop {
            thread::sleep(Duration::from_secs(1));
        }
    }


    

fn main() {
    thread::spawn(|| {
        websocket_server();
    });
    thread::sleep(Duration::from_secs(1));
    //communicate_with_websocket("127.0.0.1:2794", "Hello");
    let token = new_token("user", "password").unwrap();
    print!("{}", token);
}

