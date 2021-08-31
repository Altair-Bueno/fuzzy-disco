FROM node as build-vue
ADD ./disco-vue ./disco-vue
WORKDIR ./disco-vue
RUN npm install
RUN npm run build

FROM rust as build-rust
ADD ./disco-core ./disco-core
WORKDIR ./disco-core
RUN rustup target add  x86_64-unknown-linux-musl
RUN cargo build --target x86_64-unknown-linux-musl --release

FROM alpine
COPY --from=build-vue disco-vue/dist/ static/
COPY --from=build-rust disco-core/target/x86_64-unknown-linux-musl/release/disco-core .
#CMD ["chmod", "+x","disco-core"]
CMD ["./disco-core"]