use crate::{
    configs::{LAYER0, LAYER1},
    enemy::Collider,
    gun::HasLifespan,
    player::{Health, Player},
    resources::{UiFont, Wave},
    ui::components::{PlayerHealthBar, WaveDisplay, WaveDisplayRoot},
    utils::InGameEntity,
};
use bevy::{
    color::Color,
    core::Name,
    hierarchy::{BuildChildren, ChildBuild},
    math::{Vec2, Vec3},
    prelude::*,
};
use std::time::Duration;

pub fn setup_health_bar(mut commands: Commands, player_query: Query<Entity, With<Player>>) {
    if let Ok(player_entity) = player_query.get_single() {
        commands.entity(player_entity).with_children(|parent| {
            parent.spawn((
                Sprite {
                    color: Color::linear_rgb(0.5, 0.5, 0.5),
                    custom_size: Some(Vec2::new(18.0, 4.0)),
                    ..default()
                },
                Transform {
                    translation: Vec3::new(0.0, 16.0, LAYER0),
                    ..default()
                },
            ));

            parent
                .spawn((
                    Sprite {
                        color: Color::linear_rgb(0.0, 1.0, 0.0),
                        custom_size: Some(Vec2::new(18.0, 4.0)),
                        ..default()
                    },
                    Transform {
                        translation: Vec3::new(0.0, 16.0, LAYER1),
                        ..default()
                    },
                ))
                .insert(PlayerHealthBar);
        });
    }
}

pub fn update_health_bar(
    player_query: Query<&Health, With<Player>>,
    mut health_bar_query: Query<(&mut Transform, &mut Sprite), With<PlayerHealthBar>>,
) {
    if let Ok(health) = player_query.get_single() {
        if let Ok((mut transform, mut sprite)) = health_bar_query.get_single_mut() {
            let health_percentage = health.0 as f32 / health.1 as f32;
            sprite.custom_size = Some(Vec2::new(18.0 * health_percentage, 4.0));
            transform.translation.x = -9.0 + (9.0 * health_percentage);
        }
    }
}

pub fn setup_wave_display(
    mut commands: Commands,
    font: Res<UiFont>,
    existing_displays: Query<Entity, With<WaveDisplayRoot>>,
) {
    if !existing_displays.is_empty() {
        return;
    }

    commands
        .spawn((
            Name::new("Wave"),
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                left: Val::Percent(50.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            InGameEntity,
            WaveDisplayRoot,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Wave 1"),
                TextFont {
                    font: font.0.clone(),
                    font_size: 50.0,
                    ..default()
                },
                TextColor::from(Color::WHITE),
                WaveDisplay,
            ));
        });
}

pub fn update_wave_display(mut wave_query: Query<&mut Text, With<WaveDisplay>>, wave: Res<Wave>) {
    if wave.is_changed() {
        if let Ok(mut text) = wave_query.get_single_mut() {
            *text = Text::from(format!("Wave {}", wave.number));
        }
    }
}

pub fn spawn_damage_text(
    commands: &mut Commands,
    font: &Handle<Font>,
    position: Vec3,
    damage: u32,
) {
    commands.spawn((
        Name::new("Damage Text"),
        Text2d::new(format!("-{}", damage)),
        TextFont {
            font: font.clone(),
            font_size: 50.0,
            ..default()
        },
        TextColor(Srgba::new(1.0, 0.0, 0.0, 1.0).into()),
        Transform {
            translation: position + Vec3::new(0.0, 50.0, 0.0),
            ..default()
        },
        Collider { radius: 5 },
        HasLifespan::new(Duration::from_secs(1)),
        InGameEntity,
    ));
}
