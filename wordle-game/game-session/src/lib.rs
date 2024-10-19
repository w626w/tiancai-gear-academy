#![no_std]

use game_session_io::*;
use gstd::{exec, msg, prelude::*, ActorId};

static mut GAME_SESSION: Option<GameSession> = None;

#[no_mangle]
extern "C" fn init() {
    let init: ActorId = msg::load().expect("Failed to load init message");
    unsafe {
        GAME_SESSION = Some(GameSession {
            wordle_program: init,
            games: Vec::new(),
            next_session_id: 1,
        });
    }
}

#[no_mangle]
extern "C" fn handle() {
    let action: Action = msg::load().expect("Failed to load handle message");
    let game_session = get_game_session_mut();
    let caller = msg::source();

    match action {
        Action::StartGame => {
            if !game_session.games.iter().any(|g| g.user == caller) {
                let session_id = game_session.next_session_id;
                game_session.next_session_id += 1;
                let new_game_status = GameStatus {
                    user: caller,
                    word: None,
                    attempts: 0,
                    status: GameState::NotStarted,
                    session_id,
                };
                game_session.games.push(new_game_status);

                msg::send(
                    game_session.wordle_program,
                    WordleAction::StartGame { user: caller },
                    0,
                )
                .expect("Failed to send StartGame action");

                msg::send_delayed(
                    exec::program_id(),
                    Action::CheckGameStatus {
                        user: caller,
                        session_id,
                    },
                    0,
                    200,
                )
                .expect("Failed to send delayed message");

                exec::wait();
            }
        }
        Action::CheckWord { word } => {
            if let Some(game_status) = game_session.games.iter_mut().find(|g| g.user == caller) {
                if let GameState::InProgress = game_status.status {
                    msg::send(
                        game_session.wordle_program,
                        WordleAction::CheckWord { user: caller, word },
                        0,
                    )
                    .expect("Failed to send CheckWord action");

                    game_status.status = GameState::InProgress;
                    exec::wait();
                }
            }
        }
        Action::CheckGameStatus { user, session_id } => {
            if msg::source() == exec::program_id() {
                if let Some(game_status) = game_session.games.iter_mut().find(|g| g.user == user) {
                    if game_status.session_id == session_id {
                        match game_status.status {
                            GameState::NotStarted => {
                                msg::send(user, Event::GameNotFound { user }, 0)
                                    .expect("Failed to send GameNotFound event");
                            }
                            GameState::InProgress => {
                                game_status.status = GameState::GameOver(Outcome::Lose);
                                msg::send(
                                    user,
                                    Event::GameOver {
                                        outcome: Outcome::Lose,
                                    },
                                    0,
                                )
                                .expect("Failed to send GameOver event");
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }
}

#[no_mangle]
extern "C" fn handle_reply() {
    let wordle_event: WordleEvent = msg::load().expect("Failed to load reply message");
    let game_session = get_game_session_mut();
    let user = *wordle_event.get_user();

    match wordle_event {
        WordleEvent::GameStarted { .. } => {
            if let Some(game_status) = game_session.games.iter_mut().find(|g| g.user == user) {
                game_status.status = GameState::InProgress;
            }
        }
        WordleEvent::WordChecked {
            correct_positions,
            contained_in_word: _,
            ..
        } => {
            if let Some(game_status) = game_session.games.iter_mut().find(|g| g.user == user) {
                game_status.attempts += 1;
                if correct_positions == vec![0, 1, 2, 3, 4] {
                    game_status.status = GameState::GameOver(Outcome::Win);
                    msg::send(
                        user,
                        Event::GameOver {
                            outcome: Outcome::Win,
                        },
                        0,
                    )
                    .expect("Failed to send GameOver event");
                } else if game_status.attempts >= 6 {
                    game_status.status = GameState::GameOver(Outcome::Lose);
                    msg::send(
                        user,
                        Event::GameOver {
                            outcome: Outcome::Lose,
                        },
                        0,
                    )
                    .expect("Failed to send GameOver event");
                } else {
                    game_status.status = GameState::InProgress;
                }
                exec::wake(msg::id()).expect("Failed to wake message");
            }
        }
    }
}

#[no_mangle]
extern "C" fn state() {
    let game_session = get_game_session();
    msg::reply(
        GameSession {
            wordle_program: game_session.wordle_program,
            games: game_session.games.clone(),
            next_session_id: game_session.next_session_id,
        },
        0,
    )
    .expect("Failed to send state reply");
}

fn get_game_session_mut() -> &'static mut GameSession {
    unsafe {
        GAME_SESSION
            .as_mut()
            .expect("GameSession is not initialized")
    }
}

fn get_game_session() -> &'static GameSession {
    unsafe {
        GAME_SESSION
            .as_ref()
            .expect("GameSession is not initialized")
    }
}
