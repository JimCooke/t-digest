ARCH    := $(shell uname -m)
OSVER   := $(shell uname -r)
PACKAGE := $(shell grep -m1 name Cargo.toml | cut -f2 -d '=' | tr -d ' "')
LNXIMG  := "ekidd/rust-musl-builder"
TRIGGER := $(shell grep tdigest ${HOME}/.git-triggers | cut -f2 -d '=')
PROJECT := "10554030"
VERSION := $(shell cat VERSION)

compile:  CMD := "cat Cargo.toml | sed 's/version =.*/version = \"${VERSION}\"/' > Cargo.toml.new" 
compile: lint
	./scripts/version.sh patch
	eval ${CMD}
	mv Cargo.toml.new Cargo.toml
	cargo build --release

lint:
	rustup default stable
	rustup component add clippy
	rustup component add rustfmt
	cargo fmt
	cargo clippy

unit-test: compile test-data
	cargo test

unit-test-verbose: compile test-data
	cargo test -- --nocapture

test-data:
	if [ ! -d data ]; then \
          mkdir data; \
	  r < r/large-uniform.r --no-save; \
	  r < r/small-uniform.r --no-save; \
	  r < r/large-normal.r --no-save; \
	  r < r/small-normal.r --no-save; \
	  r < r/large-skew.r --no-save; \
	  r < r/small-skew.r --no-save; \
	  r < r/mass-point-left.r --no-save; \
	  r < r/mass-point-right.r --no-save; \
	fi
	if [ ! -d centroids ]; then \
          mkdir centroids; \
        fi

build: darwin linux 
	git add -A
	git commit -m "Make build ${VERSION}"
	git pull origin master
	git push origin master

darwin: darwin_clean refresh
	cargo build --release
	cp -p target/release/${PACKAGE} staging/${PACKAGE}_${VERSION}_darwin_${ARCH}

darwin_clean: stagedir
	rm -f staging/${PACKAGE}_${VERSION}_darwin_${ARCH}

linux: linux_clean
	if [ -f ${PWD}/CBAInternalRootCA.pem ]; then \
	  docker run --rm -it --name "rust-compiler" \
	  -e SSL_CERT_FILE=/etc/ssl/certs/CBAInternalRootCA.pem \
	  -v "${HOME}"/.cargo:/.cargo \
	  -v "${HOME}"/.rustup:/.rustup \
	  -v "${PWD}/CBAInternalRootCA.pem":/etc/ssl/certs/CBAInternalRootCA.pem:ro \
	  -v "${PWD}":/home/rust/src ${LNXIMG} cargo build --release; \
	else \
	  docker run --rm -it --name "rust-compiler" \
	  -v "${HOME}"/.cargo:/.cargo \
	  -v "${HOME}"/.rustup:/.rustup \
	  -v "${PWD}":/home/rust/src ${LNXIMG} cargo build --release; \
	fi
	cp -p target/x86_64-unknown-linux-musl/release/${PACKAGE} staging/${PACKAGE}_${VERSION}_linux_${ARCH}

linux_clean: stagedir
	rm -f staging/${PACKAGE}_${VERSION}_linux_${ARCH}

patch:
	./scripts/version.sh patch

feature:
	./scripts/version.sh feature

upgrade:
	./scripts/version.sh upgrade

refresh:
	rustup default stable
	rustup update
	cargo update

test: downloaddir stagedir
	curl --request POST \
	  --form token=${TRIGGER} \
	  --form ref=master \
	  --form "variables[STAGE]=test" \

release : downloaddir stagedir
	curl --request POST \
	  --form token=${TRIGGER} \
	  --form ref=master \
	  --form "variables[STAGE]=release" \
	  https://gitlab.com/api/v4/projects/${PROJECT}/trigger/pipeline

push : downloaddir stagedir
	curl --request POST \
	  --form token=${TRIGGER} \
	  --form ref=master \
	  --form "variables[STAGE]=push" \
	  https://gitlab.com/api/v4/projects/${PROJECT}/trigger/pipeline

clean:
	rm -rf target rc data centroids 

downloaddir:
	if [ ! -d releases ]; then \
	  mkdir releases; \
	fi

stagedir:
	if [ ! -d staging ]; then \
	  mkdir staging; \
	fi

.PHONY: clean linux_clean darwin_clean
