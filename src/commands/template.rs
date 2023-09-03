use git2::Repository;
use inquire::Select;
use phf::phf_map;
use spinoff::{spinners, Color, Spinner};
use std::env;

const PROJECTS: [&str; 2] = ["Official webiste", "Algo view"];

static URLS: phf::Map<&'static str, &'static str> = phf_map! {
    "Official webiste" => "https://github.com/BuckshotPlusPlus/Official-Website",
    "Algo view" => "https://github.com/BuckshotPlusPlus/Algo_ViewLibrary",
};

pub fn run() {
    let project = Select::new("Which template should I download?", PROJECTS.to_vec()).prompt();
    match project {
        Ok(choice) => {
            let mut spinner = Spinner::new(
                spinners::Dots,
                "Download template in progress...",
                Color::Blue,
            );

            let url = match URLS.get(choice) {
                Some(u) => u,
                None => return spinner.fail("Url cannot be found"),
            };

            let current_dir = match env::current_dir() {
                Ok(p) => p,
                Err(_) => return spinner.fail("Current directory cannot be found"),
            };

            match Repository::clone(url, current_dir) {
                Ok(repo) => repo,
                Err(e) => return spinner.fail(&format!("Failed to clone: {}", e)),
            };

            spinner.success("Done!");
        }
        Err(_) => panic!("There was an error, please try again"),
    }
}
