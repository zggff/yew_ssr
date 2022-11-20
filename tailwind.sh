#!/bin/bash

if [[ $TRUNK_PROFILE == "release" ]]; then
    NODE_ENV=production npx tailwindcss -i ./styles/tailwind.scss -o ./styles/output.css --minify
else
    npx tailwindcss -i ./styles/tailwind.scss -o ./styles/output.css
fi
