mod api;

use k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinition;
use kube::CustomResourceExt;
use std::fs;
use std::fs::File;

fn main() {
    fs::create_dir_all("target/kubernetes").expect("TODO: panic message");
    write_crd_to_yaml(&api::game_types::Game::crd())
}

fn write_crd_to_yaml(crd: &CustomResourceDefinition) {
    let file_path = format!(
        "target/kubernetes/{name}-{version}.yaml",
        name = crd.metadata.name.clone().unwrap(),
        version = crd.spec.versions.first().unwrap().name
    );
    let file = File::create(file_path).expect("TODO: panic message");
    serde_yaml::to_writer(file, crd).expect("TODO: panic message");
}
