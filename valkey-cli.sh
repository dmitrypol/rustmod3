valkey-cli flushall

for i in {1..10}; do
    valkey-cli set default-$i $i
done

for i in {1..20}; do
    valkey-cli --user user1 --pass "" set user1-$i $i
done

for i in {1..30}; do
    valkey-cli --user user2 --pass "" set user2-t$i $i
done

for i in {1..40}; do
    valkey-cli --user user3 --pass "" set user3-$i $i
done

