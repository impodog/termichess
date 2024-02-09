use std::fmt::Display;

use serde::{Deserialize, Serialize};

pub type RoomCode = u64;

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub room: RoomCode,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub player: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommandRequest {
    pub room: RoomCode,
    // The player of chess, white(true) or black(false)
    pub player: bool,
    pub cmd: String,
    pub board: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryRequest {
    pub room: RoomCode,
    // The player of chess, white(true) or black(false)
    pub player: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryResponse {
    pub cmd: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IsOkRequest {
    pub room: RoomCode,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IsOkResponse {
    pub ok: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogoutRequest {
    pub room: RoomCode,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogBackRequest {
    pub room: RoomCode,
    pub player: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogBackResponse {
    pub board: String,
}

#[derive(Debug, Deserialize)]
pub struct ServerError;

impl std::error::Error for ServerError {}

impl Display for ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Server Error")
    }
}
