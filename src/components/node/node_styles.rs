use egui::Color32;
use derivative::Derivative;

#[derive(Derivative)]
#[derivative(Debug)]
pub(crate) struct NodeStyles {
    pub(crate) header_color: Color32,
    pub(crate) body_color: Color32,
}

impl Default for NodeStyles {
    fn default() -> Self {
        Self {
            header_color: Color32::from_rgb(0, 255, 0),
            body_color: Color32::from_rgb(32, 32, 32)
        }
    }
}