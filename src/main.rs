mod api;
mod controller;

use crate::controller::game_controller::GameReconciler;
use crate::controller::world_controller::WorldReconciler;
use crate::controller::ControllerRunner;

#[tokio::main]
async fn main() {
    let _ = tokio::join!(
        tokio::spawn(async {
            ControllerRunner::run::<GameReconciler>().await;
        }),
        tokio::spawn(async {
            ControllerRunner::run::<WorldReconciler>().await;
        }),
    );
}
