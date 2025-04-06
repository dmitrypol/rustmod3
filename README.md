Rust module demo using https://github.com/valkey-io/valkeymodule-rs

```bash
valkey-server --enable-module-command yes --aclfile users.acl
valkey-cli
module load ...
# run in bash to load data
./valkey-cli.sh

# in valkey-cli run
127.0.0.1:6379> info modules
# Modules
module:name=rustmod3,ver=1,api=1,filters=1,usedby=[],using=[],options=[]

# rustmod3_stats
rustmod3_commands:default=117,user1=20,user2=30,user3=40
```
