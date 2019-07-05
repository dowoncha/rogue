use std::cell::RefCell;
use types::BoxResult;
use engine::Engine;
use entity::Entity;

pub struct GameClient {
    engine: RefCell<Engine>
}

impl GameClient {
    pub fn new() -> Self {
        Self {
            engine: RefCell::new(Engine::new())
        }
    }

    pub fn init(
        &self,
        args: Vec<String>
    ) -> BoxResult<()> {
        let screen_width = 80;
        let screen_height = 50;

        let mut engine = self.engine.borrow_mut();

        engine.init(
            screen_width,
            screen_height
        );

        let player = Entity::new(screen_width as i32 / 2, screen_height as i32 / 2, '@', 0xff);
        let npc = Entity::new(screen_width as i32 / 2 - 5, screen_height as i32 / 2, '@', 0x33);

        engine.register_entity("player", player);
        engine.register_entity("npc", npc);

        Ok(())
    }

    pub fn run(&self) -> BoxResult<()>  {
        self.engine.borrow_mut().run()
    }

    pub fn load_map(&self, filename: &str) -> BoxResult<()> {
        self.engine.borrow_mut().load_map(filename)
    }
}