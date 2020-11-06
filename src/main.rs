type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let matches = clap::App::new(clap::crate_name!())
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about(clap::crate_description!())
        .arg(
            clap::Arg::with_name("PATTERN")
                .help("The pattern to repeat")
                .takes_value(true)
                .required(true),
        )
        .arg(
            clap::Arg::with_name("COUNT")
                .help("The number of times to generate")
                .takes_value(true)
                .required(true),
        )
        .arg(
            clap::Arg::with_name("OUTPUT_FILE")
                .long("write")
                .help("The outputfile to write to")
                .takes_value(true),
        )
        .get_matches();

    let pattern = matches.value_of("PATTERN").unwrap();
    let count = matches.value_of("COUNT").unwrap().parse::<usize>()?;

    let generator = StringGenerator::new(pattern, count);

    match matches.value_of("OUTPUT_FILE") {
        Some(output_file) => {
            write(
                std::io::BufWriter::new(std::fs::File::create(output_file)?),
                generator,
            )?;
        }
        None => {
            write(std::io::stdout(), generator)?;
        }
    }
    Ok(())
}

fn write<R: std::io::Write>(mut file: R, generator: StringGenerator) -> Result<()> {
    for string in generator {
        writeln!(file, "{}", string)?;
    }
    file.flush()?;
    Ok(())
}

struct StringGenerator {
    pattern: String,
    begin_count: usize,
    end_count: usize,
}

impl StringGenerator {
    pub fn new(pattern: &str, count: usize) -> Self {
        Self {
            pattern: pattern.to_string(),
            end_count: count,
            begin_count: 0,
        }
    }

    fn parse_and_replace(&self) -> String {
        self.pattern.replace("[]", &format!("{}", self.begin_count))
    }
}

impl Iterator for StringGenerator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.begin_count == self.end_count {
            None
        } else {
            self.begin_count += 1;
            Some(self.parse_and_replace())
        }
    }
}
