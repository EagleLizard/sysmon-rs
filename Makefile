VERSION ?= 1.0.0
COMMIT_HASH := $(shell git rev-parse --short HEAD)
BUILD_DATETIME := $(shell date +%Y-%m-%d_%H-%M-%S)  # Including time in the format YYYYMMDD-HHMMSS

IMAGE_NAME := sysmon-rs
TAG := $(VERSION)-$(BUILD_DATETIME)

build:
	docker build -t $(IMAGE_NAME):$(TAG) .
	docker tag $(IMAGE_NAME):$(TAG) $(IMAGE_NAME):latest
watch:
	cargo watch -x run