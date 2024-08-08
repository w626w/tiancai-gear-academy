use game_session_io::*;
use gtest::{Log, Program, ProgramBuilder, System};

#[test]
fn test_start_game() {
    let sys = System::new();
    sys.init_logger();

    let wordle_program = Program::from_file(&sys, "target/wasm32-unknown-unknown/release/wordle.opt.wasm");
    let game_session_program = Program::from_file(&sys, "target/wasm32-unknown-unknown/release/game_session.opt.wasm");

    // Initialize the Game Session with Wordle program address
    let result = game_session_program.send("init", wordle_program.id());
    assert!(result.log().is_empty());

    // Start a game for a user
    let user: u64 = 1;
    let result = game_session_program.send("handle", GameAction::StartGame);
    assert!(result.contains(&GameEvent::GameStarted { user }));
}

#[test]
fn test_check_word_correct_guess() {
    let sys = System::new();
    sys.init_logger();

    let wordle_program = Program::from_file(&sys, "target/wasm32-unknown-unknown/release/wordle.opt.wasm");
    let game_session_program = Program::from_file(&sys, "target/wasm32-unknown-unknown/release/game_session.opt.wasm");

    // Initialize the Game Session with Wordle program address
    let _ = game_session_program.send("init", wordle_program.id());

    // Start a game for a user
    let user: u64 = 1;
    let _ = game_session_program.send("handle", GameAction::StartGame);

    // Simulate Wordle program's GameStarted reply
    sys.send(wordle_program.id(), WordleEvent::GameStarted { user });

    // Check the word "house" (correct guess)
    let word = "house".to_string();
    let result = game_session_program.send("handle", GameAction::CheckWord { word });

    // Simulate Wordle program's WordChecked reply
    sys.send(wordle_program.id(), WordleEvent::WordChecked {
        user,
        correct_positions: vec![0, 1, 2, 3, 4],
        contained_in_word: vec![]
    });

    assert!(result.contains(&GameEvent::GameOver { outcome: Outcome::Win }));
}

#[test]
fn test_check_word_incorrect_guess() {
    let sys = System::new();
    sys.init_logger();

    let wordle_program = Program::from_file(&sys, "target/wasm32-unknown-unknown/release/wordle.opt.wasm");
    let game_session_program = Program::from_file(&sys, "target/wasm32-unknown-unknown/release/game_session.opt.wasm");

    // Initialize the Game Session with Wordle program address
    let _ = game_session_program.send("init", wordle_program.id());

    // Start a game for a user
    let user: u64 = 1;
    let _ = game_session_program.send("handle", GameAction::StartGame);

    // Simulate Wordle program's GameStarted reply
    sys.send(wordle_program.id(), WordleEvent::GameStarted { user });

    // Check an incorrect word "human"
    let word = "human".to_string();
    let result = game_session_program.send("handle", GameAction::CheckWord { word });

    // Simulate Wordle program's WordChecked reply
    sys.send(wordle_program.id(), WordleEvent::WordChecked {
        user,
        correct_positions: vec![0],
        contained_in_word: vec![1]
    });

    assert!(result.log().contains(&GameEvent::WordChecked {
        correct_positions: vec![0],
        contained_in_word: vec![1]
    }));

    // Check game status (assuming this check happens after all attempts)
    let session_id = 1;
    let result = game_session_program.send("handle", GameAction::CheckGameStatus { user, session_id });

    assert!(result.contains(&GameEvent::GameOver { outcome: Outcome::Lose }));
}

#[test]
fn test_check_word_multiple_attempts() {
    let sys = System::new();
    sys.init_logger();

    let wordle_program = Program::from_file(&sys, "target/wasm32-unknown-unknown/release/wordle.opt.wasm");
    let game_session_program = Program::from_file(&sys, "target/wasm32-unknown-unknown/release/game_session.opt.wasm");

    // Initialize the Game Session with Wordle program address
    let _ = game_session_program.send("init", wordle_program.id());

    // Start a game for a user
    let user: u64 = 1;
    let _ = game_session_program.send("handle", GameAction::StartGame);

    // Simulate Wordle program's GameStarted reply
    sys.send(wordle_program.id(), WordleEvent::GameStarted { user });

    let mut attempts = 0;

    for word in &["human", "horse", "heron", "happy", "honey"] {
        attempts += 1;
        let result = game_session_program.send("handle", GameAction::CheckWord { word: word.to_string() });

        // Simulate Wordle program's WordChecked reply
        sys.send(wordle_program.id(), WordleEvent::WordChecked {
            user,
            correct_positions: vec![0],
            contained_in_word: vec![1]
        });

        assert!(result.log().contains(&GameEvent::WordChecked {
            correct_positions: vec![0],
            contained_in_word: vec![1]
        }));
    }

    // On the last attempt, simulate a losing condition
    attempts += 1;
    let word = "humor".to_string();
    let result = game_session_program.send("handle", GameAction::CheckWord { word });

    // Simulate Wordle program's WordChecked reply
    sys.send(wordle_program.id(), WordleEvent::WordChecked {
        user,
        correct_positions: vec![0],
        contained_in_word: vec![1]
    });

    assert!(result.contains(&GameEvent::GameOver { outcome: Outcome::Lose }));
}

#[test]
fn test_game_timeout() {
    let sys = System::new();
    sys.init_logger();

    let wordle_program = Program::from_file(&sys, "target/wasm32-unknown-unknown/release/wordle.opt.wasm");
    let game_session_program = Program::from_file(&sys, "target/wasm32-unknown-unknown/release/game_session.opt.wasm");

    // Initialize the Game Session with Wordle program address
    let _ = game_session_program.send("init", wordle_program.id());

    // Start a game for a user
    let user: u64 = 1;
    let _ = game_session_program.send("handle", GameAction::StartGame);

    // Simulate Wordle program's GameStarted reply
    sys.send(wordle_program.id(), WordleEvent::GameStarted { user });

    // Fast forward time (simulate delay)
    sys.spend_blocks(200);

    // After the delay, check the game status, this should trigger the timeout check
    let session_id = 1;
    let result = game_session_program.send("handle", GameAction::CheckGameStatus { user, session_id });

    // The result should contain a GameOver event indicating that the game was lost due to timeout
    assert!(result.contains(&GameEvent::GameOver { outcome: Outcome::Lose }));
}
