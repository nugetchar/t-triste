use bevy::math::{vec2, vec3, Vec2};
use bevy::prelude::*;

use crate::{cursor::Cursor, piece_builder::SQUARE_WIDTH};

trait Piece2 {
    fn rotate(&mut self);
    fn move_it(&mut self, cursor: &Res<Cursor>);
}

struct Rect {
    positions: Vec<Vec2>,
}

struct PositionMarker;

impl Rect {
    pub fn new(positions: Vec<Vec2>) -> Self {
        if positions.is_empty() {
            panic!("WTF, insert positions please")
        }
        // ÇA MARCHE !
        // T'es le meilleur, mec
        // C'est incroyable
        // une telle abnégation
        // fais moi l'amour
        // marie-moi 6 AOUT 2022 J'AI UNE DATE , ARLLELSEZE RLVEEEEZZZGOOOO
        // je veux un enfant de toi QUAND TU VEUX GROS
        // mais sans avoir un enfant de toi
        // ok 
        // lourd
        // putain ça part en craquage
        // mdr 
        // Je tente un truc dans la query, de mettre le trait

        let rec = Rect { positions };
        if !rec.is_horizontal() && !rec.is_vertical() {
            panic!("WTF, this is not a rectangle");
        }
        rec
    }

    fn is_horizontal(&self) -> bool {
        let first_y = self.positions.first().unwrap().y;
        self.positions.iter().all(|pos| first_y == pos.y)
    }
    fn is_vertical(&self) -> bool {
        let first_x = self.positions.first().unwrap().x;
        self.positions.iter().all(|pos| first_x == pos.x)
    }
}

impl Piece2 for Rect {
    fn rotate(&mut self) {
        let position_length = self.positions.len();
        let middle_index = position_length / 2;
        let central_piece_position = self.positions[middle_index];

        let compute_delta = |idx| (idx as f32 - middle_index as f32) * SQUARE_WIDTH as f32;

        if self.is_vertical() {
            for (idefix, pos) in self.positions.iter_mut().enumerate() {
                pos.x = central_piece_position.x - compute_delta(idefix);
                pos.y = central_piece_position.y;
            }
        } else if self.is_horizontal() {
            for (idefix, pos) in self.positions.iter_mut().enumerate() {
                pos.y = central_piece_position.y - compute_delta(idefix);
                pos.x = central_piece_position.x;
            }
        }
    }

    fn move_it(&mut self, cursor: &Res<Cursor>) {
        let first_pos = self.positions.first_mut().unwrap();

        let delta_x = -first_pos.x + cursor.current_pos.x;
        let delta_y = -first_pos.y + cursor.current_pos.y;

        first_pos.x = cursor.current_pos.x;
        first_pos.y = cursor.current_pos.y;

        for pos in self.positions.iter_mut().skip(1) {
            pos.x = pos.x + delta_x;
            pos.y = pos.y + delta_y;
        }
    }
}

// Plugin
pub struct RectPlugin;

impl Plugin for RectPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(spawn_piece.system())
            .add_system_to_stage(CoreStage::PreUpdate, clear_rect.system())
            .add_system(rotate.system())
            .add_system(move_piece.system())
            .add_system(draw_piece.system());
    }
}

// System
fn spawn_piece(mut commands: Commands) {
    let rectangle = Rect {
        positions: vec![vec2(100., 100.), vec2(150., 100.), vec2(200., 100.)],
    };
    commands.spawn().insert(rectangle);
}

fn clear_rect(mut commands: Commands, query: Query<Entity, With<PositionMarker>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn draw_piece(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query: Query<&Rect>,
) {
    let rect_material = materials.add(Color::rgb(0.68, 0.1, 1.03).into());
    for rec in query.iter().flat_map(|r| &r.positions) {
        commands
            .spawn_bundle(SpriteBundle {
                material: rect_material.clone(),
                sprite: Sprite::new(Vec2::new(
                    (SQUARE_WIDTH - 1) as f32,
                    (SQUARE_WIDTH - 1) as f32,
                )),
                transform: Transform::from_translation(vec3(rec.x, rec.y, 0.)),
                ..Default::default()
            })
            .insert(PositionMarker);
    }
}

fn rotate(mouse_button_input: Res<Input<MouseButton>>, mut query: Query<&mut Rect>) {
    if mouse_button_input.just_pressed(MouseButton::Right) {
        for mut rect in query.iter_mut() {
            rect.rotate()
        }
    }
}

fn move_piece(cursor: Res<Cursor>, mut query: Query<&mut Rect>) {
    if cursor.is_pressed {
        for mut rect in query.iter_mut() {
            rect.move_it(&cursor);
        }
    }
}

#[cfg(test)]
mod tests {
    use bevy::math::vec2;

    use super::*;

    #[test]
    fn test_rotate_90() {
        // Given
        let mut rectangle = Rect::new(vec![vec2(200., 50.), vec2(200., 100.), vec2(200., 150.)]);

        // When
        rectangle.rotate();

        // Then
        assert_eq!(
            rectangle.positions,
            vec![vec2(250., 100.), vec2(200., 100.), vec2(150., 100.)]
        );
    }

    #[test]
    fn test_rotate_180() {
        // Given
        let mut rectangle = Rect::new(vec![vec2(250., 100.), vec2(200., 100.), vec2(150., 100.)]);

        // When
        rectangle.rotate();

        // Then
        assert_eq!(
            rectangle.positions,
            vec![vec2(200., 150.), vec2(200., 100.), vec2(200., 50.)]
        );
    }

    #[test]
    fn test_rotate_270() {
        // Given
        let mut rectangle = Rect::new(vec![vec2(200., 150.), vec2(200., 100.), vec2(200., 50.)]);

        // When
        rectangle.rotate();

        // Then
        assert_eq!(
            rectangle.positions,
            vec![vec2(250., 100.), vec2(200., 100.), vec2(150., 100.)]
        );
    }

    #[test]
    fn test_rotate_360() {
        // Given
        let mut rectangle = Rect::new(vec![vec2(150., 100.), vec2(200., 100.), vec2(250., 100.)]);

        // When
        rectangle.rotate();

        // Then
        assert_eq!(
            rectangle.positions,
            vec![vec2(200., 150.), vec2(200., 100.), vec2(200., 50.)]
        );
    }
}