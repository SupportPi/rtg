use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use rand::prelude::*;
// Generates Files of Random Text 
// based on words stored in a local (to the directory) dict.txt
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut arg_num_words:i32 = 50;
    let mut arg_output_path = "./output.txt";
    let mut arg_delim = " ";
    let mut c = 0;
    let mut arg_trg: String = String::from("");


    // if substring "-help" or "-h" is anywhere within the Args
    let arg_str: String = args.clone().into_iter().collect();
    if arg_str.contains("-help") || arg_str.contains("-h"){
        println!("
                    \t-help -h -> help
                    \t-o -out -output -> file output path
                    \t-d -delim -delimiter -> delimiter
                    \t-l -len -length -> # words
        ");
        // if only arg, then exits
        if args.len() == 2 {
            std::process::exit(0)
        }
    }

    while c < args.len() {
        if args[c] == "help" && arg_trg == "" {
            print!("did you mean -h or -help?")
        }

        if arg_trg == "" {
            if args[c] == "-o" || args[c] == "-d" || args[c] == "-l" || args[c] == "-len" {
                arg_trg = String::from(args[c].as_str());
            } else if args[c] == "-output" || args[c] == "-out" || args[c] == "-delim" || args[c] == "delimiter" || args[c] == "-length" {
                arg_trg = String::from(args[c].as_str());
            }

        } else {
            if args[c].split("").collect::<String>().contains(&"-") {
                panic!("Invalid Argument: {}", args[c]);
            }

            if arg_trg == "-o" || arg_trg == "-out" || arg_trg == "-output" {
                arg_output_path = args[c].as_str();
            }

            if arg_trg == "-d" || arg_trg == "-delim" || arg_trg == "-delimiter" {
                if args[c] == "\\n" {
                    arg_delim = "\n";
                } else {
                    arg_delim = args[c].as_str();
                }
            }

            if arg_trg == "-l" || arg_trg == "-len" || arg_trg == "-length"{
                arg_num_words = args[c].parse::<i32>().unwrap();
            }
            arg_trg= String::from("");
        }

        c += 1;
    }
    print!("\n");

    let mut rng = thread_rng();

    let path = Path::new("dict.txt");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => print!("{} accessed!\n", display),
    } 
    
    let split_s = s.split_whitespace();

    let mut dict: Vec<&str> = Vec::new();
    let mut rtg: Vec<&str> = Vec::new();    
    for s in split_s {
        dict.push(s);
    }


    let mut c = 0;
    let dict_len = dict.len() - 1;
    while c < arg_num_words {
       let gen: usize = rng.gen_range(0..=dict_len).try_into().unwrap();
       rtg.push(dict[gen]);
       rtg.push(arg_delim);
       c += 1;
    }

    let o_string: String = rtg.into_iter().collect();
    
    let o_path = Path::new(arg_output_path);
    let mut o_file = match File::create(o_path) {
        Err(why) => panic!("couldn't create file at {}: {}", arg_output_path, why),
        Ok(file) => file,
    };

    match o_file.write_all(o_string.as_str().as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", arg_output_path, why),
        Ok(_) => print!("created file at {}", arg_output_path),
    };
    std::process::exit(0);
}
