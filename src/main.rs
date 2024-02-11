use ::termichess::prelude::*;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    lazy_static::initialize(&CONFIG);

    loop {
        let mode = dialoguer::Select::new()
            .with_prompt("Main Menu")
            .items(&["Play Online", "Play Local", "Information", "Quit"])
            .default(0)
            .interact()
            .unwrap();
        match mode {
            0 => remote::play_remotely().await,
            1 => local::play_locally(),
            2 => {
                println!(
                    "Welcome to {}! This is a simple chess game written in {}. Licensed under {}.",
                    console::style("TermiChess").bold().magenta(),
                    console::style("Rust").bold().cyan(),
                    console::style("MIT License").bold().cyan()
                );
                println!("Currently, you can play {} against another player, or play {} using a personal server.\n",
                    console::style("locally").bold().green(),
                    console::style("online").bold().green()
                );
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
