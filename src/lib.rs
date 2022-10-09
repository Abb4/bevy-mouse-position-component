//! # bevy-mouse-position-component
//! Wrapps the implementation of [Custom Camera Projection](https://bevy-cheatbook.github.io/cookbook/custom-projection.html#custom-camera-projection) into a `MousePosition2d` component, which is updated every frame to contain the cursor coordinates as world coordinates.
//!
//! # Usage
//!
//! Add [`MousePositionPlugin`] to your app:
//!
//! ```ignore
//! use bevy::prelude::*;
//! use bevy_mouse_position_component::{MousePosition2d, MousePositionPlugin};
//!
//! fn main() {
//!     App::new()
//!         .add_plugins(DefaultPlugins)
//!         .add_plugin(MousePositionPlugin)
//!         .run();
//! }
//! ```
//!
//! Add [`MousePosition2d`] component to your camera(-bundle):
//!
//! ```ignore
//! fn main() {
//!     App::new()
//!         // ...
//!         .add_startup_system(add_camera_with_tracking)
//!         // ...
//! }
//!
//!
//! fn add_camera_with_tracking(mut commands: Commands) {
//!     commands
//!         .spawn()
//!         .insert_bundle(Camera2dBundle::default())
//!         .insert(MousePosition2d::default()); // component added here
//! }
//! ```
//!
//! Finally add some system to query [`MousePosition2d`] and use `world_pos`:
//!
//! ```ignore
//! fn main() {
//!     App::new()
//!         // ...
//!         .add_system(print_camera_position)
//!         // ...
//! }
//!
//! fn print_camera_position(query: Query<&MousePosition2d>) {
//!     let mouse_position = query.single();
//!
//!     println!("{}", mouse_position.world_pos);
//! }
//! ```
//!
//! # Limitations
//!
//! [`MousePosition2d`] must be attached to a single `Camera` in the world, otherwise the plugin panics.

use bevy::{
    prelude::{App, Camera, Component, GlobalTransform, Plugin, Query, Res, Vec2, Windows},
    render::camera::RenderTarget,
};

pub struct MousePositionPlugin;

impl Plugin for MousePositionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_mouse_position_component);
    }
}

#[derive(Component, Default)]
pub struct MousePosition2d {
    pub world_pos: Vec2,
}

fn update_mouse_position_component(
    wnds: Res<Windows>,
    mut q_camera: Query<(&Camera, &GlobalTransform, &mut MousePosition2d)>,
) {
    // source: https://bevy-cheatbook.github.io/cookbook/custom-projection.html#custom-camera-projection

    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK
    let (camera, camera_transform, mut mouse_position) = q_camera.single_mut();

    // get the window that the camera is displaying to (or the primary window)
    let wnd = if let RenderTarget::Window(id) = camera.target {
        wnds.get(id).unwrap()
    } else {
        wnds.get_primary().unwrap()
    };

    // check if the cursor is inside the window and get its position
    if let Some(screen_pos) = wnd.cursor_position() {
        // get the size of the window
        let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

        // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

        // matrix for undoing the projection and camera transform
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();

        // use it to convert ndc to world-space coordinates
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

        mouse_position.world_pos = world_pos.truncate();
    }
}
