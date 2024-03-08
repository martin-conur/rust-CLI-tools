use clap::{Arg, ArgAction, Command};

fn main() {
    let matches = Command::new("echor")
        .version("0.1.0")
        .author("Martin Contreras Uribe <martincontrerasur@gmail.com>")
        .about("Rust echo")
        .arg_required_else_help(true)
        .arg(
            Arg::new("text")
                .value_name("TEXT")
                .help("input text")
                .action(ArgAction::Append)
                .required(true),
        )
        .arg(
            Arg::new("omit_newline")
                .short('n')
                .help("do not print newline")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    let text_vec: Vec<&str> = matches
        .get_many("text")
        .unwrap()
        .map(String::as_str)
        .collect();

    let text = text_vec.join(" ");

    let omit_newline: bool = *matches.get_one("omit_newline").unwrap();

    print!("{}{}", text, if omit_newline {""} else {"\n"});
    // println!("{:#?}", matches);

}
