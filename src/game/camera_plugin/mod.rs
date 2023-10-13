use bevy::prelude::*;

pub struct CameraPlugin;


impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup);
            //.add_systems(Update, move_camera);
    }
}

fn setup(mut cmd: Commands) {
    cmd.spawn(Camera2dBundle::default());
}

const CAM_SPEED: f32 = 1000.0;

fn move_camera(mut query: Query<&mut Transform, With<Camera>>, input: Res<Input<KeyCode>>, time: Res<Time>) {
    if let Ok(mut trans) = query.get_single_mut() {
        println!("Cam pos: {:?}", trans);

        if input.pressed(KeyCode::W) {
            trans.translation.y += CAM_SPEED * time.delta_seconds();
        }

        if input.pressed(KeyCode::S) {
            trans.translation.y -= CAM_SPEED * time.delta_seconds();
        }

        if input.pressed(KeyCode::D) {
            trans.translation.x += CAM_SPEED * time.delta_seconds();
        }

        if input.pressed(KeyCode::A) {
            trans.translation.x -= CAM_SPEED * time.delta_seconds();
        }
    }
}