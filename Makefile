DEFAULT = build

build:
	cargo build
clean:
	rm -rf ./target/
docker-build:
	docker build . --tag ip-server:latest