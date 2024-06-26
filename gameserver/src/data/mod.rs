mod event_graph;
mod templates;
mod tsv_util;

use std::{collections::HashMap, path::Path};

use anyhow::{bail, Result};
pub use event_graph::*;
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
                let path = concat!("assets/TemplateCollections/", stringify!($template_type), "TemplateCollection.tsv");
                let data = std::fs::read_to_string(path)?;
                [<$template_type:snake:upper _COLLECTION>].set(tsv_util::from_str(&data)?).unwrap();
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
    AvatarConfig;
    UnlockConfig;
    MainCityObject;
    NPCTransform;
}

static EVENT_GRAPH_COLLECTION: OnceCell<HashMap<i32, ConfigEventGraph>> = OnceCell::const_new();

fn init_binoutput() -> Result<()> {
    let _ = EVENT_GRAPH_COLLECTION.set(serde_json::from_str(
        std::fs::read_to_string("assets/BinOutput/EventGraphCollection.json")?.as_str(),
    )?);
    Ok(())
}

pub fn init_assets() -> Result<()> {
    if !Path::new("assets/").exists() {
        bail!(
            "Assets directory not found! Make sure you have it in the same directory with executable."
        )
    }

    init_template_collections()?;
    init_binoutput()
}

pub fn get_main_city_object(tag_id: i32, npc_id: i32) -> Option<&'static MainCityObjectTemplate> {
    iter_main_city_object_collection().find(|o| o.tag_id == tag_id && o.npc_id == npc_id)
}

pub fn is_transform_in_section(transform_id: &str, section_id: i32) -> bool {
    iter_n_p_c_transform_collection()
        .find(|t| t.id == transform_id)
        .map_or(false, |t| t.section == section_id)
}

pub fn get_event_graph(id: i32) -> Option<&'static ConfigEventGraph> {
    EVENT_GRAPH_COLLECTION.get().unwrap().get(&id)
}
