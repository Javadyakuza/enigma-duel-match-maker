#![feature(decl_macro)] // helps us with the routing of our application
extern crate rocket;

use std::collections::HashMap;
use std::mem::ManuallyDrop;
use std::str::FromStr;
use std::sync::Mutex;

use helpers::{
    create_game_room, create_key_hash, determine_winner, finish_game_room, get_rand_questions,
};
use models::{
    FinishGameParams, MatchFound, MatchResult, MatchRoomState, OngoingMatches, UserMatches,
};
use rocket::request::{Form, Request};
use rocket::*;
use rocket_cors::{AllowedHeaders, AllowedMethods, AllowedOrigins, Error};

use rocket_contrib::json::Json;
pub mod helpers;
pub mod models;
pub use models::{MakeMatch, Queue};
// #[get("/isFound/<>")]
// fn count(curr_queue: State<Queue>) -> String {
//     let fetched_queue = curr_queue.queue.lock().unwrap();
//     fetched_queue.get(&7).unwrap();
//     format!("Number of visits: {}", 2)
// }
#[post("/find-match", data = "<match_param>")]
fn find_match(
    q_queue: State<Queue>,
    ongo_queue: State<OngoingMatches>,
    users_matches: State<UserMatches>,
    match_param: Form<MakeMatch>,
) -> Json<Result<MatchFound, String>> {
    // consider add ing the user to the queue
    let mut fetched_queue = q_queue.queue.lock().unwrap_or_else(|e| e.into_inner());
    let mut fetched_user_matches = users_matches
        .matches
        .lock()
        .unwrap_or_else(|e| e.into_inner());
    let mut fetched_ongoing_queue = ongo_queue.queue.lock().unwrap_or_else(|e| e.into_inner());

    match fetched_user_matches.get(&match_param.user) {
        // some other user has already made a match with this user
        Some(res) => {
            // fetching the game room state
            let game_room_state = fetched_ongoing_queue.get(res).unwrap();
            let contestant = if game_room_state.contestant1 == match_param.user {
                game_room_state.contestant2.clone()
            } else {
                game_room_state.contestant1.clone()
            };
            Json(Ok(MatchFound {
                questions: game_room_state.questions.clone(),
                contestant,
            }))
        }
        // tha match has not been already made and its time to build it rn
        None => {
            // check if the room has been crated and do the operations
            let specific_amount_queue = fetched_queue.get_mut(&match_param.entry_amount);
            match specific_amount_queue {
                Some(res) => {
                    // initializing a match that maybe some match got found got replaced
                    let mut _match: MatchFound = MatchFound {
                        questions: get_rand_questions(),
                        contestant: "un-init".to_string(),
                    };

                    for (idx, user) in res.into_iter().enumerate() {
                        if user != &match_param.user {
                            _match.contestant = user.clone();
                            // deleting the requested
                            res.remove(idx);
                            break;
                        }
                    }

                    if _match.contestant == "un-init".to_string() {
                        for (_, user) in res.into_iter().enumerate() {
                            if user == &match_param.user {
                                return Json(Err("no one found, wait".to_string()));
                            }
                        }
                        res.push(match_param.user.clone());
                        return Json(Err("added you, wait".to_string()));
                    }

                    for (idx, user) in res.into_iter().enumerate() {
                        if user == &match_param.user {
                            // deleting the requestor
                            res.remove(idx);
                            break;
                        }
                    }
                    // adding to ongoing matches
                    let game_room_key: String =
                        create_key_hash(&match_param.user, &_match.contestant.clone());
                    let _ = fetched_ongoing_queue
                        .insert(
                            game_room_key.clone(),
                            MatchRoomState {
                                questions: _match.questions.clone(),
                                con_1_res: None,
                                con_2_res: None,
                                contestant1: match_param.user.clone(),
                                contestant2: _match.contestant.clone(),
                                con_1_fetched: false,
                                con_2_fetched: false,
                            },
                        )
                        .is_none();

                    // adding to the users game rooms
                    let _ = fetched_user_matches
                        .insert(match_param.user.clone(), game_room_key.clone())
                        .is_none();
                    let _ = fetched_user_matches
                        .insert(_match.contestant.clone(), game_room_key.clone())
                        .is_none();

                    println!(
                        "{}{}{}",
                        match_param.user.clone(),
                        _match.contestant.clone(),
                        match_param.entry_amount
                    );
                    create_game_room(
                        match_param.user.clone(),
                        _match.contestant.clone(),
                        match_param.entry_amount * 2,
                    )
                    .unwrap();
                    // sending the request to contract to create a game room
                    return Json(Ok(_match));
                }

                None => {
                    let _ = fetched_queue
                        .insert(match_param.entry_amount, vec![match_param.user.clone()])
                        .is_none();
                    Json(Err("added you, wait".to_string()))
                }
            }
        }
    }
}

