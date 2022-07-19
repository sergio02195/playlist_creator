FROM rust:latest

WORKDIR /usr/src/playlist_creator_core
COPY . .

RUN cargo install --path .

CMD ["playlist_creator_core"]