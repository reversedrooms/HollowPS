mod templates;

use std::path::Path;

use anyhow::{bail, Result};
use paste::paste;
pub use templates::*;
use tokio::sync::OnceCell;

macro_rules! template_collections {
    ($($template_type:ident;)*) => {
        $(paste! {
            static [<$template_type:snake:upper _COLLECTION>]: OnceCell<Vec<[<$template_type Template>]>> = OnceCell::const_new();
        })*

        fn init_template_collections() -> Result<()> {
            $(paste! {
                let path = concat!("assets/TemplateCollections/", stringify!($template_type), "TemplateCollection.json");
                let data = std::fs::read_to_string(path)?;
                [<$template_type:snake:upper _COLLECTION>].set(serde_json::from_str(&data)?).unwrap();
            })*

            Ok(())
        }

        $(paste! {
            pub fn [<iter_ $template_type:snake _collection>]() -> ::std::slice::Iter<'static, [<$template_type Template>]> {
                [<$template_type:snake:upper _COLLECTION>].get().unwrap().iter()
            }
        })*
    };
}

template_collections! {
    MainCityObject;
}

pub fn init_assets() -> Result<()> {
    if !Path::new("assets/").exists() {
        bail!(
            "Assets directory not found! Make sure you have it in the same directory with executable."
        )
    }

    init_template_collections()?;
    Ok(())
}

pub fn get_main_city_object(tag_id: i32, npc_id: i32) -> Option<&'static MainCityObjectTemplate> {
    iter_main_city_object_collection().find(|o| o.tag_id == tag_id && o.npc_id == npc_id)
}
