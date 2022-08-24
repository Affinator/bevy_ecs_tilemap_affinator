use bevy::prelude::*;
use bevy::utils::HashMap;

use crate::map::TilemapSize;

use super::TilePos;

#[derive(Debug, Clone)]
pub enum TileStorageMode {
    DenseVec,
    SparseHashMap
}

impl Default for TileStorageMode {
    fn default() -> Self {
        TileStorageMode::DenseVec
    }
}

/// Used to store tile entities for fast look up.
/// Tile entities are stored in a grid. The grid is always filled with None.
#[derive(Component, Default, Debug, Clone)]
pub struct TileStorage {
    tiles_dense_vec: Vec<Option<Entity>>,
    tiles_sparse_map: HashMap<TilePos, usize>, // mapping from TilePos to vec indices; TODO implement custom hasher for TilePos
    tiles_sparse_vec: Vec<Option<Entity>>, // vec of Entities (consistent with tiles_spare_back_vec)
    tiles_sparse_back_vec: Vec<TilePos>, // vec of TilePos (consistent with tiles_spare_vec)
    tile_storage_mode: TileStorageMode,
    pub size: TilemapSize,
}

/** Our assumption of the ratio of populated vs empty tiles in sparse TileStorages **/
const SPARSE_TO_DENSE_RATIO : usize = 4;

impl TileStorage {
    /// Creates a new tile storage that is empty.
    pub fn empty(size: TilemapSize, mode : TileStorageMode) -> Self {
        match mode {
            TileStorageMode::DenseVec => {
                Self {
                    tiles_dense_vec: vec![None; size.count()],
                    tiles_sparse_vec: Vec::with_capacity(0),
                    tiles_sparse_back_vec: Vec::with_capacity(0),
                    tiles_sparse_map: HashMap::with_capacity(0),
                    tile_storage_mode: mode,
                    size,
                }
            }
            TileStorageMode::SparseHashMap => {
                Self {
                    tiles_dense_vec: Vec::with_capacity(0),
                    tiles_sparse_vec: Vec::with_capacity(size.count()/SPARSE_TO_DENSE_RATIO),
                    tiles_sparse_back_vec: Vec::with_capacity(size.count()/SPARSE_TO_DENSE_RATIO),
                    tiles_sparse_map: HashMap::with_capacity(size.count()/SPARSE_TO_DENSE_RATIO),
                    tile_storage_mode: mode,
                    size,
                }
            }
        }
    }


    /// Gets a tile entity for the given tile position.
    pub fn get(&self, tile_pos: &TilePos) -> Option<Entity> {
        match self.tile_storage_mode {
            TileStorageMode::DenseVec => {
                self.tiles_dense_vec[crate::helpers::pos_2d_to_index(tile_pos, &self.size)]
            }
            TileStorageMode::SparseHashMap => {
                match self.tiles_sparse_map.get(tile_pos) {
                    None => {None}
                    Some(index) => {self.tiles_sparse_vec[*index]}
                }
            }
        }
    }

    /// Sets a tile entity for the given tile position.
    pub fn set(&mut self, tile_pos: &TilePos, tile_entity: Option<Entity>) {
        match self.tile_storage_mode {
            TileStorageMode::DenseVec => {
                self.tiles_dense_vec[crate::helpers::pos_2d_to_index(tile_pos, &self.size)] = tile_entity;
            }
            TileStorageMode::SparseHashMap => {
                match tile_entity {
                    None => { // remove that tile from the map; note: we do not write None, we remove
                        self.remove(tile_pos);
                    }
                    Some(_) => { // add a tile to the map or replace existing tile
                        match self.tiles_sparse_map.get(tile_pos) {
                            None => { // new tile
                                self.tiles_sparse_vec.push(tile_entity);
                                self.tiles_sparse_back_vec.push(*tile_pos);
                                self.tiles_sparse_map.insert(*tile_pos, self.tiles_sparse_vec.len() - 1);
                            }
                            Some(existing_index) => { // update existing tile
                                self.tiles_sparse_vec[*existing_index] = tile_entity;
                            }
                        }

                    }
                }
            }
        }
    }

    pub fn remove(&mut self, tile_pos: &TilePos) {
        match self.tile_storage_mode {
            TileStorageMode::DenseVec => {
                self.set(tile_pos, None);
            }
            TileStorageMode::SparseHashMap => {
                let removed_index = self.tiles_sparse_map.remove(tile_pos);
                if let Some(removed_index) = removed_index {
                    if self.tiles_sparse_vec.len() > 1 {
                        let _ = self.tiles_sparse_vec.swap_remove(removed_index);
                        let replacement_tile_pos = self.tiles_sparse_back_vec[removed_index];
                        self.tiles_sparse_map.insert(replacement_tile_pos, removed_index);
                    }
                    else {
                        self.tiles_sparse_vec.clear();
                        self.tiles_sparse_back_vec.clear();
                    }
                }
            }
        }

    }


    /// Returns an iterator with all of the positions in the grid. No order guaranteed
    pub fn iter(&self) -> impl std::iter::Iterator<Item = &Option<Entity>> {
        match self.tile_storage_mode {
            TileStorageMode::DenseVec => {self.tiles_dense_vec.iter()}
            TileStorageMode::SparseHashMap => {self.tiles_sparse_vec.iter()}
        }
    }

    /// Returns an immutable iterator with all of the positions in the grid. No order guaranteed
    pub fn iter_mut(&mut self) -> impl std::iter::Iterator<Item = &mut Option<Entity>> {
        match self.tile_storage_mode {
            TileStorageMode::DenseVec => {self.tiles_dense_vec.iter_mut()}
            TileStorageMode::SparseHashMap => {self.tiles_sparse_vec.iter_mut()}
        }
    }
}
