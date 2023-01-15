use std::f32::consts::PI;

pub mod instance;
// pub mod mesh;
pub mod light;
// pub mod model;
pub mod app;
pub mod material;
pub mod projection;
pub mod renderer;
pub mod texture;
pub mod vertex;

pub fn degrees(degrees: f32) -> f32 {
    (degrees) * (180.0 / PI)
}

#[cfg(test)]
pub mod test {
    use legion::{system, systems::CommandBuffer};

    use crate::{
        app::App,
        renderer::model::{Mesh, Transform},
    };

    #[test]
    pub fn cube_test() {
        #[system]
        pub fn create_entities(command: &mut CommandBuffer) {
            command.extend(vec![
                (Mesh::cube(2.0), Transform::new().translate(0.0, 1.0, 0.0)),
                (Mesh::cube(2.0), Transform::new().translate(1.0, 1.0, 0.0)),
                (Mesh::cube(2.0), Transform::new().translate(2.0, 1.0, 0.0)),
                (Mesh::cube(2.0), Transform::new().translate(3.0, 1.0, 0.0)),
            ]);
        }

        pollster::block_on(App::new().init_system(create_entities_system()).build());
    }
}
