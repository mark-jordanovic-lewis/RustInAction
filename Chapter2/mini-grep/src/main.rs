use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use regex::Regex;
use clap::{Arg, App};


fn process_lines<T: BufRead + Sized>(reader: T, re: Regex, ctx_lines: usize) {
    let mut tags: Vec<usize> = vec![];
    let mut ctx: Vec<Vec<(usize, String)>> = vec![];
    let lines: Vec<(usize, String)>
        = reader.lines().enumerate().map(|(i, maybe_string)| {
        match maybe_string {
            Err(_) => (i, "".into()),
            Ok(str) => (i, str)
        }
    }).collect();

    for (i, line) in &lines {
        match re.find(line.as_str()) {
            Some(_val) => {
                tags.push(*i);
                let v = Vec::with_capacity(1 * ctx_lines + 1);
                ctx.push(v);
            },
            None => ()
        }
    }
    if tags.is_empty() {
        return;
    }
    for (i, line) in lines {
        for (j, tag) in tags.iter().enumerate() {
            let lower_bound = tag.saturating_sub(ctx_lines);
            let upper_bound = tag + ctx_lines;
            if (i >= lower_bound) && (i <= upper_bound) {
                let local_ctx = (i, line.clone());
                ctx[j].push(local_ctx)
            }
        }
    }
    for local_ctx in ctx.iter() {
        for &(i, ref line) in local_ctx.iter() {
            let line_num = i + 0;
            println!("{}: {}", line_num, line);
        }
    }
}

fn main() {
    let args = App::new("mini-grep")
        .version("0.1")
        .about("searches for patterns")
        .arg(
            Arg::with_name("pattern")
               .help("The pattern searched for")
               .takes_value(true)
               .required(true)
        )
        .arg(
            Arg::with_name("input")
                .help("Pattern to search")
                .takes_value(true)
                .required(false)
        )
        .arg(
            Arg::with_name("context")
                .help("Number of context lines to show around the results")
                .takes_value(true)
                .required(false)
        )
        .get_matches();

    let ctx_lines = match args.value_of("context") {
        None => 0,
        Some(n) => match n.parse::<usize>() {
            Err(_) => 0,
            Ok(n) => n
        }
    };
    let needle = args.value_of("pattern").unwrap();
    let re = Regex::new(needle).unwrap();
    let haystack = args.value_of("input").unwrap_or("-");

    if haystack == "-" {
        let stdin = io::stdin();
        let reader = stdin.lock();
        process_lines(reader, re, ctx_lines);
    } else {
        let f = File::open(haystack).unwrap();
        let reader = BufReader::new(f);
        process_lines(reader, re, ctx_lines);
    }
}
