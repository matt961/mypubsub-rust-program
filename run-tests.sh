#!/usr/bin/env bash

podman network create testnet
podman run --name rabbitmq --network testnet -p 15672:15672 rabbitmq:management
podman run -d --rm -it --name pub-rabbitmq --network testnet pub-rabbitmq:latest
podman run -d --rm -it --name sub-rabbitmq --network testnet sub-rabbitmq:latest
podman logs --follow pub-rabbitmq sub-rabbitmq
