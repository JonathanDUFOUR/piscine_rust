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
	let mut paragraph: Vec<String> = Vec::new();
	let mut i: usize = 0;

	__skip_empty_lines(lines, &mut i);
	while i < lines.len() {
		paragraph = __get_paragraph(&lines, &mut i);
		paragraphs.push(paragraph);
		paragraph = Vec::new();
		__skip_empty_lines(lines, &mut i);
	}
	if !paragraph.is_empty() {
		paragraphs.push(paragraph);
	}
	return paragraphs;
}

fn main() {
	assert_eq!(2, ftkit::ARGS.len(), "Wrong number of arguments");

	let column_number: usize = ftkit::ARGS[1].parse::<usize>().unwrap();
	let lines: Vec<String> = __read_lines_from_stdin();
	let paragraphs: Vec<Vec<String>> = __split_into_paragraphs(&lines);

	println!("column_number: {}", column_number);
	println!("lines: {:?}", lines);
	println!("paragraphs: {:?}", paragraphs);
}
