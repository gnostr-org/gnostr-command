.PHONY:- help
-:
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?##/ {printf "\033[36m%-15s\033[0m %s\n", $$1, $$2}' $(MAKEFILE_LIST)
	@echo
more:## 	more help
	@sed -n 's/^##//p' ${MAKEFILE_LIST} | column -t -s ':' |  sed -e 's/^/	/'
	#$(MAKE) -f Makefile help

#USAGE: node run.js [node|rust|nodeServer|rustServer]
-include npm.mk
node-run-js-node:## 	node-run-js-node
	node run.js node
node-run-js-rust:## 	node-run-js-rust
	node run.js rust
node-run-js-nodeserver:## 	node-run-js-nodeserver
	node run.js nodeServer
node-run-js-rustserver:## 	node-run-js-rustserver
	node run.js rustServer
