use bevy::asset::{AssetLoader, LoadedAsset};

use super::records::SpaceStationRecord;

pub struct SpaceStationLoader;

impl AssetLoader for SpaceStationLoader {
    fn extensions(&self) -> &[&str] { &["station"] }

    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, anyhow::Result<(), anyhow::Error>> {
        Box::pin(async move {
            let station_record = ron::de::from_bytes::<SpaceStationRecord>(bytes)?;

            let Some(idle_path) = station_record.idle_animation.to_str() else {
                return Err(anyhow::anyhow!(
                    "Cannot convert SpaceStationRecord.idle_animation to string: {:?}",
                    &station_record.idle_animation
                ));
            };
            let idle_animation_handle = load_context.get_handle(idle_path);

            let station_definition = station_record.to_definition(&idle_animation_handle);

            load_context
                .set_default_asset(LoadedAsset::new(station_definition).with_dependency(idle_path.into()));

            Ok(())
        })
    }
}

pub struct SpaceStationsLoader;

impl AssetLoader for SpaceStationsLoader {
    fn extensions(&self) -> &[&str] { &["stations"] }

    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, anyhow::Result<(), anyhow::Error>> {
        Box::pin(async move {
            let station_records = ron::de::from_bytes::<Vec<SpaceStationRecord>>(bytes)?;
            for station_record in station_records {
                let Some(idle_path) = station_record.idle_animation.to_str() else {
                    return Err(anyhow::anyhow!(
                        "Cannot convert SpaceStationRecord.idle_animation to string: {:?}",
                        &station_record.idle_animation
                    ));
                };
                let idle_animation_handle = load_context.get_handle(idle_path);

                let station_definition = station_record.to_definition(&idle_animation_handle);

                load_context.set_labeled_asset(
                    &station_record.name,
                    LoadedAsset::new(station_definition).with_dependency(idle_path.into()),
                );
            }

            Ok(())
        })
    }
}
