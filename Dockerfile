FROM grauwoelfchen/rust-vet-tools:stable

LABEL maintainer="Yasuhiro Яша Asaka <yasuhiro.asaka@grauwoelfchen.net>"

WORKDIR /tmp

COPY make.conf /etc/portage/make.conf

RUN set -eux; \
  emerge -qv \
    dev-lang/python-exec \
    dev-lang/python-exec-conf \
    dev-python/certifi \
    dev-python/setuptools \
    dev-python/setuptools_scm \
    dev-python/mako \
    dev-python/toml \
    dev-python/markupsafe \
  \
    dev-libs/libxml2 \
    dev-libs/gobject-introspection \
    dev-util/gdbus-codegen \
    dev-util/itstool \
    dev-util/meson \
    x11-base/xcb-proto \
  \
    =dev-libs/glib-2.68.2 \
    =dev-util/glib-utils-2.68.2 \
    =x11-libs/gtk+-3.24.29 \
  ;

WORKDIR /
CMD ["/bin/bash"]
