# Snapdragon 7 Gen 3 Gaming Edition - ARM64 v8.2 Optimized
FROM alvrme/alpine-android:android-34-jdk17

# ARM64 v8.2 CPU features & Adreno 720 GPU optimization
ENV RUSTFLAGS="-C target-feature=+v8.2a,+neon,+fp-armv8,+crypto,+dotprod -C opt-level=3 -C lto=fat -C codegen-units=1 -C link-arg=-Wl,--gc-sections -C embed-bitcode=yes"
ENV GRADLE_OPTS="-XX:+UseG1GC -XX:MaxGCPauseMillis=500 -XX:+ParallelRefProcEnabled -XX:+UnlockExperimentalVMOptions -XX:G1NewCollectionHeuristicPercent=35"

# Install Gradle 8
RUN wget https://services.gradle.org/distributions/gradle-8.14.2-bin.zip -P /tmp && \
    unzip /tmp/gradle-8.14.2-bin.zip -d /opt && \
    ln -s /opt/gradle-8.14.2/bin/gradle /usr/local/bin/gradle && \
    rm /tmp/gradle-8.14.2-bin.zip

# Install Rust toolchain with ARM64 target
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && \
    source $HOME/.cargo/env && \
    rustup target add aarch64-linux-android

# Install build dependencies optimized for ARM64
RUN apk add --no-cache musl-dev libgcc build-base clang llvm lld openssl-dev

# ARM-based host compatibility
RUN case "$ARCH" in \
    aarch64|armeb|armel|armhf|armv7) \
        apk add --no-cache gcompat \
        ;; \
    esac

RUN mkdir /app

# Install patched xbuild
COPY ./patches/ /app/patches
RUN source $HOME/.cargo/env && cargo install --path /app/patches/xbuild/xbuild --force

# Copy project files
COPY . /app
WORKDIR /app

# Build optimized APK
RUN source $HOME/.cargo/env && x build --release --platform android --arch arm64 --format apk
