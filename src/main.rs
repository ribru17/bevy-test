use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;
// use bevy::winit::WinitSettings;

const SPEED: f32 = 0.1;
const SENS: f32 = 0.00012;
const RENDER_DISTANCE: f32 = 50.0;

// Keeps track of pitch and yaw
#[derive(Default)]
struct InputState {
    pitch: f32,
    yaw: f32,
}

fn main() {
    App::new()
        .init_resource::<InputState>()
        // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
        // .insert_resource(WinitSettings::desktop_app())
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_startup_system(initial_grab_cursor)
        .add_system(button_system)
        .add_system(move_cam)
        .add_system(enforce_render_distance)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    mut windows: ResMut<Windows>,
) {
    if let Some(window) = windows.get_primary_mut() {
        window.set_title("RB GAME".to_string());

    }

    // plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
    // cube
    // commands.spawn_bundle(PbrBundle {
    //     mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
    //     material: materials.add(Color::rgb(0.8, 0.1, 0.1).into()),
    //     transform: Transform::from_xyz(0.0, 0.5, 0.0),
    //     ..default()
    // });
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::UVSphere { radius: 1.0, sectors: 36, stacks: 36 })),
        material: materials.add(Color::rgb(0.0, 0.0, 0.6).into()),
        transform: Transform::from_xyz(0.0, 1.0, 0.0),
        ..default()
    });
    // light
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    // ui
    commands.spawn_bundle(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            // size: Size::new(Val::Percent(100.0), Val::Auto),
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            // display: Display::None,
            ..default()
        },
        visibility: Visibility { is_visible: false },
        color: Color::NONE.into(),
        ..default()
    })
    .with_children(|parent| {
        parent.spawn_bundle(NodeBundle {
            style: Style {
                // size: Size::new(Val::Percent(100.0), Val::Px(100.0)),
                size: Size::new(Val::Percent(100.0), Val::Auto),
                border: UiRect::all(Val::Px(2.0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::ColumnReverse,
                // position: UiRect::all(Val::Px(0.0)),
                ..default()
            },
            // visibility: Visibility { is_visible: false },
            color: Color::rgb(0.0, 0.5, 0.0).into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle::from_section(
                "Paused",
                TextStyle {
                    font: asset_server.load("open-sans/OpenSans-Bold.ttf"),
                    font_size: 50.0,
                    color: Color::WHITE,
                    
                },
                ).with_style(Style {
                    margin: UiRect::all(Val::Px(5.0)),
                    align_self: AlignSelf::Center,
                    ..default()
                }),
            );
            
            parent.spawn_bundle(TextBundle::from_section(
                " ",
                TextStyle {
                    font: asset_server.load("open-sans/OpenSans-Bold.ttf"),
                    font_size: 50.0,
                    color: Color::WHITE,
                },
                ).with_style(Style {
                    margin: UiRect::all(Val::Px(5.0)),
                    align_self: AlignSelf::Center,
                    ..default()
                })
            );

            parent.spawn_bundle(ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                color: Color::GRAY.into(),
                // visibility: Visibility { is_visible: false },
                ..default()
            })
            .with_children(|parent| {
                parent.spawn_bundle(TextBundle::from_section(
                    "Quit",
                    TextStyle {
                        font: asset_server.load("open-sans/OpenSans-Bold.ttf"),
                        font_size: 30.0,
                        color: Color::WHITE,
                    }
                ));
                
            });

        });

    });

}

