FROM citusdata/citus
RUN apt-get update && apt-get install -y curl gcc make build-essential libz-dev zlib1g-dev strace libssl-dev pkg-config clang

# Install Rust using the official installer script
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Installing pgrx and pointing to installed citus
RUN cargo install --locked cargo-pgrx && cargo pgrx init \
    --pg16=/usr/lib/postgresql/16/bin/pg_config

# package
RUN apt-get install -y postgresql-server-dev-16

WORKDIR /tmp/hamming
COPY . .
RUN cargo pgrx install --release 