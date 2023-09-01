fn __get_index_of_the_after_last_word_of_the_line(
	paragraph_words: &[String],
	column_number: usize,
	begin: usize,
) -> usize {
	use unicode_width::UnicodeWidthStr;

	let mut line_end: usize = begin + 1;
	let mut line_len: usize = UnicodeWidthStr::width(paragraph_words[begin].as_str());

	if line_len < column_number {
		while line_end < paragraph_words.len() {
			line_len += 1 + UnicodeWidthStr::width(paragraph_words[line_end].as_str());
			if line_len > column_number {
				break;
			}
			line_end += 1;
		}
	}
	return line_end;
}

fn __word_len_sum(words: &[String]) -> usize {
	let mut sum: usize = 0;

	for word in words {
		sum += word.len();
	}
	return sum;
}

fn __print_justified_line(line: &[String], column_number: usize) {
	let word_len_sum: usize = __word_len_sum(line);
	let space_len_sum: usize = column_number - word_len_sum;
	let total_space_block_number: usize = line.len() - 1;
	let first_space_block_number: usize = space_len_sum % total_space_block_number;
	let second_space_block_number: usize = total_space_block_number - first_space_block_number;
	let second_space_block_width: usize = space_len_sum / total_space_block_number;
	let first_space_block_width: usize = second_space_block_width + 1;
	let mut i0: usize = 1;

	print!("{}", line[0]);
	for _i1 in 0..first_space_block_number {
		print!(
			"{:>field_width$}",
			line[i0],
			field_width = first_space_block_width + line[i0].len()
		);
		i0 += 1;
	}
	for _i1 in 0..second_space_block_number {
		print!(
			"{:>field_width$}",
			line[i0],
			field_width = second_space_block_width + line[i0].len()
		);
		i0 += 1;
	}
}

fn __print_line(line: &[String]) {
	print!("{}", line[0]);
	for i in 1..line.len() {
		print!(" {}", line[i]);
	}
}

fn __print_justified_paragraph(paragraph_words: &[String], column_number: usize) {
	let mut begin: usize = 0;
	let mut end: usize;

	while begin < paragraph_words.len() {
		end = __get_index_of_the_after_last_word_of_the_line(paragraph_words, column_number, begin);
		if end < paragraph_words.len() && end - begin > 1 {
			__print_justified_line(&paragraph_words[begin..end], column_number);
		} else {
			__print_line(&paragraph_words[begin..end]);
		}
		println!();
		begin = end;
	}
}

fn main() {
	assert_eq!(2, ftkit::ARGS.len(), "Wrong number of arguments");

	let column_number: usize = ftkit::ARGS[1].parse::<usize>().unwrap();
	let mut paragraph_words: Vec<String> = Vec::new();
	let mut line: String = ftkit::read_line();

	while line == "\n" {
		line = ftkit::read_line();
	}

	// First paragraph
	while !line.is_empty() && line != "\n" {
		for word in line.split_whitespace() {
			paragraph_words.push(word.to_string());
		}
		line = ftkit::read_line();
	}
	__print_justified_paragraph(&paragraph_words, column_number);
	paragraph_words.clear();
	while line == "\n" {
		line = ftkit::read_line();
	}

	// Next paragraphs
	while !line.is_empty() {
		while !line.is_empty() && line != "\n" {
			for word in line.split_whitespace() {
				paragraph_words.push(word.to_string());
			}
			line = ftkit::read_line();
		}
		println!();
		__print_justified_paragraph(&paragraph_words, column_number);
		paragraph_words.clear();
		while line == "\n" {
			line = ftkit::read_line();
		}
	}
}
