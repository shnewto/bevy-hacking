use bevy::prelude::*;
mod audio;
mod sprites;
mod map;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .insert_resource(audio::ThemeClock(Timer::from_seconds(audio::THEME_LENGTH, true)))
        .insert_resource(ClearColor(Color::rgb(map::BG_RED, map::BG_GREEN, map::BG_BLUE)))
        .init_resource::<audio::Theme>()
        .add_startup_system(sprites::ball_setup.system())
        .add_startup_system(audio::play_theme.system())
        .add_system(audio::repeat_theme.system())
        .run();
}