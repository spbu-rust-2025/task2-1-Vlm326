use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .expect("Error: can't read from stdin");
    let mut length = input.len();
    while length > 0 {
        match input.as_bytes().windows(length).find(|slice| {
            let iter = slice.iter();
            iter.clone().eq(iter.clone().rev())
        }) {
            Some(palindrome) => {
                println!("{}", String::from_utf8_lossy(palindrome));
                break;
            }
            None => length -= 1,
        }
    }
}
