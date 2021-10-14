use crate::components::node::{
    node_interaction::node_interaction,
    node_styles::NodeStyles
};
use crate::components::grid::GridCtx;
use derivative::Derivative;

#[derive(Derivative)]
#[derivative(Default, Debug)]
pub struct NodeCtx {
    id: usize,
    pub(crate) pos: egui::Pos2,
    size: egui::Vec2,
    draggable: bool,
    styles: NodeStyles,

    header_size: egui::Vec2,
}

pub fn create_node(grid_ctx: &mut GridCtx, node_ctx: &mut NodeCtx, id: usize, ui: &mut egui::Ui) {
    if node_ctx.size == egui::Vec2::ZERO {
        node_ctx.init(id);
    }
    let rect = egui::Rect::from_min_size(node_ctx.pos, node_ctx.size);
    let rect = grid_ctx.apply_pan_zoom_rect(rect);
    let rect = grid_ctx.editor_rect_to_screen_rect(rect);

    let sense = if !ui.input().pointer.middle_down() { egui::Sense::click_and_drag() } else { egui::Sense::hover() };
    let mut response = ui.allocate_rect(rect, sense);

    node_interaction(grid_ctx, node_ctx, &mut response, ui);

    ui.painter().rect_filled(rect, 10.0, node_ctx.styles.body_color);


    // Draw header

    let header = egui::Rect::from_min_size(node_ctx.pos, node_ctx.header_size);
    let header = grid_ctx.apply_pan_zoom_rect(header);
    let header = grid_ctx.editor_rect_to_screen_rect(header);

    // ui.(|ui| {
    //     ui.painter().rect_filled(header, 10.0, node_ctx.styles.header_color);
    //     ui.label("Node title");
    // });

    ui.painter().rect_filled(header, 10.0, node_ctx.styles.header_color);

    // ui.label("Node title");
}

impl NodeCtx {
    fn init(&mut self, id: usize) {
        *self = Self {
            id,
            pos: [0.0, 0.0].into(),
            size: [200.0, 100.0].into(),
            draggable: true,
            header_size: [200.0, 40.0].into(),

            ..Default::default()
        }
    }
}