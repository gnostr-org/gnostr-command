DOCKER=$(shell which docker)
export DOCKER
PWD=$(shell echo `pwd`)
export PWD

-:
	echo $(PWD)
	echo $(DOCKER)

docker:docker-build docker-run
docker-build:
	$(DOCKER) build -t miniscript .
docker-run:
	$(DOCKER) run --rm -v $(PWD):/src   miniscript sh -c "make && install ./miniscript /usr/local/bin && miniscript"
