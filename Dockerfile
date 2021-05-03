##
##  Copyright 2021 Catgirl.Moe contributors <https://catgirl.moe>
##
##  Licensed with GNU Affero General Public License v3.0 or later
##

FROM node:alpine as tailwind

ARG NODE_ENV

COPY tailwind.config.js tailwind.config.js
COPY templates templates
RUN NODE_ENV=$NODE_ENV npx tailwindcss-cli@latest build -o tailwind.css


FROM rust:slim as builder

# Install required dependencies
#RUN apk add --no-cache musl-dev openssl-dev mariadb-dev npm
RUN DEBIAN_FRONTEND=noninteractive apt-get update
RUN DEBIAN_FRONTEND=noninteractive apt-get -y upgrade
RUN DEBIAN_FRONTEND=noninteractive apt-get -y install --no-install-recommends default-libmysqlclient-dev
RUN DEBIAN_FRONTEND=noninteractive apt-get clean
RUN rm -rf /var/lib/apt/lists/*

# Create new project
RUN USER=root cargo new --bin memebin

# Create user for memebin and chown directory
RUN addgroup --system --gid 1000 memebin && adduser --system --uid 1000 --gid 1000 memebin
RUN chown -R memebin:memebin /memebin

# Switch to the project directory and user
WORKDIR /memebin
USER memebin

# Expose port and set entrypoint
EXPOSE 8080
CMD ["./target/release/memebin"]

# Precompile dependencies
COPY --chown=memebin:memebin Cargo.toml Cargo.toml
RUN cargo build --release

# Remove precompile garbage
RUN rm src/*.rs
RUN rm target/release/deps/memebin*

# Copy over assets
COPY --chown=memebin:memebin assets assets/
RUN mkdir -p ./assets/css/
COPY --chown=memebin:memebin --from=tailwind tailwind.css ./assets/css/tailwind.css

# Add the source code and build the final project
COPY --chown=memebin:memebin src src
COPY --chown=memebin:memebin templates templates
RUN cargo build --release