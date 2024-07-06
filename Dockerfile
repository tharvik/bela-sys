FROM tonistiigi/binfmt AS emulators

FROM --platform=linux/arm/v7 scratch AS bela

ADD sysroot.tar.gz /
COPY --from=emulators /usr/bin/qemu-arm /usr/bin/

RUN ["/usr/bin/qemu-arm", "--help"]
RUN ["/usr/bin/stat", "/"]

ENV RUSTUP_HOME=/opt/rustup CARGO_HOME=/opt/cargo
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | \
	sh -s -- -y --no-modify-path --profile minimal 
ENV PATH=${PATH}:${CARGO_HOME}/bin
