use derivative::Derivative;

#[derive(Derivative)]
#[derivative(Default, Debug)]
pub struct GridCtx {
    cell_size: f32,

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
}

// private functions

impl GridCtx {
    fn draw(&mut self, rect: egui::Rect, ui: &mut egui::Ui) {
        self.draw_background(rect, ui);

        self.cell_size = 50.0;
        
        let line_color = (1.0, egui::Rgba::from_rgb(0.5, 0.5, 0.5));
        let canvas_size = rect.size();

        let mut x = self.panning.x.rem_euclid(self.cell_size);
        let mut y = self.panning.y.rem_euclid(self.cell_size);

        while x < canvas_size.x {
            ui.painter().line_segment([
                self.editor_space_to_screen_space([x, 0.0].into()),
                self.editor_space_to_screen_space([x, canvas_size.y].into()),
            ], line_color
            );
    
            x += self.cell_size;
        }
    
        while y < canvas_size.y {
            ui.painter().line_segment(
                [
                    self.editor_space_to_screen_space([0.0, y].into()),
                    self.editor_space_to_screen_space([canvas_size.x, y].into()),
                ],
                line_color,
            );
            y += self.cell_size;
        }
    }

    fn draw_background(&mut self, rect: egui::Rect, ui: &mut egui::Ui) {
        ui.painter().rect_filled(rect, 0.0, egui::Color32::from_rgb(2, 5, 9));
    }

    fn editor_space_to_screen_space(&self, pos: egui::Pos2) -> egui::Pos2 {
        pos + self.canvas_rect_screen_space.min.to_vec2()
    }
}