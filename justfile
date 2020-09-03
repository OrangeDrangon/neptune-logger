build:
    cargo build -j $(nproc)

release:
    cargo build --release -j $(nproc)

run:
    cargo run -j $(nproc)

revert_migration:
    diesel migration revert

run_migration:
    diesel migration run