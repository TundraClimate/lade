use crate::search_package;
use colored::Colorize;

pub fn search_package(package: &str) {
    let lade_result = search_package::lade(package);
    if lade_result.is_none() {
        let rade_result = search_package::rade(package);
        if rade_result.is_none() {
            println!("{} {}", ">>>".red().bold(), "Package not found.".bold());
        }

        if let Some(result) = rade_result {
            println!(
                "{} {}",
                ">>>".green().bold(),
                "Package found in rade package list.".bold()
            );
            println!("Name: {}", package);
            println!("Version: {}", result.version);
            println!("Repository: {}", result.repository);
            println!("Download: {}", result.download);
            println!("Language: {}", result.language);
            println!("Capacity: {}", result.capacity);
            if !["", "None"].contains(&result.dependencies.as_str()) {
                println!("Dependencies: {}", result.dependencies);
            }
            println!();
        }
    }

    if let Some(result) = lade_result {
        println!(
            "{} {}",
            ">>>".green().bold(),
            "Package found in lade package list.".bold()
        );

        println!("Name: {}", result.name);
        println!("Version: {}", result.version);
        println!("Repository: {}", result.repository);

        if let Some(download) = result.download {
            println!("Download: {}", download);
        }

        println!("Description: {}", result.description);
        println!("License: {}", result.license);

        if !result.authors.is_empty() {
            print!("Authors: ");
            for author in &result.authors {
                if author == result.authors.last().unwrap() {
                    print!("{}", author);
                } else {
                    print!("{}, ", author);
                }
            }
        }
        println!();

        if !result.dependencies.is_empty() {
            print!("Dependencies: ");
            println!("{}", result.dependencies.join(", "));
        }
        println!();
    }
}
