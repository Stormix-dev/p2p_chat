# Chat P2P in Rust
## Descrizione
Questo progetto è una semplice implementazione di una chat Peer-to-Peer (P2P) in Rust, che permette la comunicazione tra un server e uno o più client. Il server riceve messaggi dai client, li deserializza, li processa e risponde con un messaggio di conferma. I messaggi sono formattati in JSON per consentire una struttura chiara e organizzata.

## Funzionalità
- **Comunicazione P2P**: Un server che accetta connessioni da più client.
- **Messaggi strutturati in JSON**: Utilizzo della libreria Serde per serializzare e deserializzare i dati.
- **Gestione asincrona**: Sfrutta il runtime di Tokio per garantire prestazioni elevate e comunicazione non bloccante.

## Tecnologie utilizzate
- **Rust**: Il linguaggio alla base del progetto, noto per la sicurezza e le prestazioni.
- **Tokio**: Libreria asincrona per la gestione di networking e concorrenza.
- **Serde**: Libreria per la serializzazione e la deserializzazione di dati in formato JSON.

## Requisiti
Per eseguire il progetto, hai bisogno di:
- Rust installato sul tuo sistema.
- Una caonnessione TCP locale aperta sulla porta 8080.

## Installazione
- Clona la repository: `git clone https://github.com/Stormix-dev/p2p_chat.git`
- Accedi alla directory del progetto: `cd p2p_chat`
- Compila il progetto: `cargo build`

## Esecuzione
### Avvia il server
- Esegui il server (dal file main.rs): `cargo run --bin server`
- Il server si metterà in ascolto sulla porta 8080 e nel terminale vedrai: `In ascolto su 127.0.0.1:8080`

### Avvia il Client
- Apri un altro terminale e avvia il client (dal file client.rs): `cargo run --bin client`
- Apparirà un messaggio: `Connesso al server! Digita un messaggio:`
- Digita un messaggio e premi `Invio`. Il server riceverà il messaggio e invierà una risposta.
