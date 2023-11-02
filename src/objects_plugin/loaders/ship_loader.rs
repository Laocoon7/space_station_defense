use bevy::asset::{AssetLoader, LoadedAsset};

use super::records::ShipRecord;

pub struct ShipLoader;

impl AssetLoader for ShipLoader {
    fn extensions(&self) -> &[&str] { &["ship"] }

    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, anyhow::Result<(), anyhow::Error>> {
        Box::pin(async move {
            let ship_record = ron::de::from_bytes::<ShipRecord>(bytes)?;

            let Some(idle_path) = ship_record.idle_animation.to_str() else {
                return Err(anyhow::anyhow!(
                    "Cannot convert ShipRecord.idle_animation to string: {:?}",
                    &ship_record.idle_animation
                ));
            };
            let idle_animation_handle = load_context.get_handle(idle_path);

            let ship_definition = ship_record.to_definition(&idle_animation_handle);

            load_context
                .set_default_asset(LoadedAsset::new(ship_definition).with_dependency(idle_path.into()));

            Ok(())
        })
    }
}

pub struct ShipsLoader;

impl AssetLoader for ShipsLoader {
    fn extensions(&self) -> &[&str] { &["ships"] }

    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, anyhow::Result<(), anyhow::Error>> {
        Box::pin(async move {
            let ship_records = ron::de::from_bytes::<Vec<ShipRecord>>(bytes)?;
            for ship_record in ship_records {
                let Some(idle_path) = ship_record.idle_animation.to_str() else {
                    return Err(anyhow::anyhow!(
                        "Cannot convert ShipRecord.idle_animation to string: {:?}",
                        &ship_record.idle_animation
                    ));
                };
                let idle_animation_handle = load_context.get_handle(idle_path);

                let ship_definition = ship_record.to_definition(&idle_animation_handle);

                load_context.set_labeled_asset(
                    &ship_record.name,
                    LoadedAsset::new(ship_definition).with_dependency(idle_path.into()),
                );
            }

            Ok(())
        })
    }
}
