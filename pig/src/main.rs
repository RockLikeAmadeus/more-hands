use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin, EguiPrimaryContextPass};
use my_library::{RandomNumberGenerator, RandomPlugin};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, Default, States)]
enum GamePhase {
    #[default]
    Player,
    Cpu,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin::default())
        .add_plugins(RandomPlugin)

        .add_systems(Startup, setup)
        .init_state::<GamePhase>()
        .add_systems(EguiPrimaryContextPass, display_score)
        .add_systems(EguiPrimaryContextPass, player.run_if(in_state(GamePhase::Player)))
        .add_systems(Update, cpu.run_if(in_state(GamePhase::Cpu)))
        .run();
}

#[derive(Resource)]
struct GameAssets {
    dice_image: Handle<Image>,
    dice_layout: Handle<TextureAtlasLayout>,
}

#[derive(Clone, Copy, Resource, Default)]
struct Scores {
    player: usize,
    cpu: usize,
}

#[derive(Component)]
struct HandDie;

#[derive(Resource)]
struct HandTimer(Timer); // Wrap Timer in a Bevy resource

fn setup(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(Camera2d::default());

    let dice_sprite_sheet = asset_server.load("dice.png");
    let dice_layout = TextureAtlasLayout::from_grid(UVec2::splat(52), 6, 1, None, None);
    let dice_texture_atlas_layout = texture_atlas_layouts.add(dice_layout);

    commands.insert_resource(GameAssets {
        dice_image: dice_sprite_sheet,
        dice_layout: dice_texture_atlas_layout
    });
    commands.insert_resource(Scores { cpu: 0, player: 0 });
    commands.insert_resource(HandTimer(Timer::from_seconds(0.5, TimerMode::Repeating)));
}

fn display_score(
    scores: Res<Scores>,
    mut egui_context: EguiContexts,
) {
    egui::Window::new("Total Scores").show(egui_context.ctx_mut().unwrap(), |ui| {
        ui.label(&format!("Player: {}", scores.player));
        ui.label(&format!("CPU: {}", scores.cpu));
    });
}

fn spawn_die(
    hand_query: &Query<(Entity, &Sprite), With<HandDie>>,
    commands: &mut Commands,
    assets: &GameAssets,
    new_roll: usize,
    color: Color,
) {
    let size_of_die_plus_padding_in_pixels = 52.0;
    let pixel_count_of_rolled_die
        = hand_query.iter().count() as f32 * size_of_die_plus_padding_in_pixels;

    let mut sprite = Sprite::from_atlas_image(
        assets.dice_image.clone(),
        TextureAtlas {
            layout: assets.dice_layout.clone(),
            index: new_roll - 1,
        }
    );
    sprite.color = color;

    commands.spawn((
        sprite,
        Transform::from_xyz(pixel_count_of_rolled_die - 400.0, 60.0, 1.0),
        HandDie
    ));
}

fn clear_die(
    hand_query: &Query<(Entity, &Sprite), With<HandDie>>,
    commands: &mut Commands,
) {
    hand_query
        .iter()
        .for_each(|(die, _)| commands.entity(die).despawn());
}

fn player(
    hand_query: Query<(Entity, &Sprite), With<HandDie>>,
    mut commands: Commands,
    rng: Res<RandomNumberGenerator>,
    assets: Res<GameAssets>,
    mut scores: ResMut<Scores>,
    mut state: ResMut<NextState<GamePhase>>,
    mut egui_context: EguiContexts,
) {
    egui::Window::new("Play Options").show(egui_context.ctx_mut().unwrap(), |ui| {
        let hand_score: usize =
            hand_query.iter().map(|(_, ts)| ts.texture_atlas.as_ref().unwrap().index + 1).sum();
        ui.label(&format!("Score for this hand: {hand_score}"));

        if ui.button("Roll Dice").clicked() {
            let new_roll = rng.val_in_range(1..7);
            if new_roll == 1 {
                // End turn!
                clear_die(&hand_query, &mut commands);
                state.set(GamePhase::Cpu);
            } else {
                spawn_die(
                    &hand_query,
                    &mut commands,
                    &assets,
                    new_roll,
                    Color::WHITE,
                );
            }
        }

        if ui.button("Pass - Keep Hand Score").clicked() {
            let hand_total: usize =
                hand_query.iter().map(|(_, ts)| ts.texture_atlas.as_ref().unwrap().index + 1).sum();
            scores.player += hand_total;
            clear_die(&hand_query, &mut commands);
            state.set(GamePhase::Cpu);
        }
    });
}

fn cpu(
    hand_query: Query<(Entity, &Sprite), With<HandDie>>,
    mut state: ResMut<NextState<GamePhase>>,
    mut scores: ResMut<Scores>,
    rng: Res<RandomNumberGenerator>,
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut timer: ResMut<HandTimer>,
    time: Res<Time>,
) {
    timer.0.tick(time.delta());
    if timer.0.just_finished() {
        let hand_total: usize
            = hand_query.iter().map(|(_, ts)| ts.texture_atlas.as_ref().unwrap().index + 1).sum();
        if hand_total < 20 && scores.cpu + hand_total < 100 {
            let new_roll = rng.val_in_range(1..=6);
            if new_roll == 1 {
                clear_die(&hand_query, &mut commands);
                state.set(GamePhase::Player);
            } else {
                spawn_die(
                    &hand_query,
                    &mut commands,
                    &assets,
                    new_roll as usize,
                    Color::Srgba(Srgba::new(0.0, 0.0, 1.0, 1.0))
                );
            }
        } else {
            scores.cpu += hand_total;
            state.set(GamePhase::Player);
            hand_query
                .iter()
                .for_each(|(entity, _)| commands.entity(entity).despawn());
        }
    }
}