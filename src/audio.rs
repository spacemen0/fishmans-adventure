use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioControl, AudioSource};

use crate::game_state::GameState;

pub struct GameAudioPlugin;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Event)]
pub enum AudioEvent {
    Kill,
    Hit,
    Fire,
    UI,
    PopUp,
    Win,
    Lose,
    LevelUp,
    PickUp,
}

#[derive(Resource)]
struct AudioHandles {
    kill: Handle<AudioSource>,
    hit: Handle<AudioSource>,
    fire: Handle<AudioSource>,
    ui: Handle<AudioSource>,
    popup: Handle<AudioSource>,
    win: Handle<AudioSource>,
    lose: Handle<AudioSource>,
    background: Handle<AudioSource>,
    level_up: Handle<AudioSource>,
    pick_up: Handle<AudioSource>,
}

impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AudioEvent>()
            .add_systems(OnEnter(GameState::Loading), load_audio_assets)
            .add_systems(OnEnter(GameState::Combat), play_background_music)
            .add_systems(Update, play_audio_event.run_if(on_event::<AudioEvent>));
    }
}

fn load_audio_assets(asset_server: Res<AssetServer>, mut commands: Commands) {
    let audio_handles = AudioHandles {
        kill: asset_server.load("audio/kill.ogg"),
        hit: asset_server.load("audio/hit.ogg"),
        fire: asset_server.load("audio/fire.ogg"),
        ui: asset_server.load("audio/ui.ogg"),
        popup: asset_server.load("audio/popup.ogg"),
        win: asset_server.load("audio/win.ogg"),
        lose: asset_server.load("audio/lose.ogg"),
        background: asset_server.load("audio/background.ogg"),
        level_up: asset_server.load("audio/level_up.ogg"),
        pick_up: asset_server.load("audio/pick_up.ogg"),
    };

    commands.insert_resource(audio_handles);
}

fn play_background_music(audio: Res<Audio>, audio_handles: Res<AudioHandles>) {
    audio
        .play(audio_handles.background.clone())
        .looped()
        .with_volume(1.2);
}

fn play_audio_event(
    audio: Res<Audio>,
    audio_handles: Res<AudioHandles>,
    mut event_reader: EventReader<AudioEvent>,
) {
    for event in event_reader.read() {
        match event {
            AudioEvent::Kill => {
                audio.play(audio_handles.kill.clone()).with_volume(0.6);
            }
            AudioEvent::Hit => {
                audio.play(audio_handles.hit.clone()).with_volume(0.05);
            }
            AudioEvent::Fire => {
                audio.play(audio_handles.fire.clone()).with_volume(0.3);
            }
            AudioEvent::UI => {
                audio.play(audio_handles.ui.clone());
            }
            AudioEvent::PopUp => {
                audio.play(audio_handles.popup.clone());
            }
            AudioEvent::Win => {
                audio.play(audio_handles.win.clone());
            }
            AudioEvent::Lose => {
                audio.play(audio_handles.lose.clone());
            }
            AudioEvent::LevelUp => {
                audio.play(audio_handles.level_up.clone()).with_volume(1.2);
            }
            AudioEvent::PickUp => {
                audio.play(audio_handles.pick_up.clone()).with_volume(0.8);
            }
        };
    }
}
