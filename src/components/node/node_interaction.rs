use crate::components::grid::GridCtx;
use egui;
use crate::components::node::NodeCtx;

pub(crate) fn node_interaction(grid_ctx: &mut GridCtx, node_ctx: &mut NodeCtx, response: &mut egui::Response, _ui: &mut egui::Ui) {
    let mut changed = false;

    changed |= node_move(grid_ctx, node_ctx, response);

    if changed {
        response.changed();
    }
}

fn node_move(grid_ctx: &mut GridCtx, node_ctx: &mut NodeCtx, response: &mut egui::Response) -> bool {
    let delta = response.drag_delta();

    if delta != egui::Vec2::ZERO {
        node_ctx.pos += delta / grid_ctx.zoom.max(f32::EPSILON);
        true
    } else {
        false
    }
}
