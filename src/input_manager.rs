use ncurses as nc;
use std::thread;

use game_state::GameState;
use client::Event;

pub fn init(event_sender: std::sync::mpsc::Sender<Event>) {
    // Input thread
    let input_manager = thread::spawn(move || {
        info!("Input manager thread started");

        loop {
            let input = nc::getch();

            let event = handle_input(input);

            if let Some(event) = event {
                event_sender.send(event)
                    .expect("Failed to send event");
            }
        }
    });
}

pub fn handle_input(input: i32) -> Option<Event> {
    match input {
        119 => {
            // 'w
            Some(Event::Move(0, -1))
        }
        115 => {
            // 'd'
            Some(Event::Move(0, 1))
        }
        100 => {
            // 'd'
            Some(Event::Move(1, 0))
        }
        97 => {
            // 'a'
            Some(Event::Move(-1, 0))
        }
        113 => {
            // 'q'
            Some(Event::Quit)
        }
        _ => None
    }
}

struct PlayerInputHandler {

}

impl PlayerInputHandler {
    pub fn handle_keys(&self, user_input: i32, game_state: GameState) {

    }
}