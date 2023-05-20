default:
	@echo "run: make serve/request"

serve:
	@cd ./api && flask --app main run --no-debug --reload

request:
	@cd ./req && cargo run --quiet

