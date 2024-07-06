FROM --platform=linux/arm/v7 scratch

ADD sysroot.tar.gz /

RUN ["/bin/ls"]

ENV RUSTUP_HOME=/opt/rustup CARGO_HOME=/opt/cargo
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | \
	sh -s -- -y --no-modify-path --profile minimal 
ENV PATH=${PATH}:${CARGO_HOME}/bin
