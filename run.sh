# (Ensure the latest Deno)
[ -x ~/.deno/bin/deno ] || curl -fsSL https://deno.land/x/install/install.sh | sh > /dev/null

# Run Deno interpreter
~/.deno/bin/deno run --allow-read --allow-env --unstable src/main.ts

# -----

# Run Rust interpreter
# cargo run