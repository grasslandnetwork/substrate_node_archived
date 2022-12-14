# Substrate port of Grassland Full Node
## This is an archived version that's no longer used. Instead, use [https://github.com/grasslandnetwork/substrate_node](https://github.com/grasslandnetwork/substrate_node)


### This is an alpha version for testing. It's not ready for use yet.

## Build
Use the following command to build the node without launching it

```sh
cargo build --release
```

Note: This node requires Grassland's [substrate inference container](https://github.com/grasslandnetwork/substrate_inference_container) in order to work.


## Run
Once the node has been built, type the following command to run it

```sh
./target/release/node-grassland --dev
```
Then start your [substrate inference container](https://github.com/grasslandnetwork/substrate_inference_container).


## Stop
If you want to stop the node, type CTRL-C


## Detailed Instructions
Detailed instructions are available in the "docs" directory
