use std::f32::consts::PI;

pub mod instance;
// pub mod mesh;
pub mod light;
// pub mod model;
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
    use legion::{Entity, World, WorldOptions};

    use crate::renderer::{
        self,
        model::{Mesh, Transform},
    };

    #[test]
    pub fn cube_test() {
        let mut world = World::default();

        pollster::block_on(renderer::graphics::run("skygen"));

        world.extend(vec![
            (
                Mesh::cube(2.0, [1.0, 0.0, 0.0, 1.0]),
                Transform::new().translate(0.0, 1.0, 0.0),
            ),
            (
                Mesh::cube(2.0, [0.0, 1.0, 0.0, 1.0]),
                Transform::new().translate(1.0, 1.0, 0.0),
            ),
            (
                Mesh::cube(2.0, [0.0, 0.0, 1.0, 1.0]),
                Transform::new().translate(2.0, 1.0, 0.0),
            ),
            (
                Mesh::cube(2.0, [1.0, 1.0, 1.0, 1.0]),
                Transform::new().translate(3.0, 1.0, 0.0),
            ),
        ]);
    }
}
