use std::collections::HashMap;

use components::{self, Component, ComponentType};

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Hash, Eq)]
pub struct Entity {
    pub id: i32,
}

impl Entity {
    fn new(id: i32) -> Self {
        Self {
            id: id,
        }
    }
}

impl std::fmt::Display for Entity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)?;

        Ok(())
    }
}

pub struct ComponentTable {
    component_data_tables: HashMap<ComponentType, HashMap<Entity, Box<dyn Component>>>,
}

pub struct EntityManager {
    entities: Vec<Entity>,
    entity_names: HashMap<Entity, String>,
    component_data_tables: HashMap<ComponentType, HashMap<Entity, Box<dyn Component>>>,
    listeners: Vec<std::sync::mpsc::Sender<String>>
}

pub struct GameObject {
    entity: Entity,
    name: String,
    prototype: Option<Entity>,
}

impl EntityManager {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
            entity_names: HashMap::new(),
            component_data_tables: HashMap::new(),
            listeners: Vec::new(),
        }
    }

    pub fn create_entity(&mut self) -> Entity {
        let index = self.entities.len() as i32;
        let entity = Entity::new(index);
        self.entities.push(Entity::new(index));

        entity
    }

    pub fn set_entity_name(&mut self, entity: Entity, name: &str) {
        self.entity_names.insert(entity, name.to_string());
    }

    pub fn get_entity_by_name(&self, name: &str) -> Option<(Entity, String)> {
        self.entity_names.iter()
            .find(|(_, search)| *search == name)
            .map(|(entity, name)| (*entity, name.clone()))
    }

    pub fn extend(&mut self, prototype: Entity, child: Entity) {
        self.add_component(child, components::Prototype { prototype: prototype });
    }

    pub fn add_boxed_component(&mut self, entity: Entity, component: Box<dyn Component>) {
        let component_type = component.get_type();

        let table = self.get_mut_component_table(component_type);

        let _ = table.insert(entity, component);
    }

    pub fn add_component<T>(&mut self, entity: Entity, component: T ) 
        where T: 'static + Component 
    {
        self.add_boxed_component(entity, Box::new(component));
    }

    fn get_component_table(&self, component_type: ComponentType) -> Option<&HashMap<Entity, Box<dyn Component>>> {
        self.component_data_tables.get(&component_type)
    }

    fn get_mut_component_table(&mut self, component_type: ComponentType) -> &mut HashMap<Entity, Box<dyn Component>> {
        // Check if table exists, create if it doesn't
        if !self
            .component_data_tables
            // .contains_key(&component.get_object_type())
            .contains_key(&component_type)
        {
            let _ = self.component_data_tables
                .insert(component_type, HashMap::new());
        }

        // Get the component's table
        let table = self
            .component_data_tables
            .get_mut(&component_type)
            .unwrap();

        table
    }

    fn get_prototype(&self, entity: Entity) -> Option<Entity> {
        self.get_component(entity, components::Prototype::get_component_type())
            .map(|component| component.as_any().downcast_ref::<components::Prototype>())
            .flatten()
            .map(|prototype| prototype.prototype)
    }

    pub fn get_component(
        &self,
        entity: Entity,
        component_type: ComponentType
    ) -> Option<&Box<dyn Component>> {
        // Get the table
        if let Some(table) = self.component_data_tables.get(&component_type) {
            return table.get(&entity);
        } else if let Some(prototype) = self.get_prototype(entity) {
            // Check the prototype
            self.get_component(prototype, component_type)
        } else {
            // Check for key not present tag

            None
        }
    }

    // pub fn get_generic_component<T: Component>(&self, entity: Entity) -> Option<&T> {
    //     let component = self.get_component(entity, T::get_component_type());
    //     component.map(|c| c.as_ref())
    // }

    pub fn get_component_mut(&mut self, entity: Entity, component_type: ComponentType) -> Option<&mut Box<dyn Component>> {
        let mut component = None;
        if let Some(table) = self.component_data_tables.get_mut(&component_type) {
            component = table.get_mut(&entity);
        }
        // } else if let Some(prototype) = self.get_prototype(entity) {
            // self.get_component_mut(prototype, component_type)
        component
    }

    pub fn get_entity_all_components(&self, entity: Entity) -> Vec<&Box<dyn Component>> {
        self.component_data_tables
            .iter()
            .filter_map(|(_, component_table)| component_table.get(&entity))
            .collect()
    }

    pub fn remove_component(&mut self, entity: Entity, component_type: ComponentType) -> Option<Box<dyn Component>> {
        self.component_data_tables
            .get_mut(&component_type)
            .map(|component_table| component_table.remove(&entity))
            .flatten()
    }

    pub fn get_entities_with_components(&self, component_type: ComponentType) -> Vec<Entity> {
        use std::iter::FromIterator;

        match self.component_data_tables.get(&component_type) {
            Some(table) => Vec::from_iter(table.keys().map(|entity| *entity)),
            None => vec![]
        }

        // self.component_data_tables.get(&component_type).ok_or()

        // let iter = table.values().map(Box::as_ref);
        // Vec::new().into_iter()
    }

    pub fn has_component(&self, entity: Entity, component_type: ComponentType) -> bool {
        if let Some(table) = self.component_data_tables.get(&component_type) {
            return table.contains_key(&entity);
        }

        return false;
    }

    pub fn get_all_components_of_type(
        &self,
        component_type: ComponentType,
    ) -> Vec<&Box<dyn Component>> {
        use std::iter::FromIterator;

        match self.component_data_tables.get(&component_type) {
            Some(table) => Vec::from_iter(table.values().into_iter()),
            None => vec![],
        }
    }

    pub fn kill_entity(&mut self, entity: Entity) {
        for (_, table) in self.component_data_tables.iter_mut() {
            let _ = table.remove(&entity);
        }

        self.entities.remove_item(&entity).unwrap();
    }

    // pub fn find_entity<T>(&self, component: T) -> Option<(Entity, &Box<T>)> 
    //     where T: 'static + Component 
    // {
    //     if let Some(table) = self.get_component_table(component.get_type()) {
    //         table.iter().find(|(_, b)| *b == component)
    //             .map(|(entity, c)| (*entity, c))
    //     } else {
    //         None
    //     }
    // }

    pub fn subscribe(&mut self, listener: std::sync::mpsc::Sender<String>) {
        self.listeners.push(listener);
    }
}

