use rlua::{Lua, Table, RegistryKey};

use std::sync::{Arc, Mutex};
use std::collections::HashMap;

use entities::EntityManager;

fn setup_register_entity(lua: &Lua) -> Arc<Mutex<HashMap<String, RegistryKey>>> {
    let entities = Arc::new(Mutex::new(HashMap::new()));

    lua.context(|lua_ctx| {

        let register_entity = {
            let entities = entities.clone();

            let register_entity = lua_ctx.create_function(move |ctx, (name, table): (String, Table)| {
                let key = ctx.create_registry_value(table)
                    .expect("should have inserted in registry");

                entities.lock().unwrap().insert(name, key);

                Ok(())
            }).unwrap();

            register_entity
        };

        lua_ctx.globals().set("register_entity", register_entity).unwrap();
    });

    entities
}

struct ScriptManager {
    lua: Lua,
    entities: Arc<Mutex<HashMap<String, RegistryKey>>>
}

impl ScriptManager {
    pub fn new() -> Self {
        Self {
            lua: Lua::new(),
            entities: Arc::new(Mutex::new(HashMap::new()))
        }
    }

    pub fn init(&mut self) {
        self.load_lua_globals();

        self.load_game_assets();
    }

    fn load_lua_globals(&mut self) {
        self.entities = setup_register_entity(&self.lua);
    }

    pub fn load_game_assets(&self) {
        self.load_asset("assets/goblin.lua");
    }

    pub fn load_asset(
        &self, 
        asset_name: &str
    ) {
        use std::io::Read;

        let mut buffer = String::new();
        let mut asset_file = std::fs::File::open(asset_name).unwrap();

        asset_file.read_to_string(&mut buffer).unwrap();

        self.lua.context(|lua_ctx| {
            lua_ctx.load(&buffer)
                .exec()
                .unwrap();
        });
    }
}

fn load_entity(entity_manager: &mut EntityManager, name: &str, table: Table) {
    let entity = entity_manager.create_entity();

    entity_manager.set_entity_name(entity, name);

    let glyph: rlua::Result<String> = table.get("glyph");

    if let Ok(glyph) = glyph {
        let glyph = glyph.chars().next().unwrap();
        entity_manager.add_component(entity, components::Render { glyph: glyph, layer: components::RenderLayer::Player });
    }

    let max_health: rlua::Result<i32> = table.get("max_health");

    if let Ok(max_health) = max_health {
        entity_manager.add_component(entity, components::Health { health: max_health, max_health: max_health });
    }

    let collidable: rlua::Result<bool> = table.get("collidable");

    if let Ok(collidable) = collidable {

    }
}

