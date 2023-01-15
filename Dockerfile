FROM rust
RUN apt-get update && apt-get install -y cmake libsdl2-dev