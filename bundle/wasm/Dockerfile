FROM rustembedded/cross:wasm32-unknown-emscripten-0.2.1

# System deps
RUN apt-get update -y
RUN apt-get install emscripten -y
RUN apt-get install gcc-multilib -y
RUN apt-get install curl -y

# Emscripten
# RUN emsdk update-tags
# RUN emsdk install sdk-incoming-64bit
# RUN emsdk activate sdk-incoming-64bit

