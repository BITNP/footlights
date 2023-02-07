mod utils;

use footlights_engine::configs::{
    structure::{ImageSizeProvider, Structure},
    style::{Style, StyleCollection},
};
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    /// Return (width, height).
    fn image_size_getter(src: &str) -> Box<[u32]>;
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, footlights-wasm!");
}

#[wasm_bindgen]
pub struct Engine {
    structure: Structure,
    styles: StyleCollection,
}

#[wasm_bindgen]
impl Engine {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            structure: Structure::default(),
            styles: StyleCollection::default(),
        }
    }

    #[wasm_bindgen]
    pub fn add_style(&mut self, id: String, val: JsValue) -> Result<(), JsValue> {
        let style: Style = serde_wasm_bindgen::from_value(val)?;

        self.styles.add(id, style);
        Ok(())
    }

    pub fn render(&self) -> Result<String, JsValue> {
        let image_size_provider = ImageSizeProviderImpl;
        let canvas = self
            .structure
            .build_canvas(&self.styles, image_size_provider)
            .map_err(|e| e.to_string())?;

        Ok(canvas.to_svg_string().map_err(|e| e.to_string())?)
    }
}

pub struct ImageSizeProviderImpl;

impl ImageSizeProvider for ImageSizeProviderImpl {
    fn get_image_size(&self, src: &str) -> (u32, u32) {
        todo!()
    }
}

#[wasm_bindgen]
pub fn new_structure() -> JsValue {
    serde_wasm_bindgen::to_value(&footlights_engine::configs::structure::Structure::default())
        .unwrap()
}

#[wasm_bindgen]
pub fn new_style() -> JsValue {
    serde_wasm_bindgen::to_value(&footlights_engine::configs::style::Style::default()).unwrap()
}
