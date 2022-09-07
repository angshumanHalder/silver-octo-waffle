Silver-Octo-Waffle
==================

This is a POC for anonymous-voting system inspired by the paper [**An efficient and effective Decentralized Anonymous Voting System**](https://arxiv.org/pdf/1804.06674.pdf). The contract is built on Near protocol. The generated ballot is signed by a ring signature to ensure anonimity and the ballot itself is ecrypted by applying unlinkable payments scheme proposed by Nicolas van Saberhagen.

**Note** : _This is not a scalable system and the amount of gas required to decrypt a ballot is quite high._ 

Runtime or Compiler dependencies
================================

1. rustc and rustup for smart contract
2. nodejs > 18
3. wasm-pack

Quick Start
===========

1. Install dependencies `npm run deps-install`
2. Deploy smart contract `npm run deploy`
3. Copy the contract name from contract/neardev/dev-account.env to frontend/.env as VITE_CONTRACT_NAME=${contractname}
4. Run `npm start` to start the frontend dev server.

Exploring The Code
==================

1. The smart-contract code lives in the `/contract` folder. In blockchain apps the smart contract is the "backend" of your app.
2. The frontend code lives in the `/frontend` folder. `/frontend/index.html` is a great
   place to start exploring. Note that it loads in `/frontend/index.tsx`,
   this is your entrypoint to learn how the frontend connects to the NEAR blockchain.
3. Ring signature and code for generating keys are in `/frontend/ring-sig/`.
4. Run tests on contract: `npm test:unit`, this will run the unit tests. 


Deploy
======

Every smart contract in NEAR has its [own associated account][NEAR accounts]. 
When you run `npm run deploy`, your smart contract gets deployed to the live NEAR TestNet with a temporary dev account.
When you're ready to make it permanent, here's how:


Step 0: Install near-cli (optional)
-------------------------------------

[near-cli] is a command line interface (CLI) for interacting with the NEAR blockchain. It was installed to the local `node_modules` folder when you ran `npm install`, but for best ergonomics you may want to install it globally:

    npm install --global near-cli

Or, if you'd rather use the locally-installed version, you can prefix all `near` commands with `npx`

Ensure that it's installed with `near --version` (or `npx near --version`)


Step 1: Create an account for the contract
------------------------------------------

Each account on NEAR can have at most one contract deployed to it. If you've already created an account such as `your-name.testnet`, you can deploy your contract to `near-blank-project.your-name.testnet`. Assuming you've already created an account on [NEAR Wallet], here's how to create `near-blank-project.your-name.testnet`:

1. Authorize NEAR CLI, following the commands it gives you:

      near login

2. Create a subaccount (replace `YOUR-NAME` below with your actual account name):

      near create-account near-blank-project.YOUR-NAME.testnet --masterAccount YOUR-NAME.testnet

Step 2: deploy the contract
---------------------------

Use the CLI to deploy the contract to TestNet with your account ID.
Replace `PATH_TO_WASM_FILE` with the `wasm` that was generated in `contract` build directory.

    near deploy --accountId near-blank-project.YOUR-NAME.testnet --wasmFile PATH_TO_WASM_FILE


Step 3: set contract name in your frontend code
-----------------------------------------------

Modify the line in `src/config.js` that sets the account name of the contract. Set it to the account id you used above.

    const CONTRACT_NAME = process.env.CONTRACT_NAME || 'near-blank-project.YOUR-NAME.testnet'

Imporovement / Enhancements
===========================

1. Move the poll data to a decentralised database or ipfs rather than storing it in smart contract.
2. We can use Diffie-Hellman key exchange to create the shared public-private key pair instead of storing them in smart contract.
3. Decrypting a ballot right now takes up high amount of gas and thus large number of ballots cannot be decrypted in a single transaction.
A better solution is to have a serverless function or a centralised backend that calls the smart contract in a loop to decrypt a single ballot over and over again rather than computing the voting result in a single smart contract call. 