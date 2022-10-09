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
