use crate::api::world_types::World;
use crate::controller::{ContextData, Error, Reconciler};
use async_trait::async_trait;
use kube::runtime::controller::Action;
use kube::ResourceExt;
use std::sync::Arc;
use std::time::Duration;

pub struct WorldReconciler;

#[async_trait]
impl Reconciler<World> for WorldReconciler {
    async fn reconcile(obj: Arc<World>, ctx: Arc<ContextData>) -> Result<Action, Error> {
        println!("reconcile request: {}", obj.name_any());
        Ok(Action::requeue(Duration::from_secs(3600)))
    }

    fn error_policy(obj: Arc<World>, err: &Error, _ctx: Arc<ContextData>) -> Action {
        eprintln!("Reconciliation error:\n{:?}.\n{:?}", err, obj);
        Action::requeue(Duration::from_secs(5))
    }
}
