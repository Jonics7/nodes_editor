use egui;
use crate::components::grid::GridCtx;

pub(crate) fn grid_interaction(grid_ctx: &mut GridCtx, response: &mut egui::Response, ui: &mut egui::Ui) {
    let mut changed = false;

    changed |= grid_panning(grid_ctx, response);
    changed |= grid_zoom(grid_ctx, ui);

    if changed {
        response.changed();
    }
}

fn grid_panning(grid_ctx: &mut GridCtx, response: &mut egui::Response) -> bool {
    let delta = response.drag_delta();

    if delta != egui::Vec2::ZERO {
        grid_ctx.panning += delta;
        true
    } else {
        false
    }
}

fn grid_zoom(grid_ctx: &mut GridCtx, ui: &mut egui::Ui) -> bool {
    let delta = ui.input().zoom_delta();

    if (delta - 1.0).abs() > 1e-9 {
        let pinch_scale = delta * grid_ctx.settings.zoom_sensivity;
        grid_ctx.zoom *= pinch_scale;
        if let Some(pinch_center) = ui.input().pointer.interact_pos() {
            let pinch_center_rect = grid_ctx.screen_space_to_editor_space(pinch_center).to_vec2();
            grid_ctx.panning = pinch_center_rect + (grid_ctx.panning - pinch_center_rect) * pinch_scale;
        }
        true
    } else {
        false
    }
}
