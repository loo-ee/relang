use std::{env, fmt::write, fs::File, io::Write, process};

pub fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Format: [ast file name]");
        process::exit(64);
    } else if args.len() == 2 {
        run(&args[1]);
    }
}

fn run(output_dir: &str) {
    define_ast(output_dir, "Test", vec![]).expect("Cannot create file");
}

fn define_ast(output_dir: &str, base_name: &str, types: Vec<String>) -> std::io::Result<()> {
    let path = format!("{}/{}.rs", output_dir, base_name.to_lowercase());
    let mut file_writer = File::create(path)?;

    write!(file_writer, "trait {} {{\n", base_name)?;
    write!(file_writer, "\tprintln!(\"Hello world\");\n")?;
    write!(file_writer, "}}")?;
    Ok(())
}
