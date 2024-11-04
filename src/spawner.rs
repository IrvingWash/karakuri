use kec::Entity;

use crate::components::ComponentPayload;

#[derive(Debug)]
pub struct Spawner {
    pub entities_to_add: Vec<ComponentPayload>,
    pub entities_to_destroy: Vec<Entity>,
}

impl Default for Spawner {
    fn default() -> Spawner {
        Spawner::new()
    }
}

impl Spawner {
    pub fn new() -> Spawner {
        Spawner {
            entities_to_add: Vec::new(),
            entities_to_destroy: Vec::new(),
        }
    }

    pub fn add_entity(&mut self, components: ComponentPayload) {
        self.entities_to_add.push(components);
    }

    pub fn destroy_entity(&mut self, entity: Entity) {
        self.entities_to_destroy.push(entity);
    }
}

#[cfg(test)]
mod tests {
    use crate::components::TagComponent;

    use super::*;

    #[test]
    fn test_add_entity() {
        let mut spawner = Spawner::new();

        spawner.add_entity(ComponentPayload {
            tag: Some(TagComponent::new(String::from("Sonic"))),
            ..Default::default()
        });

        spawner.add_entity(ComponentPayload {
            tag: Some(TagComponent::new(String::from("Tails"))),
            ..Default::default()
        });

        assert_eq!(spawner.entities_to_add.len(), 2);
    }

    #[test]
    fn test_remove_entity() {
        let mut spawner = Spawner::new();

        spawner.destroy_entity(Entity::new(5));
        spawner.destroy_entity(Entity::new(7));

        assert_eq!(spawner.entities_to_destroy.len(), 2);
    }
}
