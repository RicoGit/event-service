
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
  --method event.EventApi/Handle \
  --data '{ "event": "test", "properties": {"fields": []}, "context": {"app": {"name": "test app"}, "ip": "192.168.0.1", "userAgent": "Smith"}, "sent_at": "1970-01-01T00:00:00Z", "timestamp": "1980-11-21T00:00:00Z" }' \
  --details
```                    

or 

```sh 
# need to be run from directory with this repo
grpcurl -plaintext -import-path ./proto \
  -proto events.proto \
  -d '{ "event": "test", "properties": {"fields": []}, "context": {"app": {"name": "test app"}, "ip": "192.168.0.1", "userAgent": "Smith"}, "sent_at": "1970-01-01T00:00:00Z", "timestamp": "1980-11-21T00:00:00Z" }' \
  [::]:50051 \
  event.EventApi/Handle

```

## Building

```sh                   
    cargo build --release
                              
    ./target/release/event-service
      
    # or with logs
         
    RUST_LOG=info ./target/release/event-service

```