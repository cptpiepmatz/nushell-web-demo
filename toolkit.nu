export def prep [] {
  ^rustup target add wasm32-unknown-unknown
  ^cargo install trunk --locked --features openssl/vendored
  ^cargo install leptosfmt
  ^git submodule update --init --recursive
}

export def fmt [
  --check
] {
  prep

  if $check {
    ^leptosfmt ./**/*.rs --check
    ^cargo fmt --check
  } else {
    ^leptosfmt ./**/*.rs
    ^cargo fmt
  }
}

export def build [] {
  prep

  ^trunk build
}
