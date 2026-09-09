use bevy::platform::collections::HashMap;
use bevy::prelude::*;

#[derive(Component)]
struct PointsText;

#[derive(Resource)]
struct TetrisRuleset {
    well_size_rows: u32,
    well_size_cols: u32,
    initial_speed: f32,
}

#[derive(Resource)]
struct GameState {
    occupied: Vec<u8>,
    cols: u32,
    points: usize,
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum TetrominoShape {
    Square,
    Stick,
    InverseHook, // J
    Hook,        // L
    InverseSkew, // S
    Skew,        // Z
    Arrow,
}

#[derive(Component)]
struct CurrentShape(TetrominoShape, u8);

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
enum TetrominoColor {
    Orange,
    Blue,
    Cyan,
    Yellow,
    Green,
    Magenta,
    Red,
}

impl TetrominoShape {
    fn to_color(&self) -> TetrominoColor {
        match self {
            TetrominoShape::Arrow => TetrominoColor::Magenta,
            TetrominoShape::Square => TetrominoColor::Yellow,
            TetrominoShape::Stick => TetrominoColor::Cyan,
            TetrominoShape::InverseSkew => TetrominoColor::Green,
            TetrominoShape::Skew => TetrominoColor::Red,
            TetrominoShape::Hook => TetrominoColor::Orange,
            TetrominoShape::InverseHook => TetrominoColor::Blue,
        }
    }

    fn spawn_new(
        &self,
        commands: &mut Commands,
        block_mesh: Handle<Mesh>,
        block_material: Handle<StandardMaterial>,
        ruleset: &TetrisRuleset,
    ) {
        let aspect_ratio = ruleset.well_size_cols as f32 / ruleset.well_size_rows as f32;

        let bound_x = 0.33;
        let bound_y = 0.33 / aspect_ratio;

        match self {
            TetrominoShape::Square => {
                for (current, pos) in [
                    (Current(1, TetrominoShape::Square, 0), TetrisPos(4, 19)),
                    (Current(2, TetrominoShape::Square, 0), TetrisPos(5, 19)),
                    (Current(3, TetrominoShape::Square, 0), TetrisPos(4, 18)),
                    (Current(4, TetrominoShape::Square, 0), TetrisPos(5, 18)),
                ] {
                    commands.spawn((
                        current,
                        pos,
                        Block,
                        Mesh3d(block_mesh.clone()),
                        MeshMaterial3d(block_material.clone()),
                        {
                            let mut transform = Transform::IDENTITY;
                            apply_transform(
                                (bound_x, bound_y),
                                (0.0, 0.0),
                                ruleset.well_size_rows,
                                ruleset.well_size_cols,
                                pos,
                                &mut transform,
                            );
                            transform
                        },
                    ));
                }
            }
            TetrominoShape::Stick => {
                for (current, pos) in [
                    (Current(1, TetrominoShape::Stick, 0), TetrisPos(3, 19)),
                    (Current(2, TetrominoShape::Stick, 0), TetrisPos(4, 19)),
                    (Current(3, TetrominoShape::Stick, 0), TetrisPos(5, 19)),
                    (Current(4, TetrominoShape::Stick, 0), TetrisPos(6, 19)),
                ] {
                    commands.spawn((
                        current,
                        pos,
                        Block,
                        Mesh3d(block_mesh.clone()),
                        MeshMaterial3d(block_material.clone()),
                        {
                            let mut transform = Transform::IDENTITY;
                            apply_transform(
                                (bound_x, bound_y),
                                (0.0, 0.0),
                                ruleset.well_size_rows,
                                ruleset.well_size_cols,
                                pos,
                                &mut transform,
                            );
                            transform
                        },
                    ));
                }
            }
            _ => {
                for (current, pos) in [
                    (Current(1, TetrominoShape::Arrow, 0), TetrisPos(4, 19)),
                    (Current(2, TetrominoShape::Arrow, 0), TetrisPos(3, 18)),
                    (Current(3, TetrominoShape::Arrow, 0), TetrisPos(4, 18)),
                    (Current(4, TetrominoShape::Arrow, 0), TetrisPos(5, 18)),
                ] {
                    commands.spawn((
                        current,
                        pos,
                        Block,
                        Mesh3d(block_mesh.clone()),
                        MeshMaterial3d(block_material.clone()),
                        {
                            let mut transform = Transform::IDENTITY;
                            apply_transform(
                                (bound_x, bound_y),
                                (0.0, 0.0),
                                ruleset.well_size_rows,
                                ruleset.well_size_cols,
                                pos,
                                &mut transform,
                            );
                            transform
                        },
                    ));
                }
            }
        }
    }

