build:
    cargo build

release:
    cargo build --release

run:
    cargo run

revert_migration:
    diesel migration revert

run_migration:
    diesel migration run