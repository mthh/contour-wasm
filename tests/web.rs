//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;

use contour_wasm::make_contours;
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn make_contours_works() {
    let data = vec![1., 1., 1., 5., 1., 1., 5., 5., 1.];
    let width = 3;
    let height = 3;
    let intervals = vec![2., 5.];
    let use_quad_tree = false;

    let res = make_contours(&data, width, height, &intervals, use_quad_tree);

    // let expected = JsValue::from_str(r#"{"features":[{"geometry":{"coordinates":[[[[0.75,1],[0,0.25],[0,1],[0,2],[1,2],[1.75,2],[1,1.25],[0.75,1]]]],"type":"MultiPolygon"},"properties":{"max_v":5,"min_v":2},"type":"Feature"}],"type":"FeatureCollection"}"#);

    assert!(res.is_ok());
    // assert_eq!(
    //     res.ok().expect("failed"),
    //     expected
    // );
}
