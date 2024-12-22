pub mod game_controller;

use futures::StreamExt;
use std::marker;
use crate::Error;
use kube::runtime::controller::Action;
use kube::runtime::Controller;
use kube::{Api, Client, Resource};
use std::sync::Arc;
use async_trait::async_trait;

pub struct ContextData {
    client: Client,
}

impl ContextData {
    pub fn new(client: Client) -> Self {
        ContextData { client }
    }
}

#[async_trait]
pub trait Reconciler<R: Resource> {
    async fn reconcile(obj: Arc<R>, ctx: Arc<ContextData>) -> Result<Action, Error>;
    fn error_policy(obj: Arc<R>, err: &Error, _ctx: Arc<ContextData>) -> Action;
}

pub struct ControllerRunner<'a, R: Resource, T: Reconciler<R>> {
    reconciler: &'a T,
    _marker: marker::PhantomData<R>,
}

impl<'a, R: Resource, T: Reconciler<R>> ControllerRunner<'a, R, T> {
    pub fn new(reconciler: &'a T) -> Self {
        ControllerRunner { reconciler, _marker: Default::default() }
    }

    #[tokio::main]
    pub async fn run(&self) -> Result<(), kube::Error> {
        let client: Client = Client::try_default()
            .await
            .expect("Expected a valid KUBECONFIG environment variable.");
        let context: Arc<ContextData> = Arc::new(ContextData::new(client.clone()));
        let resource_objects = Api::<R>::all(client);

        Controller::new(resource_objects.clone(), Default::default())
            .run(
                Reconciler::reconcile,
                Reconciler::error_policy,
                context,
            )
            .for_each(|reconciliation_result| async move {
                match reconciliation_result {
                    Ok(resource) => {
                        println!("Reconciliation successful. Resource: {:?}", resource);
                    }
                    Err(reconciliation_err) => {
                        eprintln!("Reconciliation error: {:?}", reconciliation_err)
                    }
                }
            })
            .await;
        Ok(())
    }
}