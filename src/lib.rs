mod utils;

use contour_isobands::{Band, ContourBuilder};
use geojson::{Feature, FeatureCollection, GeoJson};
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn make_contours(
    data: &[f64],
    width: usize,
    height: usize,
    intervals: &[f64],
    use_quad_tree: bool,
) -> Result<String, JsValue> {
    utils::set_panic_hook();

    let res: Vec<Band> = ContourBuilder::new(width, height)
        .use_quad_tree(use_quad_tree)
        .contours(data, intervals)
        .map_err(|err| JsError::new(&format!("Failed to build contours: {}", err)))?;

    let features: Vec<Feature> = res
        .iter()
        .map(|band| band.to_geojson())
        .collect::<Vec<geojson::Feature>>();

    Ok(GeoJson::from(FeatureCollection {
        bbox: None,
        features,
        foreign_members: None,
    })
    .to_string())
}
