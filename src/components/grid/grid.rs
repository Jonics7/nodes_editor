use derivative::Derivative;

#[derive(Derivative)]
#[derivative(Default, Debug)]
pub struct GridCtx {
    #[derivative(Default(value = "[[0.0; 2].into(); 2].into()"))]
    canvas_rect_screen_space: egui::Rect,
    
    canvas_origin_screen_space: egui::Vec2,
    panning: egui::Vec2,
}

pub fn build(grid_ctx: &mut GridCtx, ui: &mut egui::Ui) {
    let rect = ui.available_rect_before_wrap_finite();

    grid_ctx.canvas_rect_screen_space = rect;
    grid_ctx.draw(rect, ui);
}

// public functions

impl GridCtx {
    fn draw(&mut self, rect: egui::Rect, ui: &mut egui::Ui) {
        ui.painter().rect_filled(rect, 0.0, egui::Color32::from_rgb(2, 5, 9));

        let grid_spacing: f32 = 50.0;
        let line_color = (1.0, egui::Rgba::from_rgb(0.5, 0.5, 0.5));
        let canvas_size = rect.size();

        let mut x = self.panning.x.rem_euclid(grid_spacing);
        let mut y = self.panning.y.rem_euclid(grid_spacing);

        while x < canvas_size.x {
            ui.painter().line_segment([
                self.editor_space_to_screen_space([x, 0.0].into()),
                self.editor_space_to_screen_space([x, canvas_size.y].into()),
            ], line_color
            );
    
            x += grid_spacing;
        }
    
        while y < canvas_size.y {
            ui.painter().line_segment(
                [
                    self.editor_space_to_screen_space([0.0, y].into()),
                    self.editor_space_to_screen_space([canvas_size.x, y].into()),
                ],
                line_color,
            );
            y += grid_spacing;
        }
    }
}

// private functions

impl GridCtx {
    fn editor_space_to_screen_space(&self, pos: egui::Pos2) -> egui::Pos2 {
        pos + self.canvas_rect_screen_space.min.to_vec2()
    }
}