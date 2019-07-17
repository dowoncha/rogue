// Scrolling map guide
// http://www.roguebasin.com/index.php?title=Scrolling_map

use ncurses as nc;

use super::{System, EntityManager, Component, Entity};
use components::{self, Render, Position};
use types::{Rect};

/**
 * Render code
 */
fn init_ncurses() {
    // Start ncurses
    nc::initscr();

    if !nc::has_colors() {
        nc::endwin();
        error!("Terminal does not support color");
        // return Err(Box::new("Terminal does not support color".to_string()));
    }

    // Allow colors
    nc::start_color();

    // colors::init();

    // Line buffering disabled
    // Signals are not interpreted and are instead passed directly to program
    // TODO: change to raw after implementing signals
    // nc::raw();
    nc::cbreak();

    // Disable echoing of chracaters
    nc::noecho();

    // Enableds reading of function keys
    nc::keypad(nc::stdscr(), true);

    nc::curs_set(nc::CURSOR_VISIBILITY::CURSOR_INVISIBLE);
}

pub fn drop_ncurses() {
    nc::endwin();
}

pub struct RenderSystem {
    map_window: Option<*mut i8>,
    player_info_window: Option<*mut i8>,
    log_window: Option<*mut i8>
}

impl RenderSystem {
    pub fn new() -> Self {
        Self {
            map_window: None,
            player_info_window: None,
            log_window: None
        }
    }

    fn create_map_window(&self, screen_width: i32, screen_height: i32) -> *mut i8 {
        let map_window_x = 20;
        let map_window_y = 0;
        let map_window_width = screen_width - map_window_x;
        let map_window_height = screen_height - 4;

        let map_window = nc::newwin(map_window_height, map_window_width, map_window_y, map_window_x);

        map_window
    }

    fn create_player_info_window(&self, _screen_width: i32, _screen_height: i32) -> *mut i8 {
        let player_info_window_x = 0;
        let player_info_window_y = 0;
        let player_info_window_width = 20;
        let player_info_window_height = 5;

        let player_info_window = nc::newwin(
            player_info_window_height, 
            player_info_window_width, 
            player_info_window_y, 
            player_info_window_x
        );

        player_info_window
    }

    fn create_log_window(&self, screen_width: i32, screen_height: i32) -> *mut i8 {
        let x = 20;
        let width = screen_width - x;

        let height = 4;
        let y = screen_height - height;

        nc::newwin(height, width, y, x)
    }

    pub fn mount(&mut self) {
        init_ncurses();

        // Create Windows
        // UI Initialization
        let mut screen_width = 80;
        let mut screen_height = 24;

        nc::getmaxyx(nc::stdscr(), &mut screen_height, &mut screen_width);

        info!("Screen size, {:?}x{:?}", screen_width, screen_height);

        self.map_window = Some(self.create_map_window(screen_width, screen_height));

        self.player_info_window = Some(self.create_player_info_window(screen_width, screen_height));

        self.log_window = Some(self.create_log_window(screen_width, screen_height));
    }

    fn get_camera_position(&self, em: &EntityManager) -> Position {
        let map_window = self.map_window.unwrap();

        let player = em.get_entities_with_components(components::Player::get_component_type())[0];
        let player_position = get_component!(em, player, components::Position).unwrap();

        let mut map_window_width = 0;
        let mut map_window_height = 0;

        nc::getmaxyx(map_window, &mut map_window_height, &mut map_window_width);

        let camera_pos = Position { 
            x: player_position.x - map_window_width / 2, 
            y: player_position.y - map_window_height / 2
        };

        camera_pos
    }

    fn get_world_position(&self, camera_pos: &Position, entity_pos: &Position) -> Position {
        let world_position = Position {
            x: entity_pos.x - camera_pos.x, 
            y: entity_pos.y - camera_pos.y
        };

        world_position
    }

    fn render_player_info(&self, entity_manager: &EntityManager) {
        let window = self.player_info_window.unwrap();

        // Player name
        let player = entity_manager.get_entities_with_components(components::Player::get_component_type())[0];
        let player_components = entity_manager.get_entity_all_components(player);

        let player_name = get_component!(entity_manager, player, components::Name).unwrap();

        nc::mvwaddstr(window, 1, 1, &player_name.name);

        let player_health = get_component!(entity_manager, player, components::Health).unwrap();
        nc::mvwaddstr(window, 2, 1, &format!("HP: {}/{}", player_health.health, player_health.max_health));

        nc::box_(window, 0, 0);

        nc::wrefresh(window);
    }

    fn render_log(&self, entity_manager: &EntityManager) {
        let window = self.log_window.unwrap();

        let player = entity_manager.get_entities_with_components(components::Player::get_component_type())[0];

        let player_log = get_component!(entity_manager, player, components::Log).unwrap();

        // Clear input log
        // nc::mvwaddch(window, 1, 1, ' ' as u64);
        // nc::clrtoeol();

        if let Some(message) = player_log.history.last() {
            debug!("Logging {}", message);
            nc::mvwaddstr(window, 1, 1, &message);
        }

        nc::box_(window, 0, 0);

        nc::wrefresh(window);
    }

    fn render_map(&self, entity_manager: &EntityManager) {
        let entities = entity_manager.get_entities_with_components(Render::get_component_type());

        // Only fetch entity's position componet if they have a render
        let render_components = entities.iter()
            .filter_map(|entity| get_component!(entity_manager, *entity, Render));

        let position_components  = entities.iter()
            .filter_map(|entity| get_component!(entity_manager, *entity, Position));

        let mut sorted_entities: Vec<(&Entity, (&Render, &Position))> = entities.iter()
            .zip(render_components.zip(position_components))
            .collect::<Vec<_>>();

            sorted_entities.sort_by(|(_, (a, _)), (_, (b, _))| a.layer.cmp(&b.layer));

        let camera_pos = self.get_camera_position(entity_manager);
        let map_window = self.map_window.unwrap();

        // TODO
        // Cache this
        let mut map_window_width = 0;
        let mut map_window_height = 0;

        nc::getmaxyx(map_window, &mut map_window_height, &mut map_window_width);

        for (_, (render, position)) in sorted_entities {
            let world_pos = self.get_world_position(&camera_pos, &position);
            if world_pos.x > 0 && world_pos.y > 0 && world_pos.x < map_window_width - 1 && world_pos.y < map_window_height - 1 {
                nc::mvwaddch(map_window, world_pos.y, world_pos.x, render.glyph as u64);
            }
        }

        nc::box_(map_window, 0, 0);

        nc::wrefresh(map_window);
    }
}

impl System for RenderSystem {
    fn process(&self, entity_manager: &mut EntityManager) {
        debug!("Rendering");

        // Check window resize
        let mut screen_size_x = 0;
        let mut screen_size_y = 0;

        nc::getmaxyx(nc::stdscr(), &mut screen_size_y, &mut screen_size_x);

        self.render_map(entity_manager);

        self.render_player_info(entity_manager);

        self.render_log(entity_manager);
    }
}

impl Drop for RenderSystem {
    fn drop(&mut self) {
        // let _ = nc::delwin(map_window);

        drop_ncurses();
    }
}