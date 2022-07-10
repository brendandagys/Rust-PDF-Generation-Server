FROM rust:1.61.0 as builder

RUN apt-get update && apt-get upgrade -y
ADD https://github.com/wkhtmltopdf/packaging/releases/download/0.12.6-1/wkhtmltox_0.12.6-1.buster_arm64.deb .
RUN apt-get install -y xvfb libfontconfig fontconfig libjpeg62 libpng16-16 libxrender1 xfonts-75dpi build-essential xorg libjpeg62-turbo
RUN dpkg -i wkhtmltox_0.12.6-1.buster_arm64.deb

WORKDIR /usr/src/rust-html-pdf-generator
COPY . .
RUN cargo install --path .


FROM debian:buster-slim

RUN mkdir pdf_files

RUN apt-get update && apt-get upgrade -y
RUN apt-get install -y xvfb libfontconfig fontconfig libjpeg62 libpng16-16 libxrender1 xfonts-75dpi build-essential xorg libjpeg62-turbo

COPY --from=builder /usr/local/cargo/bin/rust-html-pdf-generator /usr/local/bin/rust-html-pdf-generator
COPY --from=builder /usr/local/bin/wkhtmltopdf /usr/local/bin/wkhtmltopdf
COPY --from=builder /usr/local/lib/libwkhtmltox.so.0 /usr/lib/libwkhtmltox.so.0
# COPY --from=builder /usr/lib/aarch64-linux-gnu/libjpeg.so.62 /usr/lib/libjpeg.so.62
# COPY --from=builder /usr/lib/aarch64-linux-gnu/libpng16.so.16 /usr/lib/libpng16.so.16

CMD ["rust-html-pdf-generator"]
# CMD ["sleep", "infinity"]
