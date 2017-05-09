build:
	cargo build

serve:
	cargo run

db_start:
	docker run --name street-rust-pg -e POSTGRES_PASSWORD=street_rust_password -e POSTGRES_USER=street_rust -p 5432:5432 -d postgres:9.6

db_stop:
	docker stop street-rust-pg

.PHONY: db_start db_stop
