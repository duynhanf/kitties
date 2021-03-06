./target/release/node-kitties \
--base-path ./data/node03 \
--chain ./kitty_spec.json \
--port 30335 \
--ws-port 9947 \
--rpc-port 9935 \
--telemetry-url "wss://telemetry.polkadot.io/submit/ 0" \
--validator \
--rpc-methods Unsafe \
--name MyNode03 \
--bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWNHp2HvF7A8hJgjVGZZ9afcF54fbBKRkz3cbZo9bCD8fF
