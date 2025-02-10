use std::path::Path;

use tournament_core::Tournament;

fn main() {
    let path = Path::new("conf.json");
    let Ok(mut trounament) = Tournament::from_json_file(path) else {
        println!("could not parse file");
        return;
    };

    println!("tournament: {:?}", trounament);

    trounament.name = "new_name".to_string();

    trounament.to_json_file(path).unwrap();
}
