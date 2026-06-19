#!/usr/bin/env bash
#
# devbox.sh — build Autofisher-3000 in a reproducible distrobox against a
# *statically linked* OpenCV, so the resulting binary runs on a layer-free host.
#
# Why: the only host library the bot needs that isn't already in the base image is
# OpenCV. We build OpenCV from source as static `.a` libs inside a Fedora container
# (matched to the host so the remaining *dynamic* deps — GStreamer/GTK/glib/libstdc++,
# all base-image, stable-SONAME — line up), then link it statically. The host then
# needs zero layered packages.
#
# Usage: ./devbox.sh <command>
#   box           create/ensure the build container
#   deps          install build dependencies inside the container (idempotent)
#   opencv        download + static-build OpenCV into the prefix (skips if present)
#   build         cargo build --release with the static-OpenCV link, inside the container
#   test          cargo test inside the container
#   update        upgrade container packages; rebuild OpenCV if its version moved
#   env           print the static-link environment (debugging)
#   in-box <cmd>  run an arbitrary command inside the container
#
set -euo pipefail

BOX="${AUTOFISHER_BOX:-autofisher}"
IMAGE="${AUTOFISHER_IMAGE:-registry.fedoraproject.org/fedora-toolbox:44}"
OPENCV_VERSION="${AUTOFISHER_OPENCV_VERSION:-4.13.0}"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
OPENCV_MINOR="${OPENCV_VERSION%.*}"                                   # 4.13.0 -> 4.13
PREFIX="$HOME/.local/share/autofisher/opencv-${OPENCV_MINOR}-static"  # shared host<->box
SRC_DIR="$HOME/.cache/autofisher/opencv-src"

# Container build dependencies. Only the dev headers of the *dynamic* deps are needed
# (so OpenCV's cmake enables those features); everything else is the compiler/toolchain.
BUILD_DEPS=(
  gcc-c++ cmake ninja-build make
  clang clang-devel llvm-devel        # libclang for the opencv crate's bindgen
  pkgconf-pkg-config curl tar
  gstreamer1-devel gstreamer1-plugins-base-devel
  gtk3-devel libpng-devel zlib-devel
)

log()  { printf '\033[1;34m[devbox]\033[0m %s\n' "$*" >&2; }
die()  { printf '\033[1;31m[devbox] error:\033[0m %s\n' "$*" >&2; exit 1; }

in_box() { distrobox enter --name "$BOX" -- "$@"; }

ensure_box() {
  if ! distrobox list 2>/dev/null | grep -qE "\|\s*${BOX}\s*\|"; then
    log "creating distrobox '$BOX' from $IMAGE"
    distrobox create --name "$BOX" --image "$IMAGE" --yes
  fi
  # Force first-run init so later commands don't interleave the setup banner.
  in_box true >/dev/null 2>&1 || true
}

# OpenCV installs its libs under lib/ or lib64/ depending on the platform; detect it.
opencv_libdir() {
  if [ -f "$PREFIX/lib64/cmake/opencv4/OpenCVConfig.cmake" ]; then echo "$PREFIX/lib64"
  else echo "$PREFIX/lib"; fi
}

# OpenCV ships its bundled 3rdparty static libs as `liblibpng.a`, `liblibjpeg-turbo.a`,
# etc. (doubled `lib`), but the opencv crate strips the `lib` prefix so rustc searches
# for `libpng.a`/`libjpeg-turbo.a`. Add single-`lib` compat symlinks so the link
# resolves. Idempotent; the prefix is under $HOME so the host can create them directly.
link_3rdparty() {
  local d; d="$(opencv_libdir)/opencv4/3rdparty"
  [ -d "$d" ] || return 0
  ( cd "$d" && for f in liblib*.a; do [ -e "$f" ] && ln -sf "$f" "${f#lib}"; done )
}

# Emit the env that statically links our OpenCV build into the bot via the opencv
# crate's "environment" probe (its cmake probe is broken on CMake 4.x — it uses the
# removed `cmake --find-package` mode). Link order: OpenCV modules (dependents first)
# → bundled 3rdparty codecs (static) → the GStreamer/GTK libs videoio+highgui need
# (OpenCV's generated opencv4.pc omits these from Libs.private, so we append them) →
# C++ runtime + base syscalls. Everything from the GTK/GStreamer block onward links
# *dynamically* against base-image libraries that survive removing the layered packages.
print_env() {
  local libdir; libdir="$(opencv_libdir)"
  local libs="static=opencv_highgui,static=opencv_objdetect,static=opencv_calib3d,static=opencv_features2d,static=opencv_flann,static=opencv_videoio,static=opencv_imgcodecs,static=opencv_imgproc,static=opencv_core"
  libs+=",static=ittnotify,static=libjpeg-turbo,static=libwebp,static=libpng,static=libtiff,static=libopenjp2,static=IlmImf,static=zlib"
  libs+=",gtk-3,gdk-3,pangocairo-1.0,cairo-gobject,atk-1.0,pango-1.0,gdk_pixbuf-2.0,gio-2.0,cairo,harfbuzz"
  libs+=",gstapp-1.0,gstpbutils-1.0,gstvideo-1.0,gstriff-1.0,gstaudio-1.0,gstbase-1.0,gstreamer-1.0,gobject-2.0,glib-2.0"
  libs+=",stdc++,dl,m,pthread,rt"
  cat <<EOF
OPENCV_INCLUDE_PATHS=$PREFIX/include/opencv4
OPENCV_LINK_PATHS=$libdir,$libdir/opencv4/3rdparty
OPENCV_LINK_LIBS=$libs
OPENCV_DISABLE_PROBES=cmake,pkg_config,vcpkg,vcpkg_cmake
EOF
}

