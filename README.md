## Setup
- Install Postgresql
- Install Rust https://www.rust-lang.org/tools/install
- Setup .env
- $ sudo apt update
- $ sudo apt install -y pkg-config libssl-dev libpq-dev
- $ cargo build
- $ cargo install diesel_cli --no-default-features --features postgres
- $ cargo run
- Migrate it by sending POST http://localhost:8080/api/migration
- $ pg_restore --verbose --clean --no-acl --no-owner -h localhost -U {dbName} latest.dump

 docker run --env-file .env -p 8080:8080 355355355/author:latest