use crate::components::node::node_interaction::node_interaction;
use crate::components::grid::GridCtx;
use derivative::Derivative;

#[derive(Derivative)]
#[derivative(Default, Debug)]
pub struct NodeCtx {
    id: usize,
    pub(crate) pos: egui::Pos2,
    size: egui::Vec2,
    draggable: bool
}

pub fn create_node(grid_ctx: &mut GridCtx, node_ctx: &mut NodeCtx, id: usize, ui: &mut egui::Ui) {
    if node_ctx.size == egui::Vec2::ZERO {
        node_ctx.init(id);
    }
    let rect = egui::Rect::from_min_size(node_ctx.pos, node_ctx.size);
    let rect = grid_ctx.apply_pan_zoom(rect);
    let rect = grid_ctx.editor_rect_to_screen_rect(rect);

    let sense = if !ui.input().pointer.middle_down() { egui::Sense::click_and_drag() } else { egui::Sense::hover() };
    let mut response = ui.allocate_rect(rect, sense);

    node_interaction(node_ctx, &mut response, ui);

    ui.painter().rect_filled(rect, 10.0, egui::Color32::from_rgb(255, 15, 30));
}

impl NodeCtx {
    fn init(&mut self, id: usize) {
        *self = Self {
            id,
            pos: [0.0, 0.0].into(),
            size: [200.0, 100.0].into(),
            draggable: true,

            // ..Default::default()
        }
    }
}