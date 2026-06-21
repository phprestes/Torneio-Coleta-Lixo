.PHONY: setup db-up db-down run clean

db:
	docker compose up -d

stop-db:
	docker compose down

setup:
	cd app && cargo build

run: db
	cd app && cargo run

clean: stop-db
	cd app && cargo clean
