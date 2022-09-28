use std::{env, error::Error, fs, io, io::Write, process};

mod scanner;
mod token;
mod value;

fn run(source: &str) {
    let mut scanner = scanner::Scanner::new(source);

    let tokens = scanner.scan_tokens();

    for token in tokens {
        println!("{:?}", token)
    }
}

fn run_file(path: &str) -> Result<(), Box<dyn Error>> {
    let source = fs::read_to_string(path)?;

    run(&source);

    Ok(())
}

fn run_prompt() -> Result<(), Box<dyn Error>> {
    loop {
        print!("> ");
        io::stdout().flush()?;

        let mut line = String::new();

        io::stdin().read_line(&mut line)?;
        if line.trim().is_empty() {
            break;
        }

        run(&line)
    }

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    match args.len() {
        // TODO: don't unwrap?
        0 => run_prompt().unwrap(),
        1 => run_file(&args[0]).unwrap(),
        _ => {
            println!("Usage: krab [script]");
            // Command was used incorrectly
            process::exit(64);
        }
    }
}
