use elementtree::Element;

use crate::configs::style::{DropShadow, PositionOption, SizeOption};

use super::foundation::{Position, PositionOptionT, Size, SizeOptionT};
use super::svg::SvgTangibleObject;

/// A struct that represents a image.
#[derive(Debug)]
pub struct Image {
    /// The path of the image.
    /// Also, the image can be a Data URLs.
    pub path: String,
    /// The size of the image.
    pub size: Size,
    /// The rounded corner radius of the image.
    ///
    /// If the value is `None`, the image is not rounded.
    pub(crate) round: Option<usize>,
    /// The drop shadow of the image.
    ///
    /// If the value is `None`, the image is not shadowed.
    pub(crate) shadow: Option<DropShadow>,
}

impl Image {
    /// Creates a new `Image` instance.
    pub fn new_from_path<T: Into<Size>>(path: String, size: T) -> Self {
        Self {
            path,
            size: size.into(),
            round: None,
            shadow: None,
        }
    }

    /// Set the size of the image.
    pub fn set_size(&mut self, size: Size) {
        self.size = size;
    }

    /// Get padding size.
    ///
    /// Padding size is the extra space around the image,
    /// which is left for the shadow.
    /// We treat the two sides in one direction with the same spacing
    /// to keep the image centered.
    ///
    /// This padding size will affect the size of the [`Image`].
    pub fn get_padding(&self) -> (usize, usize) {
        if let Some(drop_shadow) = &self.shadow {
            drop_shadow.get_clearance()
        } else {
            (0, 0)
        }
    }
}

impl SizeOptionT for Image {
    fn get_size_option(&self) -> SizeOption {
        let padding = self.get_padding();
        SizeOption::Absolute(
            (self.size.0 as usize + padding.0 * 2) as u32,
            (self.size.1 as usize + padding.1 * 2) as u32,
        )
    }
}

impl PositionOptionT for Image {
    fn get_position_option(&self) -> PositionOption {
        PositionOption::Center
    }
}

