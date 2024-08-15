#configuration parameters
ASSET_CONTROLLER_CONTRACT_ID = CCGL4ZNT66PLV4GLISA5KNW6VM3MEK5JTVIRSQJK5L6U3UOOQ7DWCDIJ
ENFORCED_WRAPPER_CONTRACT_ID = CBWTKZ2LDBBD7ZKGOR5NTD6HI3CJRV4SCZMOAPJUF5ASWWKPFSZMFQ4I
OPTIONAL_WRAPPER_CONTRACT_ID = CCZT5TYGUEOMDDZUV2BNNI5CZ5ZRRTOA4SVOPEUTB47PPXLXRI3ANQ23
CAMPAIGN_CONTRACT_ID = CDAJ6XSWXW5Y4UO7MSH53JVCXDPFMKVHJPVNVL56ESUC46IWYIMLYAH4

STELLAR_CLI = stellar
WASM_DIR = ../../target/wasm32-unknown-unknown/release
BINDINGS_DIR = ./.bindings
WASM_ENFORCED_WRAPPER_FILE = enforced_classic_asset_wrapper.wasm
WASM_OPTIONAL_WRAPPER_FILE = optional_classic_asset_wrapper.wasm
WASM_ASSET_CONTROLLER_FILE = asset_controller.wasm
WASM_CAMPAIGN_FILE = campaign.wasm

# scripts
UPLOAD_AND_DEPLOY_SCRIPT = ./src/deploy/upload-and-deploy.ts

CYAN = [36m
GREEN = [32m
YELLOW = [33m
BLUE = [34m
RESET = [0m

help h: 
	@echo 
	@echo $(CYAN)MAKE HELP: $(RESET)
	@echo  
	@echo Use the comand 'make' followed by the desired target.
	@echo For example: $(BLUE)make deploy-asset-controller $(RESET)
	@echo  
	@echo Also, feel free to directly edit this file to customize the parameters with your own values.
	@echo  

	@echo $(YELLOW)Configuration scripts$(RESET)
	@echo $(BLUE)make $(GREEN)deploy-asset-controller:$(RESET)                      Deploy the asset controller contract to testnet and logs both the contract id and wasm hash
	@echo $(BLUE)make $(GREEN)deploy-campaign:$(RESET)                              Deploy the campaign contract to testnet and logs both the contract id and wasm hash
	@echo $(BLUE)make $(GREEN)deploy-enforced-wrapper:$(RESET)                      Deploy the enforced asset wrapper contract to testnet and logs both the contract id and wasm hash
	@echo $(BLUE)make $(GREEN)deploy-optional-wrapper:$(RESET)                      Deploy the optional asset wrapper contract to testnet and logs both the contract id and wasm hash
	@echo $(BLUE)make $(GREEN)generate-bindings-asset-controller$(RESET)            Generate TypeScript bindings for the asset controller contract and outputs to the ./bindings directory. Make sure the contract is deployed in testnet for it to work. You can customize the contract id by directly changing this file.
	@echo $(BLUE)make $(GREEN)generate-bindings-campaign$(RESET)                    Generate TypeScript bindings for the campaign contract and outputs to the ./bindings directory. Make sure the contract is deployed in testnet for it to work. You can customize the contract id by directly changing this file.
	@echo $(BLUE)make $(GREEN)generate-bindings-enforced-wrapper$(RESET)            Generate TypeScript bindings for the enforced asset wrapper contract and outputs to the ./bindings directory. Make sure the contract is deployed in testnet for it to work. You can customize the contract id by directly changing this file.
	@echo $(BLUE)make $(GREEN)generate-bindings-optional-wrapper$(RESET)            Generate TypeScript bindings for the optional asset wrapper contract and outputs to the ./bindings directory. Make sure the contract is deployed in testnet for it to work. You can customize the contract id by directly changing this file.

	@echo $(YELLOW)Tests$(RESET)
	@echo $(BLUE)make $(GREEN)test-asset-controller-e2e:$(RESET)                    Runs end-to-end tests for the combination of the Enforced Classic Wrapper with the Asset Controller contracts in testnet.
	@echo $(BLUE)make $(GREEN)test-campaign-e2e:$(RESET)   						Runs end-to-end tests for the combination of the Optional Classic Wrapper with the Campaign contracts in testnet.

	@echo 



deploy-asset-controller:
	npx ts-node -r tsconfig-paths/register $(UPLOAD_AND_DEPLOY_SCRIPT) $(WASM_DIR)/$(WASM_ASSET_CONTROLLER_FILE)

deploy-campaign:
	npx ts-node -r tsconfig-paths/register $(UPLOAD_AND_DEPLOY_SCRIPT) $(WASM_DIR)/$(WASM_CAMPAIGN_FILE)

deploy-enforced-wrapper:
	npx ts-node -r tsconfig-paths/register $(UPLOAD_AND_DEPLOY_SCRIPT) $(WASM_DIR)/$(WASM_ENFORCED_WRAPPER_FILE)

deploy-optional-wrapper:
	npx ts-node -r tsconfig-paths/register $(UPLOAD_AND_DEPLOY_SCRIPT) $(WASM_DIR)/$(WASM_OPTIONAL_WRAPPER_FILE)

generate-bindings-asset-controller:
	$(STELLAR_CLI) contract bindings typescript --contract-id $(ASSET_CONTROLLER_CONTRACT_ID) --output-dir $(BINDINGS_DIR)/asset_controller --network testnet --overwrite

generate-bindings-campaign:
	$(STELLAR_CLI) contract bindings typescript --contract-id $(CAMPAIGN_CONTRACT_ID) --output-dir $(BINDINGS_DIR)/campaign --network testnet --overwrite

generate-bindings-enforced-wrapper:
	$(STELLAR_CLI) contract bindings typescript --contract-id $(ENFORCED_WRAPPER_CONTRACT_ID) --output-dir $(BINDINGS_DIR)/enforced_classic_asset_wrapper --network testnet --overwrite

generate-bindings-optional-wrapper:
	$(STELLAR_CLI) contract bindings typescript --contract-id $(OPTIONAL_WRAPPER_CONTRACT_ID) --output-dir $(BINDINGS_DIR)/optional_classic_asset_wrapper --network testnet --overwrite

test-asset-controller-e2e:
	npx jest ./src/tests/asset-controller.integration.test.ts

test-campaign-e2e:
	npx jest ./src/tests/campaign.integration.test.ts