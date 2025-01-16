use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioControl, AudioSource};

use crate::{configs::*, game_state::GameState};

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
            .add_systems(Startup, play_background_music)
            .add_systems(Update, play_audio_event.run_if(on_event::<AudioEvent>));
    }
}

fn load_audio_assets(asset_server: Res<AssetServer>, mut commands: Commands) {
    let audio_handles = AudioHandles {
        kill: asset_server.load(AUDIO_KILL_PATH),
        hit: asset_server.load(AUDIO_HIT_PATH),
        fire: asset_server.load(AUDIO_FIRE_PATH),
        ui: asset_server.load(AUDIO_UI_PATH),
        popup: asset_server.load(AUDIO_POPUP_PATH),
        win: asset_server.load(AUDIO_WIN_PATH),
        lose: asset_server.load(AUDIO_LOSE_PATH),
        background: asset_server.load(AUDIO_BACKGROUND_PATH),
        level_up: asset_server.load(AUDIO_LEVEL_UP_PATH),
        pick_up: asset_server.load(AUDIO_PICK_UP_PATH),
    };

    commands.insert_resource(audio_handles);
}

fn play_background_music(audio: Res<Audio>, audio_handles: Res<AudioHandles>) {
    audio
        .play(audio_handles.background.clone())
        .looped()
        .with_volume(0.8);
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
                audio.play(audio_handles.hit.clone()).with_volume(0.1);
            }
            AudioEvent::Fire => {
                audio.play(audio_handles.fire.clone()).with_volume(0.2);
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
