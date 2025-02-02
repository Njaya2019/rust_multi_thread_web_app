# rust multi thread web app
A very simple rust web app that demonstrates processing of incomming requests with number of threads available.
Also to allow gracefully shut down of threads, this is simulated by only taking 2 requests and watching the threads
shutdown before the main thread.

## Language
```
Rust
```

## Views
### Home
```
GET 127.0.0.1:7878/
```

### Invalid resource
```
GET 127.0.0.1:7878/invalid_resource
```

### Run
```
cargo run
```
