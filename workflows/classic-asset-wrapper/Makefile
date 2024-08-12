#configuration parameters
ASSET_CONTROLLER_CONTRACT_ID = CADQXEKDRPVJBUMJJNWV5IPKGO7GCOESH6YIFAPJAL7KPI6VTPUTKDT4
WRAPPER_CONTRACT_ID = CCDCBEQUHYRBGZX7I7RRA67NO3H2GUVJMIAPXX4AOJYOTNR3AEBZXALS


STELLAR_CLI = stellar
WASM_DIR = ../../target/wasm32-unknown-unknown/release
BINDINGS_DIR = ./.bindings
WASM_WRAPPER_FILE = wrapper_interface.wasm
WASM_ASSET_CONTROLLER_FILE = asset_controller.wasm

# scripts
UPLOAD_AND_DEPLOY_SCRIPT = ./src/deploy/upload-and-deploy.ts

CYAN = [36m
GREEN = [32m
YELLOW = [33m
BLUE = [34m
RESET = [0m

help h: 
	@echo.
	@echo $(CYAN)MAKE HELP: $(RESET)
	@echo.
	@echo Use the comand 'make' followed by the desired target.
	@echo For example: $(BLUE)make deploy-asset-controller $(RESET)
	@echo.
	@echo Also, feel free to directly edit this file to customize the parameters with your own values.
	@echo.

	@echo $(YELLOW)Configuration scripts$(RESET)
	@echo $(BLUE)make $(GREEN)deploy-asset-controller:$(RESET)                      Deploy the asset controller contract to testnet and logs both the contract id and wasm hash
	@echo $(BLUE)make $(GREEN)deploy-wrapper:$(RESET)                               Deploy the asset wrapper contract to testnet and logs both the contract id and wasm hash
	@echo $(BLUE)make $(GREEN)generate-bindings-asset-controller$(RESET)            Generate TypeScript bindings for the asset controller contract and outputs to the ./bindings directory. Make sure the contract is deployed in testnet for it to work. You can customize the contract id by directly changing this file.
	@echo $(BLUE)make $(GREEN)generate-bindings-wrapper$(RESET)                     Generate TypeScript bindings for the asset controller contract and outputs to the ./bindings directory. Make sure the contract is deployed in testnet for it to work. You can customize the contract id by directly changing this file.
 
	@echo $(YELLOW)Tests$(RESET)
	@echo $(BLUE)make $(GREEN)test-asset-controller-e2e:$(RESET)                    Runs end-to-end tests for the combination of the Classic Wrapper with the Asset Controller contracts in testnet.
	
	@echo.



deploy-asset-controller:
	npx ts-node -r tsconfig-paths/register $(UPLOAD_AND_DEPLOY_SCRIPT) $(WASM_DIR)/$(WASM_ASSET_CONTROLLER_FILE)

deploy-wrapper:
	npx ts-node -r tsconfig-paths/register $(UPLOAD_AND_DEPLOY_SCRIPT) $(WASM_DIR)/$(WASM_WRAPPER_FILE)

generate-bindings-asset-controller:
	$(STELLAR_CLI) contract bindings typescript --contract-id $(ASSET_CONTROLLER_CONTRACT_ID) --output-dir $(BINDINGS_DIR)/asset_controller --network testnet

generate-bindings-wrapper:
	$(STELLAR_CLI) contract bindings typescript --contract-id $(WRAPPER_CONTRACT_ID) --output-dir $(BINDINGS_DIR)/classic_asset_wrapper --network testnet

test-asset-controller-e2e:
	npx jest ./src/tests/asset-controller.integration.test.ts