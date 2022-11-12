//! Panel provides a container widget with options for framing and layout
use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Panel {
    frame: Frame,   // frame properties
    layout: Layout, // layout properties
}

// Constructors and builders
impl Panel {
    pub fn new<T: AsRef<str>>(id: T) -> Self {
        Self {
            frame: Frame::new(),
            layout: Layout::horz(id),
        }
    }

    /// Set the frame's properties
    pub fn with_frame(self, f: impl FnOnce(Frame) -> Frame) -> Self {
        Self {
            frame: f(self.frame),
            ..self
        }
    }

    /// Set layout to use
    pub fn with_layout(self, f: impl FnOnce(Layout) -> Layout) -> Self {
        Self {
            layout: f(self.layout),
            ..self
        }
    }
}

// Getters
impl Panel {
    /// Get the frame's properties
    pub fn frame(&self) -> Frame {
        self.frame
    }

    /// Get a shared reference to the layout
    pub fn layout(&self) -> Layout {
        self.layout.rc_ref()
    }
}

// Setters
impl Panel {
    /// Set the frame's properties
    pub fn set_frame(&mut self, f: impl FnOnce(Frame) -> Frame) {
        self.frame = f(self.frame);
    }
}

// Utility functions
impl Panel {
    /// Draw the widget on the screen
    /// * `ui` is the Macroquad Ui engine
    /// * returns true when clicked in the current frame
    pub fn show(&mut self, ui: &mut Ui) {
        self.show_pf(ui, None, |_, _| {})
    }

    /// Draw the widget on the screen
    /// * `ui` is the Macroquad Ui engine
    /// * `f` is a lambda for child layout creation
    /// * returns true when clicked in the current frame
    pub fn show_f(&mut self, ui: &mut Ui, f: impl FnOnce(&mut Ui, &Layout)) {
        self.show_pf(ui, None, f)
    }

    /// Draw the widget on the screen
    /// * `ui` is the Macroquad Ui engine
    /// * `layout` parent layout to draw button within
    /// * returns true when clicked in the current frame
    pub fn show_p(&mut self, ui: &mut Ui, layout: &Layout) {
        self.show_pf(ui, Some(layout), |_, _| {})
    }

    /// Draw the widget on the screen
    /// * `ui` is the Macroquad Ui engine
    /// * `layout` parent layout to draw button within
    /// * `f` is a lambda for child layout creation
    /// * returns true when clicked in the current frame
    pub fn show_pf(&mut self, ui: &mut Ui, layout: Option<&Layout>, f: impl FnOnce(&mut Ui, &Layout)) {
        if let Some(parent) = layout {
            parent.subs_append(&self.layout);
        }

        // Draw panel
        let (pos, size) = self.layout.shape();
        draw_rectangle(pos.x, pos.y, size.x, size.y, self.frame.fill);

        // Draw widgets
        f(ui, &self.layout)
    }
}

// Unit tests
// -------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_foo() {
        assert_eq!(vec2(2., 2.), vec2(2., 2.));
    }
}
