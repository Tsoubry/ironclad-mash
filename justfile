image_name := "ironclad-mash"
image_tag := "0.2.0"

default: release

fmt:
    cargo +nightly fmt

run:
    cargo run

build:
    cargo build

release:
    cargo build --release

deny:
    cargo deny check advisories bans sources

check:
    cargo check

check-fmt:
    cargo +nightly fmt --all -- --check

clippy:
    cargo clippy -- -D warnings

test:
    cargo test -- --nocapture

build-image:
	podman build -t {{image_name}}:{{image_tag}}

run-image: build-image
	podman run -it --rm \
		--name {{image_name}} \
		-p 8080:8080 \
		{{image_name}}:{{image_tag}}

check-all: check check-fmt clippy test deny
