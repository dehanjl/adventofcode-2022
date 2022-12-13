use std::path::{Path, PathBuf};
use std::{env, fs};
use structopt::StructOpt;

fn find_input_file(opt: &Opt) -> Option<PathBuf> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    path.push("inputs");
    if opt.real {
        path.push("real");
    } else {
        path.push("example");
    }

    let bin = env::args()
        .next()
        .as_ref()
        .map(Path::new)
        .and_then(Path::file_name)
        .and_then(std::ffi::OsStr::to_str)
        .map(String::from);

    if let Some(bin) = bin {
        path.push(bin);
        path.set_extension("txt");
        if path.exists() {
            return Some(path);
        }
    }

    None
}

pub fn runner(f: impl Fn(&str)) {
    let opt = Opt::from_args();

    let filepath = find_input_file(&opt).expect("Oh no! I couldn't find the input file :(");

    let input = fs::read_to_string(filepath).expect("Oh no! I couldn't read the file :(");
    f(&input);
}
#[derive(StructOpt)]
struct Opt {
    #[structopt(short, long)]
    real: bool,
}
