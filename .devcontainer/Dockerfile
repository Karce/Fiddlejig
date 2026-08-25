FROM registry.fedoraproject.org/fedora-toolbox:44

RUN dnf install -y \
    gcc-c++ cmake ninja-build make \
    clang clang-devel llvm-devel \
    pkgconf-pkg-config curl tar git gh \
    gstreamer1-devel gstreamer1-plugins-base-devel \
    gtk3-devel libpng-devel zlib-devel openssl-devel \
    fish \
    && dnf clean all

ARG OPENCV_VERSION=4.13.0
ARG OPENCV_PREFIX=/opt/opencv-static

RUN curl -fL -o /tmp/opencv.tar.gz https://github.com/opencv/opencv/archive/refs/tags/${OPENCV_VERSION}.tar.gz && \
    tar -xzf /tmp/opencv.tar.gz -C /tmp && \
    cd /tmp/opencv-${OPENCV_VERSION} && \
    cmake -S . -B build -G Ninja \
      -DCMAKE_BUILD_TYPE=Release \
      -DCMAKE_INSTALL_PREFIX=${OPENCV_PREFIX} \
      -DCMAKE_INSTALL_LIBDIR=lib \
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
      -DBUILD_EXAMPLES=OFF -DBUILD_DOCS=OFF -DOPENCV_ENABLE_NONFREE=OFF && \
    cmake --build build && \
    cmake --install build && \
    cd ${OPENCV_PREFIX}/lib/opencv4/3rdparty && for f in liblib*.a; do [ -e "$f" ] && ln -sf "$f" "${f#lib}"; done && \
    rm -rf /tmp/opencv*

ENV OPENCV_INCLUDE_PATHS=/opt/opencv-static/include/opencv4
ENV OPENCV_LINK_PATHS=/opt/opencv-static/lib,/opt/opencv-static/lib/opencv4/3rdparty
ENV OPENCV_LINK_LIBS=static=opencv_highgui,static=opencv_objdetect,static=opencv_calib3d,static=opencv_features2d,static=opencv_flann,static=opencv_videoio,static=opencv_imgcodecs,static=opencv_imgproc,static=opencv_core,static=ittnotify,static=libjpeg-turbo,static=libwebp,static=libpng,static=libtiff,static=libopenjp2,static=IlmImf,static=zlib,gtk-3,gdk-3,pangocairo-1.0,cairo-gobject,atk-1.0,pango-1.0,gdk_pixbuf-2.0,gio-2.0,cairo,harfbuzz,gstapp-1.0,gstpbutils-1.0,gstvideo-1.0,gstriff-1.0,gstaudio-1.0,gstbase-1.0,gstreamer-1.0,gobject-2.0,glib-2.0,stdc++,dl,m,pthread,rt
ENV OPENCV_DISABLE_PROBES=cmake,pkg_config,vcpkg,vcpkg_cmake

RUN useradd -m -s /usr/bin/fish vscode && \
    echo "vscode ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers.d/vscode
USER vscode
