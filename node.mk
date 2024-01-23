node-sdk-run-js-node:###       node-sdk-run-js-node
##     node-sdk-run-js-node
		node sdk/run.js node
node-sdk-run-js-node-6102:###  node-sdk-run-js-node-6102
##     node-sdk-run-js-node-6102
		node sdk/run.js node 6102
node-sdk-run-js-rust:###       node-sdk-run-js-rust
##     node-sdk-run-js-rust
		node sdk/run.js rust
node-sdk-run-js-rust-2106:###  node-sdk-run-js-rust-2106
##     node-sdk-run-js-rust-2106
		node sdk/run.js rust 2106



.PHONY:examples-nodejs
examples-nodejs:##     examples-nodejs
		cd examples-nodejs && make
examples-nodejs-node:##        examples-nodejs-node
		cd examples-nodejs && node run.js node
examples-nodejs-rust:##        examples-nodejs-rust
		cd examples-nodejs && node run.js rust
examples-nodejs-nodeserver:##  examples-nodejs-nodeserver
		cd examples-nodejs && node run.js nodeServer
examples-nodejs-rustserver:##  examples-nodejs-rustserver
		cd examples-nodejs && node run.js rustServer
