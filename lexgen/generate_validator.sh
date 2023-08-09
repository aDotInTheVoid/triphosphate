#!/bin/bash
set -eoxu pipefail

npm run generate
# We don't need any of this stuff to call from TS, we just want lexicons.validate.
rm -rf ./validator/lexicon/types/
rm ./validator/lexicon/index.ts
rm ./validator/lexicon/util.ts

npm run build