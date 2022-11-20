#!/bin/bash

if [[ $TRUNK_PROFILE == "release" ]]; then
    NODE_ENV=production tailwindcss -i ./styles/input.css -o ./styles/output.css --minify
else
    tailwindcss -i ./styles/input.css -o ./styles/output.css
fi
