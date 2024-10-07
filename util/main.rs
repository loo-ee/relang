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
    define_ast(
        output_dir,
        "Expr",
        vec![
            "Binary: Expr left, Token operator, Expr right".to_string(),
            "Grouping : Expr expression".to_string(),
            "Literal: Object value".to_string(),
            "Unary: Token operator, Expr right".to_string(),
        ],
    )
    .expect("Cannot create file");
}

fn define_ast(output_dir: &str, base_name: &str, types: Vec<String>) -> std::io::Result<()> {
    let path = format!("{}/{}.rs", output_dir, base_name.to_lowercase());
    let mut file_writer = File::create(path)?;

    for t in &types {
        let parts: Vec<&str> = t.split(":").collect();
        let mut class_name: Option<&str> = None;
        let mut fields: Option<&str> = None;

        match parts.get(0) {
            Some(first) => class_name = Some(first.trim()),
            None => println!("Cannot extract first part in types"),
        }

        match parts.get(1) {
            Some(second) => fields = Some(second.trim()),
            None => println!("Cannot extract second part in types"),
        }

        if class_name.is_some() && fields.is_some() {
            define_type(
                &mut file_writer,
                base_name,
                class_name.unwrap(),
                fields.unwrap(),
            )
            .expect("Cannot run define_type fn");
        }
    }

    Ok(())
}

fn define_type(
    file_writer: &mut File,
    base_name: &str,
    class_name: &str,
    field_list: &str,
) -> std::io::Result<()> {
    write!(file_writer, "\nstruct {} {{\n", class_name)?;

    let fields: Vec<&str> = field_list.split(",").collect();
    for field in &fields {
        let name: Vec<&str> = field.trim().split(" ").collect();
        let mut first_part: &str = "";
        let mut second_part: &str = "";

        println!("{:?}", &name);

        match &name.get(0) {
            Some(first) => first_part = first,
            None => println!("Cannot get first part in field name"),
        }

        match &name.get(1) {
            Some(second) => second_part = second,
            None => println!("Cannot get second part in field name"),
        }

        write!(file_writer, "\t{}:{},\n", first_part, second_part)?;
    }

    write!(file_writer, "}}\n")?;
    write!(file_writer, "impl {} {{\n", class_name)?;
    write!(file_writer, "\tpub fn new(\n")?;

    for field in &fields {
        let name: Vec<&str> = field.split(" ").collect();

        match name.get(1) {
            Some(second) => write!(file_writer, "\t\t{}:{},\n", second, second)?,
            None => println!("Cannot get second part in field name"),
        }
    }

    write!(file_writer, "\t) -> {} {{\n", class_name)?;
    write!(file_writer, "\t\t{} {{\n", class_name)?;

    for field in &fields {
        let name: Vec<&str> = field.split(" ").collect();

        match name.get(1) {
            Some(second) => write!(file_writer, "\t\t\t{},\n", &second)?,
            None => println!("Cannot get second part in field name"),
        }
    }

    write!(file_writer, "\t\t}}\n")?;
    write!(file_writer, "\t}}\n")?;
    write!(file_writer, "}}\n")?;

    Ok(())
}
