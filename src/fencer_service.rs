use crate::tournament::fencer_server;
use crate::tournament::{
    GetAllFencersRequest, GetAllFencersResponse, UpdateFencersRequest, UpdateFencersResponse,
};
use crate::tournament_core::Tournament;

use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug)]
pub struct FencerService {
    tournament: Arc<Mutex<Option<Tournament>>>,
}

impl FencerService {
    pub fn new(tournament: Arc<Mutex<Option<Tournament>>>) -> Self {
        Self { tournament }
    }
}

fn not_loaded() -> tonic::Status {
    tonic::Status::new(tonic::Code::Internal, "not loaded jet".to_string())
}

#[tonic::async_trait]
impl fencer_server::Fencer for FencerService {
    async fn get_all_fencers(
        &self,
        _request: tonic::Request<GetAllFencersRequest>,
    ) -> std::result::Result<tonic::Response<GetAllFencersResponse>, tonic::Status> {
        let Some(ref mut tournament) = *self.tournament.lock().await else {
            return Err(not_loaded());
        };

        let fencers = tournament.get_all_fencers()?;
        Ok(tonic::Response::new(GetAllFencersResponse { fencers }))
    }

    async fn update_fencers(
        &self,
        request: tonic::Request<UpdateFencersRequest>,
    ) -> std::result::Result<tonic::Response<UpdateFencersResponse>, tonic::Status> {
        let Some(ref mut tournament) = *self.tournament.lock().await else {
            return Err(not_loaded());
        };

        tournament.update_fencers(request.into_inner().fencers);

        Ok(tonic::Response::new(UpdateFencersResponse {}))
    }
}
