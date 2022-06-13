# Bridge watcher

This project watch carbonable smart contract on juno network and is looking for transfer_nft event to create entries in the DB.


## How to install

install libpg
```bash
brew install libpg
```

install diesel:
```bash
cargo install diesel_cli --no-default-features --features postgres 
``` 

build and install project:

```bash
cargo build
cargo install --path ..
```

## How to use it

### Configure the project

Now that the project is installed you need to configure it. You can do that using the *config* binary

The config contain an lcd-address and the destination wallet for the nft transfert.
```bash
config --lcd-address=https://lcd-juno.itastakers.com --wallet-dest=juno1lwah780cd0cpq3qmv30sw4qf7twx9zajfx2dh6
```

You can get the current config like that :

```bash
bridge_config
```

### Add/Remove/list contract to watch

You can add a new contract like that :
```bash
bridge_new_contract --name carbo --address juno1xkjjmt6nfxke5fa9mtsupuqvf4h2kk26u4srzul7h5a6dcxznwqs0d5y4f
```

You can list all the bridged contracts like that : 

```bash
bridge_contract 
```

And you can remove a contract to the watch this way

```bash
bridge_delete_contract --name carbo2
```

### Launching the bridge

Now you can just launch the bridge like that

```bash
RUST_LOG=info bridge_watch 
```

if you want more or less log level you can
change the RUST_LOG variable to debug.


## Test session

```bash
# setup DB
diesel setup
diesel migration run

# configure the bridge
RUST_LOG=info bridge_config --lcd-address=https://lcd-juno.itastakers.com  --wallet-dest=juno1a500tdpehjejf8vcerte8a5kd2vgqkevwy8j3g

# add NFT contract to watch 
RUST_LOG=info bridge_new_contract --name carbo1 --address juno12uzam70vndkakuupksvsasrxwsehz8n8j5s8sw2y0vv5d94dxh0qwmddq4

# add another NFT contract to watch 
RUST_LOG=info bridge_new_contract --name carbo2 --address juno13g5r0tmmngmm9d0clwa7exjamxxxag5p5fgdra7qjtaexdg6yprq5298fn

# run the bridge watcher
RUST_LOG=info bridge_watch
```