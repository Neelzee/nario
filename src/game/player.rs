
pub mod player_controller {
    use bevy::prelude::{Plugin, Update, Startup, Input, KeyCode, Res, With, Transform, Query, Commands};

    use super::player::Player;

    pub struct PlayerControllerPlugin;

    impl Plugin for PlayerControllerPlugin {
        fn build(&self, app: &mut bevy::prelude::App) {
            app
                .add_systems(Startup, setup)
                .add_systems(Update, player_controller);
        }
    }

    fn setup(mut cmd: Commands) {
        cmd.spawn(
            (
                Player,
                Transform::from_xyz(0.0, 0.0, 0.0)
            )
        );
    }

    pub const PLAYER_SPEED: f32 = 10.0;

    fn player_controller(input: Res<Input<KeyCode>>, mut query: Query<&mut Transform, With<Player>>) {

        if let Ok(mut trans) = query.get_single_mut() {

            println!("Player pos: {:?}", trans.translation);

            if input.pressed(KeyCode::W) {
                trans.translation.y += PLAYER_SPEED;
            }

            if input.pressed(KeyCode::S) {
                trans.translation.y -= PLAYER_SPEED;
            }

            if input.pressed(KeyCode::D) {
                trans.translation.x += PLAYER_SPEED;
            }

            if input.pressed(KeyCode::A) {
                trans.translation.x -= PLAYER_SPEED;
            }
        }

    }
}


pub mod player {
    use bevy::prelude::Component;

    #[derive(Component)]
    pub struct Player;
}