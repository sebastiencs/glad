#!/bin/sh

BASE_PATH="$(dirname $(realpath $0))"


cd "${BASE_PATH}/../../../"

python -m glad --out-path "${BASE_PATH}/build" --extensions="GL_OES_EGL_image,EGL_KHR_image" --api="gl:core,egl,gles2" --merge rust # --mx
