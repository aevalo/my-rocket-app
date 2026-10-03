# my-rocket-app
Rocket app with Docker

## Description
This app follows the instructions from: https://oneuptime.com/blog/post/2026-02-08-how-to-containerize-a-rocket-rust-application-with-docker/view

## Building and Running

### Locally
In one shell:
```shell
cargo run
```

In another shell:
```shell
curl http://localhost:8000
```

### With Docker
```shell
# Build the image
docker build -t my-rocket-app:latest .

# Run the container
docker run -d -p 8000:8000 --name rocket-app my-rocket-app:latest

# Or use docker compose
docker compose up

# Test
curl http://localhost:8000
curl http://localhost:8000/health
curl http://localhost:8000/users

# If you started the server with docker compose, remember to shut it down
docker compose down

# Check image size
docker images my-rocket-app
```
