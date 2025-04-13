mod models; 
use models::message::Message; // Importa la struttura Message

use std::io::{self, BufRead}; // Usa std per gestire input sincrono
use tokio::net::TcpStream;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use serde_json::to_string;

#[tokio::main]
async fn main() {
    // Connettiti al server
    let stream = TcpStream::connect("127.0.0.1:8080").await.unwrap();

    // Divide il flusso in lettore e scrittore per lavorare con entrambi.
    let (reader, mut writer) = stream.into_split();

    // Usa un buffer per leggere dati dal flusso.
    let mut buf_reader = BufReader::new(reader);

    println!("Connesso al server! Digita un messaggio:");

    // Stringa per raccogliere l'input dell'utente.
    let mut input = String::new();

    // Legge una riga dalla console.
    while io::stdin().read_line(&mut input).unwrap() > 0 {
        // Crea un messaggio da inviare
        let message = Message {
            sender: "Client".to_string(),
            content: input.trim().to_string(),
        };

        // Serializza l'oggetto 'Message' in JSON.
        let serialized = to_string(&message).unwrap();

        // Invia il messaggio al server e aggiunge una nuova linea per separare i messaggi.
        writer.write_all(serialized.as_bytes()).await.unwrap();
        writer.write_all(b"\n").await.unwrap();

        // Aspetta la risposta del server.
        let mut response = String::new();

        // Legge la risposta dal server.
        buf_reader.read_line(&mut response).await.unwrap();
        println!("Risposta dal server: {}", response);

        input.clear();
    }
}