    fn rotate_map(map: [u8; 16]) -> [u8; 16] {
        let mut result = [0; 16];

        for row in 0..4 {
            let target_row = &map[row * 4..row * 4 + 4];
            let target_column = [3 - row, 7 - row, 11 - row, 15 - row];

            for (val, idx) in target_row
                .iter()
                .copied()
                .zip(target_column.iter().copied())
            {
                result[idx] = val;
            }
        }

        // rotation can leave empty tailing rows - we always want our tetromino to stick to bottom of the 4x4 grid.
        let tailing_zero_rows = result
            .iter()
            .fold(0usize, |acc, &i| if i != 0 { 0 } else { acc + 1 })
            / 4;

        for i in (0..16).rev() {
            if result[i] != 0 {
                result[i + tailing_zero_rows * 4] = result[i];
                result[i] = 0;
            }
        }

        result
    }

    #[rustfmt::skip]
    fn to_map(&self) -> [u8; 16] {
        match self {
            TetrominoShape::Square => [
                0, 0, 0, 0,
                0, 0, 0, 0,
                1, 2, 0, 0,
                3, 4, 0, 0,
            ],
            TetrominoShape::Arrow => [
                0, 0, 0, 0,
                0, 0, 0, 0,
                0, 1, 0, 0,
                2, 3, 4, 0,
            ],
            TetrominoShape::Hook => [
                0, 0, 0, 0,
                0, 0, 0, 0,
                0, 0, 1, 0,
                2, 3, 4, 0
            ],
            TetrominoShape::InverseHook => [
                0, 0, 0, 0,
                0, 0, 0, 0,
                1, 0, 0, 0,
                2, 3, 4, 0
            ],
            TetrominoShape::Skew => [
                0, 0, 0, 0,
                0, 0, 0, 0,
                1, 2, 0, 0,
                0, 3, 4, 0
            ],
            TetrominoShape::InverseSkew => [
                0, 0, 0, 0,
                0, 0, 0, 0,
                0, 1, 2, 0,
                3, 4, 0, 0
            ],
            TetrominoShape::Stick => [
                0, 0, 0, 0,
                0, 0, 0, 0,
                0, 0, 0, 0,
                1, 2, 3, 4
            ]
        }
    }
}

#[derive(Event)]
struct BlockPlaced;

#[derive(Event)]
struct RotationRequested;

impl GameState {
    fn new(rows: u32, cols: u32) -> Self {
        Self {
            occupied: vec![0; (rows * cols) as usize],
            cols,
            points: 0,
        }
    }

    fn place_block(&mut self, pos: TetrisPos) {
        self.occupied[(pos.1 as u32 * self.cols + pos.0 as u32) as usize] = 1;
    }

    fn colliding(&self, pos: TetrisPos) -> bool {
        self.occupied[(pos.1 as u32 * self.cols + pos.0 as u32) as usize] == 1
    }

    fn full_lines(&self) -> Vec<u32> {
        let rows = self.occupied.len() as u32 / self.cols;

        let mut full_lines = vec![];
        for y in 0..rows {
            let is_full = !self.occupied[(y * self.cols) as usize..]
                .iter()
                .take(self.cols as usize)
                .any(|v| *v == 0);

            if is_full {
                full_lines.push(y);
            }
        }

        full_lines
    }