#[post("/finish-match", data = "<finish_param>")]
fn finish_match(
    ongo_queue: State<OngoingMatches>,
    users_matches: State<UserMatches>,
    finish_param: Form<FinishGameParams>,
) -> Json<Result<String, String>> {
    // consider add ing the user to the queue
    let mut fetched_user_matches = users_matches
        .matches
        .lock()
        .unwrap_or_else(|e| e.into_inner());
    let mut fetched_ongoing_queue = ongo_queue.queue.lock().unwrap_or_else(|e| e.into_inner());

    match fetched_user_matches.get(&finish_param.contestant) {
        Some(room_key) => match fetched_ongoing_queue.get_mut(room_key) {
            Some(res) => {
                if res.con_1_res.is_some() && res.con_2_res.is_some() {
                    if finish_param.contestant == res.contestant1 {
                        res.con_1_fetched = true;
                        let con_2_fetched_res = res.con_2_fetched;
                        let con_2 = res.contestant2.clone();
                        let con_2_result = res.con_2_res.clone().unwrap().to_bin_string();
                        let winner = determine_winner(&finish_param.contestant, &con_2);

                        if con_2_fetched_res {
                            println!("finishing {}{}", room_key.clone(), winner);
                            finish_game_room(room_key.clone(), winner).unwrap();
                            fetched_ongoing_queue.remove(room_key);
                            fetched_user_matches.remove(&finish_param.contestant);
                            fetched_user_matches.remove(&con_2);
                        }
                        return Json(Ok(con_2_result));
                    } else {
                        res.con_2_fetched = true;
                        let con_1_fetched_res = res.con_1_fetched;
                        let con_1 = res.contestant2.clone();
                        let con_1_result = res.con_1_res.clone().unwrap().to_bin_string();
                        let winner = determine_winner(&finish_param.contestant, &con_1);
                        if con_1_fetched_res {
                            println!("finishing {}{}", room_key.clone(), winner);
                            finish_game_room(room_key.clone(), winner).unwrap();
                            fetched_ongoing_queue.remove(room_key);
                            fetched_user_matches.remove(&finish_param.contestant);
                            fetched_user_matches.remove(&con_1);
                        }
                        return Json(Ok(con_1_result));
                    }
                }

                if finish_param.contestant == res.contestant1 {
                    res.con_1_res = Some(MatchResult {
                        q1: finish_param.q1,
                        q2: finish_param.q2,
                        q3: finish_param.q3,
                    });
                    return Json(Err("wait".to_string()));
                } else {
                    res.con_2_res = Some(MatchResult {
                        q1: finish_param.q1,
                        q2: finish_param.q2,
                        q3: finish_param.q3,
                    });
                    return Json(Err("wait".to_string()));
                }
            }
            None => Json(Err("failed to fetch game room".to_string())),
        },
        None => Json(Err("user doesn't have game room".to_string())),
    }
}

// implement the upload results
#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Oh no the {} path doesn't exists !!", req.uri())
}
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::Response;

pub struct Cors;

impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    fn on_response(&self, request: &Request, response: &mut Response) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}
fn main() {
    let allowed_origins = AllowedOrigins::all();

    let allowed_origins: AllowedOrigins = AllowedOrigins::all();
    let allowed_methods: AllowedMethods = ["Get", "Post", "Delete"]
        .iter()
        .map(|s| FromStr::from_str(s).unwrap())
        .collect();
    let allowed_headers: AllowedHeaders = AllowedHeaders::all();
    let options = rocket_cors::Cors {
        allowed_origins: allowed_origins,
        allowed_methods: allowed_methods,
        allowed_headers: allowed_headers,
        allow_credentials: true,
        ..Default::default()
    };

    rocket::ignite()
        .attach(Cors)
        .attach(options)
        .manage(Queue {
            queue: Mutex::new(HashMap::new()),
        })
        .manage(OngoingMatches {
            queue: Mutex::new(HashMap::new()),
        })
        .manage(UserMatches {
            matches: Mutex::new(HashMap::new()),
        })
        .register(catchers![not_found])
        .mount("/api", routes![find_match, finish_match])
        .launch();
    // needs the "cargo build and then cargo run to be ran oin the fucking serve"
}
