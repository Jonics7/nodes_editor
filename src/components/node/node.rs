use derivative::Derivative;

#[derive(Derivative)]
#[derivative(Default, Debug)]
pub struct NodeCtx {
    id: usize,
    size: egui::Vec2,
    draggable: bool
}

pub fn create_node(node_ctx: &mut NodeCtx, position: egui::Pos2, id: usize, ui: &mut egui::Ui) {
    node_ctx.init(id);
    let rect = egui::Rect::from_min_size(position, node_ctx.size);
    ui.painter().rect_filled(rect, 10.0, egui::Color32::from_rgb(255, 15, 30));
}

impl NodeCtx {
    fn init(&mut self, id: usize) {
        *self = Self {
            id,
            size: [200.0, 100.0].into(),
            draggable: true,

            // ..Default::default()
        }
    }
}