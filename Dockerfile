FROM rust AS build
WORKDIR /usr/src/app
COPY Cargo.lock Cargo.lock
COPY Cargo.toml Cargo.toml
COPY ./src ./src
RUN cargo build --release

FROM gcr.io/distroless/cc
COPY .env .env
COPY --from=build /usr/src/app/target/release/addinfo-client .
CMD ["/addinfo-client"]
