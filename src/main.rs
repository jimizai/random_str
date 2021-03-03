use clap::{App, Arg};
use rand::{thread_rng, Rng};

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                        abcdefghijklmnopqrstuvwxyz\
                        0123456789)(*&^%$#@!~";

fn get_random_str(len: u32) -> String {
    let mut rng = thread_rng();
    (0..len)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect::<String>()
}

fn main() {
    let matches = App::new("Random String Generator")
        .version("0.0.1")
        .author("dyk <woshitiancai359@gmail.com>")
        .about("Randomly generate strings according to length")
        .arg(
            Arg::with_name("length")
                .short("l")
                .long("length")
                .takes_value(true)
                .help("length for generate strings"),
        )
        .get_matches();
    let length = matches.value_of("length").unwrap_or("32");
    match length.parse::<u32>() {
        Ok(length) => println!("{}", get_random_str(length)),
        Err(_) => println!("Length must be a unsigned number"),
    };
}
