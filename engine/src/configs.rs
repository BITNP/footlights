//! Config structs for the templates.

/// The `structure` module contains the structure of the template.
///
/// Currently, the structure is a vector of layers.
/// By default, the structure is a vector of two layers:
/// a background layer and an image layer.
/// In the future, the structure will be a tree and
/// can be customized by the user.
pub mod structure {

    // TODO:
    // 1. add serde.
    // 2. (Structure, StyleCollection) -> Canvas
    // 3. Refactor CLI.
    pub(crate) struct Structure {
        layers: Vec<Layer>,
    }

    impl Structure {
        pub(crate) fn from_vec(layers: Vec<Layer>) -> Self {
            Self { layers }
        }
    }

    impl std::default::Default for Structure {
        fn default() -> Self {
            let bg = Layer {
                ty: LayerType::Background,
                id: "bg".to_string(),
                style: "bg".to_string(),
            };

            let img = Layer {
                ty: LayerType::Image,
                id: "image".to_string(),
                style: "image".to_string(),
            };

            Self::from_vec(vec![bg, img])
        }
    }

    pub(crate) enum LayerType {
        Image,
        Background,
    }

    pub(crate) struct Layer {
        ty: LayerType,
        id: String,
        style: String,
    }
}

/// The `style` module contains the style of the template.
///
/// The style is a collection of styles.
/// Each style is a struct that contains the style of a layer.
///
/// In the future, the style may be a CSS compatible struct,
/// and then parsed into the descrete style struct.
pub mod style {
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct StyleCollection {
        styles: HashMap<String, Style>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct Style {
        position: PositionOption,
        size: SizeOption,
        image: Option<String>,
        round: Option<usize>,
        shadow: Option<DropShadow>,
        // Serde into Background for now.
        color: Option<String>,
    }

    /// Position of the element, costomized by the user.
    #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
    pub enum PositionOption {
        /// The element is positioned at the center of the parent element.
        Center,
        /// The element is positioned at the absolute position of the parent element.
        Absolute(u32, u32),
    }

    /// Size of the element, costomized by the user.
    #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
    pub enum SizeOption {
        /// The element is sized to fit the content (the child element).
        /// The argument is the padding of the element. (in px)
        FitContent(u32),
        /// The element is absolute sized in x and y direction. (in px)
        Absolute(u32, u32),
    }

    /// A struct that represents a drop shadow.
    ///
    /// See [the official documentation](https://www.w3.org/TR/filter-effects/#feDropShadowElement).
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct DropShadow {
        /// The x offset of the drop shadow.
        pub x: usize,
        /// The y offset of the drop shadow.
        pub y: usize,
        #[serde(default = "default_blur")]
        /// The standard deviation for the blur operation in the drop shadow.
        pub blur: usize,
        #[serde(default = "default_opacity")]
        /// Opacity of the effect.
        pub opacity: f32,
    }

    fn default_blur() -> usize {
        7
    }

    fn default_opacity() -> f32 {
        0.6
    }

    impl std::default::Default for DropShadow {
        fn default() -> Self {
            Self {
                x: 5,
                y: 5,
                blur: 7,
                opacity: 0.6,
            }
        }
    }

    impl DropShadow {
        #[cfg(test)]
        pub(crate) fn new(x: usize, y: usize, blur: usize) -> Self {
            Self {
                x,
                y,
                blur,
                opacity: 0.6,
            }
        }

        /// Get the utmost clearance for the drop shadow on one side.
        ///
        /// According to gaussian blur, a pixel will be affected
        /// by the pixels no more than (3 standard deviations + 1) px.
        pub(crate) fn get_clearance(&self) -> (usize, usize) {
            let x = self.x + 3 * self.blur + 1;
            let y = self.y + 3 * self.blur + 1;
            (x, y)
        }
    }

}
