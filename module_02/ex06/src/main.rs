#[derive(Debug, PartialEq)]
enum Token<'a> {
	Word(&'a str),
	RedirectStdout,
	RedirectStdin,
	Pipe,
}

/// Searches for the next token in a given string.
///
/// # Parameters
/// * `s` - The string to search in.
///
/// # Return
/// - `Some(Token::Word(word))` - The next token is a word.
/// - `Some(Token::RedirectStdout)` - The next token is `>`.
/// - `Some(Token::RedirectStdin)` - The next token is `<`.
/// - `Some(Token::Pipe)` - The next token is `|`.
/// - `None` - There are no more tokens.
///
/// # Example
/// ```
/// let mut s: &str = "ls -l|<input.txt cat -e>output.txt";
///
/// assert_eq!(next_token(&mut s), Some(Token::Word("ls")));
/// assert_eq!(next_token(&mut s), Some(Token::Word("-l")));
/// assert_eq!(next_token(&mut s), Some(Token::Pipe));
/// assert_eq!(next_token(&mut s), Some(Token::RedirectStdin));
/// assert_eq!(next_token(&mut s), Some(Token::Word("input.txt")));
/// assert_eq!(next_token(&mut s), Some(Token::Word("cat")));
/// assert_eq!(next_token(&mut s), Some(Token::Word("-e")));
/// assert_eq!(next_token(&mut s), Some(Token::RedirectStdout));
/// assert_eq!(next_token(&mut s), Some(Token::Word("output.txt")));
/// assert_eq!(next_token(&mut s), None);
/// ```
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

/// Finds out and prints every token that composes a given string.
///
/// # Parameters
/// * `s` - The string to search in.
///
/// # Example
/// ```
/// let mut s: &str = "ls -l|<input.txt cat -e>output.txt";
///
/// print_all_tokens(&mut s);
/// ```
fn print_all_tokens(mut s: &str) {
	while let Some(token) = next_token(&mut s) {
		println!("{token:?}");
	}
}

fn main() {
	if ftkit::ARGS.len() != 2 {
		eprintln!("error: exacltly one argument expected");
		return;
	}

	print_all_tokens(&ftkit::ARGS[1]);
}