    fn clear_lines(&mut self) {
        let rows = self.occupied.len() as u32 / self.cols;
        let full_lines = {
            let mut full_lines = self.full_lines();
            full_lines.reverse();
            full_lines
        };

        for y_line in full_lines {
            for y_up in y_line + 1..rows {
                for x in 0..self.cols {
                    self.occupied[((y_up - 1) * self.cols + x) as usize] =
                        self.occupied[(y_up * self.cols + x) as usize];
                }
            }
        }
    }

    fn should_rest(&self, pos: &TetrisPos) -> bool {
        pos.1 == 0 || self.occupied[(((pos.1 as u32 - 1) * self.cols) + pos.0 as u32) as usize] == 1
    }
}

#[derive(Resource)]
struct BlockAssets {
    mesh: Handle<Mesh>,
    materials: HashMap<TetrominoColor, Handle<StandardMaterial>>,
}

impl BlockAssets {
    fn get_material(&self, shape: TetrominoShape) -> Handle<StandardMaterial> {
        self.materials[&shape.to_color()].clone()
    }
}

#[derive(Resource)]
struct GravityTimer(Timer);

#[derive(Resource)]
struct SpawnTimer(Timer);

#[derive(Component)]
struct PlayArea;

#[derive(Component)]
struct Block;

#[derive(Component, Clone, Copy)]
struct TetrisPos(i32, i32);

#[derive(Component)]
struct Current(u8, TetrominoShape, u8);

impl TetrisRuleset {
    fn new(well_size_rows: u32, well_size_cols: u32, initial_speed: f32) -> Self {
        Self {
            well_size_rows,
            well_size_cols,
            initial_speed,
        }
    }
}

impl Default for TetrisRuleset {
    fn default() -> Self {
        TetrisRuleset::new(20, 10, 0.2)
    }
}

fn apply_transform(
    bounds: (f32, f32),
    offset: (f32, f32),
    rows: u32,
    cols: u32,
    pos: TetrisPos,
    transform: &mut Transform,
) {
    let x_len = 0.33 / cols as f32;
    let y_len = 0.33 / cols as f32;

    let x_step = bounds.0 / cols as f32;
    let y_step = bounds.1 / rows as f32;

    let left_s = offset.0 - bounds.0 / 2.0;
    let bottom_s = offset.1 - bounds.1 / 2.0;

    transform.translation.x = left_s + (pos.0 as f32) * x_step + x_len / 2.0;
    transform.translation.y = bottom_s + (pos.1 as f32) * y_step + y_len / 2.0;
    transform.translation.z = -1.0;
}

fn setup_ui(mut commands: Commands) {
    commands.spawn((
        PointsText,
        Text::new("Points: 0"),
        TextShadow::default(),
        TextLayout::justify(Justify::Center),
        Node {
            position_type: PositionType::Absolute,
            top: px(10),
            right: px(10),
            ..default()
        },
    ));
}

fn setup_game_area(
    mut commands: Commands,
    ruleset: Res<TetrisRuleset>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let aspect_ratio = ruleset.well_size_cols as f32 / ruleset.well_size_rows as f32;

    let bound_x = 0.33;
    let bound_y = 0.33 / aspect_ratio;

    let bottom_wall = meshes.add(Cuboid::new(bound_x + 0.02, 0.01, 0.11));
    let side_wall = meshes.add(Cuboid::new(0.01, bound_y + 0.01, 0.11));
    let back_wall = meshes.add(Cuboid::new(bound_x, bound_y, 0.01));

    let white_material = materials.add(StandardMaterial::from_color(Color::WHITE));

    let block_mesh = meshes.add(Cuboid::new(
        0.33 / ruleset.well_size_cols as f32,
        0.33 / ruleset.well_size_cols as f32,
        0.1,
    ));

    commands.spawn(DirectionalLight {
        illuminance: 1000.0,
        ..default()
    });

    commands.spawn((
        PlayArea,
        Transform::from_xyz(0.0, 0.0, -1.0),
        Visibility::default(),
        children![
            (
                Mesh3d(side_wall.clone()),
                MeshMaterial3d(white_material.clone()),
                Transform::from_xyz(-bound_x / 2.0 - 0.01 / 2.0, 0.0, 0.0)
            ),
            (
                Mesh3d(side_wall.clone()),
                MeshMaterial3d(white_material.clone()),
                Transform::from_xyz(bound_x / 2.0 + 0.01, 0.0, 0.0)
            ),
            (
                Mesh3d(back_wall),
                MeshMaterial3d(white_material.clone()),
                Transform::from_xyz(0.0, 0.0, 0.0)
            ),
            (
                Mesh3d(bottom_wall),
                MeshMaterial3d(white_material),
                Transform::from_xyz(0.0, -bound_y / 2.0, 0.0)
            )
        ],
    ));

    commands.spawn((
        Camera3d::default(),
        // Projection::from(OrthographicProjection::default_3d()),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    commands.insert_resource(GravityTimer(Timer::from_seconds(
        ruleset.initial_speed,
        TimerMode::Repeating,
    )));

