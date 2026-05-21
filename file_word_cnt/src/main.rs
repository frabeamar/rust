use clap::Parser;
use std::fs::File;
// need to import a Trait, before a method can call on it
use std::io::{self, BufRead, BufReader};

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    input: String,
}

fn main() {
    let args = Args::parse();
    println!("{}", args.input);
    match count_words_in_file(&args.input){
        Ok(count) => println!("There are {} words in {}", count, args.input),
        Err(_e)=>eprintln!("No file found")
    }
    

}


fn count_words_in_file(filename:&str)->Result<usize, io::Error>{
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut tot_wc = 0;
    for line in reader.lines(){
        let line = line.unwrap();
        let wc: usize = line.split_whitespace().count();
        println!("{} {}", wc, line);
        tot_wc += wc;
    }
    return Ok(tot_wc)
}


