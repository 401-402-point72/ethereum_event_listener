# ethereum_event_listener
Event listener for the Ethereum Blockchain

## Repo Structure
```sh
.
├── ethereum_event_listener
│   ├── Cargo.lock
│   ├── Cargo.toml
│   ├── src
│   │   ├── main.rs
│   │   ├── web_3
│   │   │   └── s_3.rs
│   │   └── web_3.rs
│   └── tests
│       └── test.json
└── readme.md
```

The `/src` folder contains all of the repo's logic for reading blocks from the blockchain, processing the blocks into json format, 
and sending the data to be stored in an AWS S3 database.

