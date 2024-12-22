mod api;
mod controller;

use crate::controller::{ControllerRunner};
use crate::controller::game_controller::GameReconciler;

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

fn main() -> Result<(), kube::Error> {
    ControllerRunner::new(&GameReconciler{}).run()
}