set shell := ["nu", "-c"]

kvsd_user := "synduser"
github_pat := env_var_or_default("GH_PAT", "")
otlp_endpoint := env_var_or_default("OTEL_EXPORTER_OTLP_ENDPOINT", "")
loki_endpoint := env_var_or_default("LOKI_ENDPOINT","")


alias format := fmt
alias integration := integration-test

# List recipe
default:
  just --list

# Run check
check:
  nix flake check --all-systems --accept-flake-config

# Format files
fmt: fmt-toml

lint:
  cargo clippy

# Format toml files
fmt-toml:
  taplo fmt --config taplo.toml **.toml

test:
  cargo nextest run

# Run integration test
integration-test:
  RUST_LOG="synd,integration,kvsd=info,info" cargo nextest run --package synd_term --features integration --test integration --no-capture 

# Update synd_api graphql schema
update-gql-schema:
  @graphql-client introspect-schema http://localhost:5959/graphql \
    --header 'authorization: github {{github_pat}}' out> crates/synd_term/gql/schema.json

# Generate graphql code
gen-gql:
  graphql-client generate \
    --schema-path crates/synd_term/gql/schema.json \
    --output-directory crates/synd_term/src/client \
    --variables-derives "Debug" \
    --response-derives "Debug" \
    --custom-scalars-module "crate::client::scalar" \
    crates/synd_term/gql/query.gql

  graphql-client generate \
    --schema-path crates/synd_term/gql/schema.json \
    --output-directory crates/synd_term/src/client \
    --variables-derives "Debug" \
    --response-derives "Debug" \
    --custom-scalars-module "crate::client::scalar" \
    crates/synd_term/gql/mutation.gql

  graphql-client generate \
    --schema-path crates/synd_api/src/client/github/schema.json \
    --output-directory crates/synd_api/src/client/github \
    --variables-derives "Debug" \
    --response-derives "Debug" \
    crates/synd_api/src/client/github/query.gql

# Run kvsd
kvsd:
  cd ../kvsd; \
  KVSD_LOG=info cargo run server --disable-tls --config ../syndicationd/.dev/kvsd_config.yaml --username {{kvsd_user}} --kvsd-dir ../syndicationd/.kvsd

# Run api
api:
  cd crates/synd_api; \
    RUST_LOG="info,synd_api=debug" \
    OTEL_EXPORTER_OTLP_ENDPOINT={{otlp_endpoint}} \
    cargo run --features "introspection" -- \
    --kvsd-host 127.0.0.1 --kvsd-port 7379 --kvsd-username {{kvsd_user}} --kvsd-password secret

# Run term
term *flags:
  cd crates/synd_term; cargo run -- --log /tmp/syndterm.log {{flags}}

# Run opentelemetry-collector-contrib
@otelcol:
  LOKI_ENDPOINT={{loki_endpoint}} \
  otelcontribcol --config=file:.dev/otelcol-config.yaml

# Run backends
backend: 
  zellij action new-tab --layout .dev/backend_layout.kdl
