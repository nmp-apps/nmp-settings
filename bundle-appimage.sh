#! /bin/sh

# STRIP doesn't work on all linux distributions
NO_STRIP=true dx bundle --platform linux --package-types "appimage" --verbose --release --features appimage_assets