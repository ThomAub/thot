# Thot

An experimentation to serve a ML Model (in TorchScript) via an http api (in rust).

## Commands

```sh
# enable nix-shell (optional)
nix-shell

# build the service
bazel build \\...

# serve the model via an http service made with actix-web
bazel run \\web\actix-wrapper
```

## HTTP api

see [samples](./samples.http) (you can use it with [REST Client for Visual Studio Code](https://marketplace.visualstudio.com/items?itemName=humao.rest-client)

```sh
### healthcheck & status
curl -i http://127.0.0.1:8080/api/status
```

### Known Issues

- current bazel usage is an anti-pattern and a slow pattern (rebuild all on each change), build it's ok to orchestrate build make in python, rust,...It's like using raw `make`.
