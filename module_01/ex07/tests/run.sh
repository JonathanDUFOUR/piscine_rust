#!/bin/sh

COLOR_RESET='\033[0m'

COLOR_TEST_CATEGORY='\033[38;2;0;210;255m'
COLOR_TEST_SUBCATEGORY='\033[38;2;175;175;175m'
COLOR_SUCCESS='\033[38;2;0;255;0m'
COLOR_FAILURE='\033[38;2;255;0;0m'

OUTPUT_DIR='./tests/output'

test() {
	echo -ne ${COLOR_TEST_CATEGORY} > /dev/stderr
	echo "[ COLUMN NUMBER ${1} ]"
	echo -ne ${COLOR_RESET} > /dev/stderr

	local basename
	local input_file
	local output_file
	local expected_file

	for input_file in $(ls ./tests/input/*.txt); do
		basename=$(basename ${input_file})
		output_file=${OUTPUT_DIR}/${basename}
		expected_file=./tests/expected/column_number_${1}/${basename}

		echo -ne ${COLOR_TEST_SUBCATEGORY} > /dev/stderr
		printf '%50s: ' ${basename}
		echo -ne ${COLOR_RESET} > /dev/stderr

		cargo run -- ${1} < ${input_file} > ${output_file} 2> /dev/null
		if diff ${output_file} ${expected_file} > /dev/null ; then
			echo -ne ${COLOR_SUCCESS} > /dev/stderr
			echo '[OK]'
			echo -ne ${COLOR_RESET} > /dev/stderr
		else
			echo -ne ${COLOR_FAILURE} > /dev/stderr
			echo '[KO]'
			echo -ne ${COLOR_RESET} > /dev/stderr
		fi
	done
}

column_number_1() {
	echo -ne ${COLOR_TEST_CATEGORY} > /dev/stderr
	echo '[ COLUMN NUMBER 1 ]'
	echo -ne ${COLOR_RESET} > /dev/stderr
}

column_number_5() {
	echo -ne ${COLOR_TEST_CATEGORY} > /dev/stderr
	echo '[ COLUMN NUMBER 5 ]'
	echo -ne ${COLOR_RESET} > /dev/stderr
}

column_number_19() {
	echo -ne ${COLOR_TEST_CATEGORY} > /dev/stderr
	echo '[ COLUMN NUMBER 19 ]'
	echo -ne ${COLOR_RESET} > /dev/stderr
}

column_number_42() {
	echo -ne ${COLOR_TEST_CATEGORY} > /dev/stderr
	echo '[ COLUMN NUMBER 42 ]'
	echo -ne ${COLOR_RESET} > /dev/stderr
}

mkdir -p ${OUTPUT_DIR}

test 0
test 1
test 5
test 19
test 42

rm -rf ${OUTPUT_DIR}

exit 0
