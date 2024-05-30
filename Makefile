name=${name}

@:
	touch src/bin/${name}.rs

run:
	OUTPUT_PATH=./output/${name}.output cargo run --release --bin ${name}
