use std::io::{Write, Read};
use std::net::TcpListener;
use std::env;
use std::collections::HashMap;
use std::str;

use serde_json;

fn handle_request(mut stream: &std::net::TcpStream) {
    // Créer un HashMap pour stocker les headers de la requête
    let mut headers_map: HashMap<String, String> = HashMap::new();

    // Lire la requête depuis le stream
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let request = str::from_utf8(&buffer).unwrap();

    // Extraire les lignes de la requête
    let lines: Vec<&str> = request.lines().collect();

    // La première ligne contient la méthode, le chemin, et la version HTTP
    let request_line_parts: Vec<&str> = lines[0].split_whitespace().collect();
    if request_line_parts.len() >= 2 && request_line_parts[0] == "GET" && request_line_parts[1] == "/ping" {
        // Extraire les headers de la requête
        for line in &lines[1..] {
            if line.is_empty() {
                break;
            }
            let header_parts: Vec<&str> = line.split(": ").collect();
            if header_parts.len() == 2 {
                headers_map.insert(header_parts[0].to_string(), header_parts[1].to_string());
            }
        }
    } else {
        // Renvoyer une réponse HTTP avec le code d'état 404
        let response = format!("HTTP/1.1 404 Not Found\r\nContent-Type: text/plain\r\n\r\n404 Not Found");
        stream.write_all(response.as_bytes()).unwrap();
    }

    // Convertir les headers en JSON
    let headers_json = serde_json::to_string(&headers_map).unwrap();

    // Construire la réponse HTTP avec les headers au format JSON
    let response = format!("HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{}", headers_json);

    // Envoyer la réponse
    if let Err(err) = stream.write_all(response.as_bytes()) {
        eprintln!("Failed to write to stream: {}", err);
    }
}

fn main() {
    let listen_port: u16 = env::var("PING_LISTEN_PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("Invalid port number");

    let listener = TcpListener::bind(("127.0.0.1", listen_port))
        .expect("Failed to bind to port");

    println!("Server listening on port {}", listen_port);

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                handle_request(&mut stream);
            }
            Err(err) => {
                eprintln!("Error accepting connection: {}", err);
            }
        }
    }
}
