#[derive(Debug)]
enum Token<'a> {
	Word(&'a str),
	RedirectStdout,
	RedirectStdin,
	Pipe,
}

fn next_token<'a>(s: &mut &'a str) -> Option<Token<'a>> {
	*s = s.trim_start();
	if s.is_empty() {
		return None;
	}
	if let Some(rest) = s.strip_prefix(">") {
		*s = rest;
		return Some(Token::RedirectStdout);
	}
	if let Some(rest) = s.strip_prefix("<") {
		*s = rest;
		return Some(Token::RedirectStdin);
	}
	if let Some(rest) = s.strip_prefix("|") {
		*s = rest;
		return Some(Token::Pipe);
	}
	for (i, c) in s.char_indices() {
		if c.is_whitespace() || c == '<' || c == '>' || c == '|' {
			let word;
			(word, *s) = s.split_at(i);
			return Some(Token::Word(word));
		}
	}
	let word = *s;
	*s = "";
	Some(Token::Word(word))
}

fn print_all_tokens(mut s: &str) {
	while let Some(token) = next_token(&mut s) {
		println!("{token:?}");
	}
}

fn main() {
	if ftkit::ARGS.len() != 2 {
		eprintln!("Error: wrong number of arguments (expected 1)");
		return;
	}

	print_all_tokens(&ftkit::ARGS[1]);
}
