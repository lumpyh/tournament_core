use tournament_core::tournament::tournament_server::TournamentServer;
use tournament_core::tournament::fencer_server::FencerServer;
use tournament_core::tournament_service::TournamentService;
use tournament_core::fencer_service::FencerService;
use tournament_core::tournament_core::Tournament;

use std::sync::Arc;
use tonic::transport::Server;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:50051".parse()?;
    let tournament = Arc::new(Mutex::new(Some(Tournament::new())));
    let tourn = TournamentService::new(tournament.clone());
    let fenc = FencerService::new(tournament);

    Server::builder()
        .add_service(TournamentServer::new(tourn))
        .add_service(FencerServer::new(fenc))
        .serve(addr)
        .await?;

    Ok(())
}
