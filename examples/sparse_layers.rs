use bevy::{prelude::*, render::texture::ImageSettings};
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::window::PresentMode;
use bevy_ecs_tilemap::prelude::*;
mod helpers;

const USE_SPARSE : bool = true;

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(Camera2dBundle::default());

    //let texture_handle: Handle<Image> = asset_server.load("tiles.png");
    let texture_handle: Handle<Image> = asset_server.load("iso_color.png");

    let tilemap_size = TilemapSize { x: 384, y: 384 };

    // Layer 1
    let mut tile_storage = TileStorage::empty(tilemap_size, TileStorageMode::DenseVec);
    let tilemap_entity = commands.spawn().id();

    bevy_ecs_tilemap::helpers::fill_tilemap(
        TileTexture(0),
        tilemap_size.clone(),
        TilemapId(tilemap_entity),
        &mut commands,
        &mut tile_storage,
    );

    //let tile_size = TilemapTileSize { x: 16.0, y: 16.0 }; // square
    let tile_size = TilemapTileSize { x: 64.0, y: 32.0 }; // iso-staggered
    let grid_size = tile_size.into();

    commands
        .entity(tilemap_entity)
        .insert_bundle(TilemapBundle {
            grid_size,
            size: tilemap_size.clone(),
            storage: tile_storage,
            texture: TilemapTexture(texture_handle.clone()),
            tile_size: tile_size.clone(),
            map_type: TilemapType::Isometric {
                neighbors_include_diagonals: true,
                coord_system: IsoCoordSystem::Staggered
            },
            transform: bevy_ecs_tilemap::helpers::get_centered_transform_2d(
                &tilemap_size,
                &tile_size,
                0.0,
            ),
            ..Default::default()
        });

    // Layer 2
    let mut tile_storage = if USE_SPARSE {
        TileStorage::empty(tilemap_size, TileStorageMode::SparseHashMap)}
    else {
        TileStorage::empty(tilemap_size, TileStorageMode::DenseVec)
    };
    let tilemap_entity = commands.spawn().id();

    for x in 0..tilemap_size.x {
        for y in 0..tilemap_size.y {
            let tile_pos = TilePos { x, y };
            if (x * y) % 3 == 0 {
                let tile_entity = commands
                    .spawn()
                    .insert_bundle(TileBundle {
                        position: tile_pos,
                        tilemap_id: TilemapId(tilemap_entity),
                        texture: TileTexture(1),
                        ..Default::default()
                    })
                    .id();
                commands.entity(tilemap_entity).add_child(tile_entity);
                tile_storage.set(&tile_pos, Some(tile_entity));
            }
        }
    }

    commands
        .entity(tilemap_entity)
        .insert_bundle(TilemapBundle {
            grid_size,
            size: tilemap_size.clone(),
            storage: tile_storage,
            texture: TilemapTexture(texture_handle.clone()),
            tile_size: tile_size.clone(),
            map_type: TilemapType::Isometric {
                neighbors_include_diagonals: true,
                coord_system: IsoCoordSystem::Staggered
            },
            transform: bevy_ecs_tilemap::helpers::get_centered_transform_2d(
                &tilemap_size,
                &tile_size,
                2.0,
            ) * Transform::from_xyz(0.0, 16.0, 0.0),
            ..Default::default()
        });

    // Layer 3
    let mut tile_storage = if USE_SPARSE {
        TileStorage::empty(tilemap_size, TileStorageMode::SparseHashMap)}
    else {
        TileStorage::empty(tilemap_size, TileStorageMode::DenseVec)
    };
    let tilemap_entity = commands.spawn().id();

    for x in 0..tilemap_size.x {
        for y in 0..tilemap_size.y {
            let tile_pos = TilePos { x, y };
            if (x * y) % 7 == 0 {
                let tile_entity = commands
                    .spawn()
                    .insert_bundle(TileBundle {
                        position: tile_pos,
                        tilemap_id: TilemapId(tilemap_entity),
                        texture: TileTexture(2),
                        ..Default::default()
                    })
                    .id();
                commands.entity(tilemap_entity).add_child(tile_entity);
                tile_storage.set(&tile_pos, Some(tile_entity));
            }
        }
    }

    commands
        .entity(tilemap_entity)
        .insert_bundle(TilemapBundle {
            grid_size,
            size: tilemap_size.clone(),
            storage: tile_storage,
            texture: TilemapTexture(texture_handle.clone()),
            tile_size: tile_size.clone(),
            map_type: TilemapType::Isometric {
                neighbors_include_diagonals: true,
                coord_system: IsoCoordSystem::Staggered
            },
            transform: bevy_ecs_tilemap::helpers::get_centered_transform_2d(
                &tilemap_size,
                &tile_size,
                4.0,
            ) * Transform::from_xyz(0.0, 32.0, 0.0),
            ..Default::default()
        });

    // Layer 4
    let mut tile_storage = if USE_SPARSE {
        TileStorage::empty(tilemap_size, TileStorageMode::SparseHashMap)}
    else {
        TileStorage::empty(tilemap_size, TileStorageMode::DenseVec)
    };
    let tilemap_entity = commands.spawn().id();

    for x in 0..tilemap_size.x {
        for y in 0..tilemap_size.y {
            let tile_pos = TilePos { x, y };
            if (x + y) % 13 == 0 {
                let tile_entity = commands
                    .spawn()
                    .insert_bundle(TileBundle {
                        position: tile_pos,
                        tilemap_id: TilemapId(tilemap_entity),
                        texture: TileTexture(3),
                        ..Default::default()
                    })
                    .id();
                commands.entity(tilemap_entity).add_child(tile_entity);
                tile_storage.set(&tile_pos, Some(tile_entity));
            }
        }
    }

    commands
        .entity(tilemap_entity)
        .insert_bundle(TilemapBundle {
            grid_size,
            size: tilemap_size.clone(),
            storage: tile_storage,
            texture: TilemapTexture(texture_handle.clone()),
            tile_size: tile_size.clone(),
            map_type: TilemapType::Isometric {
                neighbors_include_diagonals: true,
                coord_system: IsoCoordSystem::Staggered
            },
            transform: bevy_ecs_tilemap::helpers::get_centered_transform_2d(
                &tilemap_size,
                &tile_size,
                5.0,
            ) * Transform::from_xyz(0.0, 48.0, 0.0),
            ..Default::default()
        });


    // Layer 5
    let mut tile_storage = if USE_SPARSE {
        TileStorage::empty(tilemap_size, TileStorageMode::SparseHashMap)}
    else {
        TileStorage::empty(tilemap_size, TileStorageMode::DenseVec)
    };
    let tilemap_entity = commands.spawn().id();

    for x in 0..tilemap_size.x {
        for y in 0..tilemap_size.y {
            let tile_pos = TilePos { x, y };
            if (x + y) % 23 == 0 {
                let tile_entity = commands
                    .spawn()
                    .insert_bundle(TileBundle {
                        position: tile_pos,
                        tilemap_id: TilemapId(tilemap_entity),
                        texture: TileTexture(4),
                        ..Default::default()
                    })
                    .id();
                commands.entity(tilemap_entity).add_child(tile_entity);
                tile_storage.set(&tile_pos, Some(tile_entity));
            }
        }
    }

    commands
        .entity(tilemap_entity)
        .insert_bundle(TilemapBundle {
            grid_size,
            size: tilemap_size.clone(),
            storage: tile_storage,
            texture: TilemapTexture(texture_handle.clone()),
            tile_size: tile_size.clone(),
            map_type: TilemapType::Isometric {
                neighbors_include_diagonals: true,
                coord_system: IsoCoordSystem::Staggered
            },
            transform: bevy_ecs_tilemap::helpers::get_centered_transform_2d(
                &tilemap_size,
                &tile_size,
                6.0,
            ) * Transform::from_xyz(0.0, 64.0, 0.0),
            ..Default::default()
        });
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: 1270.0,
            height: 720.0,
            title: String::from("Sparse Layers Example"),
            present_mode: PresentMode::AutoNoVsync,
            ..Default::default()
        })
        .insert_resource(ImageSettings::default_nearest())
        .add_plugins(DefaultPlugins)
        .add_plugin(TilemapPlugin)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(startup)
        .add_system(helpers::camera::movement)
        .run();
}
