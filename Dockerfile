FROM ubuntu

# update ubuntu
RUN apt-get update -y

# get ubuntu deps
RUN apt-get install -y libmysqlclient-dev
RUN apt-get install -y curl
RUN apt-get install -y build-essential

# set env for rustup (from official rust alpine dockerfile)
ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH \
    RUST_VERSION=1.51.0

# get rustup
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > /rustup.sh
RUN chmod u+x /rustup.sh
RUN /rustup.sh -y


# install cargo modules
RUN cargo install diesel_cli --no-default-features --features mysql

# copy deps & lockfile
COPY ./Cargo.toml .
COPY ./Cargo.lock .

#copy source code.
COPY . .

# build
RUN cargo check

CMD ["cargo", "run"]
