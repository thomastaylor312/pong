use amethyst::{core::transform::Transform, prelude::*, renderer::Camera};

pub fn initialize(world: &mut World) {
    // Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left.
    let mut transform = Transform::default();
    transform.set_translation_xyz(super::ARENA_WIDTH * 0.5, super::ARENA_HEIGHT * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(super::ARENA_WIDTH, super::ARENA_HEIGHT))
        .with(transform)
        .build();
}
