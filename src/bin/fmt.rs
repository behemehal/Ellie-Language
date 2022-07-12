use ellie_engine::cli_options;

fn main() {
    let app = cli_options::generate_elliefmt_options();
    let matches = app.get_matches();

    match matches.subcommand() {
        Some(("format", _)) => {
            unimplemented!()
        }
        Some(("analyze", _)) => {
            unimplemented!()
        }
        _ => unreachable!("clap should ensure we don't get here"),
    }
}
