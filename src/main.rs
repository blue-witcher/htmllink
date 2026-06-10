use std::{env, process::ExitCode};

fn create_html_link(link: &str) {
    let pre = "<html>\n<head>\n";
    let post = "\n</head>\n</html>";
    println!(
        "{}<meta http-equiv=\"refresh\" content=\"0; url={}\" />{}",
        pre, link, post
    );
}

fn main() -> ExitCode {
    // dump cli arguments to args variable
    let args = env::args().skip(1).collect::<Vec<String>>().join(" ");
    if args.len() != 0 {
        create_html_link(&args);
        ExitCode::from(0)
    } else {
        eprintln!("You need to provide a URL via command-line argument.");
        ExitCode::from(1)
    }
}
