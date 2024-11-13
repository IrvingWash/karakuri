use kec::Registry;

struct HP(u8);
struct Speed(u8);
struct Position(u8);
#[allow(dead_code)]
struct Power(u8);
#[allow(dead_code)]
struct UnusedComponent(u8);

#[test]
fn test_lifecycle() {
    let mut registry = Registry::new();

    // Create entities
    let mario = registry.create_entity();
    registry.add_component(&mario, HP(3));
    registry.add_component(&mario, Speed(5));
    registry.add_component(&mario, Position(10));

    let gumba = registry.create_entity();
    registry.add_component(&gumba, Speed(2));
    registry.add_component(&gumba, Position(3));

    // Mario gets hit
    registry.get_component_mut::<HP>(&mario).unwrap().0 -= 1;

    assert_eq!(registry.get_component::<HP>(&mario).unwrap().0, 2);

    // Update positions
    let movable_entities = registry
        .query()
        .with_component::<Position>()
        .with_component::<Speed>()
        .build();

    for entity in &movable_entities {
        let mut position = registry.get_component_mut::<Position>(entity).unwrap();
        let speed = registry.get_component::<Speed>(entity).unwrap();

        position.0 += speed.0;
    }

    assert_eq!(registry.get_component::<Position>(&mario).unwrap().0, 15);
    assert_eq!(registry.get_component::<Position>(&gumba).unwrap().0, 5);
}

#[test]
fn test_querying() {
    let mut registry = Registry::new();

    let chun_li = registry.create_entity();
    let blanka = registry.create_entity();

    registry.add_component(&chun_li, HP(100));
    registry.add_component(&chun_li, Power(10));

    let entities_with_hp = registry.query().with_component::<HP>().build();
    assert_eq!(entities_with_hp.len(), 1);
    assert_eq!(entities_with_hp[0], chun_li);

    let entities_with_hp_and_power = registry
        .query()
        .with_component::<HP>()
        .with_component::<Power>()
        .build();
    assert_eq!(entities_with_hp.len(), 1);
    assert_eq!(entities_with_hp_and_power[0], chun_li);

    registry.add_component(&blanka, HP(90));

    let entities_with_hp = registry.query().with_component::<HP>().build();
    assert_eq!(entities_with_hp.len(), 2);
    assert!(entities_with_hp[0] != entities_with_hp[1]);

    registry.add_component(&blanka, Power(9));

    let entities_with_hp_and_power = registry
        .query()
        .with_component::<HP>()
        .with_component::<Power>()
        .build();
    assert_eq!(entities_with_hp.len(), 2);
    assert!(entities_with_hp_and_power[0] != entities_with_hp_and_power[1]);

    let entities = registry
        .query()
        .with_component::<HP>()
        .with_component::<Power>()
        .with_component::<UnusedComponent>()
        .build();

    assert_eq!(entities.len(), 0);
}

#[test]
fn test_build_with_no_signatures() {
    let mut registry = Registry::new();

    let entities = registry.query().with_component::<HP>().build();
    assert!(entities.is_empty());

    let entities = registry.query().build();
    assert!(entities.is_empty());
}
