use bevy::prelude::*;

#[derive(Resource)]
pub struct SpaceStationImage(pub Handle<Image>);

impl FromWorld for SpaceStationImage {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource::<AssetServer>().expect("Failed to get asset server");
        Self(asset_server.load("sprites/space_station.png"))
    }
}
