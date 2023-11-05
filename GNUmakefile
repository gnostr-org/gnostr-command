DOCKER=$(shell which docker)
export DOCKER
PWD=$(shell echo `pwd`)
export PWD

-:
	echo $(PWD)
	echo $(DOCKER)

docker:docker-build docker-run
docker-build:
	$(DOCKER) build -f miniscript.dockerfile -t miniscript .
docker-make-miniscript:
	rm ./miniscript || echo
	$(DOCKER) run --rm -v $(PWD):/src   miniscript sh -c "make miniscript"
docker-install-miniscript:
	$(DOCKER) run --rm -v $(PWD):/src   miniscript sh -c "install miniscript /usr/local/bin/ && which miniscript"
.PHONY:docker-miniscript
docker-miniscript:
	rm miniscript || true
	$(DOCKER) run --rm -v $(PWD):/src   miniscript sh -c "make miniscript ##ls"

## docker run --rm --volume /Users/Shared/bitcoincore-dev/miniscript-templates/docker:/src miniscript sh -c 'rm -f ./miniscript || echo && make miniscript && install ./miniscript /usr/local/bin/ && which miniscript'
## g++ -O3 -g0 -Wall -std=c++17 -march=native -flto -Ibitcoin bitcoin/util/strencodings.cpp bitcoin/util/spanparsing.cpp bitcoin/script/script.cpp bitcoin/script/miniscript.cpp compiler.cpp main.cpp -o miniscript

.PHONY:test-command
test-command:
	@cat $< $@ && exit;

example-commands:
	@printf "\n"
	@printf "./docker-miniscript \"make miniscript >/dev/null && ls\""
	@printf "\n"
	@printf "./docker-miniscript \"make miniscript >/dev/null && cat 1.miniscript | ./miniscript\""
	@printf "\n"
	@printf "./docker-miniscript \"echo \'thresh(3, pk(key_1), pk(key_2), pk(key_3), older(12960))\' | ./miniscript\""
	@printf "\n"
	@printf "./docker-miniscript \"echo \'and(pk(A),or(pk(B),or(9@pk(C),older(1000))))\' | ./miniscript\""
	@printf "\n"


-include Makefile
