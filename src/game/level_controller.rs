use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use crate::utils::Const::LEVEL_IDS;


pub struct LevelControllerPlugin;


impl Plugin for LevelControllerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup)
            .add_systems(Update, toggle_levels)
            .insert_resource(LdtkSettings {
            level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation  {
                load_level_neighbors: false,
            },
            ..Default::default()
        });
    }
}


fn setup(mut cmd: Commands, asset_server: Res<AssetServer>) {
    let level_set = LevelSet::from_iid((*LEVEL_IDS.get(0).unwrap()).to_string());

    cmd.spawn(
        LdtkWorldBundle {
            ldtk_handle: asset_server.load("level/lvl0.ldtk"),
            level_set,
            transform: Transform::from_xyz(-256., -144., 0.),
            ..Default::default()
        }
    );
}


fn toggle_levels(input: Res<Input<KeyCode>>, mut level_sets: Query<&mut LevelSet>) {
    if input.just_pressed(KeyCode::Space) {
        let level_to_toggle = *LEVEL_IDS.get(0).unwrap();

        let mut level_set = level_sets.single_mut();
        if level_set.iids.contains(level_to_toggle) {
            level_set.iids.remove(level_to_toggle);
        } else {
            level_set.iids.insert(level_to_toggle.to_string());
        }
    }
}