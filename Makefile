##@ General
.PHONY: generate-crds
generate-crds:
	 cargo run --bin crdgen

##@ Deployment

.PHONY: install
install: ## Install CRDs into the K8s cluster specified in ~/.kube/config.
	@$(foreach file, $(wildcard target/kubernetes/*-v1alpha1.yaml), kubectl apply -f $(file);)

.PHONY: uninstall
uninstall: ## Uninstall CRDs from the K8s cluster specified in ~/.kube/config.
	@$(foreach file, $(wildcard target/kubernetes/*-v1alpha1.yaml), kubectl delete -f $(file);)