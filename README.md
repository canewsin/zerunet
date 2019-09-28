# zerunet
Pronounced Z-Rune-Net.
zerunet is an implementation of the ZeroNet client written entirely in the rust programming language.

## IMPORTANT
In the current state of development, getting a working prototype is the number one priority, and this means that some parts may be implemented in a way that is fast to write rather than fast to execute. It is also possible outdated packages will be used temporarily until it's worth replacing them with active ones or, alternatively, update them ourselves.

## Potential Advantages of Rust
- Faster than python
- Less error-prone

## Code of Conduct
Code of Conducts are for wussies, anyone is welcome to contribute to zerunet or use it in any way they deem useful. We do not care what crimes, fictitious or real, anyone is accused of having committed.

# Imported Components
- Bitcoin utilities: rust-bitcoin
- Cryptography: sha2, secp256k1, ripemd160, signatory
- Serialization: serde, serde_json, json_filter_sorted (local)
- InfluxDB: influxdb
- SQLite
- BitTorrent
- Tor Controller
- HTTP Server

# Included Components
- Content Manager
- Site Manager

- [ ] Content Manager
  - [x] Content struct
  - [x] Content serialization
  - [x] Content verification
  - [ ] Content signing
- [ ] Site Manager
- [ ] Peer Manager
- [ ] Web Interface
- [ ] Tor Integration
- [ ] Logging
