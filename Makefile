BIN = ./target/debug/diagram --svg_indent=""

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
	${BIN} --output svg/path.svg       examples/path.dml examples/markers.dml
	${BIN} --output svg/rotate.svg     examples/rotate.dml
	${BIN} --output svg/rotate2.svg    examples/rotate2.dml
	${BIN} --output svg/rotate3.svg    examples/rotate3.dml
	${BIN} --output svg/simple.svg     examples/simple.dml
	${BIN} --output svg/style.svg      examples/style.dml
	${BIN} --output svg/styled.svg     examples/style.dml examples/stylesheet.dml
	${BIN} --output svg/text.svg       examples/text.dml
	${BIN} --output svg/use.svg        examples/use.dml

.PHONY: is_gold
is_gold: examples
	diff -r svg svg.gold

