use kec::{Entity, Registry};
use kutils::Size;
use kwindow::{AssetStorage, FpsController, InputProcessor, Timer, Window, WindowCtx};

use crate::{
    adapters::{EventSenderAdapter, InputProcessorAdapter, RegistryAdapter, TimerAdapter},
    components::{BehaviorComponent, ComponentPayload},
    errors::panic_queried,
    systems::{
        physics_system::AffectParams, AnimatorSystem, CameraSystem, PhysicsSystem, RendererSystem,
    },
    Event, EventBuss, GameConfig, Scene, UpdateContext,
};

pub struct Game {
    fps_controller: FpsController,
    input_processor: InputProcessor,
    registry: Registry,
    scene: Scene,
    ctx: WindowCtx,
    asset_storage: AssetStorage,
    renderer_system: RendererSystem,
    animator_system: AnimatorSystem,
    physics_system: PhysicsSystem,
    camera_system: CameraSystem,
    timer: Timer,
    event_buss: EventBuss,
    debug: bool,
}

impl Game {
    #[inline]
    pub fn new(config: &GameConfig) -> Self {
        let Window {
            fps_controller,
            renderer,
            input_processor,
            ctx,
            asset_storage,
        } = Window::new(
            config.title,
            config.resolution,
            &config.clear_color,
            config.target_fps,
        );

        Self {
            fps_controller,
            input_processor,
            registry: Registry::new(),
            scene: Scene::new(),
            ctx,
            asset_storage,
            renderer_system: RendererSystem::new(renderer),
            animator_system: AnimatorSystem::default(),
            physics_system: PhysicsSystem::default(),
            camera_system: CameraSystem::default(),
            timer: Timer::new(),
            event_buss: EventBuss::default(),
            debug: config.debug,
        }
    }

    #[inline]
    pub fn set_scene(&mut self, entities: Vec<ComponentPayload>) {
        self.scene.create_initial_entities(entities);
    }

    #[inline]
    pub fn add_texture(&mut self, name: &'static str, path: &'static str) -> Result<(), String> {
        self.asset_storage.add_texture(name, path, &mut self.ctx)
    }

    #[inline]
    pub fn start(&mut self) {
        loop {
            let time = self.fps_controller.time(&self.ctx);
            let delta_time = self.fps_controller.delta_time(&self.ctx);

            if self.input_processor.should_close(&self.ctx) {
                break;
            }

            let (entities_to_start, entities_to_destroy) =
                self.scene.sync(&mut self.registry, &self.asset_storage);

            self.start_entities(&entities_to_start, delta_time);

            self.destroy_entities(entities_to_destroy, delta_time);

            self.update_entities(delta_time, time);

            self.render();
        }
    }

    #[inline]
    pub fn resolution(&self) -> Size {
        self.renderer_system.resolution(&self.ctx)
    }

    fn start_entities(&mut self, entities_to_start: &[Entity], delta_time: f64) {
        for entity in entities_to_start {
            self.registry
                .get_component_mut::<Box<dyn BehaviorComponent>>(entity)
                .unwrap_or_else(|| panic_queried::<dyn BehaviorComponent>(entity))
                .start(UpdateContext {
                    entity,
                    delta_time,
                    registry: &RegistryAdapter::new(&self.registry),
                    input_processor: &InputProcessorAdapter::new(&self.input_processor, &self.ctx),
                    spawner: self.scene.spawner(),
                    timer: &mut TimerAdapter::new(&mut self.timer),
                    event_sender: &mut EventSenderAdapter::new(&mut self.event_buss),
                });
        }
    }

    fn destroy_entities(&mut self, entities_to_destroy: Vec<Entity>, delta_time: f64) {
        for entity in &entities_to_destroy {
            if let Some(mut behavior) = self
                .registry
                .get_component_mut::<Box<dyn BehaviorComponent>>(entity)
            {
                behavior.destroy(UpdateContext {
                    delta_time,
                    registry: &RegistryAdapter::new(&self.registry),
                    entity,
                    input_processor: &InputProcessorAdapter::new(&self.input_processor, &self.ctx),
                    spawner: self.scene.spawner(),
                    timer: &mut TimerAdapter::new(&mut self.timer),
                    event_sender: &mut EventSenderAdapter::new(&mut self.event_buss),
                });
            }
        }

        self.scene.set_entities_to_remove(entities_to_destroy);
    }

    fn update_entities(&mut self, delta_time: f64, time: f64) {
        let updateable_entities = self
            .registry
            .query()
            .with_component::<Box<dyn BehaviorComponent>>()
            .build();

        self.event_buss
            .add(Event::Timer(self.timer.consume_finished_timers(time)));
        let events = self.event_buss.consume_events();

        for entity in &updateable_entities {
            let mut behavior = self
                .registry
                .get_component_mut::<Box<dyn BehaviorComponent>>(entity)
                .unwrap_or_else(|| panic_queried::<dyn BehaviorComponent>(entity));

            behavior.update(UpdateContext {
                delta_time,
                registry: &RegistryAdapter::new(&self.registry),
                entity,
                input_processor: &InputProcessorAdapter::new(&self.input_processor, &self.ctx),
                spawner: self.scene.spawner(),
                timer: &mut TimerAdapter::new(&mut self.timer),
                event_sender: &mut EventSenderAdapter::new(&mut self.event_buss),
            });

            if !events.is_empty() {
                behavior.notify(
                    &events,
                    UpdateContext {
                        delta_time,
                        registry: &RegistryAdapter::new(&self.registry),
                        entity,
                        input_processor: &InputProcessorAdapter::new(
                            &self.input_processor,
                            &self.ctx,
                        ),
                        spawner: self.scene.spawner(),
                        timer: &mut TimerAdapter::new(&mut self.timer),
                        event_sender: &mut EventSenderAdapter::new(&mut self.event_buss),
                    },
                );
            }
        }

        self.camera_system.update(&mut self.registry);
        self.physics_system.affect(AffectParams {
            registry: &mut self.registry,
            delta_time,
            input_processor: &self.input_processor,
            spawner: self.scene.spawner(),
            timer: &mut self.timer,
            ctx: &self.ctx,
            event_buss: &mut self.event_buss,
        });

        self.animator_system.animate(&mut self.registry, time);
    }

    fn render(&mut self) {
        let fps = self.ctx.get_fps().to_string();
        let resolution = self.resolution();

        let mut handle = self.renderer_system.start_frame(&mut self.ctx);

        self.renderer_system
            .draw_sprites(&mut handle, &mut self.registry, &self.asset_storage);

        if self.debug {
            self.renderer_system
                .draw_box_colliders(&mut handle, &mut self.registry);

            self.renderer_system
                .draw_fps(&mut handle, &fps, &resolution);
        }

        self.renderer_system.finish_frame(handle);
    }
}
