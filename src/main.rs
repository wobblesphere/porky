use porky::get_branches;
use porky::checkout_branch;
use porky::get_current_branch;
use std::process;
use dialoguer::{
    Select,
    theme::ColorfulTheme
};
use console::Term;

fn main() -> std::io::Result<()> {
    let branches = get_branches();
    if branches.len() < 1 {
        eprintln!("No branches found");
        process::exit(1);
    }

    let current_branch = get_current_branch();
    let current_branch_index = branches.iter().position(|r| r == &current_branch).unwrap();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .items(&branches)
        .default(current_branch_index)
        .interact_on_opt(&Term::stderr())?;

    match selection {
        Some(index) => checkout_branch(&branches[index].clone()),
        None => println!("You did not select a branch")
    }

    Ok(())
}
