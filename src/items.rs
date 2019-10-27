use super::{EntityManager, components};

pub fn spawn_potion_of_healing(
    em: &mut EntityManager,
    x: i32,
    y: i32
) {
    let health_potion = em.create_entity();

    em.add_component(health_potion, components::Position { x: x, y: y });
    em.add_component(health_potion, components::Render { glyph: '!', layer: components::RenderLayer::Item });
    em.add_component(health_potion, components::Consumable);
    // em.add_component(health_potion, components::Script)
}