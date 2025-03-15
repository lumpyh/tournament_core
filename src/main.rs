use tournament_core::tournament::tournament_server::TournamentServer;
use tournament_core::tournament_service::TournamentService;

use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:50051".parse()?;
    let tourn = TournamentService::default();

    Server::builder()
        .add_service(TournamentServer::new(tourn))
        .serve(addr)
        .await?;

    Ok(())
}
