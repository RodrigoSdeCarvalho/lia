.PHONY: init up down

init:
	@echo "Initializing the database..."
	docker-compose up -d
	@echo "Waiting for the database to start..."
	sleep 5
	@echo "Database initialized."

up:
	@echo "Starting the database..."
	docker-compose up -d
	@echo "Database started."

down:
	@echo "Stopping the database..."
	docker-compose down
	@echo "Database stopped."

build:
	@echo "Building lia..."
	cargo build --release
	@echo "lia built."

install:
	@echo "Installing lia..."
	./install.sh
	@echo "lia installed."
