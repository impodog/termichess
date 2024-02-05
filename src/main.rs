use ::termichess::prelude::*;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    lazy_static::initialize(&CONFIG);

    loop {
        let mode = dialoguer::Select::new()
            .with_prompt("Main Menu")
            .items(&["Chess - Local", "Chess - Remote", "Information", "Quit"])
            .default(0)
            .interact()
            .unwrap();
        match mode {
            0 => local::play_locally(),
            1 => remote::play_remotely().await,
            2 => {
                println!(
                    "Welcome to {}! This is a simple chess game written in {}.",
                    console::style("TermiChess").bold().cyan(),
                    console::style("Rust").bold().cyan()
                );
                println!("Currently, you can play locally against another player, or connect to a server and play online.\n");
                println!(
                    "For more references, please check the {} file.",
                    console::style("README.md").bold().blue()
                );
                println!()
            }
            3 => {
                println!("Goodbye!");
                break;
            }
            _ => unreachable!(),
        }
    }
}
