use crate::portal::Portal;
use crate::resources::Wave;
use crate::state::GameState;
use crate::UiFont;
use bevy::prelude::*;

pub struct DialogPlugin;

impl Plugin for DialogPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ShowDialogEvent>()
            .add_event::<CloseDialogEvent>()
            .add_systems(
                Update,
                (
                    show_dialog,
                    close_dialog,
                    handle_dialog_navigation,
                    handle_dialog_confirmation,
                )
                    .run_if(in_state(GameState::Combat).or_else(in_state(GameState::Paused))),
            )
            .init_resource::<ActiveDialog>()
            .init_resource::<SelectedOption>();
    }
}

#[derive(Resource, Default)]
pub struct ActiveDialog(pub Option<Entity>);

#[derive(Resource, Default)]
pub struct SelectedOption(pub usize);

#[derive(Component)]
pub struct DialogBox;

#[derive(Component, Clone)]
pub enum DialogType {
    Portal,
}

#[derive(Component)]
pub enum DialogButton {
    TravelToTown,
    StartNewWaveCycle,
}

#[derive(Event)]
pub struct ShowDialogEvent(pub DialogType);

#[derive(Event)]
pub struct CloseDialogEvent;

fn show_dialog(
    mut commands: Commands,
    mut ev_show_dialog: EventReader<ShowDialogEvent>,
    active_dialog: Res<ActiveDialog>,
    font: Res<UiFont>,
    mut selected_option: ResMut<SelectedOption>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for event in ev_show_dialog.read() {
        if active_dialog.0.is_none() {
            selected_option.0 = 0;
            let dialog_entity = commands
                .spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            height: Val::Px(200.0),
                            position_type: PositionType::Absolute,
                            bottom: Val::Px(0.0),
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        background_color: Color::srgba(0.1, 0.1, 0.1, 0.9).into(),
                        ..default()
                    },
                    DialogBox,
                    event.0.clone(),
                ))
                .id();

            match event.0 {
                DialogType::Portal => spawn_portal_dialog(&mut commands, dialog_entity, &font.0),
            }
            next_state.set(GameState::Paused);
            commands.insert_resource(ActiveDialog(Some(dialog_entity)));
        }
    }
}

fn spawn_portal_dialog(commands: &mut Commands, parent: Entity, handle: &Handle<Font>) {
    commands.entity(parent).with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            "Portal Options",
            TextStyle {
                font: handle.clone(),
                font_size: 24.0,
                color: Color::WHITE,
            },
        ));
        parent
            .spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(200.0),
                        height: Val::Px(50.0),
                        margin: UiRect::all(Val::Px(10.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: BackgroundColor(Color::srgb(0.5, 0.5, 0.5)),
                    ..default()
                },
                DialogButton::TravelToTown,
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Travel to NPC town",
                    TextStyle {
                        font: handle.clone(),
                        font_size: 18.0,
                        color: Color::WHITE,
                    },
                ));
            });
        parent
            .spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(200.0),
                        height: Val::Px(50.0),
                        margin: UiRect::all(Val::Px(10.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: BackgroundColor(Color::srgb(0.5, 0.5, 0.5)),
                    ..default()
                },
                DialogButton::StartNewWaveCycle,
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Start new wave cycle",
                    TextStyle {
                        font: handle.clone(),
                        font_size: 18.0,
                        color: Color::WHITE,
                    },
                ));
            });
    });
}

fn close_dialog(
    mut commands: Commands,
    mut ev_close_dialog: EventReader<CloseDialogEvent>,
    active_dialog: Res<ActiveDialog>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if !ev_close_dialog.is_empty() {
        ev_close_dialog.clear();
        if let Some(entity) = active_dialog.0 {
            commands.entity(entity).despawn_recursive();
            commands.insert_resource(ActiveDialog(None));
            next_state.set(GameState::Combat);
        }
    }
}

fn handle_dialog_navigation(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut selected_option: ResMut<SelectedOption>,
    mut query: Query<(&DialogButton, &mut BackgroundColor)>,
) {
    if keyboard_input.just_pressed(KeyCode::ArrowUp)
        || keyboard_input.just_pressed(KeyCode::ArrowDown)
    {
        selected_option.0 = 1 - selected_option.0;
    }

    for (i, (_, mut color)) in query.iter_mut().enumerate() {
        if i == selected_option.0 {
            *color = BackgroundColor(Color::srgb(0.0, 0.8, 0.0));
        } else {
            *color = BackgroundColor(Color::srgb(0.5, 0.5, 0.5));
        }
    }
}

fn handle_dialog_confirmation(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    selected_option: Res<SelectedOption>,
    active_dialog: Res<ActiveDialog>,
    dialog_type_query: Query<&DialogType>,
    mut ev_close_dialog: EventWriter<CloseDialogEvent>,
    mut wave: ResMut<Wave>,
    mut next_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
    portal_query: Query<Entity, With<Portal>>,
) {
    if keyboard_input.just_pressed(KeyCode::Enter) {
        if let Some(dialog_entity) = active_dialog.0 {
            if let Ok(dialog_type) = dialog_type_query.get(dialog_entity) {
                match dialog_type {
                    DialogType::Portal => handle_portal_dialog_confirmation(
                        selected_option.0,
                        &mut ev_close_dialog,
                        &mut wave,
                        &mut next_state,
                        &mut commands,
                        &portal_query,
                    ),
                }
            }
        }
    }
}

fn handle_portal_dialog_confirmation(
    selected_option: usize,
    ev_close_dialog: &mut EventWriter<CloseDialogEvent>,
    wave: &mut ResMut<Wave>,
    next_state: &mut ResMut<NextState<GameState>>,
    commands: &mut Commands,
    portal_query: &Query<Entity, With<Portal>>,
) {
    match selected_option {
        0 => {
            ev_close_dialog.send(CloseDialogEvent);
            next_state.set(GameState::Town);
        }
        1 => {
            ev_close_dialog.send(CloseDialogEvent);
            wave.number += 1;
            wave.enemies_total = crate::utils::calculate_enemies_per_wave(wave.number);
            wave.enemies_left = wave.enemies_total;
            wave.enemies_spawned = 0;
            wave.requires_portal = false;
            wave.portal_spawned = false;

            for portal_entity in portal_query.iter() {
                commands.entity(portal_entity).despawn();
            }
        }
        _ => {}
    }
}
