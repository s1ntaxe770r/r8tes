default:
	just --list 

# validate k8s manifests
validate:
	kubectl apply --dry-run=client -f manifests/role.yaml
	kubectl apply --dry-run=client -f manifests/controller-manager.yaml

# deploy the controller-manager 
deploy:
	kubectl apply -f manifests/role.yaml  
	kubectl apply -f manifests/controller-manager.yaml  

# run 
run:
	RUST_LOG=info cargo run -q

watch:
  RUST_LOG=info cargo watch -c -w src -x run

