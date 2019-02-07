// Simple Windowed Example
// Super simplistic test to show off the use of the library in its current state
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

extern crate pushrod;

use piston_window::*;
use pushrod::core::main::*;
use pushrod::core::point::*;
use pushrod::core::window::*;
use pushrod::event::event::*;
use pushrod::widget::widget::*;
use pushrod::widget::box_widget::*;

struct ExampleListener {}

impl ExampleListener {
    fn new() -> Self {
        Self {}
    }

    fn handle_mouse_move(&self, point: &Point) {
        eprintln!("X={} Y={}", point.x, point.y);
    }

    fn handle_mouse_down(&self, button: &MouseButton) {
        match button {
            MouseButton::Left => eprintln!("Left mouse button pressed."),
            _ => eprintln!("Other mouse button pressed."),
        }
    }

    fn handle_mouse_up(&self, button: &MouseButton) {
        match button {
            MouseButton::Left => eprintln!("Left mouse button released."),
            _ => eprintln!("Other mouse button released."),
        }
    }

    fn handle_mouse_scroll(&self, point: &Point) {
        eprintln!("Scroll: X={} Y={}", point.x, point.y);
    }
}

impl EventListener for ExampleListener {
    fn handle_event(&self, event: &PushrodEvent) {
        match event {
            PushrodEvent::MouseEvent { point } => self.handle_mouse_move(&point),
            PushrodEvent::MouseDownEvent { button } => self.handle_mouse_down(&button),
            PushrodEvent::MouseUpEvent { button } => self.handle_mouse_up(&button),
            PushrodEvent::MouseScrollEvent { point } => self.handle_mouse_scroll(&point),
        }
    }
}

fn main() {
    let opengl = OpenGL::V3_2;
    let prod: Pushrod = Pushrod::new(opengl);
    let mut pushrod_window: PushrodWindow = PushrodWindow::new(
        WindowSettings::new("Pushrod Window", [800, 600])
            .opengl(opengl)
            .build()
            .unwrap_or_else(|error| panic!("Failed to build PistonWindow: {}", error)),
    );

    pushrod_window.window.set_max_fps(60);
    pushrod_window.window.set_ups(60);

    let mut base_widget = BaseWidget::new();
    base_widget.set_origin(50, 50);
    base_widget.set_size(200, 200);
    base_widget.set_color([0.5, 0.5, 0.5, 1.0]);
    pushrod_window.add_widget(Box::new(base_widget));

    let mut box_widget = BoxWidget::new();
    box_widget.set_origin(275, 50);
    box_widget.set_size(200, 200);
    box_widget.set_color([0.0, 1.0, 0.0, 1.0]);
    box_widget.set_border_color([1.0, 0.0, 0.0, 1.0]);
    box_widget.set_border_thickness(4);
    pushrod_window.add_widget(Box::new(box_widget));

    let mut box_1 = BoxWidget::new();
    box_1.set_origin(500, 50);
    box_1.set_size(200, 200);
    box_1.set_color([0.5, 0.5, 1.0, 1.0]);
    box_1.set_border_color([0.0, 0.0, 1.0, 1.0]);
    box_1.set_border_thickness(2);
    let box_1_id = pushrod_window.add_widget(Box::new(box_1));

    let mut box_2 = BoxWidget::new();
    box_2.set_origin(550, 75);
    box_2.set_size(100, 50);
    box_2.set_color([0.75, 0.75, 1.0, 1.0]);
    box_2.set_border_color([1.0, 0.0, 1.0, 1.0]);
    box_2.set_border_thickness(1);
    pushrod_window.add_widget_to_parent(Box::new(box_2), box_1_id);

    let mut box_3 = BoxWidget::new();
    box_3.set_origin(550, 175);
    box_3.set_size(100, 50);
    box_3.set_color([0.75, 0.75, 1.0, 1.0]);
    box_3.set_border_color([1.0, 0.0, 1.0, 1.0]);
    box_3.set_border_thickness(1);
    pushrod_window.add_widget_to_parent(Box::new(box_3), box_1_id);

    prod.add_window(pushrod_window);
    prod.add_event_listener_for_window(Box::new(ExampleListener::new()));

    // Runs the main event loop
    prod.run();
}