impl SvgTangibleObject for Image {
    fn to_svg(&self, size: Size, position: Position, id: String) -> (Element, Option<Element>) {
        let mut svg = Element::new("svg");
        svg.set_attr("width", size.0.to_string());
        svg.set_attr("height", size.1.to_string());
        svg.set_attr("x", position.0.to_string());
        svg.set_attr("y", position.1.to_string());

        let padding = self.get_padding();

        if self.round.is_some() || self.shadow.is_some() {
            let mut defs = Element::new("defs");

            if let Some(round) = self.round {
                let mut clip_path = Element::new("clipPath");
                clip_path.set_attr("id", format!("clip-{}", id));
                let mut rect = Element::new("rect");
                rect.set_attr("width", (size.0 - padding.0 as u32 * 2).to_string());
                rect.set_attr("height", (size.1 - padding.1 as u32 * 2).to_string());
                rect.set_attr("x", padding.0.to_string());
                rect.set_attr("y", padding.1.to_string());
                rect.set_attr("rx", round.to_string());
                clip_path.append_child(rect);
                defs.append_child(clip_path);
            }

            if let Some(ds) = &self.shadow {
                let mut filter = Element::new("filter");
                filter.set_attr("id", format!("shadow-{}", id));
                let mut drop_shadow = Element::new("feDropShadow");
                drop_shadow.set_attr("dx", ds.x.to_string());
                drop_shadow.set_attr("dy", ds.y.to_string());
                drop_shadow.set_attr("stdDeviation", ds.blur.to_string());
                drop_shadow.set_attr("flood-opacity", "0.6");

                filter.append_child(drop_shadow);
                defs.append_child(filter);
            }

            svg.append_child(defs);
        }

        let mut img = Element::new("image");
        img.set_attr("href", self.path.clone());
        img.set_attr("width", (size.0 - padding.0 as u32 * 2).to_string());
        img.set_attr("height", (size.1 - padding.1 as u32 * 2).to_string());
        img.set_attr("x", padding.0.to_string());
        img.set_attr("y", padding.1.to_string());
        if self.round.is_some() {
            img.set_attr("clip-path", format!("url(#clip-{})", id));
        }
        if self.shadow.is_some() {
            let mut rect = Element::new("rect");
            rect.set_attr("filter", format!("url(#shadow-{})", id));
            rect.set_attr("width", (size.0 - padding.0 as u32 * 2).to_string());
            rect.set_attr("height", (size.1 - padding.1 as u32 * 2).to_string());
            rect.set_attr("x", padding.0.to_string());
            rect.set_attr("y", padding.1.to_string());
            rect.set_attr("rx", self.round.unwrap_or(0).to_string());
            svg.append_child(rect);
        }
        svg.append_child(img);

        (svg, None)
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::compare_svg;
    use super::*;
    use anyhow::Result;

    #[test]
    fn drop_shadow_range() {
        let drop_shadow = DropShadow {
            x: 0,
            y: 0,
            blur: 0,
            opacity: 0.5,
        };
        assert_eq!(drop_shadow.get_clearance(), (1, 1));

        let drop_shadow = DropShadow {
            x: 0,
            y: 0,
            blur: 1,
            opacity: 0.5,
        };

        assert_eq!(drop_shadow.get_clearance(), (4, 4));

        let drop_shadow = DropShadow {
            x: 1,
            y: 2,
            blur: 1,
            opacity: 0.5,
        };

        assert_eq!(drop_shadow.get_clearance(), (5, 6));
    }

    #[test]
    fn svg_image_default() -> Result<()> {
        let img = Image::new_from_path("./assets/input.png".to_string(), (100, 100));

        let (xml, defs) = img.to_svg(Size(100, 100), Position(10, 20), "1".to_string());

        assert!(defs.is_none());

        const EXPECT: &str = r#"
<svg x="10" y="20" height="100" width="100">
    <image width="100" height="100" x="0" y="0"  href="./assets/input.png"/>
</svg>
        "#;
        compare_svg(&xml, EXPECT)?;

        Ok(())
    }

    #[test]
    fn svg_image_round_effect() -> Result<()> {
        let mut img = Image::new_from_path("./assets/input.png".to_string(), (100, 100));
        img.round = Some(15);

        let (xml, defs) = img.to_svg(Size(100, 100), Position(0, 0), "1".to_string());

        assert!(defs.is_none());

        const EXPECT: &str = r#"
<svg x="0" y="0" height="100" width="100">
    <defs>
        <clipPath id="clip-1">
            <rect width="100" height="100" x="0" y="0" rx="15" />
        </clipPath>
    </defs>
    <image width="100" height="100" x="0" y="0" href="./assets/input.png" clip-path="url(#clip-1)" />
</svg>
        "#;
        compare_svg(&xml, EXPECT)?;

        Ok(())
    }

    #[test]
    fn svg_image_shadow_effect() -> Result<()> {
        let mut img = Image::new_from_path("./assets/input.png".to_string(), (100, 100));
        img.shadow = Some(DropShadow::new(5, 5, 3));

        let (xml, defs) = img.to_svg(Size(1030, 1030), Position(0, 0), "1".to_string());

        assert!(defs.is_none());

        const EXPECT: &str = r#"
<svg x="0" y="0" height="1030" width="1030">
    <defs>
        <filter id='shadow-1'>
            <feDropShadow dx="5" dy="5" stdDeviation="3" flood-opacity="0.6" />
        </filter>
    </defs>
    <rect width="1000" height="1000" x="15" y="15" rx="0" filter="url(#shadow-1)" />
    <image height="1000" width="1000" href="./assets/input.png" x="15" y="15" />
</svg>
        "#;
        compare_svg(&xml, EXPECT)?;

        Ok(())
    }

    #[test]
    fn svg_image_complex_effect() -> Result<()> {
        let mut img = Image::new_from_path("./assets/input.png".to_string(), (100, 100));
        img.round = Some(15);
        img.shadow = Some(DropShadow::new(5, 5, 3));

        let (xml, defs) = img.to_svg(Size(1030, 1030), Position(0, 0), "1".to_string());

        assert!(defs.is_none());

        const EXPECT: &str = r#"
<svg x="0" y="0" height="1030" width="1030">
    <defs>
        <clipPath id="clip-1">
            <rect width="1000" height="1000" x="15" y="15" rx="15" />
        </clipPath>
        <filter id='shadow-1'>
            <feDropShadow dx="5" dy="5" stdDeviation="3" flood-opacity="0.6" />
        </filter>
    </defs>
    <rect width="1000" height="1000" x="15" y="15" rx="15" filter="url(#shadow-1)" />
    <image height="1000" width="1000" href="./assets/input.png" x="15" y="15" clip-path="url(#clip-1)" />
</svg>
        "#;
        compare_svg(&xml, EXPECT)?;

        Ok(())
    }
}
