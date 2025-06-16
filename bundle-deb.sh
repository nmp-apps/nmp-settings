#! /bin/sh

# STRIP doesn't work on some linux distributions
NO_STRIP=true dx bundle --platform linux --package-types "deb" --verbose --release