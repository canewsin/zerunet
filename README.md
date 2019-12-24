![Build](http://localhost:43110/14mVJrvB1XqtC4Aq55BmXyf9yXUN9iWwd8/img/build.svg)
![Tests](http://localhost:43110/14mVJrvB1XqtC4Aq55BmXyf9yXUN9iWwd8/img/tests.svg)
![Coverage](http://localhost:43110/14mVJrvB1XqtC4Aq55BmXyf9yXUN9iWwd8/img/coverage.svg)

# zerunet
Pronounced \\zē·rün·net\\ or z-rune-net.
zerunet is an implementation of the ZeroNet client written entirely in the rust programming language.


## IMPORTANT
In the current state of development, getting a working prototype is the number one priority, and this means that some parts may be implemented in a way that is fast to write rather than fast to execute. It is also possible outdated packages will be used temporarily until it's worth replacing them with active ones or, alternatively, update them ourselves.

## Why do we need another ZeroNet?
- Rust is compiled to assembly for maximum efficiency
- A solid rust implementation should be less error-prone

## Code of Conduct
Code of Conducts are for wussies, anyone is welcome to contribute to zerunet or use it in any way they deem useful. We do not care what crimes, fictitious or real, anyone is accused of having committed.

## Imported Libraries
- Bitcoin utilities: rust-bitcoin
- Cryptography: sha2, secp256k1, ripemd160, signatory
- Serialization: serde, serde_json, json_filter_sorted (local)
- InfluxDB: influxdb
- SQLite: rusqlite
- BitTorrent
- Tor Controller
- I2P: i2p-rs
- HTTP Server+WebSocket: actix-web

## Measurements stored to InfluxDB
- Active connections
- Upload (req & bandwidth)
- Download (req & bandwidth)

# Included Components
- Content Manager
- Site Manager

Checklist:
- [ ] Content Manager
  - [x] Content struct
  - [x] Content serialization
  - [x] Content verification
  - [x] Content signing
- [ ] Site Manager
  - [x] Site Manager Actor
  - [ ] Store zites in database
  - [ ] Get zites from database
- [ ] User Manager
  - [ ] Create user identities
  - [ ] Allow using multiple identities from same ID provider
    - [ ] Reload page on ID change to prevent data collection
    - [ ] Make settings storage depend on
- [ ] Web Interface
  - [x] Serve zites
    - [ ] Content type detection
    - [ ] Do a safety check for '../' in paths
    - [x] Serve ZeroNet wrapper
      - [x] Don't serve html without nonce
      - [ ] Don't allow editing of files unless permission is given
  - [ ] API/WebSocket
    - [x] WebSocket endpoint
    - [ ] Implement wrapper actions
- [ ] Peer connectivity
  - [ ] Clearnet
  - [x] LAN Discovery
  - [ ] F2F connections
  - [ ] Tor Integration
  - [ ] I2P Integration
- [ ] Logging
  - [x] env_logger
  - [ ] Influx Logger
