use crate::tournament::{
    tournament_server, AddBewerbRequest, AddBewerbResponse, AddDayRequest, AddDayResponse,
    AddGroupToArenaRequest, AddGroupToArenaResponse, ChangeNameRequest, ChangeNameResponse,
    FreeUpGroupRequest, FreeUpGroupResponse, GetAllFreeGroupsRequest, GetAllFreeGroupsResponse,
    GetDayDataRequest, GetDayDataResponse, GetSimpleBewerbsRequest, GetSimpleBewerbsResponse,
    GetSimpleDaysRequest, GetSimpleDaysResponse, LoadRequest, LoadResponse, RemoveBewerbRequest,
    RemoveBewerbResponse, RemoveDayRequest, RemoveDayResponse, SaveRequest, SaveResponse,
};

use crate::tournament_core::Tournament;

use std::path::Path;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug)]
pub struct TournamentService {
    tournament: Arc<Mutex<Option<Tournament>>>,
}

impl TournamentService {
    pub fn new(tournament: Arc<Mutex<Option<Tournament>>>) -> Self {
        Self { tournament }
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
        tournament.inner.name = change_name_request.name;

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
        tournament.add_day(day);

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
        _request: tonic::Request<GetSimpleDaysRequest>,
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

    async fn add_bewerb(
        &self,
        request: tonic::Request<AddBewerbRequest>,
    ) -> std::result::Result<tonic::Response<AddBewerbResponse>, tonic::Status> {
        let mut tourn_mut = self.tournament.lock().await;
        let Some(ref mut tournament) = *tourn_mut else {
            return Err(tonic::Status::new(
                tonic::Code::Internal,
                "not loaded jet".to_string(),
            ));
        };

        let req = request.into_inner();
        tournament.add_bewerb(req.name, req.n_rounds, req.n_groups);

        Ok(tonic::Response::new(AddBewerbResponse {}))
    }

    async fn remove_bewerb(
        &self,
        request: tonic::Request<RemoveBewerbRequest>,
    ) -> std::result::Result<tonic::Response<RemoveBewerbResponse>, tonic::Status> {
        let mut tourn_mut = self.tournament.lock().await;
        let Some(ref mut tournament) = *tourn_mut else {
            return Err(tonic::Status::new(
                tonic::Code::Internal,
                "not loaded jet".to_string(),
            ));
        };

        let req = request.into_inner();
        tournament.remove_bewerb(req.bewerb_id);

        Ok(tonic::Response::new(RemoveBewerbResponse {}))
    }

    async fn get_simple_bewerbs(
        &self,
        _request: tonic::Request<GetSimpleBewerbsRequest>,
    ) -> std::result::Result<tonic::Response<GetSimpleBewerbsResponse>, tonic::Status> {
        let mut tourn_mut = self.tournament.lock().await;
        let Some(ref mut tournament) = *tourn_mut else {
            return Err(tonic::Status::new(
                tonic::Code::Internal,
                "not loaded jet".to_string(),
            ));
        };

        let data = tournament
            .get_bewerbs()
            .iter()
            .map(|x| (*x).into())
            .collect();

        Ok(tonic::Response::new(GetSimpleBewerbsResponse { data }))
    }

    async fn get_all_free_groups(
        &self,
        _request: tonic::Request<GetAllFreeGroupsRequest>,
    ) -> std::result::Result<tonic::Response<GetAllFreeGroupsResponse>, tonic::Status> {
        let mut tourn_mut = self.tournament.lock().await;
        let Some(ref mut tournament) = *tourn_mut else {
            return Err(tonic::Status::new(
                tonic::Code::Internal,
                "not loaded jet".to_string(),
            ));
        };

        let groups = tournament
            .get_all_free_groups()
            .iter()
            .map(|x| x.into())
            .collect();

        Ok(tonic::Response::new(GetAllFreeGroupsResponse { groups }))
    }

    async fn add_group_to_arena(
        &self,
        request: tonic::Request<AddGroupToArenaRequest>,
    ) -> std::result::Result<tonic::Response<AddGroupToArenaResponse>, tonic::Status> {
        let mut tourn_mut = self.tournament.lock().await;
        let Some(ref mut tournament) = *tourn_mut else {
            return Err(tonic::Status::new(
                tonic::Code::Internal,
                "not loaded jet".to_string(),
            ));
        };

        let req = request.into_inner();
        let Some(group_id) = req.group_id else {
            return Err(tonic::Status::new(
                tonic::Code::InvalidArgument,
                "group_id not set".to_string(),
            ));
        };

        let Some(arena_id) = req.arena_id else {
            return Err(tonic::Status::new(
                tonic::Code::InvalidArgument,
                "arena_id not set".to_string(),
            ));
        };

        tournament.add_group_to_arena(&group_id.into(), &arena_id.into())?;

        Ok(tonic::Response::new(AddGroupToArenaResponse {}))
    }

    async fn free_up_group(
        &self,
        request: tonic::Request<FreeUpGroupRequest>,
    ) -> std::result::Result<tonic::Response<FreeUpGroupResponse>, tonic::Status> {
        let mut tourn_mut = self.tournament.lock().await;
        let Some(ref mut tournament) = *tourn_mut else {
            return Err(tonic::Status::new(
                tonic::Code::Internal,
                "not loaded jet".to_string(),
            ));
        };

        let Some(group_id) = request.into_inner().group_id else {
            return Err(tonic::Status::new(
                tonic::Code::InvalidArgument,
                "group_id not set".to_string(),
            ));
        };

        tournament.freeup_group(&group_id.into())?;

        Ok(tonic::Response::new(FreeUpGroupResponse {}))
    }
}
