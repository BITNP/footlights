//! Config structs for the templates.

use thiserror::Error;

#[derive(Debug, Error)]
/// The error type for the config module.
pub enum ConfigError {
    /// Style is not in the style collection.
    #[error("Style \"{0}\" is not in the style collection.")]
    StyleNotInCollectionError(String),
}

/// The `structure` module contains the structure of the template.
///
/// Currently, the structure is a vector of layers.
/// By default, the structure is a vector of two layers:
/// a background layer and an image layer.
/// In the future, the structure will be a tree and
/// can be customized by the user.
pub mod structure {
    use serde::{Deserialize, Serialize};

    use crate::{background::Background, image::Image, svg::SvgTangibleObject, Canvas};

    use super::{style::StyleCollection, ConfigError};
    use anyhow::Result;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Structure {
        layers: Vec<Layer>,
    }

    pub trait ImageSizeProvider {
        fn get_image_size(&self, src: &str) -> (u32, u32);
    }

    impl Structure {
        pub(crate) fn from_vec(layers: Vec<Layer>) -> Self {
            Self { layers }
        }

        /// Build the canvas from the structure and style collections.
        pub fn build_canvas<I: ImageSizeProvider>(
            &self,
            style_collections: &StyleCollection,
            image_size_provider: I,
        ) -> Result<Canvas> {
            let mut canvas = Canvas::default();
            for layer in &self.layers {
                // Get the style of the layer.
                let style = style_collections
                    .get(&layer.style)
                    .ok_or_else(|| ConfigError::StyleNotInCollectionError(layer.style.clone()))?;

                let layer: Box<dyn SvgTangibleObject> = match layer.ty {
                    LayerType::Image => {
                        let image = Image {
                            path: style.image.clone().expect("Image path is not set."),
                            size: image_size_provider
                                .get_image_size(&style.image.clone().unwrap())
                                .into(),
                            round: style.round.clone(),

                            shadow: style.shadow.clone(),
                        };

                        Box::new(image)
                    }
                    LayerType::Background => {
                        let bg_type = style
                            .color
                            .clone()
                            .expect("Color is not set for background.");
                        let background = Background {
                            bg_type,
                            blur: style.blur.clone(),
                        };

                        Box::new(background)
                    }
                };

                canvas.add_layer_on_top(layer);
            }

            Ok(canvas)
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
                id: "img".to_string(),
                style: "img".to_string(),
            };

            Self::from_vec(vec![bg, img])
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub(crate) enum LayerType {
        Image,
        Background,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub(crate) struct Layer {
        ty: LayerType,
        id: String,
        style: String,
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn structure_serialization() {
            let structure = Structure::default();
            let json = serde_yaml::to_string(&structure).unwrap();

            let structure_new: Structure = serde_yaml::from_str(&json).unwrap();
            assert_eq!(structure_new.layers.len(), 2);
        }
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

    use crate::background::BackgroundType;

    #[derive(Debug, Clone, Serialize, Deserialize, Default)]
    pub struct StyleCollection {
        styles: HashMap<String, Style>,
    }

    impl StyleCollection {
        pub(crate) fn new(styles: HashMap<String, Style>) -> Self {
            Self { styles }
        }

        pub(crate) fn get(&self, id: &str) -> Option<&Style> {
            self.styles.get(id)
        }

        pub fn add(&mut self, id: String, style: Style) {
            self.styles.insert(id, style);
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize, Default)]
    pub struct Style {
        pub(crate) position: Option<PositionOption>,
        pub(crate) size: Option<SizeOption>,
        pub(crate) image: Option<String>,
        pub(crate) round: Option<usize>,
        pub(crate) shadow: Option<DropShadow>,
        // FIXME: Serde into Background for now.
        pub(crate) color: Option<BackgroundType>,
        pub(crate) blur: Option<usize>,
    }

    /// Position of the element, costomized by the user.
    #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
    pub enum PositionOption {
        /// The element is positioned at the center of the parent element.
        Center,
        /// The element is positioned at the absolute position of the parent element.
        Absolute(u32, u32),
    }

    /// Size of the element, costomized by the user.
    #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
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
    #[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
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

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn style_serialization() {
            let style = Style {
                position: Some(PositionOption::Center),
                size: Some(SizeOption::FitContent(10)),
                image: Some("image.png".to_string()),
                round: Some(10),
                shadow: Some(DropShadow::new(5, 5, 7)),
                color: Some(BackgroundType::Pure(crate::foundation::Color(
                    "red".to_owned(),
                ))),
                blur: Some(10),
            };

            let json = serde_yaml::to_string(&style).unwrap();

            let style_new: Style = serde_yaml::from_str(&json).unwrap();

            assert_eq!(style_new.position, Some(PositionOption::Center));
            assert_eq!(style_new.size, Some(SizeOption::FitContent(10)));
            assert_eq!(style_new.image, Some("image.png".to_string()));
            assert_eq!(style_new.round, Some(10));
            assert_eq!(style_new.shadow, Some(DropShadow::new(5, 5, 7)));
            assert_eq!(style_new.blur, Some(10));
        }
    }
}
