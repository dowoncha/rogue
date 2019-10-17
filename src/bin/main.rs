#![feature(drain_filter)]

#[macro_use]
extern crate log;
extern crate ncurses;
extern crate rand;

extern crate env_logger;

extern crate rogue;

use rogue::{
    Component,
    file_logger, 
    EntityManager, 
    Map,
};

use rogue::systems::*;
use rogue::map::{simple_map_gen};
use rogue::components::{self, Position, Input, Render, RenderLayer, Collidable, Walk};
use std::time::{Instant, Duration};

fn create_map_entities(map: &Map, em: &mut EntityManager) {
    // Create tile entity prototypes

    for y in 0..map.height() {
        for x in 0..map.width() {
            let tile = em.create_entity();

            let cell = map.get_cell_ref(x as i32, y as i32);

            let glyph = cell.glyph;

            em.add_component(tile, Render { glyph: glyph, layer: RenderLayer::Map });
            em.add_component(tile, Position{ x: x as i32, y: y as i32});

            if glyph == '-' || glyph == '|' || glyph == '#' {
                em.add_component(tile, Collidable);
            }
        }
    }
}

fn populate_map(map: &Map, em: &mut EntityManager) {
    // let (goblin, _) = em.get_entity_by_name("Goblin").unwrap();

    // Create a monster at the center of each room
    for room in &map.rooms {
        let center = room.center();
        rogue::monsters::create_zombie(em, center.0, center.1);

        // let g_entity = em.create_entity();

        // em.add_component(g_entity, components::Prototype { prototype: goblin });
        // em.add_component(g_entity, components::Position { x: center.0 + 1, y: center.1 + 1 });

        // em.find_entity(components::Metaname("Goblin"));
    }
}

struct Game {
    // script_manager: ScriptManager,
    renderer: CursesRenderer,
    entity_manager: EntityManager,
    system_manager: SystemManager,
    render_system: RenderSystem,
    input_system: InputSystem,
    headless: bool,
    initialized: bool,
    running: bool
}

impl Game {
    pub fn new() -> Self {
        Self {
            input_system: InputSystem::new(),
            render_system: RenderSystem::new(),
            entity_manager: EntityManager::new(),
            system_manager: SystemManager::new(),
            renderer: CursesRenderer::new(),
            // script_manager: ScriptManager::new(),
            headless: false,
            initialized: false,
            running: false
        }
    }

    pub fn init(&mut self, args: Vec<String>) {
        info!("Initializing game");

        self.handle_args(args);

        if !self.headless {
            self.render_system.mount(&mut self.entity_manager);

            self.input_system.mount(&mut self.entity_manager);
        }

        self.register_game_systems();

        // self.script_manager.init();

        // self.script_manager.load_game_assets();

        self.load_game_entities();

        debug!("{:?}", self.entity_manager);

        self.initialized = true;
    }

    fn handle_args(&mut self, args: Vec<String>) {
        info!("Handling args");

        if args.iter().any(|arg| arg == "--headless") {
            info!("Headless mode");
            self.headless = true;
        }
    }

    fn register_game_systems(&mut self) {
        let system_manager = &mut self.system_manager;
        system_manager.register_system(Chronos::new());
        system_manager.register_system(TurnSystem::new());
        system_manager.register_system(RandomWalkAiSystem);
        system_manager.register_system(WalkSystem);
        system_manager.register_system(CollisionSystem);
        system_manager.register_system(AttackSystem);
        system_manager.register_system(DamageSystem);
        system_manager.register_system(MoveSystem);
        system_manager.register_system(LootSystem);
        system_manager.register_system(EventLogSystem);
        system_manager.register_system(Reaper);
        system_manager.register_system(Janitor);

        self.system_manager.mount(&mut self.entity_manager);
    }

    fn load_game_entities(
        &mut self, 
    ) {
        let map_width = 100;
        let map_height = 100;

        // let map = create_map();
        let map = simple_map_gen(map_width, map_height);

        let player_pos = map.rooms.first().unwrap().center();

        create_map_entities(&map, &mut self.entity_manager);
        let player_components = self.create_player("gromash", player_pos.0, player_pos.1);

        let player = self.entity_manager.create_entity();

        for component in player_components {
            self.entity_manager.add_boxed_component(player, component);
        }

        populate_map(&map, &mut self.entity_manager);
    }

    fn create_player(
        &self,
        name: &str,
        x: i32, 
        y: i32
    ) -> Vec<Box<dyn Component>> {
        vec![
            Box::new(components::Player),
            Box::new(components::Name { name: name.to_string() }),
            Box::new(Input::new()),
            Box::new(Render { glyph: '@', layer: RenderLayer::Player }),
            Box::new(Position{ x: x, y: y}),
            Box::new(Collidable),
            Box::new(components::Health { health: 100, max_health: 100 }),
            Box::new(Walk::new()),
            Box::new(components::Log::new()),
            Box::new(components::Energy { amount: 0 }),
            Box::new(components::Speed { amount: 10 })
        ]
    }

    pub fn run(&mut self) {
        if !self.initialized {
            panic!("Game was not initialized");
        }

        self.running = true;

        let mut last_time = Instant::now();

        while self.is_running() {
            let current = Instant::now();

            let elapsed: Duration = current.duration_since(last_time);

            self.handle_input();

            self.update(elapsed);

            self.render();

            last_time = current;
        }

        self.cleanup();
    }

    fn handle_input(&mut self) {
        if self.headless { 
            return;
        }

        self.input_system.process(&mut self.entity_manager);

        // check if quit was entered
        match self.input_system.get_last_input() { 
            Some(113) => {
                self.quit();
            },
            _ => {}
        }
    }
    
    fn update(&mut self, elapsed: Duration) {
        self.system_manager.process_systems(&mut self.entity_manager);
    }

    fn render(&mut self) {
        if !self.headless {
            self.render_system.process(&mut self.entity_manager);
        }
    }

    fn cleanup(&mut self) {
        if !self.headless {
            self.system_manager.unmount(&mut self.entity_manager);

            self.render_system.unmount();
        }
    }

    fn quit(&mut self) {
        self.running = false;
    }

    fn is_running(&self) -> bool {
        self.running
    }
}

trait Renderer {
    fn init(&mut self);
}

struct GameClient<R> {
    renderer: R,
    initialized: bool,
    running: bool
}

impl<R> GameClient<R: Renderer> {
    pub fn new() -> Self {
        Self {
            renderer: R,
            initialized: false,
            running: false
        }
    }

    pub fn init(&mut self, args: Vec<String>) {
        self.renderer.init();

        self.initialized = true;
    }

    pub fn run(&mut self) {
        if !self.initialized {
            panic!("Game client was not initialized");
        }

        self.running = true;

        while self.running {

        }
    }
}

#[test]
fn game_should_init_renderer() {
    let mut game = Game::new();
    game.init(vec![String::from("--headless")]);

    assert!(game.initialized);
}

#[test]
fn it_should_have_gametime_entity() {
    let mut game = Game::new();

    game.init(vec!["--headless".to_string()]);

    game.entity_manager.get_entity_by_name("GameTime").unwrap();
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().collect::<Vec<_>>();
    
    if args.iter().any(|arg| arg == "--headless") {
        env_logger::init()
            .expect("Failed to init env logger");

        info!("Env logger initialized");
    } else {
        file_logger::init()
            .expect("Failed to init file logger");

        info!("File logger initialized");
    }

    let mut game = Game::new();

    game.init(args);

    game.run();

    Ok(())
}

