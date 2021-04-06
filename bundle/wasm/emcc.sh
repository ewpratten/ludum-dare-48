#! /bin/bash

set -e

emcc "-s" "USE_GLFW=3" "-s" "ASSERTIONS=1" "-s" "ASYNCIFY=1" $@ 