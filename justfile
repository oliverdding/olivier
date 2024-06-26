# list all recipes
default:
    @just --list --justfile {{ justfile() }}

registry := "ccr.ccs.tencentyun.com"
repository := "sngapm/otel-collector"
tag := "0.1.3"
arch_list := "linux/amd64,linux/arm64"

# build docker image
docker:
    docker buildx build --push --platform={{ arch_list }} -t {{ registry }}/{{ repository }}:{{ tag }} .

# generate entities from schema
generate-entities:
    sea-orm-cli generate entity --serde-skip-deserializing-primary-key --with-serde both --with-copy-enums -o ./entity/src/entities

# generate new migration for schema
generate-migrate *NAME:
    sea-orm-cli migrate generate --local-time "{{ NAME }}"

# start development dependencies by compose.yaml
up *PARAMETERS:
    docker compose up -d {{ PARAMETERS }}

# stop development dependencies by compose.yaml
down *PARAMETERS:
    docker compose down {{ PARAMETERS }}
