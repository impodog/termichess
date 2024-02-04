use ::termichess::prelude::*;

fn main() {
    lazy_static::initialize(&CONFIG);
    loop {
        let mode = dialoguer::Select::new()
            .with_prompt("Main Menu")
            .items(&["Chess - Local", "Information", "Quit"])
            .default(0)
            .interact()
            .unwrap();
        match mode {
            0 => local::play_locally(),
            1 => {
                println!(
                    "Welcome to {}! This is a simple chess game written in {}.",
                    console::style("TermiChess").bold().cyan(),
                    console::style("Rust").bold().cyan()
                );
                println!("Currently, you can play locally against another player.\n");
                println!(
                    "For more references, please check the {} file.",
                    console::style("README.md").bold().blue()
                );
                println!()
            }
            2 => {
                println!("Goodbye!");
                break;
            }
            _ => unreachable!(),
        }
    }
}
