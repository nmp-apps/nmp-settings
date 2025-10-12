#! /bin/sh

# STRIP doesn't work on some linux distributions
NO_STRIP=true dx bundle --linux --package-types "appimage" --verbose --release