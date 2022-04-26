#![feature(string_remove_matches)]
use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;

#[cfg(test)]
#[test]
fn parsy_parster() {
	let x = "<p>this is a paragraph</p>";
	let mut meta = String::new();
	let mut count = 0;

	for i in x.chars() {
		match i {
			'<' => count = 0,
			'>' => count = 1,
			_ => (),
		}

		if count == 1 {
			meta.push(i);
		}
	}
	meta.remove_matches('>');

	assert_eq!(meta, "this is a paragraph")
}


#[cfg(test)]
#[test]
fn open_html() -> std::io::Result<()>{
	let file = File::open("rust.html")?;
	let buf_reader = BufReader::new(file);
	let mut contents = Vec::new();
	
	for line in buf_reader.lines() {
		contents.push(line?);
	}

	assert_eq!(contents[0], "<!doctype html>");
	Ok(())
}