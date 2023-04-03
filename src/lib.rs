mod utils;

use contour::ContourBuilder as IsoLinesBuilder;
use contour_isobands::ContourBuilder as IsoBandsBuilder;
use geojson::{Feature, FeatureCollection, GeoJson};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Deserialize)]
struct Options {
    use_quad_tree: Option<bool>,
    x_origin: Option<f64>,
    y_origin: Option<f64>,
    x_step: Option<f64>,
    y_step: Option<f64>,
}

fn serialize_result(features: Vec<Feature>) -> Result<JsValue, JsError> {
    let s = serde_wasm_bindgen::Serializer::new().serialize_maps_as_objects(true);

    let res = GeoJson::from(FeatureCollection {
        bbox: None,
        features,
        foreign_members: None,
    })
    .serialize(&s)
    .map_err(|err| {
        JsError::new(&format!(
            "Failed to transform GeoJSON feature collection to JS Object: {}",
            err
        ))
    })?;

    Ok(res)
}

#[wasm_bindgen(js_name = "isolines")]
pub fn isolines(
    data: &[f64],
    width: usize,
    height: usize,
    thresholds: &[f64],
    options: &JsValue,
) -> Result<JsValue, JsError> {
    utils::set_panic_hook();
    let (x_origin, y_origin, x_step, y_step) = if options.is_null() || options.is_undefined() {
        (0.0, 0.0, 1.0, 1.0)
    } else {
        let options: Options = serde_wasm_bindgen::from_value(options.into())?;
        (
            options.x_origin.unwrap_or(0.0),
            options.y_origin.unwrap_or(0.0),
            options.x_step.unwrap_or(1.0),
            options.y_step.unwrap_or(1.0),
        )
    };

    let features: Vec<Feature> = IsoLinesBuilder::new(width as u32, height as u32, true)
        .x_origin(x_origin)
        .y_origin(y_origin)
        .x_step(x_step)
        .y_step(y_step)
        .lines(data, thresholds)
        .map_err(|err| JsError::new(&format!("Failed to build isolines: {}", err)))?
        .into_iter()
        .map(|band| band.to_geojson())
        .collect::<Vec<Feature>>();

    serialize_result(features)
}

#[wasm_bindgen(js_name = "isobands")]
pub fn isobands(
    data: &[f64],
    width: usize,
    height: usize,
    intervals: &[f64],
    options: &JsValue,
) -> Result<JsValue, JsError> {
    utils::set_panic_hook();
    let (use_quad_tree, x_origin, y_origin, x_step, y_step) =
        if options.is_null() || options.is_undefined() {
            (true, 0.0, 0.0, 1.0, 1.0)
        } else {
            let options: Options = serde_wasm_bindgen::from_value(options.into())?;
            (
                options.use_quad_tree.unwrap_or(true),
                options.x_origin.unwrap_or(0.0),
                options.y_origin.unwrap_or(0.0),
                options.x_step.unwrap_or(1.0),
                options.y_step.unwrap_or(1.0),
            )
        };

    let features: Vec<Feature> = IsoBandsBuilder::new(width, height)
        .use_quad_tree(use_quad_tree)
        .x_origin(x_origin)
        .y_origin(y_origin)
        .x_step(x_step)
        .y_step(y_step)
        .contours(data, intervals)
        .map_err(|err| JsError::new(&format!("Failed to build isobands: {}", err)))?
        .into_iter()
        .map(|band| band.to_geojson())
        .collect::<Vec<Feature>>();

    serialize_result(features)
}
