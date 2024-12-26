pub mod game_controller;
pub mod world_controller;

use async_trait::async_trait;
use futures::stream::StreamExt;
use k8s_openapi::NamespaceResourceScope;
use kube::runtime::controller::Action;
use kube::runtime::Controller;
use kube::{Api, Client, Resource};
use serde::de::DeserializeOwned;
use std::fmt::Debug;
use std::hash::Hash;
use std::marker;
use std::sync::Arc;

#[async_trait]
pub trait Reconciler<K: Resource<Scope = NamespaceResourceScope>> {
    async fn reconcile(obj: Arc<K>, ctx: Arc<ContextData>) -> Result<Action, Error>;
    fn error_policy(obj: Arc<K>, err: &Error, _ctx: Arc<ContextData>) -> Action;
}
pub struct ControllerRunner<K: Resource<Scope = NamespaceResourceScope>> {
    _resource_marker: marker::PhantomData<K>,
}

impl<
        K: Resource<Scope = NamespaceResourceScope>
            + Clone
            + DeserializeOwned
            + Debug
            + Send
            + Sync
            + 'static,
    > ControllerRunner<K>
{
    pub async fn run<T: Reconciler<K>>()
    where
        <K as Resource>::DynamicType: Default,
        <K as Resource>::DynamicType: std::cmp::Eq,
        <K as Resource>::DynamicType: Hash,
        <K as Resource>::DynamicType: Clone,
        <K as kube::Resource>::DynamicType: Debug,
        <K as kube::Resource>::DynamicType: Unpin,
    {
        let client: Client = Client::try_default()
            .await
            .expect("Expected a valid KUBECONFIG environment variable.");
        let context: Arc<ContextData> = Arc::new(ContextData::new(client.clone()));
        let crd_api: Api<K> = Api::all(client);

        Controller::new(crd_api, Default::default())
            .run(<T>::reconcile, <T>::error_policy, context)
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
    }
}

pub struct ContextData {
    client: Client,
}

impl ContextData {
    pub fn new(client: Client) -> Self {
        ContextData { client }
    }
}

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
