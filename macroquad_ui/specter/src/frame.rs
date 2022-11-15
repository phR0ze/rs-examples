//! Frame provides properties for controlling how a widget's frame appears
//! * background properties
//! * stroke properties
use crate::prelude::*;

/// Frame is not a widget or container but rather provides a set of properties for manipulating a
/// widget's or container's
/// * background properties
/// * stroke properties
#[derive(Clone, Debug)]
pub struct Frame {
    //pub rounding: Rounding,
    //pub shadow: Shadow,
    /// Color to fill the frame with
    pub fill: Color,
    //pub stroke: Stroke,
    image: Option<Image>,     // background image to use
    image_clk: Option<Image>, // background image to use when clicked
    image_hov: Option<Image>, // background image to use when hovered
}

// Constructors and builders
impl Frame {
    pub fn new() -> Self {
        Frame {
            fill: GRAY,
            image: None,
            image_clk: None,
            image_hov: None,
        }
    }

    /// Set the fill color
    pub fn with_fill(self, color: Color) -> Self {
        Self {
            fill: color,
            ..self
        }
    }

    /// Set background image to use
    pub fn with_image<T: Into<Option<Image>>>(self, image: T) -> Self {
        Self {
            image: image.into(),
            ..self
        }
    }

    /// Set background image to use
    pub fn with_image_clk<T: Into<Option<Image>>>(self, image: T) -> Self {
        Self {
            image_clk: image.into(),
            ..self
        }
    }

    /// Set background image to use
    pub fn with_image_hov<T: Into<Option<Image>>>(self, image: T) -> Self {
        Self {
            image_hov: image.into(),
            ..self
        }
    }
}

// Getters
impl Frame {
    /// Get the fill color
    pub fn fill(&self) -> Color {
        self.fill
    }
}

// Setters
impl Frame {
    /// Set the fill color
    pub fn set_fill(&mut self, color: Color) -> &mut Self {
        self.fill = color;
        self
    }
}

// Unit tests
// -------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_frame_setters() {
        let mut frame = Frame::new();
        assert_eq!(vec2(2., 2.), vec2(2., 2.));
    }
}
