.PHONY: init up down build install

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

migrate:
	@echo "Running migrations..."
	cd back-end && sqlx migrate run
	cd ..
	@echo "Migrations complete."

build:
	@echo "Building lia..."
	cargo build --release
	@echo "lia built."

install:
	@echo "Ensuring install script has correct Unix file type..."
	@dos2unix ./scripts/install.sh 2>/dev/null || sed -i 's/\r//' ./scripts/install
	@chmod +x ./scripts/install.sh
	@./scripts/install.sh
	@echo "lia installed."