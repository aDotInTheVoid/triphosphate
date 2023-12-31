#!/bin/bash
set -eoxu pipefail

./node_modules/.bin/esbuild --bundle ./validator/polyfill/text_encoder.js --global-name=triphosphate_textencoder --outfile=./dist/text_encoder.js
./node_modules/.bin/esbuild ./validator/index.ts --bundle --global-name=triphosphate_bridge --alias:crypto=./validator/polyfill/crypto_noop.js --outfile=./dist/validator.js


echo "var TextEncoder = triphosphate_textencoder.TextEncoder;" >> ./dist/text_encoder.js
echo "var TextDecoder = triphosphate_textencoder.TextDecoder;" >> ./dist/text_encoder.js

cat ./dist/text_encoder.js ./dist/validator.js > ./dist/bundle.js
