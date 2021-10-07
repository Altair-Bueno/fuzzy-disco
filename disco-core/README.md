# `disco-core`

<!-- cargo-sync-readme start -->

This crate contains the source code for `disco-core`, the main component on
`fuzzy-disco`. The program mainly focuses on two tasks

- Providing a fast and reliable JSON API
- Serving a website written in [Vue.js](../disco-vue)

# Required services

- [Mongodb](https://www.mongodb.com/es)
- [Redis](https://redis.io)

# API

You can find the whole documentation for the API under the [`api`](https://docs.rs/disco-core/latest/disco-core/api/)
module

# Build and run

1. Install the rust toolchain from the [official website](https://www.rust-lang.org)
2. Start a Mongodb database. You can either use a Docker container
(recommended) or install mongo on your local machine
2. Clone this repo and cd to disco-core

```bash
git clone https://github.com/Altair-Bueno/fuzzy-disco
cd disco-core
```

3. Set up the following environment variables:

```bash
export MONGODB_URI="mongodb://<username>:<password>@<ip>:<port>/"
export REDIS_URI="redis://<username>:<password>@<ip>:<port>/"
```

4. Copy your static website to `static/`
```bash
cp <static> static/
```

> If you don't have any website, just create an empty `static/` folder


5. Build and run

```bash
cargo run --release
cargo run
```

<!-- cargo-sync-readme end -->
