#!/bin/bash
set -eoxu pipefail

git clone https://github.com/bluesky-social/atproto.git
cd atproto/

commit=$(git log -n 1 --pretty='%H' ./lexicons/)

cd ..

rm -rf ./lexicons/
mv atproto/lexicons/ ./lexicons/

curl "https://raw.githubusercontent.com/bluesky-social/atproto/main/LICENSE" > ./lexicons/LICENSE

git add ./lexicons/

cargo r --bin lexgen
./generate_validator.sh
cargo fmt --all

git add ./validator/
git add ../src/lex/

git commit -m "Update lexicons to commit ${commit}

See https://github.com/bluesky-social/atproto/commit/${commit}
"

rm -rf atproto/
