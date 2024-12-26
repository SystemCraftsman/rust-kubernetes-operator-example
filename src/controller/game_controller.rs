use std::sync::Arc;
use std::time::Duration;
use async_trait::async_trait;
use kube::ResourceExt;
use kube::runtime::controller::Action;
use crate::api::game_types::Game;
use crate::controller::{ContextData, Error, Reconciler};

pub struct GameReconciler;

#[async_trait]
impl Reconciler<Game> for GameReconciler {
    async fn reconcile(obj: Arc<Game>, _ctx: Arc<ContextData>) -> Result<Action, Error>{
        println!("reconcile request: {}", obj.name_any());
        Ok(Action::requeue(Duration::from_secs(3600)))
    }

    fn error_policy(obj: Arc<Game>, err: &Error, _ctx: Arc<ContextData>) -> Action{
        eprintln!("Reconciliation error:\n{:?}.\n{:?}", err, obj);
        Action::requeue(Duration::from_secs(5))
    }
}
