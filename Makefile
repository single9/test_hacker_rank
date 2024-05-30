name=${name}

@:
	bash ./scripts/create_bin.sh ${name}

run:
	OUTPUT_PATH=./output/${name}.output cargo run --release --bin ${name}
