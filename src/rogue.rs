use specs::{VecStorage, 
    Dispatcher, DispatcherBuilder, 
    Component, System, Entity,
    Join, ReadStorage, World };

use ::ncurses;
use ::types::Color;

#[derive(Debug)]
struct Position(i32, i32);

#[derive(Debug)]
struct Render(char, Color);

impl Component for Position {
    // Vec Storage for components in almost every entity
    type Storage = VecStorage<Position>;
}

impl Component for Render {
    type Storage = VecStorage<Render>;
}

// struct RenderSystemData<'a> {
//     position: ReadStorage<'a, Position>,
//     graphics: ReadStorage<'a, Render>
// }

struct RenderSystem;

impl<'a> System<'a> for RenderSystem {
    type SystemData = (ReadStorage<'a, Position>, ReadStorage<'a, Render>);

    fn run(&mut self, data: Self::SystemData) {
        ncurses::clear();

        for (pos, gfx) in (&data.0, &data.1).join() {
            ncurses::mvaddch(pos.0, pos.1, gfx.0 as u64);
        }
    }
}

struct InputSystem;

impl<'a> System<'a> for InputSystem {
    
}

pub struct RogueGame {
    world: World
    // dispatcher: Option<Dispatcher>
}

impl RogueGame {
    pub fn new() -> RogueGame {
        RogueGame {
            world: World::new(),
            // player:
            // dispatcher: None
        }
    }

    // Game setup
    pub fn init(&mut self) {
        self.register_components();
        
        self.register_systems();

        // log!("Initilizing curses renderer");
        // Start ncurses mode
        ncurses::initscr();
        // Line buffering disabled
        // ncurses::raw();

        // Disable key echo to screen
        ncurses::noecho();
    } 

    fn register_components(&mut self) {
        self.world.register::<Position>();
        self.world.register::<Render>();
    }

    fn register_systems(&mut self) {
    }

    pub fn run(&mut self) {
        self.world.create_entity()
             .with(Position(50, 50))
             .with(Render('@', Color::new()))
             .build();

        let mut dispatcher = DispatcherBuilder::new()
            .add(RenderSystem, "renderer", &[])
            .build();

        dispatcher.dispatch(&mut self.world.res);
        // self.world.maintain();
        
        loop {
            ncurses::refresh();
            
            // Get input
            let key = ncurses::getch();

            // Update
            // dispatcher.dispatch(&mut self.world.res);

            // Render

            // Refresh all windows without updating
            // Refresh standard screen
            ncurses::wnoutrefresh(ncurses::stdscr());

            // Update main screen
            ncurses::doupdate();
        }
    }
}

impl Drop for RogueGame {
    fn drop(&mut self) {
        ncurses::endwin();
    }
}