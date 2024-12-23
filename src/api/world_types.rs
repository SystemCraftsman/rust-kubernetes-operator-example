use k8s_openapi::serde::{Deserialize, Serialize};
use kube::{CustomResource};
use schemars::JsonSchema;

#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[kube(kind = "World", group = "kubegame.systemcraftsman.com", version = "v1alpha1", namespaced)]
#[kube(status = "WorldStatus")]
pub struct WorldSpec {
    game: String,
    description: String,
}

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema)]
pub struct WorldStatus {
    ready: bool,
    message: String,
}