default:
    @just --list

serve:
    trunk serve

fmt: format

format:
    cargo fmt
    leptosfmt "./**/*.rs"
