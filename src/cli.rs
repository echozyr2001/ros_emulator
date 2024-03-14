use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
  #[clap(required = true, help = "The file path to the lua script.")]
  pub file_path: String,
}

pub fn parse_args() -> Args {
  Args::parse()
}
