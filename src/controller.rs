use futures::stream::StreamExt;
pub mod game_controller;
pub mod world_controller;

use crate::api::game_types::Game;
use crate::{controller, Error};
use async_trait::async_trait;
use k8s_openapi::NamespaceResourceScope;
use kube::runtime::controller::Action;
use kube::runtime::Controller;
use kube::{Api, Client, Resource, ResourceExt};
use serde::de::DeserializeOwned;
use std::fmt::Debug;
use std::hash::Hash;
use std::marker;
use std::sync::Arc;
use std::time::Duration;

pub struct ContextData {
    client: Client,
}

impl ContextData {
    pub fn new(client: Client) -> Self {
        ContextData { client }
    }
}

#[async_trait]
pub trait Reconciler<K: Resource<Scope = NamespaceResourceScope>> {
    async fn reconcile(obj: Arc<K>, ctx: Arc<ContextData>) -> Result<Action, Error>;
    fn error_policy(obj: Arc<K>, err: &Error, _ctx: Arc<ContextData>) -> Action;
}
pub struct ControllerRunner<'a, K: Resource<Scope = NamespaceResourceScope>, T: Reconciler<K>> {
    reconciler: &'a T,
    _marker: marker::PhantomData<K>,
}

impl<
        'a,
        K: Resource<Scope = NamespaceResourceScope>
            + Clone
            + DeserializeOwned
            + Debug
            + Send
            + Sync
            + 'static,
        T: Reconciler<K>,
    > ControllerRunner<'a, K, T>
{
    pub fn new(reconciler: &'a T) -> Self {
        ControllerRunner {
            reconciler,
            _marker: Default::default(),
        }
    }

    #[tokio::main]
    pub async fn run(&self)
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
