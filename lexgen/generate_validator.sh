#!/bin/bash
set -eoxu pipefail

./node_modules/.bin/lex gen-api ./validator/lexicon/ ./lexicons/*/*/*/*;
# # We don't need any of this stuff to call from TS, we just want lexicons.validate.
rm -rf ./validator/lexicon/types/
rm ./validator/lexicon/index.ts
rm ./validator/lexicon/util.ts

./build_validator.sh