git_revision := `git rev-parse --short HEAD`
app_version := `awk -F'"' '/^\[package\]/{p=1} p && /^version *=/{print $2; exit}' Cargo.toml`
build_date := `date -u +%Y-%m-%dT%H:%M:%SZ`
container_runner := "docker"
container_image := "ghcr.io/uzi0702/teotwahbew"

test:
    cargo test

build: test
    cargo build --release

container-local:
    {{container_runner}} build \
        --build-arg GIT_REVISION={{git_revision}} \
        --build-arg BUILD_DATE={{build_date}} \
        --build-arg VERSION={{app_version}} \
        -t {{container_image}}:latest -t {{container_image}}:{{app_version}} \
        -f Containerfile \
        .

container:
    sh .github/scripts/build_docker.sh
