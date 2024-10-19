#![no_std]

use gmeta::{In, InOut, Metadata, Out};
use gstd::{prelude::*, ActorId};

#[derive(Debug, Encode, Decode, TypeInfo, Clone)]
pub enum Action {
    StartGame,
    CheckWord { word: String },
    CheckGameStatus { user: ActorId, session_id: u64 },
}

#[derive(Debug, Encode, Decode, TypeInfo, Clone)]
pub enum Event {
    GameStarted { user: ActorId },
    GameOver { outcome: Outcome },
    GameNotFound { user: ActorId },
}

#[derive(Debug, Encode, Decode, TypeInfo, Clone)]
pub struct GameSession {
    pub wordle_program: ActorId,
    pub games: Vec<GameStatus>,
    pub next_session_id: u64,
}

#[derive(Debug, Encode, Decode, TypeInfo, Clone)]
pub struct GameStatus {
    pub user: ActorId,
    pub word: Option<String>,
    pub attempts: u8,
    pub status: GameState,
    pub session_id: u64,
}

#[derive(Debug, Encode, Decode, TypeInfo, Clone)]
pub enum GameState {
    NotStarted,
    InProgress,
    GameOver(Outcome),
}

#[derive(Debug, Encode, Decode, TypeInfo, Clone)]
pub enum Outcome {
    Win,
    Lose,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub enum WordleAction {
    StartGame { user: ActorId },
    CheckWord { user: ActorId, word: String },
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub enum WordleEvent {
    GameStarted {
        user: ActorId,
    },
    WordChecked {
        user: ActorId,
        correct_positions: Vec<u8>,
        contained_in_word: Vec<u8>,
    },
}

pub struct GameSessionMetadata;

impl Metadata for GameSessionMetadata {
    type Init = In<ActorId>;
    type Handle = InOut<Action, Event>;
    type Others = ();
    type Reply = ();
    type Signal = ();
    type State = Out<GameSession>;
}

impl WordleEvent {
    pub fn get_user(&self) -> &ActorId {
        match self {
            WordleEvent::GameStarted { user } => user,
            WordleEvent::WordChecked { user, .. } => user,
        }
    }
}
