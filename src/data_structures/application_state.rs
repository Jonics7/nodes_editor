use crate::components::grid;

pub struct ApplicationState {
    pub grid_context: grid::GridCtx
}

impl Default for ApplicationState {
    fn default() -> Self {
        Self {
            grid_context: grid::GridCtx::default()
        }
    }
}