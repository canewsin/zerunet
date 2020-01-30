![Build](http://localhost:43110/14mVJrvB1XqtC4Aq55BmXyf9yXUN9iWwd8/img/build.svg)
![Tests](http://localhost:43110/14mVJrvB1XqtC4Aq55BmXyf9yXUN9iWwd8/img/tests.svg)
![Coverage](http://localhost:43110/14mVJrvB1XqtC4Aq55BmXyf9yXUN9iWwd8/img/coverage.svg)

# zerunet
Pronounced \\zē·rün·net\\ or z-rune-net.
zerunet is an implementation of the ZeroNet client written entirely
in the rust programming language.

## IMPORTANT
In the current state of development, getting a working prototype is the
number one priority, and this means that some parts may be implemented
in a way that is fast to write rather than fast to execute. It is also
possible outdated packages will be used temporarily until it's worth
replacing them with active ones or, alternatively, update them ourselves.

## Why do we need another ZeroNet?
- Rust is compiled to assembly for maximum efficiency
- Unlike Python, Rust has been designed with concurrency in mind,
  this means it will be able to run in multiple threads, reducing
  zite lag when many things are going on in the backend.
- Rust has one of the fastest web server libraries: Actix-web, and a
  lot of what ZeroNet does is web serving. Actix itself is an actor
  framework, and zerunet will be build using the actor model for
  safe and intuitive concurrency.
- A solid Rust implementation should be less error-prone due to its
  really strict type system and memory ownership.
- This will allow me to implement additional features I'd like to see in ZeroNet:
  - Peer connections over I2P and LokiNet
  - Implement Merger-like functionality using JSON-LD
  - Integrate IPFS, RetroShare, GnuNet or even Freenet
  - Store measurements and logs in InfluxDB for optimization during development,
    as well as end-user monitoring of resources used by zerunet.
  When porting ZeroNet to Rust, I'll keep these in mind when structuring my code.

## Code of Conduct
Code of Conducts are for wussies, anyone is welcome to contribute to
zerunet or use it in any way they deem useful. We do not care about
your past, your present or your future. We do not care about what crimes,
fictitious or real, anyone is accused of having committed. Just write
good code that does what it's supposed to do!

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

# Roadmap

1. Limited API prototype
  - All official sites (Hello, Talk, Me, etc.) work passively (anything that does not alter data)
    - Channel Subscriptions (site Actor has Vec<Addr<Websocket>>)
  - Find peers on LAN
  - Make connections to peers over clearnet
2. Online Peer Connections
  - Find peers through trackers and pex.
  - Make connections to peers over Tor, I2P and maybe LokiNet.
3. Full Core API prototype
  - All API functions that are not part of a plugin are implemented.
4. Priority Plugin Pack
  - Bigfile
  - Multiuser
  - Cryptmessage
  - Optional
5. Plugin Pack #2
  - Merger
  - Mute
  - Cors
  - Newsfeed
  - MergerSite
6. Additional Features
  - IPFS
  - RetroShare
  - GnuNet
  - Freenet

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
    - [ ] Websockets can listen to changes on specific site
    - [ ] Websockets can listen to changes to any site
    - [ ] Implement wrapper actions
    - [ ] Core API
- [ ] Peer connectivity
  - [ ] Clearnet
  - [x] LAN Discovery
  - [ ] F2F connections
  - [ ] Tor Integration
  - [ ] I2P Integration
- [ ] Peer messaging
  - [ ] GetFile
- [ ] Logging
  - [x] env_logger
  - [ ] Influx Logger
