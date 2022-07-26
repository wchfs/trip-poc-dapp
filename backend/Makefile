COMPOSE_CMD := docker compose
BAKE_CMD := docker buildx bake -f docker-bake.hcl -f docker-bake.override.hcl
PROD_CMD := $(COMPOSE_CMD) -f docker-compose.yml -f docker-compose.override.yml
TESTNET_CMD := $(COMPOSE_CMD) -f docker-compose.yml -f docker-compose-testnet.yml -f docker-compose.override.yml
TESTNET_CONTRACT_CMD := docker compose --env-file network_env/env.goerli -f deploy-testnet.yml

.PHONY: bake bake-machine bake-server prod prod-down host host-down deploy-contract deploy-contract-down testnet-prod testnet-prod-down testnet-host testnet-host-down

bake:
	$(BAKE_CMD) --load

bake-machine:
	$(BAKE_CMD) machine --load

bake-server:
	$(BAKE_CMD) server

prod:
	$(PROD_CMD) up

prod-down:
	$(PROD_CMD) down -v

host:
	$(PROD_CMD) -f docker-compose-host.yml up

host-down:
	$(PROD_CMD) -f docker-compose-host.yml down -v

deploy-contract:
	$(TESTNET_CONTRACT_CMD) up && make deploy-contract-down

deploy-contract-down:
	$(TESTNET_CONTRACT_CMD) down -v

testnet-prod:
	$(TESTNET_CMD) up

testnet-prod-down:
	$(TESTNET_CMD) down -v

testnet-host:
	$(TESTNET_CMD) -f docker-compose-host-testnet.yml up

testnet-host-down:
	$(TESTNET_CMD) -f docker-compose-host-testnet.yml down -v