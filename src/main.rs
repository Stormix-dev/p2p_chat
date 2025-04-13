mod models;
use models::message::Message;

use serde_json::{from_str, to_string}; // funzioni per serializzare e deserializzare JSON.
use tokio::net::{TcpListener, TcpStream}; // TcpListener per accettare connessioni e TcpStream per comunicare con i client.
use tokio::io::{AsyncWriteExt, AsyncBufReadExt, BufReader}; // funzioni asincrone per leggere e scrivere sui flussi.

// Funzione per gestire una connessione client
async fn handle_client(stream: TcpStream) {
    let (reader, mut writer) = stream.into_split(); // Divide il flusso in lettore e scrittore per lavorare con entrambi.
    let mut buf_reader = BufReader::new(reader); // Usa un buffer per leggere dati dal lettore.
    let mut line = String::new(); // Stringa per memorizzare le righe di input.

    while buf_reader.read_line(&mut line).await.unwrap() > 0 { // Legge i dati inviati dal client.

        // Prova a deserializzare il messaggio ricevuto
        if let Ok(message) = from_str::<Message>(&line.trim()) { 

            // Stampa il messaggio ricevuto.
            println!("Messaggio ricevuto da {}: {}", message.sender, message.content);

            // Crea una risposta da inviare al client.
            let response = Message {
                sender: "Server".to_string(),
                content: "Messaggio ricevuto con successo!".to_string(),
            };

            // Serializza la struttura 'Message' in JSON.
            let serialized = to_string(&response).unwrap();

            // Invia il messaggio serializzato al client.
            writer.write_all(serialized.as_bytes()).await.unwrap();

             // Aggiunge una nuova linea per separare i messaggi.
            writer.write_all(b"\n").await.unwrap(); 
        } else {
            // Messaggio non valido
            println!("Errore nella deserializzazione del messaggio.");
        }
        // Pulisce la stringa per leggere la prossima riga.
        line.clear();
    }
}

// Macro per abilitare il runtime asincrono di Tokio.
#[tokio::main]
async fn main() {
    // Crea un listener che ascolta sulla porta 8080.
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("In ascolto su 127.0.0.1:8080");

    // Accetta nuove connessioni in arrivo.
    while let Ok((stream, addr)) = listener.accept().await {
        println!("Connessione da {}", addr);

        // Crea un nuovo task per gestire la connessione senza bloccare il server.
        tokio::spawn(handle_client(stream));
    }
}
