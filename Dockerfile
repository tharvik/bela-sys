FROM --platform=linux/arm/v7 scratch

ADD sysroot.tar.gz /
ADD qemu-arm /usr/bin/qemu-arm
# ubuntu compat
ADD qemu-arm /usr/libexec/qemu-binfmt/arm-binfmt-P
ADD qemu-arm /usr/libexec/qemu-binfmt/armeb-binfmt-P

ENV RUSTUP_HOME=/opt/rustup CARGO_HOME=/opt/cargo
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | \
	sh -s -- -y --no-modify-path --profile minimal 
ENV PATH=${PATH}:${CARGO_HOME}/bin