impl std::fmt::Debug for EntityManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Entities")?;

        // for entity in &self.entities {
        //     writeln!(f, "{:?}", entity)?;
        // }

        Ok(())
    }
}

#[cfg(test)]
mod entity_manager_tests {
    use super::*;

    #[derive(Debug, PartialEq)]
    pub struct TestComponent;

    impl Component for TestComponent {
        derive_component!();
    }

    #[test]
    fn test_create_entity() {
        let mut entity_manager = EntityManager::new();

        let entity = entity_manager.create_entity();

        let entity2 = entity_manager.create_entity();
        assert_ne!(entity2, entity);
    }

    #[test]
    fn test_add_component() {
        let mut entity_manager = EntityManager::new();
        let entity = entity_manager.create_entity();

        // let component: Option<&Box<dyn Component>> =
        //     entity_manager.get_component(entity, TestComponent::get_component_type());
        // assert!(component.is_none());

        // let test_component = TestComponent;

        // entity_manager.add_component(entity, test_component);

        // let component: Option<&Box<dyn Component>> =
        //     entity_manager.get_component(entity, TestComponent::get_component_type());

        // assert!(component.is_some());
    }

    #[test]
    fn test_get_entities_with_components() {
        let mut em = EntityManager::new();

        let entity = em.create_entity();
        let component = TestComponent;

        let entities = em.get_entities_with_components(TestComponent::get_component_type());

        assert_eq!(entities.len(), 0);

        em.add_component(entity, component);

        let entities = em.get_entities_with_components(TestComponent::get_component_type());

        assert_eq!(entities.len(), 1);
    }

    #[test]
    fn test_entity_metaname() {
        let mut em = EntityManager::new();

        let entity = em.create_entity();

        em.set_entity_name(entity, "the beast");

        let (expected_entity, _) = em.get_entity_by_name("the beast").unwrap();

        assert_eq!(entity, expected_entity);
    }

    // IT should fail to set name if the name is already set
}