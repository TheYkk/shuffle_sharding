cargo build --example bard --release
cargo build --example ahash --release
cargo build --release

hyperfine --export-markdown benc.md --runs 50 './target/release/examples/bard' './target/release/examples/ahash' './target/release/shuffle_sharding'