    commands.insert_resource(SpawnTimer(Timer::from_seconds(
        ruleset.initial_speed,
        TimerMode::Repeating,
    )));

    let orange_material = materials.add(StandardMaterial::from_color(Color::linear_rgb(
        1.0, 0.647, 0.0,
    )));

    let blue_material = materials.add(StandardMaterial::from_color(Color::linear_rgb(
        0.0, 0.0, 1.0,
    )));

    let cyan_material = materials.add(StandardMaterial::from_color(Color::linear_rgb(
        0.0, 1.0, 1.0,
    )));

    let green_material = materials.add(StandardMaterial::from_color(Color::linear_rgb(
        0.0, 1.0, 0.0,
    )));

    let magenta_material = materials.add(StandardMaterial::from_color(Color::linear_rgb(
        1.0, 0.0, 1.0,
    )));

    let red_material = materials.add(StandardMaterial::from_color(Color::linear_rgb(
        1.0, 0.0, 0.0,
    )));

    let yellow_material = materials.add(StandardMaterial::from_color(Color::linear_rgb(
        1.0, 1.0, 0.0,
    )));

    let materials_map = HashMap::from_iter(
        [
            (TetrominoColor::Blue, blue_material.clone()),
            (TetrominoColor::Cyan, cyan_material.clone()),
            (TetrominoColor::Green, green_material.clone()),
            (TetrominoColor::Magenta, magenta_material.clone()),
            (TetrominoColor::Orange, orange_material.clone()),
            (TetrominoColor::Red, red_material.clone()),
            (TetrominoColor::Yellow, yellow_material.clone()),
        ]
        .into_iter(),
    );

    commands.insert_resource(BlockAssets {
        mesh: block_mesh,
        materials: materials_map,
    });

    commands.insert_resource(GameState::new(
        ruleset.well_size_rows,
        ruleset.well_size_cols,
    ));
}

fn gravity_system(
    mut timer: ResMut<GravityTimer>,
    mut commands: Commands,
    time: Res<Time>,
    mut state: ResMut<GameState>,
    query: Query<(Entity, &mut TetrisPos, &mut Transform), (With<Current>, With<Block>)>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        let should_rest = query.iter().any(|(_, pos, _)| state.should_rest(pos));

        if should_rest {
            for (entity, pos, _) in query {
                commands.entity(entity).remove::<Current>();
                state.place_block(*pos);
            }
            commands.trigger(BlockPlaced);
        } else {
            for (_, mut pos, mut transform) in query {
                pos.1 = pos.1.saturating_sub(1);
                apply_transform(
                    (0.33, 0.66),
                    (0.0, 0.0),
                    20,
                    10,
                    *pos.as_mut(),
                    transform.as_mut(),
                );
            }
        }
    }
}

