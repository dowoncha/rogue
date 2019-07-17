use ncurses as nc;

use super::{System, EntityManager, Component, Entity};
use components::{Render, Position};

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
    map_window: Option<*mut i8>
}

impl RenderSystem {
    pub fn new() -> Self {
        Self {
            map_window: None
        }
    }

    pub fn mount(&mut self) {
        init_ncurses();

        // Create Windows
        // UI Initialization
        let mut screen_width = 80;
        let mut screen_height = 40;

        nc::getmaxyx(nc::stdscr(), &mut screen_height, &mut screen_width);

        info!("Screen size, {:?}x{:?}", screen_width, screen_height);

        let map_window = nc::newwin(screen_height - 5, screen_width - 20, 0, 20);
        nc::box_(map_window, 0, 0);

        self.map_window = Some(map_window);
    }
}

impl Drop for RenderSystem {
    fn drop(&mut self) {
        // let _ = nc::delwin(map_window);

        drop_ncurses();
    }
}

impl System for RenderSystem {
    fn process(&self, entity_manager: &mut EntityManager) {
        debug!("Rendering");
        let entities = entity_manager.get_entities_with_components(Render::get_component_type());

        let render_components = entities.iter()
            .filter_map(|entity| get_component!(entity_manager, *entity, Render));

        let position_components  = entities.iter()
            .filter_map(|entity| get_component!(entity_manager, *entity, Position));

        let mut sorted_entities: Vec<(&Entity, (&Render, &Position))> = entities.iter()
            .zip(render_components.zip(position_components))
            .collect::<Vec<_>>();

            sorted_entities.sort_by(|(_, (a, _)), (_, (b, _))| a.layer.cmp(&b.layer));

        let map_window = self.map_window.unwrap();

        for (_, (render, position)) in sorted_entities {
            nc::mvwaddch(map_window, position.y, position.x, render.glyph as u64);
        }

        nc::wrefresh(map_window);

        // nc::refresh();
    }
}