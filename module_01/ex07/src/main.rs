use unicode_width::UnicodeWidthStr;

fn __read_lines_from_stdin() -> Vec<String> {
	let mut lines: Vec<String> = Vec::new();
	let mut line: String = ftkit::read_line();

	while !line.is_empty() {
		lines.push(line.trim().to_string());
		line = ftkit::read_line();
	}

	return lines;
}

fn __skip_empty_lines(lines: &Vec<String>, i: &mut usize) {
	while *i < lines.len() && lines[*i].is_empty() {
		*i += 1;
	}
}

fn __get_paragraph(lines: &Vec<String>, i: &mut usize) -> Vec<String> {
	let mut paragraph: Vec<String> = Vec::new();

	while *i < lines.len() && !lines[*i].is_empty() {
		for word in lines[*i].split_whitespace() {
			paragraph.push(word.to_string());
		}
		*i += 1;
	}
	return paragraph;
}

fn __split_into_paragraphs(lines: &Vec<String>) -> Vec<Vec<String>> {
	let mut paragraphs: Vec<Vec<String>> = Vec::new();
	let mut paragraph: Vec<String>;
	let mut i: usize = 0;

	while i < lines.len() {
		__skip_empty_lines(lines, &mut i);
		paragraph = __get_paragraph(&lines, &mut i);
		if !paragraph.is_empty() {
			paragraphs.push(paragraph);
		}
	}
	return paragraphs;
}

fn __get_line_end(paragraph: &[String], column_number: usize, line_begin: usize) -> usize {
	let mut line_end: usize = line_begin + 1;
	let mut line_len: usize = UnicodeWidthStr::width(paragraph[line_begin].as_str());

	if line_len < column_number {
		while line_end < paragraph.len() {
			line_len += 1 + UnicodeWidthStr::width(paragraph[line_end].as_str());
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

fn __print_justified_paragraph(paragraph: &[String], column_number: usize) {
	let mut line_begin: usize = 0;
	let mut line_end: usize;

	while line_begin < paragraph.len() {
		line_end = __get_line_end(paragraph, column_number, line_begin);
		if line_end < paragraph.len() && line_end - line_begin > 1 {
			__print_justified_line(&paragraph[line_begin..line_end], column_number);
		} else {
			__print_line(&paragraph[line_begin..line_end]);
		}
		println!();
		line_begin = line_end;
	}
}

fn main() {
	assert_eq!(2, ftkit::ARGS.len(), "Wrong number of arguments");

	let column_number: usize = ftkit::ARGS[1].parse::<usize>().unwrap();
	let lines: Vec<String> = __read_lines_from_stdin();
	let paragraphs: Vec<Vec<String>> = __split_into_paragraphs(&lines);

	for paragraph in &paragraphs {
		__print_justified_paragraph(paragraph, column_number);
		println!();
	}
}