fn spawn_new_piece(
    query: Query<&Current>,
    ruleset: Res<TetrisRuleset>,
    assets: Res<BlockAssets>,
    time: Res<Time>,
    mut spawn_timer: ResMut<SpawnTimer>,
    mut gravity_timer: ResMut<GravityTimer>,
    mut commands: Commands,
) {
    if query.is_empty() && spawn_timer.0.tick(time.delta()).just_finished() {
        let block_mesh = assets.mesh.clone();
        use rand::prelude::*;

        let mut prng = rand::rng();

        let sampled = *[
            TetrominoShape::Arrow,
            TetrominoShape::Stick,
            TetrominoShape::Square,
        ]
        .sample(&mut prng, 1)
        .next()
        .unwrap();

        sampled.spawn_new(
            &mut commands,
            block_mesh,
            assets.get_material(sampled),
            ruleset.as_ref(),
        );

        gravity_timer.0.reset();
    }
}

fn controls(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    ruleset: Res<TetrisRuleset>,
    state: Res<GameState>,
    query: Query<(&mut TetrisPos, &mut Transform), (With<Current>, With<Block>)>,
) {
    let aspect_ratio: f32 = ruleset.well_size_cols as f32 / ruleset.well_size_rows as f32;

    let bound_x = 0.33;
    let bound_y = 0.33 / aspect_ratio;

    if keyboard.just_pressed(KeyCode::KeyA) {
        let not_hitting_wall = query.iter().all(|(pos, _)| pos.0 > 0);

        if not_hitting_wall {
            let not_colliding = !query
                .iter()
                .any(|(pos, _)| state.colliding(TetrisPos(pos.0 - 1, pos.1)));

            if not_colliding {
                for (mut pos, mut transform) in query {
                    pos.0 -= 1;
                    apply_transform(
                        (bound_x, bound_y),
                        (0.0, 0.0),
                        ruleset.well_size_rows,
                        ruleset.well_size_cols,
                        *pos,
                        transform.as_mut(),
                    );
                }
            }
        }
    } else if keyboard.just_pressed(KeyCode::KeyD) {
        let not_hitting_wall = !query
            .iter()
            .any(|(pos, _)| pos.0 as u32 == ruleset.well_size_cols - 1);

        if not_hitting_wall {
            let not_colliding = !query
                .iter()
                .any(|(pos, _)| state.colliding(TetrisPos(pos.0 + 1, pos.1)));

            if not_colliding {
                for (mut pos, mut transform) in query {
                    pos.0 += 1;
                    apply_transform(
                        (bound_x, bound_y),
                        (0.0, 0.0),
                        ruleset.well_size_rows,
                        ruleset.well_size_cols,
                        *pos,
                        transform.as_mut(),
                    );
                }
            }
        }
    } else if keyboard.just_pressed(KeyCode::KeyW) {
        commands.trigger(RotationRequested);
    }
}

const POINTS_PER_CLEAR: usize = 100;

