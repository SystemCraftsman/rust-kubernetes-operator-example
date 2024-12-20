use k8s_openapi::serde::{Deserialize, Serialize};
use kube::{CustomResource};
use schemars::JsonSchema;

#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[kube(kind = "Game", group = "kubegame.systemcraftsman.com", version = "v1alpha1", namespaced)]
#[kube(status = "GameStatus")]
pub struct GameSpec {
    database: Database,
}

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema)]
pub struct GameStatus {
    ready: bool,
    message: String,
}

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema)]
pub struct Database {
    username: String,
    password: String,
}