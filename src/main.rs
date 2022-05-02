use clap::Parser;
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// message to echo
    #[clap(short, long)]
    message: String,

    /// Number of times to echo
    #[clap(short, long, default_value_t = 1)]
    count: u8,
}
fn main() {
    let args = Args::parse();
    for _ in 0..args.count {
        println!("{}", args.message)
    }
}
pub struct ConfigFileReader {}

#[cfg(test)]
mod tests {
    /// it should create test config files if not exist
    #[test]
    fn it_creates_config() {
        assert_eq!(2 + 2, 5);
    }
    ///it should not overwrite the file configif it already exists
    #[test]
    fn it_doesnt_overwrite_config() {
        assert_eq!(2 + 2, 5);
    }
}
