mod api;
mod controller;

use api::game_types::Game;
use futures::StreamExt;
use kube::{
    runtime::controller::{Action, Controller},
    Api, Client, ResourceExt,
};
use std::{sync::Arc, time::Duration};

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

struct ContextData {
    client: Client,
}

impl ContextData {
    pub fn new(client: Client) -> Self {
        ContextData { client }
    }
}

#[tokio::main]
async fn main() -> Result<(), kube::Error> {
    let client: Client = Client::try_default()
        .await
        .expect("Expected a valid KUBECONFIG environment variable.");
    let context: Arc<ContextData> = Arc::new(ContextData::new(client.clone()));
    let games = Api::<Game>::all(client);

    Controller::new(games.clone(), Default::default())
        .run(reconcile, on_error, context)
        .for_each(|_| futures::future::ready(()))
        .await;

    Ok(())
}

async fn reconcile(obj: Arc<Game>, ctx: Arc<ContextData>) -> Result<Action, Error> {
    println!("reconcile request: {}", obj.name_any());
    Ok(Action::requeue(Duration::from_secs(3600)))
}

fn on_error(obj: Arc<Game>, err: &Error, _ctx: Arc<ContextData>) -> Action {
    eprintln!("Reconciliation error:\n{:?}.\n{:?}", err, obj);
    Action::requeue(Duration::from_secs(5))
}
