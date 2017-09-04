
use specs::{
    Dispatcher, DispatcherBuilder, 
    Component, System, Entity, Entities, World, Join, RunNow,
    VecStorage, HashMapStorage, ReadStorage, WriteStorage};

// use ::types::Color;
// use ::renderer::Renderer;
use ::dungeon::Dungeon;

use std::sync::mpsc;
use std::sync::{Arc, Mutex, RwLock};

use ::time;
use std::env;

use std::collections::HashMap;

// use sdl2::{Sdl};
// use sdl2::image::{LoadTexture, INIT_PNG, INIT_JPG};
// use sdl2::keyboard::Keycode;
// use sdl2::pixels::Color;
// use sdl2::render::{Canvas, Texture, TextureCreator};

#[derive(Debug)]
struct Position(i32, i32);

#[derive(Debug)]
pub struct Sprite(u8, Color);

#[derive(Debug)]
struct Input;

impl Component for Position {
    // Vec Storage for components in almost every entity
    type Storage = VecStorage<Position>;
}

impl Component for Input {
    type Storage = HashMapStorage<Input>;
}

impl Component for Sprite {
    type Storage = VecStorage<Sprite>;
}

struct RenderSystem<'c> {
    canvas: &'c mut ::sdl2::render::Canvas<::sdl2::video::Window>
}

// #[derive(SystemData)]
// struct TextureManager<'t> {
//     textures: HashMap<String, Texture<'t>>
// }

impl<'c> RenderSystem<'c> {
    pub fn new(canvas: &'c mut Canvas<::sdl2::video::Window>) -> RenderSystem<'c> {
        RenderSystem {
            canvas
        }
    }
}

#[derive(SystemData)]
struct RenderData<'a> {
    entities: Entities<'a>,
    positions: ReadStorage<'a, Position>,
    sprites: ReadStorage<'a, Sprite>
}

impl<'a, 'c> System<'a> for RenderSystem<'c> {
    type SystemData = RenderData<'a>;

    fn run(&mut self, data: Self::SystemData) {
        // Clear the canvas
        self.canvas.clear();

        self.canvas.set_draw_color(Color::RGB(255, 0, 0));

        // self.canvas.copy(&texture, None, None).expect("Render failed");

        for (pos, sprite) in (&data.positions, &data.sprites).join() {
            // pos is destination rectangle
        }

        // Update the canvs
        self.canvas.present();
    }
}

type InputReceiverChannel = mpsc::Receiver<u32>;

struct InputSystem {
    receiver: InputReceiverChannel
}

impl InputSystem {
    pub fn new(receiver: InputReceiverChannel) -> InputSystem {
        InputSystem {
            receiver
        }
    }
}

impl<'a> System<'a> for InputSystem {
    type SystemData = WriteStorage<'a, Position>;

    fn run(&mut self, mut data: Self::SystemData) {
       
    }
}

struct GameTime {
    clock: u64
}

pub struct RogueGame {
    world: World,
    game_time: GameTime,
    sdl_ctx: ::sdl2::Sdl,
    video_subsystem: ::sdl2::VideoSubsystem,
    timer_subsystem: ::sdl2::TimerSubsystem,
    canvas: ::sdl2::render::Canvas<::sdl2::video::Window>
}

// Timer constants
const UPDATE_INTERVAL_MS: u64 = 14;
const UPDATE_PER_FRAME_LIMIT: u64 = 10;

impl RogueGame {
    pub fn init(/* env args */) -> Result<RogueGame, String> {
        // Initialize SDL 2
        let sdl_ctx = ::sdl2::init()?;
        
        // Initialize sdl subsystems
        let video_subsystem = sdl_ctx.video()
            .expect("Failed to init video subsystem");
        let timer_subsystem = sdl_ctx.timer()
            .expect("Failed to init timer subsystem");

        // Initialize sdl2 image
        let _image_context = ::sdl2::image::init(INIT_PNG | INIT_JPG).unwrap();

        // Create a window and convert into a canvas
        let window = video_subsystem.window("Rogue", 800, 600)
            .position_centered()
            .opengl()
            .build()
            .expect("Failed to create window");

        let canvas = window.into_canvas()
            .build()
            .expect("Failed to convert into canvas");

        let mut game = RogueGame {
            world: World::new(),
            game_time: GameTime {
                clock: 0
            },
            sdl_ctx,
            video_subsystem,
            timer_subsystem,
            canvas
        };

        // Register ECS components
        game.register_components();

        Ok(game)
    }

    // Run
    fn register_components(&mut self) {
        self.world.register::<Position>();
        // self.world.register::<Glyph>();
        self.world.register::<Input>();
        self.world.register::<Sprite>();
    }

    pub fn run(&mut self) -> Result<(), String> {
        info!("Starting main loop");
        // Main event communication channel
        let (tx, rx) = mpsc::channel();

        let player = self.world.create_entity()
             .with(Position(50, 50))
             .with(Input)
             .with(Sprite(64, Color::RGB(0, 0, 255)))
             .build();

        let mut input_system = InputSystem::new(rx);

        // Register the Specs systems to dispatcher
        let mut dispatcher = DispatcherBuilder::new()
            .add_thread_local(RenderSystem::new(&mut self.canvas))
            .build();

        let mut event_pump = self.sdl_ctx.event_pump()
            .expect("Failed to get event pump");

        // FPS calculation
        let mut last_time = self.timer_subsystem.ticks() as u32;
        let mut lag = 0.0f64;

        'main: loop {
            use sdl2::event::Event;

            // FPS calculation
            let actual_time = self.timer_subsystem.ticks();
            let elapsed = actual_time - last_time;

            last_time = actual_time;
            lag += elapsed as f64;
            
            input_system.run_now(&self.world.res);

            while lag >= UPDATE_INTERVAL_MS as f64 {
                dispatcher.dispatch(&mut self.world.res);

                // Handle created and deleted entities
                self.world.maintain();   
                lag -= UPDATE_INTERVAL_MS as f64;
            }

            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'main
                    },
                    _ => {
                        // Get user input and send over channel
                        // tx.send(key as u32);
                    }
                }
            }
        }

        Ok(())
    }
}

impl Drop for RogueGame {
    fn drop(&mut self) {
    }
}
