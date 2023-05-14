FROM rust
WORKDIR /usr/src/benchmark

# https://github.com/nodesource/distributions
RUN curl -fsSL https://deb.nodesource.com/setup_20.x | bash - && apt-get install -y nodejs
RUN cargo install hyperfine

COPY ./benchmark Cargo.toml Cargo.lock .
COPY ./crates ./crates
COPY ./xtask ./xtask

RUN npm i
RUN node run.js