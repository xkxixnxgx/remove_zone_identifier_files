compile:
	rustc ./src/main.rs

create_test_file:
	touch ./test/test:Zone.Identifier

run: compile
	./main
