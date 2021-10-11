use derivative::Derivative;

#[derive(Derivative)]
#[derivative(Default, Debug)]
pub struct GridSettings {
    #[derivative(Default(value = "50.0"))]
    cell_size: f32,
    #[derivative(Default(value = "1.0"))]
    pub(crate) zoom_sensitivity: f32,
    #[derivative(Default(value = "0.25"))]
    pub(crate) min_zoom: f32,
    #[derivative(Default(value = "4.0"))]
    pub(crate) max_zoom: f32,
}

// public functions

impl GridSettings {
    pub fn get_cell_size(&self) -> f32 {
        self.cell_size
    }
}

// private functions