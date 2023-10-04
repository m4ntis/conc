use crate::gba::Gba;

#[derive(clap::Args)]
pub struct Args {
    file: std::path::PathBuf,
}

pub fn run(args: Args) {
    println!("Called run on {}\n", args.file.display());

    let g = Gba::new();
    println!(
        "Can't run gba files yet, here's a dump of the inital GBA struct \
         for now :)\n{:?}",
        g
    );
}
