# rm_text_share
A project to share the text conversion files stored on the remarkable over the internet in realtime to enable a smooth interface to augment a person's digitally stored text filesystem with writing done on their remarkable tablet.

# Build
```
cargo install cross
cross build --target armv7-unknown-linux-gnueabihf --release
```

# Deploy
```
./deploy.sh
```