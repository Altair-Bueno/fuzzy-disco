# `disco-core`

<!-- cargo-sync-readme start -->

This crate contains the source code for `disco-core`, the main component on
`fuzzy-disco`. The program mainly focuses on two tasks

- Providing a fast and reliable JSON API
- Serving a website written in [Vue.js](../disco-vue)

# API

You can find the whole documentation for the API under the [`api`](src/api/)
module

# Build and run

1. Install the rust toolchain from
   the [official website](https://www.rust-lang.org)
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
```

4. Copy your static website to `static/`

```bash
cp <static> static/
```

5. Build and run

```bash
cargo run --release
cargo run
```

<!-- cargo-sync-readme end -->