fn clear_lines(
    _event: On<BlockPlaced>,
    ruleset: Res<TetrisRuleset>,
    mut state: ResMut<GameState>,
    mut tiles: Query<(Entity, &mut TetrisPos, &mut Transform), (With<Block>, Without<Current>)>,
    mut commands: Commands,
) {
    let crash_lines = {
        let mut full_lines = state.full_lines();
        full_lines.reverse();
        full_lines
    };

    state.points += crash_lines.len() * POINTS_PER_CLEAR;

    for (entity, mut pos, mut transform) in tiles.iter_mut() {
        if crash_lines.contains(&(pos.1 as u32)) {
            commands.entity(entity).despawn();
        } else {
            let aspect_ratio = ruleset.well_size_cols as f32 / ruleset.well_size_rows as f32;

            let bound_x = 0.33;
            let bound_y = 0.33 / aspect_ratio;

            let y = pos.1;
            let should_fall = crash_lines
                .iter()
                .filter(|&crash_y| *crash_y < y as u32)
                .count();

            if should_fall > 0 {
                pos.1 = pos.1.saturating_sub(should_fall as i32);
                apply_transform(
                    (bound_x, bound_y),
                    (0.0, 0.0),
                    ruleset.well_size_rows,
                    ruleset.well_size_cols,
                    *pos,
                    transform.as_mut(),
                );
            }
        }
    }

    state.clear_lines();
}

fn update_ui(mut pts_text: Query<&mut Text, With<PointsText>>, state: Res<GameState>) {
    let mut pts_text = pts_text.single_mut().unwrap();
    pts_text.0 = format!("Points: {}", state.points);
}

fn rotate_current(
    _event: On<RotationRequested>,
    ruleset: Res<TetrisRuleset>,
    mut query: Query<(&mut TetrisPos, &mut Transform, &mut Current)>,
) {
    let aspect_ratio: f32 = ruleset.well_size_cols as f32 / ruleset.well_size_rows as f32;

    let bound_x = 0.33;
    let bound_y = 0.33 / aspect_ratio;

    let mut new_positions = HashMap::with_capacity(4);
    for (tetris_pos, _, current) in query.iter() {
        let Current(idx, shape, current_rotation) = current;

        let adjustment = match *shape {
            TetrominoShape::Arrow => {
                if *current_rotation == 0 {
                    [(0, -1), (0, 1), (-1, 0), (-2, -1)]
                } else if *current_rotation == 1 {
                    [(0, 0), (0, 0), (1, 1), (2, 2)]
                } else if *current_rotation == 2 {
                    [(0, 0), (2, 0), (1, -1), (0, -2)]
                } else {
                    [(0, 1), (-2, -1), (-1, 0), (0, 1)]
                }
            }
            TetrominoShape::Stick => {
                if *current_rotation % 2 == 0 {
                    [(2, 0), (1, -1), (0, -2), (-1, -3)]
                } else {
                    [(-2, 0), (-1, 1), (0, 2), (1, 3)]
                }
            }
            TetrominoShape::Square => [(0, 0), (0, 0), (0, 0), (0, 0)],
            _ => {
                unimplemented!();
            }
        }[(*idx - 1) as usize];

        let mut new_tetris_pos = tetris_pos.clone();
        new_tetris_pos.0 += adjustment.0;
        new_tetris_pos.1 += adjustment.1;

        if !((0..ruleset.well_size_cols as i32).contains(&new_tetris_pos.0)
            && (0..ruleset.well_size_rows as i32).contains(&new_tetris_pos.1))
        {
            return;
        } else {
            new_positions.insert(*idx, new_tetris_pos);
        }
    }

    for (mut tetris_pos, mut transform, mut current) in query.iter_mut() {
        let Current(idx, _, _) = current.as_mut();

        let new_tetris_pos = new_positions.get(idx).unwrap();

        tetris_pos.0 = new_tetris_pos.0;
        tetris_pos.1 = new_tetris_pos.1;

        apply_transform(
            (bound_x, bound_y),
            (0.0, 0.0),
            ruleset.well_size_rows,
            ruleset.well_size_cols,
            *tetris_pos,
            transform.as_mut(),
        );

        current.2 += 1;
        current.2 %= 4;
    }
}

fn main() {
    App::new()
        .insert_resource(TetrisRuleset::default())
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup_game_area, setup_ui))
        .add_systems(Update, controls)
        .add_systems(
            Update,
            (update_ui, (spawn_new_piece, gravity_system).chain()),
        )
        .add_observer(clear_lines)
        .add_observer(rotate_current)
        .run();
}
