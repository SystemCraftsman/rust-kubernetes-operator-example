mod api;

use k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinition;
use kube::CustomResourceExt;
use std::fs;
use std::fs::File;

fn main() {
    fs::create_dir_all("target/kubernetes").expect("Error creating directory 'target/kubernetes'");
    write_crd_to_yaml(&api::game_types::Game::crd());
    write_crd_to_yaml(&api::world_types::World::crd());
}

fn write_crd_to_yaml(crd: &CustomResourceDefinition) {
    let file_path = format!(
        "target/kubernetes/{name}-{version}.yaml",
        name = crd.metadata.name.clone().unwrap(),
        version = crd.spec.versions.first().unwrap().name
    );
    let file = File::create(file_path).expect("Error creating YAML file");
    serde_yaml::to_writer(file, crd).expect("Error writing to YAML file");
}
