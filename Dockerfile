FROM --platform=$BUILDPLATFORM tonistiigi/xx AS xx

FROM --platform=$BUILDPLATFORM alpine AS builder
COPY --from=xx / /
ARG TARGETPLATFORM

RUN apk add clang
RUN xx-apk add gcc musl-dev

RUN echo 'int main() { return 0; }' > main.c
RUN xx-clang --static -o binary main.c && \
    xx-verify --static binary

#RUN file binary; false

# --

FROM --platform=linux/arm/v7 scratch AS bela
ADD sysroot.tar.gz /
RUN ["/bin/true"]

#FROM --platform=linux/arm/v7 scratch AS runner
#COPY --from=builder binary /binary
#RUN ["/binary"]
