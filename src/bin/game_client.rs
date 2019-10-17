
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