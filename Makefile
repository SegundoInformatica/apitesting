default:
	@echo "run: make serve/request"

serve:
	@cd ./api && ./serve

request:
	@cd ./req && cargo run --quiet

