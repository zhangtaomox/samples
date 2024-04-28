use std::io::BufRead;
use structopt::StructOpt;
use rand::Rng;

#[derive(StructOpt, Debug)]
#[structopt(name = "samples", about = "sample lines from a text file(plain text, csv, log...) or from stdin", version = "1.0.0")]
pub struct Opt {
    #[structopt(name = "k", short, long)]
    pub k: usize,
    #[structopt(name = "file", short, long, default_value = "-")]
    pub file: String,
}

fn main() {
    let opt = Opt::from_args();
    let file = std::fs::File::open(opt.file).map_err(|e| eprintln!("ERROR: {}", e)).unwrap_or_else(|_| std::process::exit(1));
    let reader = std::io::BufReader::new(file);

    let mut result_lines: Vec<String> = Vec::with_capacity(opt.k);
    let mut rng = rand::thread_rng();
    for (i, line) in reader.lines().enumerate() {
        let line = line.map_err(|e| eprintln!("ERROR: {}", e)).unwrap_or_else(|_| std::process::exit(1));
        if i < opt.k {
            result_lines.push(line);
        } else {
            let r = rng.gen_range(0..=i);
            if r < opt.k {
                result_lines[r] = line;
            }
        }
    }
    for line in result_lines {
        println!("{line}");
    }
}
