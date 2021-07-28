use bevy::prelude::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .insert_resource(ThemeClock(Timer::from_seconds(8.0, true)))
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .init_resource::<Theme>()
        .add_startup_system(sprite_setup.system())
        .add_startup_system(first_theme.system())
        .add_system(play_theme.system())
        .run();
}

struct Theme {
    track: Handle<AudioSource>,
}

struct ThemeClock(Timer);

impl FromWorld for Theme {
    fn from_world(world: &mut World) -> Self {
        let world = world.cell();
        let asset_server = world.get_resource_mut::<AssetServer>().unwrap();
        Theme { track: asset_server.load("audio/wanderball-overworld-theme.mp3").into() }
    }
}

fn first_theme(audio: Res<Audio>, theme: Res<Theme>,) {
    audio.play(theme.track.clone());
}


fn play_theme(
    time: Res<Time>,
    mut timer: ResMut<ThemeClock>,
    audio: Res<Audio>,
    theme: Res<Theme>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        // theme.0.clone();
        audio.play(theme.track.clone());
    }
}

fn sprite_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("textures/ball.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(4.0, 4.0), 1, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(6.0)),
            ..Default::default()
        });
}
