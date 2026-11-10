#!/bin/sh
# fetch-nn.sh DEST: puts the game's neural network (nn/, about 38 MB, the
# LSTM that writes what other civilizations say to you in their embassies)
# into DEST/nn. The game will not start without it. It is not in the git
# repository, only in upstream's v0.4.0 release zip, so it is taken from
# there, checked against the zip's sha256.
set -eu
DEST=$1
URL=https://github.com/cody2007/arcane_fortune/releases/download/v0.4.0/arcane_fortune_v0_4_0.zip
SHA256=86b7a5643eb40240f0b8f2d3312aad1ef293885e4eb3a795c09cf61b1a17e43e
TMP=$(mktemp -d)
trap 'rm -rf "$TMP"' EXIT
curl -sSfL -o "$TMP/af.zip" "$URL"
echo "$SHA256  $TMP/af.zip" | sha256sum -c - >/dev/null 2>&1 \
  || echo "$SHA256  $TMP/af.zip" | shasum -a 256 -c - >/dev/null
unzip -q "$TMP/af.zip" 'arcane_fortune_v0_4_0/game/nn/*' -d "$TMP"
mkdir -p "$DEST"
rm -rf "$DEST/nn"
mv "$TMP/arcane_fortune_v0_4_0/game/nn" "$DEST/nn"
