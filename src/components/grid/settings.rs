use derivative::Derivative;

#[derive(Derivative)]
#[derivative(Default, Debug)]
pub struct GridSettings {
    #[derivative(Default(value = "50.0"))]
    cell_size: f32,
}

// public functions

impl GridSettings {
    pub fn get_cell_size(&self) -> f32 {
        self.cell_size
    }

    pub fn set_cell_size(&mut self, size: f32) {
        self.cell_size = size;
    }
}

// private functions

impl GridSettings {
    fn default(&self) -> Self {
        GridSettings {
            cell_size: 50.0,
            // ..Default::default()
        }
    }
}