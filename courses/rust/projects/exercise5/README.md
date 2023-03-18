## Set up an official redis server

```
docker compose  -up d
```

### Connect to default port by installing redis-cli 

```
❯ redis-cli
127.0.0.1:6379>
... 
```

### Run redis simple client(default port= 6379)
```
REDIS_PORT=6380 cargo run --bin redis-simple-client
```

### Run redis simple server(default port= 6379)
```
REDIS_PORT=6380 cargo run --bin redis-simple-server
```