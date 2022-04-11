#

./target/release/node-kitties build-spec --disable-default-bootnode --chain kit > kitty_spec.json

#

./target/release/node-kitties \
--base-path ./data/node01 \
--chain ./kitty_spec.json \
--port 30333 \
--ws-port 9945 \
--rpc-port 9933 \
--telemetry-url "wss://telemetry.polkadot.io/submit/ 0" \
--validator \
--rpc-methods Unsafe \
--name MyNode01 \
--node-key 0xc49a632bc29af393b9fbe2571cd5b84ae6ad664d2ea70bcff500f43b02d0ef8d

./target/release/node-kitties \
--base-path ./data/node02 \
--chain ./kitty_spec.json \
--port 30334 \
--ws-port 9946 \
--rpc-port 9934 \
--telemetry-url "wss://telemetry.polkadot.io/submit/ 0" \
--validator \
--rpc-methods Unsafe \
--name MyNode02 \
--bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWNHp2HvF7A8hJgjVGZZ9afcF54fbBKRkz3cbZo9bCD8fF \
