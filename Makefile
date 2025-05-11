IMAGE_NAME=ironclad-mash
IMAGE_TAG=0.1.0

fmt:
	cargo +nightly fmt

run:
	cargo run

build-image:
	podman build -t $(IMAGE_NAME):$(IMAGE_TAG) .

run-image: build-image
	podman run -it --rm \
		--name $(IMAGE_NAME) \
		-p 8080:8080 \
		$(IMAGE_NAME):$(IMAGE_TAG)
