# Build containers with podman (or docker)

```
# start subscriber
cargo build --release --bin subapp && podman build -f Dockerfile_sub-rabbitmq -t sub-rabbitmq:latest

# start publisher
cargo build --release --bin pubapp && podman build -f Dockerfile_pub-rabbitmq -t pub-rabbitmq:latest
```

# Create resources and start containers

_Note: will error on first two commands if resources exist, but there is no
issue._

```
./run-tests.sh
```

CTRL+c to stop following logs

To stop containers:

```
podman stop pub-rabbitmq sub-rabbitmq
```
