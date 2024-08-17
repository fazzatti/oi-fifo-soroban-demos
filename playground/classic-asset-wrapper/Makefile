#configuration parameters
PROBATION_CONTRACT_ID = CCGL4ZNT66PLV4GLISA5KNW6VM3MEK5JTVIRSQJK5L6U3UOOQ7DWCDIJ
ENFORCED_WRAPPER_CONTRACT_ID = CBWTKZ2LDBBD7ZKGOR5NTD6HI3CJRV4SCZMOAPJUF5ASWWKPFSZMFQ4I
OPTIONAL_WRAPPER_CONTRACT_ID = CCZT5TYGUEOMDDZUV2BNNI5CZ5ZRRTOA4SVOPEUTB47PPXLXRI3ANQ23
CAMPAIGN_CONTRACT_ID = CDAJ6XSWXW5Y4UO7MSH53JVCXDPFMKVHJPVNVL56ESUC46IWYIMLYAH4
COUNTER_CONTRACT_ID = CCLRLRUXXWO3RQNGGGXB6J243VPKAV7R7A4HW7OR2BEKE3KWW2VCXOXK

STELLAR_CLI = stellar
WASM_DIR = ../../target/wasm32-unknown-unknown/release
BINDINGS_DIR = ./.bindings
WASM_ENFORCED_WRAPPER_FILE = enforced_classic_asset_wrapper.wasm
WASM_OPTIONAL_WRAPPER_FILE = optional_classic_asset_wrapper.wasm
WASM_PROBATION_FILE = probation.wasm
WASM_CAMPAIGN_FILE = campaign.wasm
WASM_COUNTER_FILE = counter.wasm

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
	@echo For example: $(BLUE)make deploy-probation $(RESET)
	@echo  
	@echo Also, feel free to directly edit this file to customize the parameters with your own values.
	@echo  

	@echo $(YELLOW)Configuration scripts$(RESET)
	@echo $(BLUE)make $(GREEN)deploy-counter:$(RESET)                               Deploy the counter contract to testnet and logs both the contract id and wasm hash
	@echo $(BLUE)make $(GREEN)deploy-probation:$(RESET)                             Deploy the probation contract to testnet and logs both the contract id and wasm hash
	@echo $(BLUE)make $(GREEN)deploy-campaign:$(RESET)                              Deploy the campaign contract to testnet and logs both the contract id and wasm hash
	@echo $(BLUE)make $(GREEN)deploy-enforced-wrapper:$(RESET)                      Deploy the enforced asset wrapper contract to testnet and logs both the contract id and wasm hash
	@echo $(BLUE)make $(GREEN)deploy-optional-wrapper:$(RESET)                      Deploy the optional asset wrapper contract to testnet and logs both the contract id and wasm hash
	@echo $(BLUE)make $(GREEN)generate-bindings-probation$(RESET)                   Generate TypeScript bindings for the probation contract and outputs to the ./bindings directory. Make sure the contract is deployed in testnet for it to work. You can customize the contract id by directly changing this file.
	@echo $(BLUE)make $(GREEN)generate-bindings-campaign$(RESET)                    Generate TypeScript bindings for the campaign contract and outputs to the ./bindings directory. Make sure the contract is deployed in testnet for it to work. You can customize the contract id by directly changing this file.
	@echo $(BLUE)make $(GREEN)generate-bindings-counter$(RESET)                     Generate TypeScript bindings for the counter contract and outputs to the ./bindings directory. Make sure the contract is deployed in testnet for it to work. You can customize the contract id by directly changing this file.
	@echo $(BLUE)make $(GREEN)generate-bindings-enforced-wrapper$(RESET)            Generate TypeScript bindings for the enforced asset wrapper contract and outputs to the ./bindings directory. Make sure the contract is deployed in testnet for it to work. You can customize the contract id by directly changing this file.
	@echo $(BLUE)make $(GREEN)generate-bindings-optional-wrapper$(RESET)            Generate TypeScript bindings for the optional asset wrapper contract and outputs to the ./bindings directory. Make sure the contract is deployed in testnet for it to work. You can customize the contract id by directly changing this file.

	@echo $(YELLOW)Tests$(RESET)
	@echo $(BLUE)make $(GREEN)test-probation-e2e:$(RESET)                           Runs end-to-end tests for the combination of the Enforced Classic Wrapper with the Probation contracts in testnet.
	@echo $(BLUE)make $(GREEN)test-campaign-e2e:$(RESET)   					     	Runs end-to-end tests for the combination of the Optional Classic Wrapper with the Campaign contracts in testnet.
	@echo $(BLUE)make $(GREEN)test-counter-e2e:$(RESET)   					     	Runs end-to-end tests for the combination of the Optional Classic Wrapper with the Counter contracts in testnet.

	@echo 



deploy-counter:
	npx ts-node -r tsconfig-paths/register $(UPLOAD_AND_DEPLOY_SCRIPT) $(WASM_DIR)/$(WASM_COUNTER_FILE)

deploy-probation:
	npx ts-node -r tsconfig-paths/register $(UPLOAD_AND_DEPLOY_SCRIPT) $(WASM_DIR)/$(WASM_PROBATION_FILE)

deploy-campaign:
	npx ts-node -r tsconfig-paths/register $(UPLOAD_AND_DEPLOY_SCRIPT) $(WASM_DIR)/$(WASM_CAMPAIGN_FILE)

deploy-enforced-wrapper:
	npx ts-node -r tsconfig-paths/register $(UPLOAD_AND_DEPLOY_SCRIPT) $(WASM_DIR)/$(WASM_ENFORCED_WRAPPER_FILE)

deploy-optional-wrapper:
	npx ts-node -r tsconfig-paths/register $(UPLOAD_AND_DEPLOY_SCRIPT) $(WASM_DIR)/$(WASM_OPTIONAL_WRAPPER_FILE)

generate-bindings-probation:
	$(STELLAR_CLI) contract bindings typescript --contract-id $(PROBATION_CONTRACT_ID) --output-dir $(BINDINGS_DIR)/probation --network testnet --overwrite

generate-bindings-campaign:
	$(STELLAR_CLI) contract bindings typescript --contract-id $(CAMPAIGN_CONTRACT_ID) --output-dir $(BINDINGS_DIR)/campaign --network testnet --overwrite

generate-bindings-counter:
	$(STELLAR_CLI) contract bindings typescript --contract-id $(COUNTER_CONTRACT_ID) --output-dir $(BINDINGS_DIR)/counter --network testnet --overwrite

generate-bindings-enforced-wrapper:
	$(STELLAR_CLI) contract bindings typescript --contract-id $(ENFORCED_WRAPPER_CONTRACT_ID) --output-dir $(BINDINGS_DIR)/enforced_classic_asset_wrapper --network testnet --overwrite

generate-bindings-optional-wrapper:
	$(STELLAR_CLI) contract bindings typescript --contract-id $(OPTIONAL_WRAPPER_CONTRACT_ID) --output-dir $(BINDINGS_DIR)/optional_classic_asset_wrapper --network testnet --overwrite

test-probation-e2e:
	npx jest ./src/tests/probation.integration.test.ts

test-campaign-e2e:
	npx jest ./src/tests/campaign.integration.test.ts

test-counter-e2e:
	npx jest ./src/tests/counter.integration.test.ts