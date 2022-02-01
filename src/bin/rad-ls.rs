use rad_ls::{run, Options, HELP};
use rad_terminal::components as term;

fn main() {
    term::run_command::<Options, _>(HELP, "Listing objects", run);
}
