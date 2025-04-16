mod container;
mod error;
pub mod tournament_core;
pub mod tournament_service;
pub mod fencer_service;

mod bewerb;
mod group;
mod round;

mod arena_slot;
pub mod day;
mod timeslot;

mod fencer;

pub mod tournament {
    tonic::include_proto!("tournament");
}
