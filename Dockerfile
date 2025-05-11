FROM docker.io/rust:alpine3.21 AS build

RUN apk update && \
    apk upgrade --no-cache && \
    apk add --no-cache lld mold musl musl-dev libc-dev cmake clang clang-dev openssl file \
        libressl-dev git make build-base bash curl wget zip gnupg coreutils gcc g++  zstd binutils ca-certificates upx

WORKDIR /ironclad-mash
COPY . ./

RUN cargo build --release


FROM docker.io/alpine:3.21 AS files

# mailcap is used for content type (MIME type) detection
# tzdata is used for timezones info
RUN apk update && \
    apk upgrade --no-cache && \
    apk add --no-cache ca-certificates mailcap tzdata

RUN update-ca-certificates

ENV USER=app-user
ENV UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"


FROM scratch

# /etc/nsswitch.conf may be used by some DNS resolvers
# /etc/mime.types may be used to detect the MIME type of files
COPY --from=files --chmod=444 \
    /etc/passwd \
    /etc/group \
    /etc/nsswitch.conf \
    /etc/mime.types \
    /etc/

COPY --from=files --chmod=444 /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
COPY --from=files --chmod=444 /usr/share/zoneinfo /usr/share/zoneinfo

COPY --from=build /ironclad-mash/target/release/ironclad-mash /bin/ironclad-mash

USER app-user:app-user

# the scratch image doesn't have a /tmp folder, you may need it
WORKDIR /tmp

WORKDIR /app

EXPOSE 8080

ENTRYPOINT ["/bin/ironclad-mash"]
