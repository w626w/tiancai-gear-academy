# Kami Wordle Gear Academy
* For the intermediate course.
* My ID is `bofu zhang 38`.

# Description
* This project contains two programs: `wordle-session` and `wordle-program`. Therefore, you need to build both programs separately using `cargo build`.

# Build
## First: Build `wordle-program`
1. Navigate to the `wordle-program` directory:
   ```sh
   cd wordle-program
   ```
2. Build the program to create `target/wasm32-unknown-unknown/debug/kami_wordle_gear_academy.opt.wasm` for the test:
   ```sh
   cargo build
   ```

## Second: Build `wordle-session`
1. Navigate to the root directory:
   ```sh
   cd ..
   ```
2. Then navigate to the `wordle-session` directory:
   ```sh
   cd wordle-session
   ```
3. Build the program:
   ```sh
   cargo build
   ```

# Test
1. Navigate to the `wordle-session` directory:
   ```sh
   cd wordle-session
   ```
2. Run the tests:
   ```sh
   cargo test
   ```


# Deployment

Game_session: 0x38622091970e6461a51516be13cea83b25b1f405fd055337cc4350fd58fe4b19
link: https://idea.gear-tech.io/programs/0x38622091970e6461a51516be13cea83b25b1f405fd055337cc4350fd58fe4b19?node=wss%3A%2F%2Ftestnet.vara.network

Wordle: 0xc6fd36abe6cfe1ca405ecd898eebb253e39658d7cb07699b4c4043c4683898f8
Link: https://idea.gear-tech.io/programs/0xc6fd36abe6cfe1ca405ecd898eebb253e39658d7cb07699b4c4043c4683898f8?node=wss%3A%2F%2Ftestnet.vara.network
