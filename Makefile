BIN = ./target/debug/diagram

.PHONY: test
test:
	cargo test

.PHONY: build
build:
	cargo build

.PHONY: examples
examples: build
	mkdir -p svg
	# ${BIN} errors.dml
	# ${BIN} --output svg/pipeline.svg   examples/pipeline.dml
	# ${BIN} --output svg/temp.svg       examples/temp.dml
	${BIN} --output svg/example_1.svg  examples/example_1.dml
	${BIN} --output svg/example_2.svg  examples/example_2.dml
	${BIN} --output svg/grid.svg       examples/grid.dml
	${BIN} --output svg/group.svg      examples/group.dml
	${BIN} --output svg/i10.svg        examples/i10.dml
	${BIN} --output svg/overlay.svg    examples/overlay.dml
	${BIN} --output svg/simple.svg     examples/simple.dml
	${BIN} --output svg/text.svg       examples/text.dml
	${BIN} --output svg/use.svg        examples/use.dml

