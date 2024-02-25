deploy-counter:
	@dfx deploy counter

deploy-frontend:
	@dfx deploy frontend
	@echo " "
	@echo "Acccess frontend here: http://$$(dfx canister id frontend).localhost:4943"

deploy: deploy-counter deploy-frontend