use egui;
use crate::components::node::NodeCtx;

pub(crate) fn node_interaction(node_ctx: &mut NodeCtx, response: &mut egui::Response, _ui: &mut egui::Ui) {
    let mut changed = false;

    changed |= node_move(node_ctx, response);

    if changed {
        response.changed();
    }
}

fn node_move(node_ctx: &mut NodeCtx, response: &mut egui::Response) -> bool {
    let delta = response.drag_delta();

    if delta != egui::Vec2::ZERO {
        node_ctx.pos += delta;
        true
    } else {
        false
    }
}
