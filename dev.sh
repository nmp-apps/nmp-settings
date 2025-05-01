#! /bin/sh

# Fixes bug with content visibility on machines with nvidia cards
WEBKIT_DISABLE_DMABUF_RENDERER=1 TEST_PLUGIN_PATH=../nmp-settings-plugin-example/target/debug/nmp-settings-plugin-example dx serve