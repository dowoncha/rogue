extern crate rogue;

use rogue::systems::CursesRenderer;

impl Renderer for CursesRenderer {
  fn new() -> Self {
    Self::new()
  }

  fn init(&mut self) {

  }
}

trait Renderer {
  fn new() -> Self; 
  fn init(&mut self);
}

struct GameClient<R: Renderer> {
  renderer: R,
  initialized: bool,
  running: bool
}

impl<R> GameClient<R> 
  where R: Renderer
{
    pub fn new() -> Self {
        Self {
            renderer: R::new(),
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

fn main() {
    let args = std::env::args().collect::<Vec<_>>();

    let mut client = GameClient::<CursesRenderer>::new();

    client.init(args); 

    client.run();
}