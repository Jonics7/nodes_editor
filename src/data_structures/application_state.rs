use crate::components::{grid, node};

pub struct ApplicationState {
    pub grid_context: grid::GridCtx,
    pub node_context: node::NodeCtx
}

impl Default for ApplicationState {
    fn default() -> Self {
        Self {
            grid_context: grid::GridCtx::default(),
            node_context: node::NodeCtx::default()
        }
    }
}