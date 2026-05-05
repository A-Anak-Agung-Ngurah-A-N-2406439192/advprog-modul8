use std::io;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;

pub mod services {
    tonic::include_proto!("services");
}

use services::{
    payment_service_client::PaymentServiceClient, PaymentRequest,
    transaction_service_client::TransactionServiceClient, TransactionRequest,
    chat_service_client::ChatServiceClient, ChatMessage,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- Memulai Payment Service ---");
    let mut payment_client = PaymentServiceClient::connect("http://[::1]:50051").await?;

    let payment_request = tonic::Request::new(PaymentRequest {
        user_id: "user_123".to_string(),
        amount: 100.0,
    });

    let payment_response = payment_client.process_payment(payment_request).await?;
    println!("RESPONSE = {:?}\n", payment_response.into_inner());

    println!("--- Memulai Transaction Service ---");
    let mut transaction_client = TransactionServiceClient::connect("http://[::1]:50051").await?;

    let transaction_request = tonic::Request::new(TransactionRequest {
        user_id: "user_123".to_string(),
    });

    let mut transaction_stream = transaction_client.get_transaction_history(transaction_request).await?.into_inner();

    while let Some(transaction) = transaction_stream.message().await? {
        println!("Menerima riwayat transaksi: {:?}", transaction);
    }
    println!("");

    println!("--- Memulai Chat Service ---");
    let mut chat_client = ChatServiceClient::connect("http://[::1]:50051").await?;
    println!("Ketik pesan Anda dan tekan Enter. (Ketik 'quit' untuk keluar)");

    let (tx, rx) = mpsc::channel(4);

    tokio::spawn(async move {
        let stdin = io::stdin();
        loop {
            let mut input = String::new();
            stdin.read_line(&mut input).expect("Gagal membaca input terminal");

            let trimmed = input.trim();
            if trimmed.eq_ignore_ascii_case("quit") {
                break;
            }

            let msg = ChatMessage {
                user_id: "client_1".to_string(),
                message: trimmed.to_string(),
            };

            if tx.send(msg).await.is_err() {
                break;
            }
        }
    });

    let outbound_stream = ReceiverStream::new(rx);
    let request = tonic::Request::new(outbound_stream);

    let mut response_stream = chat_client.chat(request).await?.into_inner();

    while let Some(response) = response_stream.message().await? {
        println!("Bot Server membalas: {}", response.message);
    }

    Ok(())
}