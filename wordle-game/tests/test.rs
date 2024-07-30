use game_session_io::*;
use gtest::{Log, Program, ProgramBuilder, System};

const GAME_SESSION_PROGRAM_ID: u64 = 1;
const WORDLE_PROGRAM_ID: u64 = 2;
const USER: u64 = 50;

#[test]
fn test_win() {
    let system = System::new();
    system.init_logger();

    let game_session_program = ProgramBuilder::from_file("target/wasm32-unknown-unknown/release/game_session.opt.wasm")
        .with_id(GAME_SESSION_PROGRAM_ID)
        .build(&system);
    let wordle_program =
        ProgramBuilder::from_file("target/wasm32-unknown-unknown/release/wordle.opt.wasm")
            .with_id(WORDLE_PROGRAM_ID)
            .build(&system);

    // Case 1: wordle_program init
    let res = wordle_program.send_bytes(USER, []);
    assert!(!res.main_failed());

    // Case 2: game_session_program init
    let res = game_session_program.send_with_value(USER, GAME_SESSION_PROGRAM_ID, 0);
    assert!(!res.main_failed());

    // Case 3: CheckWord - failed: The user is not in the game
    let res = game_session_program.send_with_value(
        USER,
        Action::CheckWord {
            word: "abcde".to_string(),
        },
        0,
    );
    assert!(res.main_failed());

    // Case 4: StartGame - success
    let res = game_session_program.send_with_value(USER, Action::StartGame, 0);
    let log = Log::builder()
        .dest(USER)
        .source(GAME_SESSION_PROGRAM_ID)
        .payload(Event::GameStarted { user: USER.into() });
    assert!(!res.main_failed() && res.contains(&log));

    // Case 5: StartGame failed: The user is already in the game
    let res = game_session_program.send_with_value(USER, Action::StartGame, 0);
    assert!(res.main_failed());

    // Case 6: CheckWord failed: Invalid word
    let res = game_session_program.send_with_value(
        USER,
        Action::CheckWord {
            word: "qwert".to_string(),
        },
        0,
    );
    assert!(res.main_failed());

    // Case 7: CheckWord failed: Invalid word
    let res = game_session_program.send_with_value(
        USER,
        Action::CheckWord {
            word: "shell".to_string(),
        },
        0,
    );
    assert!(res.main_failed());

    // Case 8: CheckWord success, but failed to guess
    let res = game_session_program.send_with_value(
        USER,
        Action::CheckWord {
            word: "house".to_string(),
        },
        0,
    );
    let log = Log::builder()
        .dest(USER)
        .source(GAME_SESSION_PROGRAM_ID)
        .payload(Event::GameOver { outcome: Outcome::Lose });
    assert!(!res.main_failed() && res.contains(&log));

    // Case 9: CheckWord success and has been guessed
    let res = game_session_program.send_with_value(
        USER,
        Action::CheckWord {
            word: "human".to_string(),
        },
        0,
    );
    let log = Log::builder()
        .dest(USER)
        .source(GAME_SESSION_PROGRAM_ID)
        .payload(Event::GameOver { outcome: Outcome::Win });
    assert!(!res.main_failed() && res.contains(&log));

    // Case 10: CheckWord failed: The user is not in the game
    let res = game_session_program.send_with_value(
        51,
        Action::CheckWord {
            word: "tests".to_string(),
        },
        0,
    );
    assert!(res.main_failed());

    let state: GameSession = game_session_program.read_state(b"").unwrap();
    println!("{:?}", state);
}

#[test]
fn test_tried_limit() {
    let system = System::new();
    system.init_logger();

    let game_session_program = ProgramBuilder::from_file("target/wasm32-unknown-unknown/release/game_session.opt.wasm")
        .with_id(GAME_SESSION_PROGRAM_ID)
        .build(&system);
    let wordle_program =
        ProgramBuilder::from_file("target/wasm32-unknown-unknown/release/wordle.opt.wasm")
            .with_id(WORDLE_PROGRAM_ID)
            .build(&system);

    // Case 1: wordle_program init
    let res = wordle_program.send_bytes(USER, []);
    assert!(!res.main_failed());

    // Case 2: game_session_program init
    let res = game_session_program.send_with_value(USER, GAME_SESSION_PROGRAM_ID, 0);
    assert!(!res.main_failed());

    // Case 3: StartGame success
    let res = game_session_program.send_with_value(USER, Action::StartGame, 0);
    let log = Log::builder()
        .dest(USER)
        .source(GAME_SESSION_PROGRAM_ID)
        .payload(Event::GameStarted { user: USER.into() });
    assert!(!res.main_failed() && res.contains(&log));

    for i in 0..5 {
        // Case 4: CheckWord success, but not guessed
        let res = game_session_program.send_with_value(
            USER,
            Action::CheckWord {
                word: "house".to_string(),
            },
            0,
        );
        if i == 4 {
            let log = Log::builder()
                .dest(USER)
                .source(GAME_SESSION_PROGRAM_ID)
                .payload(Event::GameOver { outcome: Outcome::Lose });
            assert!(!res.main_failed() && res.contains(&log));
        } else {
            let log = Log::builder()
                .dest(USER)
                .source(GAME_SESSION_PROGRAM_ID)
                .payload(Event::GameOver { outcome: Outcome::Lose });
            assert!(!res.main_failed() && res.contains(&log));
        }
    }
    let state: GameSession = game_session_program.read_state(b"").unwrap();
    println!("{:?}", state);
}

#[test]
#[ignore]
fn test_delayed_logic() {
    let system = System::new();
    system.init_logger();

    let game_session_program = ProgramBuilder::from_file("target/wasm32-unknown-unknown/release/game_session.opt.wasm")
        .with_id(GAME_SESSION_PROGRAM_ID)
        .build(&system);
    let wordle_program =
        ProgramBuilder::from_file("target/wasm32-unknown-unknown/release/wordle.opt.wasm")
            .with_id(WORDLE_PROGRAM_ID)
            .build(&system);

    // Case 1: wordle_program init
    let res = wordle_program.send_bytes(USER, []);
    assert!(!res.main_failed());

    // Case 2: game_session_program init
    let res = game_session_program.send_with_value(USER, GAME_SESSION_PROGRAM_ID, 0);
    assert!(!res.main_failed());

    // Case 3: StartGame success
    let res = game_session_program.send_with_value(USER, Action::StartGame, 0);
    let log = Log::builder()
        .dest(USER)
        .source(GAME_SESSION_PROGRAM_ID)
        .payload(Event::GameStarted { user: USER.into() });
    assert!(!res.main_failed() && res.contains(&log));

    // Case 4: Delayed equal to 200 blocks (10 minutes) for the delayed message
    let result = system.spend_blocks(200);
    println!("{:?}", result);
    let log = Log::builder()
        .dest(USER)
        .source(GAME_SESSION_PROGRAM_ID)
        .payload(Event::GameOver { outcome: Outcome::Lose });
    assert!(result[0].contains(&log));
    let state: GameSession = game_session_program.read_state(b"").unwrap();
    println!("{:?}", state);
}
