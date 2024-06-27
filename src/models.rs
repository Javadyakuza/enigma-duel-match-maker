use std::{collections::HashMap, sync::Mutex};

use rocket::{FromForm, State};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, FromForm)]
pub struct MakeMatch {
    pub user: String,
    pub entry_amount: i32,
}

#[derive(Serialize, Deserialize, FromForm)]

pub struct MatchFound {
    pub questions: String,
    pub contestant: String,
}

#[derive(Serialize, Deserialize, FromForm)]
pub struct MatchResult {
    pub q1: bool,
    pub q2: bool,
    pub q3: bool,
}

#[derive(Serialize, Deserialize)]
pub struct MatchRoomState {
    pub questions: String,
    pub con_1_res: Option<MatchResult>,
    pub con_2_res: Option<MatchResult>,
    pub contestant1: String,
    pub contestant2: String,
    pub con_1_fetched: bool,
    pub con_2_fetched: bool,
}

pub struct Queue {
    pub queue: Mutex<HashMap<i32, Vec<String>>>,
}

pub struct UserMatches {
    pub matches: Mutex<HashMap<String, String>>,
}

pub struct OngoingMatches {
    pub queue: Mutex<HashMap<String, MatchRoomState>>,
}
