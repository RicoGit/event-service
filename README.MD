
# Event service

// todo

## Running

```sh      
  # simple run
  cargo run 
                    
  # or with logs
  RUST_LOG=debug cargo run 
```      

Call Grpc service

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
# need to be run from directory with this repo
grpcurl -plaintext -import-path ./proto \
  -proto helloworld.proto \
  -d '{"name": "Tonic"}' \
  [::]:50051 \
  helloworld.Greeter/SayHello

```

## Building

```sh                   
    cargo build --release
                              
    ./target/release/event-service
      
    # or with logs
         
    RUST_LOG=info ./target/release/event-service

```