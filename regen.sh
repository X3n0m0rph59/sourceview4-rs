#!/bin/bash

set -o errexit
set -o nounset
set -o pipefail


CRATE_API=sourceview4
CRATE_SYS=sourceview4-sys

CONFIG_API=${CRATE_API}/Gir.toml
CONFIG_SYS=${CRATE_SYS}/gir-${CRATE_API}.toml

GIR=./target/release/gir
GIR_DEF=GtkSource-4.gir

# Build the gir tool
git submodule update --init
(cd gir && cargo build --release)

# Copy the gir
cp ${GIR_DEF} gir-files/

# Regenerate the sys crate
rm -rf ${CRATE_SYS}/{build.rs, src/auto}
${GIR} -c ${CONFIG_SYS} -d gir-files -o ${CRATE_SYS}

# Regenerate the api crate
rm -rf ${CRATE_API}/src/auto
${GIR} -c ${CONFIG_API} -d gir-files -o ${CRATE_API}

rm gir-files/{${GIR_DEF}
