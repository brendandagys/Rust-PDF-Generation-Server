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

COPY --from=builder /usr/local/cargo/bin/rust-html-pdf-generator /usr/local/bin/rust-html-pdf-generator
COPY --from=builder /usr/local/bin/wkhtmltopdf /usr/local/bin/wkhtmltopdf
COPY --from=builder /usr/local/lib/libwkhtmltox.so.0 /usr/lib/libwkhtmltox.so.0

COPY --from=builder /lib/aarch64-linux-gnu/libexpat.so.1 /usr/lib/libexpat.so.1
COPY --from=builder /lib/aarch64-linux-gnu/libm.so.6 /lib/aarch64-linux-gnu/libm.so.6
COPY --from=builder /usr/lib/aarch64-linux-gnu/libX11.so.6 /usr/lib/libX11.so.6
COPY --from=builder /usr/lib/aarch64-linux-gnu/libXau.so.6 /usr/lib/libXau.so.6
COPY --from=builder /usr/lib/aarch64-linux-gnu/libXdmcp.so.6 /usr/lib/libXdmcp.so.6
COPY --from=builder /usr/lib/aarch64-linux-gnu/libXext.so.6 /usr/lib/libXext.so.6
COPY --from=builder /usr/lib/aarch64-linux-gnu/libXrender.so.1 /usr/lib/libXrender.so.1
COPY --from=builder /usr/lib/aarch64-linux-gnu/libbrotlicommon.so.1 /usr/lib/libbrotlicommon.so.1
COPY --from=builder /usr/lib/aarch64-linux-gnu/libbrotlidec.so.1 /usr/lib/libbrotlidec.so.1
COPY --from=builder /usr/lib/aarch64-linux-gnu/libbsd.so.0 /usr/lib/libbsd.so.0
COPY --from=builder /usr/lib/aarch64-linux-gnu/libcrypto.so.1.1 /usr/lib/libcrypto.so.1.1
COPY --from=builder /usr/lib/aarch64-linux-gnu/libfontconfig.so.1 /usr/lib/libfontconfig.so.1
COPY --from=builder /usr/lib/aarch64-linux-gnu/libfreetype.so.6 /usr/lib/libfreetype.so.6
COPY --from=builder /usr/lib/aarch64-linux-gnu/libjpeg.so.62 /usr/lib/libjpeg.so.62
COPY --from=builder /usr/lib/aarch64-linux-gnu/libm.so /usr/lib/libm.so
COPY --from=builder /usr/lib/aarch64-linux-gnu/libmd.so.0 /usr/lib/libmd.so.0
COPY --from=builder /usr/lib/aarch64-linux-gnu/libpng16.so.16 /usr/lib/libpng16.so.16
COPY --from=builder /usr/lib/aarch64-linux-gnu/libssl.so.1.1 /usr/lib/libssl.so.1.1
COPY --from=builder /usr/lib/aarch64-linux-gnu/libxcb.so.1 /usr/lib/libxcb.so.1

CMD ["rust-html-pdf-generator"]
