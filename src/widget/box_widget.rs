// Box Widget
// Extensible widget for the widget library - handles drawing a box with a border and a fill color
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use piston_window::*;

use crate::core::callbacks::*;
use crate::core::point::*;
use crate::widget::config::*;
use crate::widget::widget::*;

/// This is the `BoxWidget`, which contains a top-level widget for display, overriding the
/// draw method to draw the base widget and the border for this box.
pub struct BoxWidget {
    config: Configurable,
    callbacks: CallbackStore,
    base_widget: BaseWidget,
}

/// Implementation of the constructor for the `BaseWidget`.  Creates a new base widget
/// that can be positioned anywhere on the screen.
impl BoxWidget {
    pub fn new() -> Self {
        Self {
            config: Configurable::new(),
            callbacks: CallbackStore::new(),
            base_widget: BaseWidget::new(),
        }
    }

    /// Sets the border color for this widget.
    pub fn set_border_color(&mut self, color: types::Color) {
        self.config()
            .set(CONFIG_COLOR_BORDER, WidgetConfig::BorderColor { color });
        self.invalidate();
    }

    /// Retrieves the border color of this widget.
    /// Defaults to black color `[0.0, 0.0, 0.0, 1.0]` if not set.
    pub fn get_border_color(&mut self) -> types::Color {
        match self.config().get(CONFIG_COLOR_BORDER) {
            Some(WidgetConfig::BorderColor { color }) => [color[0], color[1], color[2], color[3]],
            _ => [0.0, 0.0, 0.0, 1.0],
        }
    }

    /// Sets the thickness of the border for this widget.
    pub fn set_border_thickness(&mut self, thickness: u8) {
        self.config()
            .set(CONFIG_BORDER_WIDTH, WidgetConfig::BorderWidth { thickness });
        self.invalidate();
    }

    /// Retrieves the border thickness of this widget.
    /// Defaults to 1 if not set.
    pub fn get_border_thickness(&mut self) -> u8 {
        match self.config().get(CONFIG_BORDER_WIDTH) {
            Some(WidgetConfig::BorderWidth { ref thickness }) => thickness.clone(),
            _ => 1,
        }
    }

    /// Helper function that sets both the color of the border and the thickness at the same time.
    pub fn set_border(&mut self, color: types::Color, thickness: u8) {
        self.set_border_color(color);
        self.set_border_thickness(thickness);
    }

    /// Function to draw a box for the point and size of this box.  Automatically draws the border
    /// along with the width of the border.  This is automatically determined by the origin, so the
    /// box is automatically drawn for the bounds of the `Widget`.
    fn draw_box(&mut self, c: Context, g: &mut G2d) {
        let origin: Point = self.get_origin();
        let size: crate::core::point::Size = self.get_size();
        let border: f64 = self.get_border_thickness() as f64;
        let color: types::Color = self.get_border_color();

        // Upper left to upper right
        line(
            color,
            border,
            [
                origin.x as f64,
                origin.y as f64 + border,
                (origin.x + size.w) as f64,
                origin.y as f64 + border,
            ],
            c.transform,
            g,
        );

        // Upper left to lower right
        line(
            color,
            border,
            [
                (origin.x + size.w) as f64 - border,
                origin.y as f64 + border,
                (origin.x + size.w) as f64 - border,
                (origin.y + size.h) as f64,
            ],
            c.transform,
            g,
        );

        // Upper left to lower left
        line(
            color,
            border,
            [
                origin.x as f64 + border,
                origin.y as f64 + border,
                origin.x as f64 + border,
                (origin.y + size.h) as f64,
            ],
            c.transform,
            g,
        );

        // Lower left to lower right
        line(
            color,
            border,
            [
                origin.x as f64,
                (origin.y + size.h) as f64 - border,
                (origin.x + size.w) as f64,
                (origin.y + size.h) as f64 - border,
            ],
            c.transform,
            g,
        );
    }
}

/// Implementation of the `BoxWidget` object with the `Widget` traits implemented.
/// This implementation is similar to the `BaseWidget`, but incorporates a drawable box inside
/// the widget.  Base widget is the `BaseWidget`.
///
/// This is basically just a box with a fill color.  Use this to draw other things like buttons,
/// text widgets, and so on, if you need anything with a drawable border.
///
/// Example usage:
/// ```no_run
/// # use piston_window::*;
/// # use pushrod::core::point::*;
/// # use pushrod::widget::widget::*;
/// # use pushrod::widget::box_widget::*;
/// # fn main() {
///    let mut box_widget = BoxWidget::new();
///
///    box_widget.set_origin(100, 100);
///    box_widget.set_size(200, 200);
///    box_widget.set_color([0.5, 0.5, 0.5, 1.0]);
///    box_widget.set_border_color([0.0, 0.0, 0.0, 1.0]);
///    box_widget.set_border_thickness(3);
///
///    // (OR)
///
///    box_widget.set_border([0.0, 0.0, 0.0, 1.0], 3);
/// # }
/// ```
impl Widget for BoxWidget {
    fn config(&mut self) -> &mut Configurable {
        &mut self.config
    }

    fn callbacks(&mut self) -> &mut CallbackStore {
        &mut self.callbacks
    }

    /// Sets the `Point` of origin for this widget and the base widget, given the X and Y
    /// coordinates.  Invalidates the widget afterward.
    fn set_origin(&mut self, x: i32, y: i32) {
        self.config().set(
            CONFIG_ORIGIN,
            WidgetConfig::Origin {
                point: Point { x, y },
            },
        );
        self.base_widget.set_origin(x, y);
        self.invalidate();
    }

    /// Sets the `Size` for this widget and the base widget, given width and height.  Invalidates the widget afterward.
    fn set_size(&mut self, w: i32, h: i32) {
        self.config().set(
            CONFIG_SIZE,
            WidgetConfig::Size {
                size: crate::core::point::Size { w, h },
            },
        );
        self.base_widget.set_size(w, h);
        self.invalidate();
    }

    /// Sets the color for this widget.  Invalidates the widget afterward.
    fn set_color(&mut self, color: types::Color) {
        self.config()
            .set(CONFIG_COLOR, WidgetConfig::Color { color });
        self.base_widget.set_color(color);
        self.invalidate();
    }

    /// Retrieves the color of this widget.
    /// Defaults to white color `[1.0; 4]` if not set.
    fn get_color(&mut self) -> types::Color {
        self.base_widget.get_color()
    }

    /// Draws the contents of the widget in this order:
    ///
    /// - Base widget first
    /// - Box graphic for the specified width
    fn draw(&mut self, c: Context, g: &mut G2d) {
        // Paint the base widget first.  Forcing a draw() call here will ignore invalidation.
        // Invalidation is controlled by the top level widget (this box).
        self.base_widget.draw(c, g);

        // Paint the box.
        self.draw_box(c, g);

        // Then clear invalidation.
        self.clear_invalidate();
    }
}
