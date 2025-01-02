# Redis strsim module

String similarity with [strsim](https://docs.rs/strsim/0.11.1/strsim/index.html) in a Redis module with [RedisLabsModules/redismodule-rs](https://github.com/RedisLabsModules/redismodule-rs).

## Quickstart

```sh
git clone https://github.com/joshrotenberg/redis-strsim-module
cd redis-strsim-module
cargo build
# redis 7.x on Mac OS
redis-server --loadmodule target/debug/libredis_strsim.dylib
# in another terminal
redis-cli 
127.0.0.1:6379> STRSIM.HAMMING hamming hammers
(integer) 3
```

## Integration tests

```sh
cargo test  --features integration-tests 
```
