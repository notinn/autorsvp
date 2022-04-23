mod querying;
use crate::querying::perform_query;
use std::fs;
    
//User settings struct
struct UserSettings {
    token: String
}

impl UserSettings {
    fn new(file: String) -> UserSettings {
        let mut lines = file.lines();
        let token = lines.next().unwrap().to_string();

        UserSettings { token }
    }
}

fn main() {
    //get settings file
    let settings_file = fs::read_to_string("settings\\settings.txt")
        .expect("Settings file read error");
    
    //Build settings struct to have access to token.
    let settings = UserSettings::new(settings_file);

    async_std::task::block_on(async {
        perform_query(settings.token).await;
        println!("Query Done.");
    })
}