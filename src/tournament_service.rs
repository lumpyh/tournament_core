use crate::tournament::{
    tournament_server, AddDayRequest, AddDayResponse, ChangeNameRequest, ChangeNameResponse,
    GetDayDataRequest, GetDayDataResponse, GetSimpleDaysRequest, GetSimpleDaysResponse,
    LoadRequest, LoadResponse, RemoveDayRequest, RemoveDayResponse, SaveRequest, SaveResponse,
};

use crate::tournament_core::Tournament;

use std::path::Path;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug)]
pub struct TournamentService {
    tournament: Arc<Mutex<Option<Tournament>>>,
}

impl Default for TournamentService {
    fn default() -> Self {
        TournamentService {
            tournament: Arc::new(Mutex::new(Some(Tournament::new()))),
        }
    }
}

#[tonic::async_trait]
impl tournament_server::Tournament for TournamentService {
    async fn change_name(
        &self,
        request: tonic::Request<ChangeNameRequest>,
    ) -> std::result::Result<tonic::Response<ChangeNameResponse>, tonic::Status> {
        let mut tourn_mut = self.tournament.lock().await;

        let Some(ref mut tournament) = *tourn_mut else {
            return Ok(tonic::Response::new(ChangeNameResponse {
                success: false,
                error: "not loaded jet".to_string(),
            }));
        };

        let change_name_request = request.into_inner();
        tournament.name = change_name_request.name;

        Ok(tonic::Response::new(ChangeNameResponse {
            success: true,
            error: "".to_string(),
        }))
    }

    async fn load(
        &self,
        request: tonic::Request<LoadRequest>,
    ) -> std::result::Result<tonic::Response<LoadResponse>, tonic::Status> {
        let mut tourn_mut = self.tournament.lock().await;

        let path_str = request.into_inner().path;
        let path = Path::new(&path_str);
        let res = Tournament::from_json_file(path);

        let response = match res {
            Ok(tourn) => {
                *tourn_mut = Some(tourn);
                LoadResponse {
                    success: true,
                    error: "".to_string(),
                }
            }
            Err(err) => LoadResponse {
                success: false,
                error: format!("{:?}", err),
            },
        };

        Ok(tonic::Response::new(response))
    }

    async fn save(
        &self,
        request: tonic::Request<SaveRequest>,
    ) -> std::result::Result<tonic::Response<SaveResponse>, tonic::Status> {
        let mut tourn_mut = self.tournament.lock().await;

        let Some(ref mut tournament) = *tourn_mut else {
            return Ok(tonic::Response::new(SaveResponse {
                success: false,
                error: "not loaded jet".to_string(),
            }));
        };

        let path_str = request.into_inner().path;
        let path = Path::new(&path_str);
        let res = tournament.to_json_file(path);

        let response = match res {
            Ok(_) => SaveResponse {
                success: true,
                error: "".to_string(),
            },
            Err(err) => SaveResponse {
                success: false,
                error: format!("{:?}", err),
            },
        };

        Ok(tonic::Response::new(response))
    }

    async fn add_day(
        &self,
        request: tonic::Request<AddDayRequest>,
    ) -> std::result::Result<tonic::Response<AddDayResponse>, tonic::Status> {
        let mut tourn_mut = self.tournament.lock().await;
        let Some(ref mut tournament) = *tourn_mut else {
            return Err(tonic::Status::new(
                tonic::Code::Internal,
                "not loaded jet".to_string(),
            ));
        };

        let Some(day) = request.into_inner().day else {
            return Err(tonic::Status::new(
                tonic::Code::InvalidArgument,
                "day is not set".to_string(),
            ));
        };
        tournament.add_day(day.into());

        Ok(tonic::Response::new(AddDayResponse {}))
    }

    async fn remove_day(
        &self,
        request: tonic::Request<RemoveDayRequest>,
    ) -> std::result::Result<tonic::Response<RemoveDayResponse>, tonic::Status> {
        let mut tourn_mut = self.tournament.lock().await;
        let Some(ref mut tournament) = *tourn_mut else {
            return Err(tonic::Status::new(
                tonic::Code::Internal,
                "not loaded jet".to_string(),
            ));
        };

        let id = request.into_inner().id;
        tournament.remove_day(id);
        Ok(tonic::Response::new(RemoveDayResponse {}))
    }

    async fn get_simple_days(
        &self,
        request: tonic::Request<GetSimpleDaysRequest>,
    ) -> std::result::Result<tonic::Response<GetSimpleDaysResponse>, tonic::Status> {
        let mut tourn_mut = self.tournament.lock().await;
        let Some(ref mut tournament) = *tourn_mut else {
            return Err(tonic::Status::new(
                tonic::Code::Internal,
                "not loaded jet".to_string(),
            ));
        };

        let days = tournament.get_simple_days();

        Ok(tonic::Response::new(GetSimpleDaysResponse { days }))
    }

    async fn get_day_data(
        &self,
        request: tonic::Request<GetDayDataRequest>,
    ) -> std::result::Result<tonic::Response<GetDayDataResponse>, tonic::Status> {
        let mut tourn_mut = self.tournament.lock().await;
        let Some(ref mut tournament) = *tourn_mut else {
            return Err(tonic::Status::new(
                tonic::Code::Internal,
                "not loaded jet".to_string(),
            ));
        };

        let id = request.into_inner().id;
        let day = tournament.get_day_data(id)?;

        Ok(tonic::Response::new(GetDayDataResponse { day: Some(day) }))
    }
}
