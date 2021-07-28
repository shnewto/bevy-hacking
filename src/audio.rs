use bevy::prelude::*;

pub const THEME_LENGTH: f32 = 11.5;

pub struct Theme {
    track: Handle<AudioSource>,
}

pub struct ThemeClock(pub Timer);

impl FromWorld for Theme {
    fn from_world(world: &mut World) -> Self {
        let world = world.cell();
        let asset_server = world.get_resource_mut::<AssetServer>().unwrap();
        Theme { track: asset_server.load("audio/wanderball-overworld-theme.mp3").into() }
    }
}

pub fn play_theme(audio: Res<Audio>, theme: Res<Theme>) {
    audio.play(theme.track.clone());
}


pub fn repeat_theme(
    time: Res<Time>,
    mut timer: ResMut<ThemeClock>,
    audio: Res<Audio>,
    theme: Res<Theme>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        play_theme(audio, theme);
    }
}
