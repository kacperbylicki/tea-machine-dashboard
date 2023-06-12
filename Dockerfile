FROM arm32v7/rust:latest

RUN apt-get update

# install Tauri dependencies
RUN apt-get install -y \
    libwebkit2gtk-4.0 \
    libgtk-3-dev \
    squashfs-tools \
    wget \
    libdbus-1-dev \
    libx11-dev \
    libssl-dev \
    libfreetype6-dev \
    libexpat1-dev \
    libxcb1-dev \
    libx11-xcb-dev \
    libxcb-dri3-dev \
    libxcb-present-dev \
    libxcb-sync-dev \
    libxshmfence-dev \
    libxrandr-dev \
    libxfixes-dev \
    libxext-dev \
    libxdamage-dev \
    libx11-dev \
    libxau-dev \
    libxcb-dri2-0-dev \
    libxcb-glx0-dev \
    libxxf86vm-dev \
    libdrm-dev \
    libgbm-dev \
    libegl1-mesa-dev \
    libudev-dev

# add a new user to prevent running as root
RUN useradd -m docker && echo "docker:docker" | chpasswd && adduser docker sudo

USER docker
