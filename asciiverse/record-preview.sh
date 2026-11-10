#!/bin/sh
# Records asciiverse/arcane-fortune.cast, the catalog preview, at 100x30
# (hub/RECORDING.md) with hub's castrec.py: the title screen, a historical
# game as England (world size and difficulty left as they are), the
# founding story and the noble houses closed, a worker set to build a City
# Hall, a month passing, then the full map turned on (Preferences, Show
# full map) and the view pulled out over Britain and Europe while two more
# months pass; it quits from the File menu. In the build list it moves
# with s, the game's own down key: castrec's arrow keys are the ESC [ form,
# which ncurses in keypad mode reads as Esc and closes the list.
#
# Usage, from the repo root, after a release build (cd arcane_fortune &&
# cargo build --release):  sh asciiverse/record-preview.sh
# It needs ../../hub (the hub checkout beside games/) and fetches nn/.
set -eu
ROOT=$(cd "$(dirname "$0")/.." && pwd)
HUB=${HUB:-$ROOT/../../hub}
RUN=$(mktemp -d)
HOME_DIR=$(mktemp -d)
trap 'rm -rf "$RUN" "$HOME_DIR"' EXIT
cp "$ROOT/arcane_fortune/target/release/arcane_fortune" "$RUN/"
cp -r "$ROOT/arcane_fortune/src/config" "$RUN/"
sh "$ROOT/asciiverse/fetch-nn.sh" "$RUN"
STEPS="w3;kenter;w1.5;sh;w1.5;kenter;w1.2;kenter;w1.2;kenter;w20"
STEPS="$STEPS;kesc;w2;kesc;w1.5;kesc;w1;kesc;w1"
STEPS="$STEPS;sh;w1.5;ss;w1.2;kenter;w2.5;kenter;w1.5;s!;w3.5"
STEPS="$STEPS;sm;w0.6;sp;w0.6;sm;w2;so;w1.5;so;w1.5;so;w2;s!;w3.5;s!;w3.5"
STEPS="$STEPS;sm;w0.6;sf;w0.6;sx;w1"
HOME=$HOME_DIR XDG_STATE_HOME= python3 "$HUB/tools/castrec.py" \
  "$ROOT/asciiverse/arcane-fortune.cast" "$RUN" "$STEPS" -- ./arcane_fortune
