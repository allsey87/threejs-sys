use std::f64;
use std::u32;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/three.js/src/core/Object3D.js")]
extern "C" {
    pub type Object3D;
    #[wasm_bindgen(constructor)]
    pub fn new() -> Object3D;   
    #[wasm_bindgen(method, js_name = translateX)]
    pub fn translate_x(this: &Object3D, distance: f64);
    #[wasm_bindgen(method, js_name = translateY)]
    pub fn translate_y(this: &Object3D, distance: f64);
    #[wasm_bindgen(method, js_name = translateZ)]
    pub fn translate_z(this: &Object3D, distance: f64);
//    #[wasm_bindgen(method, setter = position)]
//    pub fn set_position(this: &Object3D, position: &Vector3);
//    #[wasm_bindgen(method, getter = rotation)]
//    pub fn rotation(this: &Mesh) -> Euler;
}

