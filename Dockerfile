# Altair Bueno
# fuzzy-disco

# Build Vue.js static webpage sources
FROM node as build-vue
ADD ./disco-vue ./disco-vue
WORKDIR ./disco-vue
RUN npm install
RUN npm run build

# Build Rocket Server
# CAUTION: Alpine linux may be tricky, use normal distribution instead
FROM rust:alpine3.14 as build-rust
ADD ./disco-core ./disco-core
WORKDIR ./disco-core
# No needed on normal rust container
RUN apk add --no-cache musl-dev
RUN rustup target add  x86_64-unknown-linux-musl
RUN cargo build --target x86_64-unknown-linux-musl --release

# Rocket server deployment
FROM alpine
WORKDIR /fuzzy-disco
COPY --from=build-vue disco-vue/dist/ static/
COPY --from=build-rust disco-core/target/x86_64-unknown-linux-musl/release/disco-core .
COPY --from=build-rust disco-core/Rocket.toml .
ENV ROCKET_CONFIG=/fuzzy-disco/Rocket.toml
CMD ["./disco-core"]