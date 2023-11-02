use bevy::prelude::*;

use super::loaders::{
    records::{ShipDefinition, SpaceStationDefinition},
    ShipLoader, ShipsLoader, SpaceStationLoader, SpaceStationsLoader,
};

pub struct ObjectsPlugin;
impl Plugin for ObjectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<ShipDefinition>();
        app.add_asset::<SpaceStationDefinition>();
        app.add_asset_loader(ShipLoader);
        app.add_asset_loader(ShipsLoader);
        app.add_asset_loader(SpaceStationLoader);
        app.add_asset_loader(SpaceStationsLoader);
    }
}
