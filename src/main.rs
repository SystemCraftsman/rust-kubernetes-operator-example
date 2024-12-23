mod api;
mod controller;

use crate::controller::game_controller::GameReconciler;
use crate::controller::world_controller::WorldReconciler;
use crate::controller::ControllerRunner;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Kubernetes reported error: {source}")]
    KubeError {
        #[from]
        source: kube::Error,
    },
    #[error("Invalid Echo CRD: {0}")]
    UserInputError(String),
}

fn main() {
    ControllerRunner::new(&GameReconciler {}).run();
    ControllerRunner::new(&WorldReconciler {}).run();
}
