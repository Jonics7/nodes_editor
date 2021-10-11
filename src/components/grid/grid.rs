use crate::components::grid::grid_interaction::grid_interaction;
use derivative::Derivative;
use crate::components::node::{self, NodeCtx};

use super::settings::GridSettings;
use egui::{Sense};

#[derive(Derivative)]
#[derivative(Default, Debug)]
pub struct GridCtx {
    pub(crate) settings: GridSettings,

    #[derivative(Default(value = "[[0.0; 2].into(); 2].into()"))]
    canvas_rect_screen_space: egui::Rect,
    
    canvas_origin_screen_space: egui::Vec2,
    pub(crate) panning: egui::Vec2,
    #[derivative(Default(value = "1.0"))]
    pub(crate) zoom: f32
}

pub fn build(grid_ctx: &mut GridCtx, node_ctx: &mut NodeCtx, ui: &mut egui::Ui) -> egui::Response {
    let rect = ui.available_rect_before_wrap_finite();
    let mut ui = ui.child_ui(rect, egui::Layout::default());
    let ui = &mut ui;
    ui.set_clip_rect(rect);

    let sense = if ui.input().pointer.middle_down() { Sense::drag() } else { Sense::hover() };
    let mut response = ui.allocate_rect(rect, sense);
    
    grid_interaction(grid_ctx, &mut response, ui);

    grid_ctx.canvas_rect_screen_space = rect;
    grid_ctx.draw(rect, ui);

    let mut node_ui = ui.child_ui(rect, egui::Layout::default());
    let node_ui = &mut node_ui;
    node::create_node(grid_ctx, node_ctx, 0, node_ui);
    response
}

// public functions

// private functions

impl GridCtx {
    fn draw(&mut self, rect: egui::Rect, ui: &mut egui::Ui) {
        self.draw_background(rect, ui);

        let cell_size = self.settings.get_cell_size() * self.zoom;
        
        let line_color = (1.0, egui::Rgba::from_rgb(0.5, 0.5, 0.5));
        let canvas_size = rect.size();

        let mut x = self.panning.x.rem_euclid(cell_size);
        let mut y = self.panning.y.rem_euclid(cell_size);

        while x < canvas_size.x {
            ui.painter().line_segment([
                self.editor_space_to_screen_space([x, 0.0].into()),
                self.editor_space_to_screen_space([x, canvas_size.y].into()),
            ], line_color
            );
    
            x += cell_size;
        }
    
        while y < canvas_size.y {
            ui.painter().line_segment(
                [
                    self.editor_space_to_screen_space([0.0, y].into()),
                    self.editor_space_to_screen_space([canvas_size.x, y].into()),
                ],
                line_color,
            );
            y += cell_size;
        }
    }

    fn draw_background(&mut self, rect: egui::Rect, ui: &mut egui::Ui) {
        ui.painter().rect_filled(rect, 0.0, egui::Color32::from_rgb(2, 5, 9));
    }

    fn editor_space_to_screen_space(&self, pos: egui::Pos2) -> egui::Pos2 {
        pos + self.canvas_rect_screen_space.min.to_vec2()
    }

    pub(crate) fn editor_rect_to_screen_rect(&self, rect: egui::Rect) -> egui::Rect {
        rect.translate(self.canvas_rect_screen_space.min.to_vec2())
    }

    pub(crate) fn screen_space_to_editor_space(&self, pos: egui::Pos2) -> egui::Pos2 {
        pos - self.canvas_rect_screen_space.min.to_vec2()
    }

    pub(crate) fn apply_pan_zoom_pos(&self, pos: egui::Pos2) -> egui::Pos2 {
        (pos.to_vec2() * self.zoom).to_pos2() + self.panning
    }

    pub(crate) fn apply_pan_zoom_rect(&self, rect: egui::Rect) -> egui::Rect {
        egui::Rect::from_min_size(self.apply_pan_zoom_pos(rect.min), rect.size() * self.zoom)
    }
}