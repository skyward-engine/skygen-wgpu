use legion::{
    systems::{Builder, ParallelRunnable, Resource},
    Resources, Schedule, World, WorldOptions,
};
use parking_lot::RwLock;
use wgpu::Features;
use winit::{
    event_loop::EventLoopBuilder, platform::windows::EventLoopBuilderExtWindows,
    window::WindowBuilder,
};

use crate::{projection::Projection, renderer::graphics::GraphicContainer};

pub enum SystemFrequency {
    Init,
    Tick,
}

pub struct App {
    world: RwLock<World>,
    resources: Resources,

    // these are all the schedulers we will be using
    schedule: Builder,
    init_schedule: Builder,
}

impl App {
    pub fn new() -> Self {
        let world = World::default();
        let resources = Resources::default();

        let schedule = Schedule::builder();
        let init_schedule = Schedule::builder();

        Self {
            world: RwLock::new(world),
            schedule,
            resources,
            init_schedule,
        }
    }

    pub fn legion_options(mut self, options: WorldOptions) -> Self {
        self.world = RwLock::new(World::new(options));
        self
    }

    pub fn system<T: ParallelRunnable + 'static>(
        mut self,
        frequency: SystemFrequency,
        system: T,
    ) -> Self {
        match frequency {
            SystemFrequency::Init => self.init_schedule.add_system(system),
            SystemFrequency::Tick => self.schedule.add_system(system),
        };

        self
    }

    pub fn init_system<T: ParallelRunnable + 'static>(self, system: T) -> Self {
        self.system(SystemFrequency::Init, system)
    }

    pub fn system_tick<T: ParallelRunnable + 'static>(self, system: T) -> Self {
        self.system(SystemFrequency::Tick, system)
    }

    pub fn resource<T: Resource + 'static>(mut self, resource: T) -> Self {
        self.resources.insert(resource);
        self
    }

    pub async fn build(mut self) {
        let mut schedule = self.schedule.build();
        let mut init_schedule = self.init_schedule.build();

        init_schedule.execute(self.world.get_mut(), &mut self.resources);
        schedule.execute(self.world.get_mut(), &mut self.resources);

        // graphics initialization
        let event_loop = EventLoopBuilder::new().with_any_thread(true).build();
        let window = WindowBuilder::new()
            .with_title("owo what's this")
            .build(&event_loop)
            .unwrap();

        let features = Features::empty();

        let mut container = GraphicContainer::new(
            &window,
            features,
            Projection::Perspective {
                aspect_ratio: 69.0,
                fov: 60.0,
            },
        )
        .await;

        event_loop.run(move |event, _, flow| {
            let container = &mut container;
            let mut world = self.world.write();

            // invoke all systems before rendering
            schedule.execute(&mut world, &mut self.resources);
            container.render(&mut world).unwrap();
        });
    }
}
