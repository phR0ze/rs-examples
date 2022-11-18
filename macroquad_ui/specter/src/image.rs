//! Image widget is composed from the Panel widget
//!
//! ## Features
//! * Calculated sizing and positioning relative to containing widget
//! * Image, clicked image and hovered image options
use crate::prelude::*;

#[derive(Clone)]
pub struct Image {
    panel: Panel, // underlying panel
}

impl Default for Image {
    fn default() -> Self {
        Self {
            panel: Panel::default(),
        }
    }
}

impl Image {
    pub fn new<T: AsRef<str>, U: Into<Texture2D>>(id: T, image: U) -> Self {
        Self {
            panel: Panel::new(id).frame(|x| x.image(image)),
        }
    }

    /// Create a new widget instance
    pub fn build<T: AsRef<str>, U: Into<Texture2D>>(&self, id: T, image: U) -> Self {
        Self {
            panel: self.panel.build(id).frame(|x| x.image(image)),
        }
    }

    /// Set the widget's frame properties
    pub fn frame(self, f: impl FnOnce(Frame) -> Frame) -> Self {
        Self {
            panel: self.panel.frame(f),
        }
    }

    /// Set the widget's id
    pub fn id<T: AsRef<str>>(self, id: T) -> Self {
        Self {
            panel: self.panel.id(id),
        }
    }

    /// Set image to use
    pub fn image<T: Into<Texture2D>>(self, image: T) -> Self {
        Self {
            panel: self.panel.frame(|x| x.image(image)),
        }
    }

    /// Set image to use
    pub fn image_clk<T: Into<Texture2D>>(self, image: T) -> Self {
        Self {
            panel: self.panel.frame(|x| x.image_clk(image)),
        }
    }

    /// Set image to use
    pub fn image_hov<T: Into<Texture2D>>(self, image: T) -> Self {
        Self {
            panel: self.panel.frame(|x| x.image_hov(image)),
        }
    }

    /// Enable listening for click interactions
    pub fn interact(self) -> Self {
        Self {
            panel: self.panel.interact(),
        }
    }

    /// Disable listening for click interactions
    pub fn no_interact(self) -> Self {
        Self {
            panel: self.panel.no_interact(),
        }
    }

    /// Set the widget's layout properties
    pub fn layout(self, f: impl FnOnce(Layout) -> Layout) -> Self {
        Self {
            panel: self.panel.layout(f),
        }
    }
}

impl LayoutManager for Image {
    /// Add the given widget to this widget's layout management
    /// * similar to `append` but consumes and returns self
    fn add(mut self, widget: impl Widget + 'static) -> Self {
        self.append(widget);
        self
    }

    /// Add the given widget to this widget's layout management
    fn append(&mut self, widget: impl Widget + 'static) {
        self.panel.append(widget);
    }
}

impl Widget for Image {
    /// Returns a reference clone to the Widget's layout
    fn layout_ptr(&self) -> Layout {
        self.panel.layout_ptr()
    }

    /// Draw the widget on the screen
    /// * `ui` is the Macroquad Ui engine
    fn show_p(&mut self, ui: &mut Ui) -> Response {
        self.panel.show_p(ui)
    }
}

// Unit tests
// -------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {

    //use super::*;

    #[test]
    fn test() {
        //
    }
}
