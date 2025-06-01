use bevy::{
    app::{Plugin, Update},
    asset::Handle,
    color::Color,
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    ecs::{
        component::Component, entity::Entity, query::With, resource::Resource, schedule::IntoScheduleConfigs, system::{Commands, Local, Query, Res, ResMut}
    },
    image::Image,
    log::info,
    math::Vec3,
    sprite::Sprite,
    state::{
        condition::in_state,
        state::{NextState, OnEnter, OnExit, States},
    },
    time::Time,
};
use bevy_asset_loader::asset_collection::AssetCollection;
use iyes_progress::{Progress, ProgressReturningSystem, ProgressTracker};

use crate::world::Position;

#[derive(States, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub enum GameState {
    #[default]
    AssetLoading,
    LaunchGame,
    Playing,
}

#[derive(Resource, AssetCollection)]
pub struct LaunchAssets {
    #[asset(path = "venture_time.png")]
    pub logo: Handle<Image>,
}

#[derive(Component)]
pub struct Logo;

// Component to track logo animation state
#[derive(Component)]
pub struct LogoAnimation {
    pub timer: f32,
    pub duration: f32,
    pub initial_scale: Vec3,
    pub target_scale: Vec3,
    pub initial_alpha: f32,
    pub target_alpha: f32,
}

impl Default for LogoAnimation {
    fn default() -> Self {
        Self {
            timer: 0.0,
            duration: 1.0, // Animation duration in seconds
            initial_scale: Vec3::splat(1.0),
            target_scale: Vec3::splat(10.0), // Scale up to 3x
            initial_alpha: 1.0,
            target_alpha: 0.0, // Fade out completely
        }
    }
}

pub fn show_loading_screen(mut commands: Commands, launch_assets: Res<LaunchAssets>) {
    commands.spawn((
        Logo,
        Position::default(),
        Sprite::from_image(launch_assets.logo.clone()),
    ));
}

pub fn prepare_logo_animation(
    mut commands: Commands,
    logo_query: Query<Entity, With<Logo>>,
) {
    for entity in logo_query.iter() {
        commands.entity(entity).insert(LogoAnimation::default());
    }
}

pub fn animate_logo(
    time: Res<Time>,
    mut commands: Commands,
    mut logo_query: Query<(Entity, &mut LogoAnimation, &mut Position, &mut Sprite), With<Logo>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (entity, mut animation, mut position, mut sprite) in &mut logo_query {
        // Update timer
        animation.timer += time.delta_secs();
        let progress = (animation.timer / animation.duration).min(1.0);
        
        // Calculate new scale using linear interpolation
        let scale_factor = animation.initial_scale.lerp(animation.target_scale, progress);
        position.scale = scale_factor;
        
        // Calculate new alpha using linear interpolation
        let alpha = (animation.target_alpha - animation.initial_alpha).mul_add(progress, animation.initial_alpha);
        
        // Apply alpha to sprite
        sprite.color = Color::srgba(1.0, 1.0, 1.0, alpha);
        
        // If animation is complete, remove the animation component
        if progress >= 1.0_f32 {
            commands.entity(entity).remove::<LogoAnimation>();
            next_state.set(GameState::Playing);
        }
    }
}

pub fn hide_loading_screen(mut commands: Commands, loading_screen: Query<Entity, With<Logo>>) {
    for entity in loading_screen.iter() {
        commands.entity(entity).despawn();
    }
}

// Time in seconds to complete a custom long-running task.
// If assets are loaded earlier, the current state will not
// be changed until the 'fake long task' is completed (thanks to 'iyes_progress')
const DURATION_LONG_TASK_IN_SECS: f64 = 2.0;

fn track_fake_long_task(time: Res<Time>) -> Progress {
    if time.elapsed_secs_f64() > DURATION_LONG_TASK_IN_SECS {
        true.into()
    } else {
        false.into()
    }
}

fn print_progress(
    progress: Res<ProgressTracker<GameState>>,
    diagnostics: Res<DiagnosticsStore>,
    mut last_done: Local<u32>,
) {
    let progress = progress.get_global_progress();
    if progress.done > *last_done {
        *last_done = progress.done;
        info!(
            "[Frame {}] Changed progress: {:?}",
            diagnostics
                .get(&FrameTimeDiagnosticsPlugin::FRAME_COUNT)
                .map_or(0., |diagnostic| diagnostic.value().unwrap_or(0.)),
            progress
        );
    }
}

pub struct LoadingSequencePlugin;

impl Plugin for LoadingSequencePlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(OnEnter(GameState::AssetLoading), show_loading_screen)
            .add_systems(
                Update,
                (
                    track_fake_long_task.track_progress::<GameState>(),
                    print_progress,
                )
                    .chain()
                    .run_if(in_state(GameState::AssetLoading)),
            )
            .add_systems(OnExit(GameState::AssetLoading), prepare_logo_animation)
            .add_systems(
                Update,
                animate_logo.run_if(in_state(GameState::LaunchGame)),
            )
            .add_systems(OnExit(GameState::LaunchGame), hide_loading_screen);
    }
}
