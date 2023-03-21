use cloudy::ProfileSet;

fn main() {
    let profile_set = match ProfileSet::load() {
        Ok(p) => p,
        Err(err) => {
            eprintln!("[Error] Issue loading profiles {}", err);
            std::process::exit(1)
        }
    };
    if profile_set.profiles.len() == 0 {
        eprintln!("[Error] need at least one AWS profile to work");
        std::process::exit(1)
    }

    println!("{:#?}", profile_set)
}
