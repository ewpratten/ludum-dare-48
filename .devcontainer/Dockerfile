FROM mcr.microsoft.com/vscode/devcontainers/rust

# Install Raylib deps
RUN apt-get update -y
RUN apt-get install libasound2-dev mesa-common-dev libx11-dev libxrandr-dev libxi-dev xorg-dev libgl1-mesa-dev libglu1-mesa-dev -y
RUN cargo install cross

