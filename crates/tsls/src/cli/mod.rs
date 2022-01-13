use log::error;

pub fn setup(lists: Vec<String>) {
    // check lists is all
    if lists.len() > 1 {
        error!("tsls: error: too many arguments");
        std::process::exit(1);
    }
    if lists[0] != "all" {
        error!("tsls: error: invalid argument");
        std::process::exit(1);
    }
}