fn move_cam(
    mut query: Query<(&Camera3d, &mut Transform)>,
    mut menu_query: Query<(&Node, &mut Visibility), Without<Parent>>,
    keys: Res<Input<KeyCode>>,
    mut motion_evr: EventReader<MouseMotion>,
    mut windows: ResMut<Windows>,
    mut state: ResMut<InputState>,
) {
    let win: &mut Window;

    if let Some(window) = windows.get_primary_mut() {
        win = window;
    } else {
        return;
    }
    // for (_camera, mut transform) in query.iter_mut() {
    let (_camera, mut transform) = query.single_mut();

        if keys.pressed(KeyCode::W) {
            let vec = transform.forward();

            transform.translation += vec.normalize() * SPEED;

        }

        if keys.pressed(KeyCode::S) {
            let vec = transform.forward();
            transform.translation -= vec.normalize() * SPEED;

        }

        if keys.pressed(KeyCode::D) {
            let vec = transform.right();
            transform.translation += vec.normalize() * SPEED;

        }

        if keys.pressed(KeyCode::A) {
            let vec = transform.right();
            transform.translation -= vec.normalize() * SPEED;

        }

        if keys.pressed(KeyCode::Space) {
            let vec = transform.up();
            // let vec = Vec3::Y;
            transform.translation += vec.normalize() * SPEED;

        }

        if keys.pressed(KeyCode::LShift) {
            let vec = transform.up();
            transform.translation -= vec.normalize() * SPEED;

        }

        if keys.just_pressed(KeyCode::Escape) {
            toggle_grab_cursor(win);
            toggle_pause_menu(&mut menu_query);
        }

        let mut delta_state = state.as_mut();
        for ev in motion_evr.iter() {
            if win.cursor_locked() {
                // Using smallest of height or width ensures equal vertical and horizontal sensitivity
                let window_scale = win.height().min(win.width());
                delta_state.pitch -=
                    (SENS * ev.delta.y * window_scale).to_radians();
                delta_state.yaw -=
                    (SENS * ev.delta.x * window_scale).to_radians();
            }

            delta_state.pitch = delta_state.pitch.clamp(-1.54, 1.54);

            // Order is important to prevent unintended roll
            transform.rotation = Quat::from_axis_angle(Vec3::Y, delta_state.yaw)
                * Quat::from_axis_angle(Vec3::X, delta_state.pitch);
        }
    // }
}

fn toggle_grab_cursor(window: &mut Window) {
    window.set_cursor_lock_mode(!window.cursor_locked());
    window.set_cursor_visibility(!window.cursor_visible());

    if window.cursor_visible() {
        let xmid = window.width() / 2.0;
        let ymid = window.height() / 2.0;

        window.set_cursor_position(Vec2::new(xmid, ymid));
    }

}

fn initial_grab_cursor(mut windows: ResMut<Windows>) {
    if let Some(window) = windows.get_primary_mut() {
        toggle_grab_cursor(window)
    }
}

fn toggle_pause_menu(
    query: &mut Query<(&Node, &mut Visibility), Without<Parent>>,
) {
    // for (_node, mut visibility) in query.iter_mut().nth(0) {
    for (_node, mut visibility) in query.iter_mut() {
        visibility.is_visible = !visibility.is_visible;
        println!("ran");

    }
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut windows: ResMut<Windows>,
) {
    for (interaction, mut color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                text.sections[0].value = "Quitting".to_string();
                *color = Color::BLACK.into();
                // println!("click");
                
                if let Some(window) = windows.get_primary_mut() {
                    window.close();
                }
            }
            Interaction::Hovered => {
                text.sections[0].value = "Quit".to_string();
                *color = Color::ORANGE_RED.into();
            }
            Interaction::None => {
                text.sections[0].value = "Quit".to_string();
                *color = Color::GRAY.into();

            }
        }
    }
}

fn enforce_render_distance(
    query: Query<(Entity, &Transform), (Without<PointLight>, Without<Node>, Without<Camera3d>)>,
    cam_query: Query<(&Camera3d, &Transform)>,
    mut commands: Commands,
) {
    let (_cam, cam_transform) = cam_query.single();

    for (e, transform) in query.iter() {
        let diff = transform.translation + cam_transform.translation;
        if diff.length() > RENDER_DISTANCE {
            commands.entity(e).despawn();

        }
    }
}
