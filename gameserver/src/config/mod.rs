mod templates;

use lazy_static::lazy_static;
pub use templates::*;

const MAIN_CITY_OBJECT_COLLECTION_JSON: &str =
    include_str!("../../TemplateCollections/MainCityObjectTemplateCollection.json");

lazy_static! {
    static ref MAIN_CITY_OBJECT_COLLECTION: Vec<MainCityObjectTemplate> =
        serde_json::from_str(MAIN_CITY_OBJECT_COLLECTION_JSON).unwrap();
}

pub fn get_main_city_object(tag_id: i32, npc_id: i32) -> Option<&'static MainCityObjectTemplate> {
    MAIN_CITY_OBJECT_COLLECTION
        .iter()
        .find(|object| object.tag_id == tag_id && object.npc_id == npc_id)
}
