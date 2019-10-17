use components::{Component, ComponentType};
use entities::*;

#[macro_use]
pub mod macros {
    #[macro_export]
    macro_rules! get_component {
        ($em:expr, $entity:expr, $component:ty) => {
            {
                $em.get_component($entity, <$component>::get_component_type())
                    .map(|component| component.as_any().downcast_ref::<$component>())
                    .flatten()
                    // .expect("No component found for entity")
            }
        };
        ($i:ident, $em: expr, $entity: expr, $component:ty) => {
            {
                $em.get_component_mut($entity, <$component>::get_component_type())
                    .map(|component| component.as_any_mut().downcast_mut::<$component>())
                    .flatten()
            }
        }
    }
}

pub trait System: std::fmt::Debug {
    fn mount(&mut self, _: &mut EntityManager) { }
    fn process(&self, _: &mut EntityManager) {}

    fn process_mut(&mut self, _: &mut EntityManager) {}

    fn unmount(&mut self, _: &mut EntityManager) { }

    fn on_add_component(&mut self, _: Entity, _: ComponentType) {}
}

mod system_manager;
pub use self::system_manager::SystemManager;

mod chronos_system;
pub use self::chronos_system::Chronos;

mod collide_system;
pub use self::collide_system::CollisionSystem;

mod render_system;
pub use self::render_system::RenderSystem;

mod input_system;
pub use self::input_system::InputSystem;

mod walk_system;
pub use self::walk_system::WalkSystem;

mod pickup_system;
pub use self::pickup_system::PickupSystem;

mod move_system;
pub use self::move_system::MoveSystem;

mod ai_system;
pub use self::ai_system::AiSystem;

mod attack_system;
pub use self::attack_system::AttackSystem;

mod damage_system;
pub use self::damage_system::DamageSystem;

mod reaper_system;
pub use self::reaper_system::Reaper;

mod loot_system;
pub use self::loot_system::LootSystem;

mod random_walk_system;
pub use self::random_walk_system::RandomWalkAiSystem;

mod janitor_system;
pub use self::janitor_system::Janitor;

mod event_log_system;
pub use self::event_log_system::EventLogSystem;

mod turn_system;
pub use self::turn_system::TurnSystem;
