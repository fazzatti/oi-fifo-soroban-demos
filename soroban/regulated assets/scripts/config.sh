# Environment Variables
export NETWORK_NAME="futurenet" # Name of the Stellar network to deploy the contract (e.g., public, testnet, futurenet)
export DATA_DIR="./data"

#Accounts
export ADMIN_PK="GALIALRZJ5EU2IJJSIQEA3D3ZIEHK5HPBHZJFUEPTGQU3MYEKKIUINTY"  #The Public key of the Admin account
export ADMIN_SK="SCVCU3KWSSLML7MVWKJJB5VKK6R3CMILO5733IBJJ3C5APBWJUOCDUVY"  #The Secret key of the Admin account

# Asset Controller
export AC_WASM="../asset-controller/target/wasm32-unknown-unknown/release/asset_controller.wasm"
export AC_DEPLOYER_ACCOUNT_SK=${ADMIN_SK}
export AC_DEPLOY_OUTPUT_FILE="ac-deploy"
export AC_CONTRACT_ID=$(<"${DATA_DIR}/${AC_DEPLOY_OUTPUT_FILE}-out")


# Regulated Asset
export RA_WASM="../regulated-asset/target/wasm32-unknown-unknown/release/regulated_asset.wasm"
export RA_DEPLOYER_ACCOUNT_SK=${ADMIN_SK}
export RA_DEPLOY_OUTPUT_FILE="ra-deploy"

export RA_ADMIN_PK=${ADMIN_PK}
export RA_ADMIN_SK=${ADMIN_SK}
export RA_DECIMAL=7
export RA_NAME="FifoCoin"
export RA_SYMBOL="FIFO"


#ACCOUNTS

export USER_PK="GALTHRNM4QZCHF7T3L5BMTWXEIUNNMBXADSFSYTYZ6WFMMIV5HYMQ7TQ"   #The Public key of the account that represents a user
export USER_SK="SDGXLEHK2ZC57TPMNXCUXYF42OPXPST3KNOP4LYA3OFTPGCTQ4SIB2S2"   #The Secret key of the account that represents a user

