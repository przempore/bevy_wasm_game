use bevy::{
    color::palettes::basic::{BLUE, LIME},
    prelude::*,
    window::PrimaryWindow,
};
use rand::prelude::*;

use super::{despawn_screen, menu::DisplayQuality, splash::GameState, Volume, TEXT_COLOR};

pub const PLAYER_SPEED: f32 = 500.0;
pub const ENEMY_SPEED: f32 = 200.0;

// This plugin will contain the game. In this case, it's just be a screen that will
// display the current settings for 5 seconds before returning to the menu
pub fn game_plugin(app: &mut App) {
    app//.add_systems(OnEnter(GameState::Game), game_setup)
        .add_systems(OnEnter(GameState::Game), spawn_camera)
        .add_systems(OnEnter(GameState::Game), spawn_player)
        .add_systems(OnEnter(GameState::Game), spawn_enemies)
        // .add_systems(Update, game.run_if(in_state(GameState::Game)))
        .add_systems(Update, player_movement.run_if(in_state(GameState::Game)))
        .add_systems(Update, enemy_movement.run_if(in_state(GameState::Game)))
        .add_systems(OnExit(GameState::Game), despawn_screen::<OnGameScreen>);
}

// Tag component used to tag entities added on the game screen
#[derive(Component)]
struct OnGameScreen;

#[derive(Resource, Deref, DerefMut)]
struct GameTimer(Timer);

fn game_setup(mut commands: Commands, display_quality: Res<DisplayQuality>, volume: Res<Volume>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    // center children
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            OnGameScreen,
        ))
        .with_children(|parent| {
            // First create a `NodeBundle` for centering what we want to display
            parent
                .spawn(NodeBundle {
                    style: Style {
                        // This will display its children in a column, from top to bottom
                        flex_direction: FlexDirection::Column,
                        // `align_items` will align children on the cross axis. Here the main axis is
                        // vertical (column), so the cross axis is horizontal. This will center the
                        // children
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::BLACK.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // Display two lines of text, the second one with the current settings
                    parent.spawn(
                        TextBundle::from_section(
                            "Will be back to the menu shortly...",
                            TextStyle {
                                font_size: 60.0,
                                color: TEXT_COLOR,
                                ..default()
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::all(Val::Px(50.0)),
                            ..default()
                        }),
                    );
                    parent.spawn(
                        TextBundle::from_sections([
                            TextSection::new(
                                format!("quality: {:?}", *display_quality),
                                TextStyle {
                                    font_size: 60.0,
                                    color: BLUE.into(),
                                    ..default()
                                },
                            ),
                            TextSection::new(
                                " - ",
                                TextStyle {
                                    font_size: 60.0,
                                    color: TEXT_COLOR,
                                    ..default()
                                },
                            ),
                            TextSection::new(
                                format!("volume: {:?}", *volume),
                                TextStyle {
                                    font_size: 60.0,
                                    color: LIME.into(),
                                    ..default()
                                },
                            ),
                        ])
                        .with_style(Style {
                            margin: UiRect::all(Val::Px(50.0)),
                            ..default()
                        }),
                    );
                });
        });
    // Spawn a 5 seconds timer to trigger going back to the menu
    let timer = Timer::from_seconds(3.0, TimerMode::Once);
    if timer.just_finished() {
        use web_sys::console;

        console::log_1(&"Timer finished Game".into());
    }
    commands.insert_resource(GameTimer(Timer::from_seconds(3.0, TimerMode::Once)));
}

// Tick the timer, and change state when finished
fn game(
    time: Res<Time>,
    mut game_state: ResMut<NextState<GameState>>,
    mut timer: ResMut<GameTimer>,
) {
    if timer.tick(time.delta()).finished() {
        game_state.set(GameState::Menu);
    }
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

#[derive(Component)]
pub struct Player {}

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            texture: asset_server.load("sprites/ball_blue_large.png"),
            ..default()
        },
        Player {},
    ));
}

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyH) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyL) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyK) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) || keyboard_input.pressed(KeyCode::KeyJ) {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

// pub fn confine_player_movement(
//     mut player_query: Query<(&mut Transform, &Handle<Image>), With<Player>>,
//     window_query: Query<&Window, With<PrimaryWindow>>,
//     images: Res<Assets<Image>>,
// ) {
//     if let Ok((mut player_transform, player_texture)) = player_query.get_single_mut() {
//         let window = window_query.get_single().unwrap();
//
//         if let Some(image) = images.get(player_texture) {
//             let half_player_size = image.size() / 2;
//             let x_min = half_player_size.x;
//         let x_max = window.width() - half_player_size.x as f32;
//         let y_min = half_player_size;
//         let y_max = window.height() - half_player_size as f32;
//         let mut translation = player_transform.translation;
//
//         if translation.x < x_min as f32 {
//             translation.x = x_min as f32;
//         } else if translation.x > x_max as f32 {
//             translation.x = x_max as f32;
//         }
//
//         if translation.y < y_min.y as f32 {
//             translation.y = y_min.y as f32;
//         } else if translation.y > y_max.y {
//             translation.y = y_max.y as f32;
//         }
//
//         player_transform.translation = translation;
//     }
// }
// }

#[derive(Component)]
pub struct Enemy {
    pub direction: Vec2,
}

pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    let half_player_size = 32.0;
    for _ in 0..6 {
        let mut x = random::<f32>() * window.width();
        if x < half_player_size {
            x = half_player_size;
        } else if x > window.width() - half_player_size {
            x = window.width() - half_player_size;
        }

        let mut y = random::<f32>() * window.height();
        if y < half_player_size {
            y = half_player_size;
        } else if y > window.height() - half_player_size {
            y = window.height() - half_player_size;
        }

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(x, y, 0.0),
                texture: asset_server.load("sprites/ball_red_large.png"),
                ..default()
            },
            Enemy {
                direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
            },
        ));
    }
}

pub fn enemy_movement(mut enemy_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
    }
}

// pub fn confine_enemy_movement(
// mut enemy_query: Query<(&mut Transform, &mut Enemy, &Handle<Image>)>,
// window_query: Query<&Window, With<PrimaryWindow>>,
// images: Res<Assets<Image>>,
// // audio: Res<AudioBundle>,
// // asset_server: Res<AssetServer>,
// ) {
// if let Ok(window) = window_query.get_single() {
//     for (transform, mut enemy, image) in enemy_query.iter_mut() {
//         if let Some(image) = images.get(image) {
//             let half_enemy_size = image.size() / 2;
//                 let x_min = half_enemy_size;
//                 let x_max = window.width() - half_enemy_size;
//                 let y_min = half_enemy_size;
//                 let y_max = window.height() - half_enemy_size;
//                 let mut direction_changed = false;
//                 if transform.translation.x < x_min.x || transform.translation.x > x_max.x {
//                     direction_changed = true;
//                     enemy.direction.x *= -1.0;
//                 }
//                 if transform.translation.y < y_min.y || transform.translation.y > y_max.y {
//                     direction_changed = true;
//                     enemy.direction.y *= -1.0;
//                 }
//
//                 // if direction_changed {
//                 //     let sound_effect_1 = asset_server.load("audio/pluck_001.ogg");
//                 //     let sound_effect_2 = asset_server.load("audio/pluck_002.ogg");
//                 //     let sound_effect = if random::<f32>() > 0.5 {
//                 //         sound_effect_1
//                 //     } else {
//                 //         sound_effect_2
//                 //     };
//                 //     audio.play(sound_effect);
//                 // }
//             }
//         }
//     }
// }
