ARG RUST_VERSION=1.75
ARG APP_NAME=api-lightning

################################################################################
# step 1: build the rust application
FROM --platform=$BUILDPLATFORM rustlang/rust:nightly AS build
#FROM --platform=$BUILDPLATFORM rust:${RUST_VERSION}-alpine AS build
ARG APP_NAME
WORKDIR /api-lightning

COPY . .

RUN apt-get update && apt-get install -y cmake
RUN cargo +nightly build --release


# step 2: create the runtime image
FROM alpine:latest AS final

RUN apk add --update curl cmake 

WORKDIR /api-lightning

COPY --from=build /api-lightning/target/release . 

# Expose the port that the application listens on.
EXPOSE 8181

#CMD ["/"]