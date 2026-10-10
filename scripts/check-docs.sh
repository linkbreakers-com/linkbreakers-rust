#!/usr/bin/env bash
# Compiles every Rust block in README.md against the generated crate, so the
# hand-written README cannot call functions the generator does not produce.
# A block without `fn main` is compiled as the body of one.
set -euo pipefail

root="$(cd "$(dirname "$0")/.." && pwd)"
examples="$root/examples"
prefix="docscheck_readme_"
trap 'rm -f "$examples"/${prefix}*.rs; rmdir "$examples" 2>/dev/null || true' EXIT
mkdir -p "$examples"

awk -v dir="$examples" -v prefix="$prefix" '
  /^```rust$/ { n++; inside = 1; body = ""; next }
  /^```$/ && inside {
    inside = 0
    file = sprintf("%s/%s%d.rs", dir, prefix, n)
    if (body ~ /fn main/) { printf "%s", body > file }
    else { printf "#![allow(unused)]\nfn main() {\n%s}\n", body > file }
    close(file)
    next
  }
  inside { body = body $0 "\n" }
' "$root/README.md"

count=$(find "$examples" -name "${prefix}*.rs" | wc -l | tr -d ' ')
if [ "$count" = "0" ]; then
  echo "No Rust blocks found in README.md" >&2
  exit 1
fi

cd "$root"
if ! cargo check --quiet --examples; then
  echo "A README.md Rust block no longer compiles against the generated SDK." >&2
  exit 1
fi
echo "Compiled $count README.md Rust blocks against the generated SDK."
