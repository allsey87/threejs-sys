use std::f64;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/three.module.js")]
extern "C" {
    pub type Scene;
    #[wasm_bindgen(constructor)]
    pub fn new() -> Scene;
    #[wasm_bindgen(method, js_name = add)]
    pub fn add(this: &Scene, mesh: &Mesh);

    pub type Object3D;
    #[wasm_bindgen(method, js_name = translateZ)]
    pub fn translate_z(this: &Object3D, distance: f64);

    #[wasm_bindgen(extends = Object3D)]
    pub type Camera;

    #[wasm_bindgen(extends = Camera)]
    pub type PerspectiveCamera;
    #[wasm_bindgen(constructor)]
    pub fn new(fov: f64, aspect: f64, near: f64, far: f64) -> PerspectiveCamera;

    pub type WebGLRenderer;
    #[wasm_bindgen(constructor)]
    pub fn new() -> WebGLRenderer;
    #[wasm_bindgen(method, js_name = setSize)]
    pub fn set_size(this: &WebGLRenderer, width: f64, height: f64);
    #[wasm_bindgen(method, getter = domElement)]
    pub fn dom_element(this: &WebGLRenderer) -> web_sys::HtmlElement; // HtmlCanvasElement ???
    #[wasm_bindgen(method)]
    pub fn render(this: &WebGLRenderer, scene: &Scene, camera: &PerspectiveCamera);

    pub type BoxGeometry;
    #[wasm_bindgen(constructor)]
    pub fn new(width: f64, height: f64, depth: f64) -> BoxGeometry;

    pub type MeshBasicMaterial;
    #[wasm_bindgen(constructor)]
    pub fn new() -> MeshBasicMaterial;
    #[wasm_bindgen(method, setter = color)]
    pub fn set_color(this: &MeshBasicMaterial, value: &Color);

    pub type Mesh;
    #[wasm_bindgen(constructor)]
    pub fn new(geometry: BoxGeometry, material: MeshBasicMaterial) -> Mesh;
    #[wasm_bindgen(method, getter = rotation)]
    pub fn rotation(this: &Mesh) -> Euler;

    pub type Euler;
    #[wasm_bindgen(method, getter = x)]
    pub fn x(this: &Euler) -> f64;
    #[wasm_bindgen(method, getter = y)]
    pub fn y(this: &Euler) -> f64;
    #[wasm_bindgen(method, getter = z)]
    pub fn z(this: &Euler) -> f64;
    #[wasm_bindgen(method, setter = x)]
    pub fn set_x(this: &Euler, value: f64);
    #[wasm_bindgen(method, setter = y)]
    pub fn set_y(this: &Euler, value: f64);
    #[wasm_bindgen(method, setter = z)]
    pub fn set_z(this: &Euler, value: f64);

    pub type Vector3;
    #[wasm_bindgen(constructor)]
    pub fn new(x: f64, y: f64, z: f64) -> Vector3;
    #[wasm_bindgen(method, js_name = getComponent)]
    pub fn get_component(this: &Vector3, index : u32) -> f64;

    pub type Color;
    #[wasm_bindgen(constructor)]
    pub fn new(r: f64, g: f64, b: f64) -> Color;

}

