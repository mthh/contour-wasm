mod utils;

use wasm_bindgen::prelude::*;
use contour_isobands::{{ContourBuilder, Band}};
use geojson::{GeoJson, FeatureCollection};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn make_contours(data: &[f64], lx: usize, ly: usize, intervals: &[f64], use_quad_tree: bool) -> String {
    utils::set_panic_hook();
    let mut _data:Vec<Vec<f64>> = Vec::new();
    for i in 0..lx {
        let mut row = Vec::new();
        for j in 0..ly {
            row.push(data[i * ly + j]);
        }
        _data.push(row);
    }
    let res = ContourBuilder::new()
        .use_quad_tree(use_quad_tree)
        .contours(&_data, intervals).unwrap();
    let features = res.iter()
        .map(|band| band.to_geojson())
        .collect::<Vec<geojson::Feature>>();
    GeoJson::from(FeatureCollection {
        bbox: None,
        features,
        foreign_members: None,
    }).to_string()
}