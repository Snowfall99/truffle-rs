FROM rustlang/rust:nightly

WORKDIR /src
COPY . ./
RUN cargo build --bin truffle_rs
# CMD ["./target/debug/truffle_rs"]
