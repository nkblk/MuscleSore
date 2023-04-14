use bevy::{prelude::*, window::PrimaryWindow, sprite::MaterialMesh2dBundle};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_mod_picking::{
    DebugEventsPickingPlugin, DefaultPickingPlugins, PickingCameraBundle, PickableBundle, PickingEvent, SelectionEvent,
};
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(DefaultPickingPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_system(print_events.in_base_set(CoreSet::PostUpdate))
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, primary_query: Query<&Window, With<PrimaryWindow>>,     mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,) {
    let Ok(window) = primary_query.get_single() else {
        return;
    };
    let window_size = Vec2::new(window.width() as f32, window.height() as f32);

    commands.spawn(Camera2dBundle::default()).insert(PickingCameraBundle::default());
    let parent = commands.spawn(SpriteBundle {
        texture: asset_server.load("a.png"),
        sprite: Sprite {
            custom_size: Some(window_size),
            ..default()
        },
        ..default()
    }).id();

    let butt = commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::RegularPolygon::new(50., 6).into()).into(),
        material: materials.add(ColorMaterial::from(Color::TURQUOISE)),
        transform: Transform::from_translation(Vec3::new(332., 8., 0.)).with_scale(Vec3::new(2.6, 1.5, 1.0)),
        ..default()
    }).insert(PickableBundle::default()).id();
    commands.entity(parent).push_children(&[butt]);

    let chest = commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::RegularPolygon::new(50., 6).into()).into(),
        material: materials.add(ColorMaterial::from(Color::TURQUOISE)),
        transform: Transform::from_translation(Vec3::new(-345.6, 186.0, 0.)).with_scale(Vec3::new(2.5, 1.0, 1.0)),
        ..default()
    }).insert(PickableBundle::default()).id();
    commands.entity(parent).push_children(&[chest]);
}

pub fn print_events(mut events: EventReader<PickingEvent>) {
    for event in events.iter() {
        match event {
            PickingEvent::Selection(e) => {
                if let SelectionEvent::JustSelected(e) = e { 
                  info!("selection ID {}", e.index() );
                }
                info!("selection {:?}", e)
              },
            PickingEvent::Hover(e) => info!("Egads! A hover event!? {:?}", e),
            PickingEvent::Clicked(e) => info!("Gee Willikers, it's a click! {:?}", e),
        }
    }
}