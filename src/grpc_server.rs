use tonic::{transport::Server, Request, Response, Status};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;

pub mod services {
    tonic::include_proto!("services");
}

use services::{
    payment_service_server::{PaymentService, PaymentServiceServer},
    PaymentRequest, PaymentResponse,
    transaction_service_server::{TransactionService, TransactionServiceServer},
    TransactionRequest, TransactionResponse,
    chat_service_server::{ChatService, ChatServiceServer},
    ChatMessage,
};

#[derive(Default)]
pub struct MyPaymentService {}

#[tonic::async_trait]
impl PaymentService for MyPaymentService {
    async fn process_payment(
        &self,
        request: Request<PaymentRequest>,
    ) -> Result<Response<PaymentResponse>, Status> {
        println!("Menerima request pembayaran: {:?}", request.into_inner());
        Ok(Response::new(PaymentResponse { success: true }))
    }
}

#[derive(Default)]
pub struct MyTransactionService {}

#[tonic::async_trait]
impl TransactionService for MyTransactionService {
    type GetTransactionHistoryStream = ReceiverStream<Result<TransactionResponse, Status>>;

    async fn get_transaction_history(
        &self,
        request: Request<TransactionRequest>,
    ) -> Result<Response<Self::GetTransactionHistoryStream>, Status> {
        println!("Menerima request riwayat transaksi untuk user: {:?}", request.into_inner().user_id);

        let (tx, rx) = mpsc::channel(4);

        tokio::spawn(async move {
            for i in 0..10 { // Simulasi mengirim 10 data
                let response = TransactionResponse {
                    transaction_id: format!("trans_{}", i),
                    status: "Completed".to_string(),
                    amount: 150.0,
                    timestamp: "2024-05-01T10:00:00Z".to_string(),
                };

                if tx.send(Ok(response)).await.is_err() {
                    break;
                }
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}

#[derive(Default)]
pub struct MyChatService {}

#[tonic::async_trait]
impl ChatService for MyChatService {
    type ChatStream = ReceiverStream<Result<ChatMessage, Status>>;

    async fn chat(
        &self,
        request: Request<tonic::Streaming<ChatMessage>>,
    ) -> Result<Response<Self::ChatStream>, Status> {
        println!("Menerima koneksi chat baru...");

        let mut stream = request.into_inner();
        let (tx, rx) = mpsc::channel(4);

        tokio::spawn(async move {
            while let Some(msg) = stream.message().await.unwrap_or(None) {
                println!("Client bilang: {}", msg.message);

                let reply = ChatMessage {
                    user_id: "server_bot".to_string(),
                    message: format!("Halo {}, pesan '{}' kamu sudah diterima!", msg.user_id, msg.message),
                };

                if tx.send(Ok(reply)).await.is_err() {
                    break;
                }
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;

    let payment_service = MyPaymentService::default();
    let transaction_service = MyTransactionService::default();
    let chat_service = MyChatService::default();

    println!("gRPC Server berjalan di {}", addr);

    Server::builder()
        .add_service(PaymentServiceServer::new(payment_service))
        .add_service(TransactionServiceServer::new(transaction_service))
        .add_service(ChatServiceServer::new(chat_service))
        .serve(addr)
        .await?;

    Ok(())
}