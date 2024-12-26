mod api;
mod controller;

use crate::controller::game_controller::GameReconciler;
use crate::controller::world_controller::WorldReconciler;
use crate::controller::ControllerRunner;

#[tokio::main]
async fn main() {
    let game_task = tokio::spawn(async {
        ControllerRunner::new(&GameReconciler {}).run().await;
    });

    let world_task = tokio::spawn(async {
        ControllerRunner::new(&WorldReconciler {}).run().await;
    });

    let _ = tokio::join!(game_task, world_task,);
}
