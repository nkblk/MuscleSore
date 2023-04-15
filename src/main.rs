use bevy::{
    prelude::*,
    sprite::{Material2d, MaterialMesh2dBundle},
    transform::commands,
    window::PrimaryWindow,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_mod_picking::*;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PickingPlugin)
        .add_plugin(InteractablePickingPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .add_system(clicky.in_base_set(CoreSet::PostUpdate))
        .add_system(clicky_bar.in_base_set(CoreSet::PostUpdate))
        .add_startup_system(setup)
        .add_startup_system(loading_bar)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    primary_query: Query<&Window, With<PrimaryWindow>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let Ok(window) = primary_query.get_single() else {
        return;
    };
    let window_size = Vec2::new(window.width() as f32, window.height() as f32);

    commands
        .spawn(Camera2dBundle::default())
        .insert(PickingCameraBundle::default());
    let parent = commands
        .spawn(SpriteBundle {
            texture: asset_server.load("a.png"),
            sprite: Sprite {
                custom_size: Some(window_size),
                ..default()
            },
            ..default()
        })
        .id();

    let butt = commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::RegularPolygon::new(50., 6).into()).into(),
            material: materials.add(ColorMaterial::from(Color::RED)),
            transform: Transform::from_translation(Vec3::new(332., 8., 0.))
                .with_scale(Vec3::new(2.6, 1.5, 1.0)),
            ..default()
        })
        .insert(PickableBundle::default())
        .id();
    commands.entity(parent).push_children(&[butt]);

    let chest = commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::RegularPolygon::new(50., 6).into()).into(),
            material: materials.add(ColorMaterial::from(Color::TURQUOISE)),
            transform: Transform::from_translation(Vec3::new(-345.6, 186.0, 0.))
                .with_scale(Vec3::new(2.5, 1.0, 1.0)),
            ..default()
        })
        .insert(PickableBundle::default())
        .id();
    commands.entity(parent).push_children(&[chest]);
}
#[derive(Component, Default, Copy, Clone)]
pub struct Bar {
    per: f32,
}

#[derive(Component, Default, Copy, Clone)]
pub struct ProgBar {
    per: f32,
}
pub fn loading_bar(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    primary_query: Query<&Window, With<PrimaryWindow>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let parent = commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Quad::new(Vec2::new(50., 100.)).into())
                .into(),
            material: materials.add(ColorMaterial::from(Color::RED)),
            transform: Transform::from_translation(Vec3::new(0., -200., 20.))
                .with_scale(Vec3::new(2.6, 1.5, 1.0)),
            ..default()
        })
        .insert(Bar { per: 0.0 })
        .insert(PickableBundle::default())
        .id();
    let per = 0.0;
    let prog = (1.0 - per) * -50.0;
    let butt = commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Quad::new(Vec2::new(50., 100.)).into())
                .into(),
            material: materials.add(ColorMaterial::from(Color::BLUE)),
            transform: Transform::from_translation(Vec3::new(0., prog, 1.))
                .with_scale(Vec3::new(1.0, per, 1.0)),
            ..default()
        })
        .insert(ProgBar { per: 0.0 })
        .id();
    commands.entity(parent).push_children(&[butt]);
}

pub fn clicky(
    mouse_button_input: Res<Input<MouseButton>>,
    touches_input: Res<Touches>,
    click_query: Query<(Entity, &Hover), Without<Bar>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query: Query<&mut Handle<ColorMaterial>>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left)
        || touches_input.iter_just_pressed().next().is_some()
    {
        for (entity, hover) in click_query.iter() {
            if hover.hovered() {
                if let Ok(x) = query.get_mut(entity) {
                    let y = materials.get_mut(&x).unwrap();
                    y.color.set_r(y.color.r() - 0.10);
                }
            }
        }
    }
    if mouse_button_input.just_pressed(MouseButton::Right)
        || touches_input.iter_just_pressed().next().is_some()
    {
        for (entity, hover) in click_query.iter() {
            if hover.hovered() {
                if let Ok(x) = query.get_mut(entity) {
                    let y = materials.get_mut(&x).unwrap();
                    y.color.set_r(y.color.r() + 0.10);
                }
            }
        }
    }
}

pub fn clicky_bar(
    mouse_button_input: Res<Input<MouseButton>>,
    touches_input: Res<Touches>,
    mut click_query: Query<(Entity, &mut Transform, &mut Bar, &Hover)>,
    mut query: Query<(Entity, &mut Transform, &mut ProgBar), Without<Bar>>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left)
        || touches_input.iter_just_pressed().next().is_some()
    {
        for (entity, mut transform, mut bar, hover) in click_query.iter_mut() {
            if hover.hovered() {
                bar.per += 0.1;
                let per = bar.per;
                let prog = (1.0 - per) * -50.0;
                for (entity, mut t, p) in query.iter_mut() {
                    *t = Transform::from_translation(Vec3::new(0., prog, 1.))
                        .with_scale(Vec3::new(1.0, per, 1.0));
                }
            }
        }
    }
    if mouse_button_input.just_pressed(MouseButton::Right)
        || touches_input.iter_just_pressed().next().is_some()
    {
        for (entity, mut transform, mut bar, hover) in click_query.iter_mut() {
            if hover.hovered() {
                bar.per -= 0.1;
                let per = bar.per;
                let prog = (1.0 - per) * -50.0;
                for (entity, mut t, p) in query.iter_mut() {
                    *t = Transform::from_translation(Vec3::new(0., prog, 1.))
                        .with_scale(Vec3::new(1.0, per, 1.0));
                }
            }
        }
    }
}
