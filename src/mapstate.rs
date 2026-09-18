use std::ops::{Index, IndexMut};
use crate::comps;
use grid::Grid;
use bevy::{platform::collections::HashSet, prelude::*};

#[derive(Resource)]
pub struct MapState {
    grid: Grid<HashSet<Entity>>,
}
impl MapState {

    pub fn new(rows: usize, cols: usize) -> Self {
        MapState {
            grid: Grid::new(rows, cols)
        }
    }

    pub fn update(
        mut map: ResMut<Self>,
        query: Query<(Entity, &comps::LastPos , &comps::Pos), Changed<comps::Pos>>
    ) {
        for (entity, last_pos, pos) in query {
            if let Some(last_pos) = last_pos.prev() {
                map[last_pos.to_pos()].remove(&entity);
                map[pos.to_pos()].insert(entity);
            }
        }
    }

}

impl Index<(i64, i64)> for MapState {
    type Output = HashSet<Entity>;

    fn index(&self, index: (i64, i64)) -> &Self::Output {
        let x = (index.0 + self.grid.rows() as i64 / 2) as usize;
        let y = (index.1 + self.grid.cols() as i64 / 2) as usize;
        &self.grid[(x, y)]
    }
}
impl IndexMut<(i64, i64)> for MapState {
    
    fn index_mut(&mut self, index: (i64, i64)) -> &mut Self::Output {
        let x = (index.0 + self.grid.rows() as i64 / 2) as usize;
        let y = (index.1 + self.grid.cols() as i64 / 2) as usize;
        &mut self.grid[(x, y)]
    }
    
}