cmd_box() { ensure_box; log "container '$BOX' ready"; }

cmd_deps() {
  ensure_box
  log "installing build deps in '$BOX'"
  in_box sudo dnf install -y "${BUILD_DEPS[@]}"
}

cmd_opencv() {
  ensure_box
  if [ -f "$(opencv_libdir)/cmake/opencv4/OpenCVConfig.cmake" ]; then
    log "static OpenCV $OPENCV_VERSION already built at $PREFIX (delete it to rebuild)"
    link_3rdparty
    return 0
  fi
  log "building static OpenCV $OPENCV_VERSION into $PREFIX (one-time, ~15-40 min)"
  in_box bash -euo pipefail -c "
    mkdir -p '$SRC_DIR' && cd '$SRC_DIR'
    if [ ! -d 'opencv-$OPENCV_VERSION' ]; then
      curl -fL -o opencv-$OPENCV_VERSION.tar.gz \
        'https://github.com/opencv/opencv/archive/refs/tags/$OPENCV_VERSION.tar.gz'
      tar xzf opencv-$OPENCV_VERSION.tar.gz
    fi
    cd 'opencv-$OPENCV_VERSION'
    cmake -S . -B build -G Ninja \
      -DCMAKE_BUILD_TYPE=Release \
      -DCMAKE_INSTALL_PREFIX='$PREFIX' \
      -DBUILD_SHARED_LIBS=OFF \
      -DCMAKE_POSITION_INDEPENDENT_CODE=ON \
      -DOPENCV_GENERATE_PKGCONFIG=ON \
      -DBUILD_LIST=core,imgproc,imgcodecs,videoio,highgui,objdetect \
      -DWITH_GSTREAMER=ON -DWITH_GTK=ON -DWITH_FFMPEG=OFF \
      -DWITH_TBB=OFF -DWITH_OPENMP=OFF -DWITH_IPP=OFF -DWITH_1394=OFF -DWITH_V4L=OFF \
      -DBUILD_PNG=ON -DBUILD_ZLIB=ON -DBUILD_JPEG=ON \
      -DBUILD_TIFF=ON -DBUILD_WEBP=ON -DBUILD_OPENJPEG=ON \
      -DBUILD_opencv_python2=OFF -DBUILD_opencv_python3=OFF -DBUILD_JAVA=OFF \
      -DBUILD_opencv_apps=OFF -DBUILD_TESTS=OFF -DBUILD_PERF_TESTS=OFF \
      -DBUILD_EXAMPLES=OFF -DBUILD_DOCS=OFF -DOPENCV_ENABLE_NONFREE=OFF
    cmake --build build
    cmake --install build
  "
  link_3rdparty
  log "static OpenCV installed at $PREFIX"
}

cmd_build() {
  ensure_box
  [ -f "$(opencv_libdir)/cmake/opencv4/OpenCVConfig.cmake" ] || die "run './devbox.sh opencv' first"
  log "building autofisher-3000 (release, static OpenCV)"
  in_box env $(print_env) bash -euo pipefail -c "cd '$SCRIPT_DIR' && cargo build --release"
}

cmd_test() {
  ensure_box
  [ -f "$(opencv_libdir)/cmake/opencv4/OpenCVConfig.cmake" ] || die "run './devbox.sh opencv' first"
  log "running tests"
  in_box env $(print_env) bash -euo pipefail -c "cd '$SCRIPT_DIR' && cargo test"
}

cmd_update() {
  ensure_box
  log "upgrading container packages"
  distrobox upgrade "$BOX" || in_box sudo dnf upgrade -y
  log "if the host OpenCV version moved, delete $PREFIX and re-run './devbox.sh opencv build'"
}

cmd_env() { print_env; }

cmd_in_box() { ensure_box; in_box "$@"; }

main() {
  local cmd="${1:-}"; shift || true
  case "$cmd" in
    box)     cmd_box ;;
    deps)    cmd_deps ;;
    opencv)  cmd_opencv ;;
    build)   cmd_build ;;
    test)    cmd_test ;;
    update)  cmd_update ;;
    env)     cmd_env ;;
    in-box)  cmd_in_box "$@" ;;
    *) sed -n '2,30p' "${BASH_SOURCE[0]}" | sed 's/^# \{0,1\}//'; exit 1 ;;
  esac
}

main "$@"
