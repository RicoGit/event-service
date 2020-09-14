
# Event service

// todo

## Run locally

```sh
# need to be run from directory with this repo
prototool grpc proto \
  --address [::]:50051 \
  --method helloworld.Greeter/SayHello \
  --data '{"name":"test"}' \
  --details
```                    

or 

```sh
grpcurl -plaintext -import-path ./proto \
  -proto helloworld.proto \
  -d '{"name": "Tonic"}' \
  [::]:50051 \
  helloworld.Greeter/SayHello

```
