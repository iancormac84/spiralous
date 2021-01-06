use bevy::prelude::*;
use bevy_flycam::PlayerPlugin;

fn setup(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut segments: ResMut<CubeSegments>,
) {
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 60.0 })),
            material: materials.add(Color::hex("7ed957").unwrap().into()),
            ..Default::default()
        })
        .spawn(LightBundle {
            light: Light {
                fov: 200.0,
                depth: 1.0..1000.0,
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 10.0, 0.0)),
            ..Default::default()
        });

    segments.0.push(
        commands
            .spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 0.5 })),
                material: materials.add(Color::hex("ffb6ff").unwrap().into()),
                transform: Transform::from_translation(Vec3::new(0.0, 25.0, 0.0)),
                ..Default::default()
            })
            .with(AlphaCube)
            .with(NextFrameTransform::default())
            .current_entity()
            .unwrap(),
    );
    for i in 1..=7 {
        segments.0.push(
            commands
                .spawn(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube { size: 0.5 })),
                    material: materials.add(Color::hex("ffb6ff").unwrap().into()),
                    transform: Transform::from_translation(Vec3::new(0.0, 25.0, 5.0 * i as f32)),
                    ..Default::default()
                })
                .with(NextFrameTransform::default())
                .current_entity()
                .unwrap(),
        )
    }
}

#[derive(Debug, Default, Copy, Clone)]
pub struct NextFrameTransform {
    pub translation: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

struct AlphaCube;

#[derive(Default)]
struct CubeSegments(Vec<Entity>);

fn cube_chain_movement(
    time: Res<Time>,
    segments: ResMut<CubeSegments>,
    alpha_cube: Query<Entity, With<AlphaCube>>,
    mut positions: Query<(&mut NextFrameTransform, &mut Transform)>,
) {
    if let Some(alpha_entity) = alpha_cube.iter().next() {
        let (mut next_frame_transform, mut transform) = positions.get_mut(alpha_entity).unwrap();
        let time_delta = time.seconds_since_startup();
        transform.translation.x =
            ((2.0 * time_delta).sin() as f32 * 5.0) + (time_delta.sin() as f32 * 5.0);
        transform.translation.y = (2.0 * time_delta).cos() as f32 * 5.0;
        transform.translation.z -= 0.01;

        next_frame_transform.translation = transform.translation;
        next_frame_transform.rotation = transform.rotation;
        next_frame_transform.scale = transform.scale;

        let segment_transforms = segments
            .0
            .iter()
            .map(|e| *positions.get_mut(*e).unwrap().0)
            .collect::<Vec<NextFrameTransform>>();

        segment_transforms
            .iter()
            .zip(segments.0.iter().skip(1))
            .for_each(|(pos, segment)| {
                let mut pos = *pos;
                pos.translation.z += 0.75;
                *positions.get_mut(*segment).unwrap().0 = pos;
            });
    }
}

fn transform_propagation(
    alpha_cube: Query<Entity, With<AlphaCube>>,
    mut q: Query<(Entity, &NextFrameTransform, &mut Transform)>,
) {
    let alpha_entity = loop {
        if let Some(entity) = alpha_cube.iter().next() {
            break entity;
        }
    };
    for (entity, next_transform, mut transform) in q.iter_mut() {
        if alpha_entity != entity {
            transform.translation = next_transform.translation;
            transform.rotation = next_transform.rotation;
            transform.scale = next_transform.scale;
        }
    }
}

#[bevy_main]
fn main() {
    App::build()
        .add_resource(ClearColor(Color::MIDNIGHT_BLUE))
        .add_resource(Msaa { samples: 4 })
        .add_resource(CubeSegments::default())
        .add_startup_system(setup.system())
        .add_system(cube_chain_movement.system())
        .add_system(transform_propagation.system())
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .run();
}
