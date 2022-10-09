# bevy-mouse-position-component
Wrapps the implementation of [Custom Camera Projection](https://bevy-cheatbook.github.io/cookbook/custom-projection.html#custom-camera-projection) into a `MousePosition2d` component, which is updated every frame to contain the cursor coordinates as world coordinates.

# Usage

Pull crate in by adding the following to `Cargo.toml`:

```
bevy-mouse-position-component = { git = "https://github.com/Abb4/bevy-mouse-position-component" }
```

The main branch contains stable releases. Development occurs on the `development` branch.


Add `MousePositionPlugin` to your app:

```rust
use bevy::prelude::*;
use bevy_mouse_position_component::{MousePosition2d, MousePositionPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(MousePositionPlugin)
        .run();
}
```

Add `MousePosition2d::default()` component to your camera(-bundle):

```rust
fn main() {
    App::new()
        // ...
        .add_startup_system(add_camera_with_tracking)
        // ...
}


fn add_camera_with_tracking(mut commands: Commands) {
    commands
        .spawn()
        .insert_bundle(Camera2dBundle::default())
        .insert(MousePosition2d::default()); // component added here
}
```

Finally add some system to query `MousePosition2d` and use `world_pos`:

```rust
fn main() {
    App::new()
        // ...
        .add_system(print_camera_position)
        // ...
}

fn print_camera_position(query: Query<&MousePosition2d>) {
    let mouse_position = query.single();

    println!("{}", mouse_position.world_pos);
}
```