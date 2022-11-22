use std::{process::{Command}};

pub fn get_branches() -> Vec<String> {
    println!("geeeeeeee! -- listing your branches");
    let command_output = Command::new("git")
                .args(["branch", "--sort=-committerdate"])
                .output()
                .expect("failed to execute get git branch command");
    let output_string  = String::from_utf8(command_output.stdout).unwrap();
    let branches = output_string.lines().map(|s| s.replace("*", "").trim().to_string()).collect();
    return branches;
}

pub fn checkout_branch(branch_name: &String) {
    println!("gee gee gee -- checking out branch {}", branch_name.trim());
    Command::new("git")
        .arg("checkout")
        .arg(branch_name.trim())
        .status()
        .expect(&format!("failed to go to branch: {}!", branch_name));
}

pub fn get_current_branch() -> String {
    let output = Command::new("git")
        .args(["branch", "--show-current"])
        .output()
        .expect("Failed to get current branch");
    let branch_name = String::from_utf8(output.stdout).unwrap();
    let branch_name = branch_name.replace("\n", "");
    return branch_name;
}

#[cfg(test)]
mod branch_switcher_tests {
    use super::*;

    struct Setup {
        test_branch_name: String,
    }
    
    impl Setup {
        fn new(branch_name: String) -> Self {
            Self {
                test_branch_name: branch_name,
            }
        }
        fn create_test_branch(&self) {
            Command::new("git")
            .args(["branch", &self.test_branch_name])
            .status()
            .expect("Failed to create test branch");
        }
        fn delete_test_branch(&self) {
            Command::new("git")
            .args(["branch", "--delete", &self.test_branch_name])
            .status()
            .expect("Failed to delete test branch");
        }
    }

    #[test]
    fn switch_branch_test() {
        let setup = Setup::new("test-switch-branch".to_string());
        setup.create_test_branch();
        let original_branch = get_current_branch();

        let branches = get_branches();
        assert!(branches.iter().any(|e| e == &setup.test_branch_name));

        checkout_branch(&setup.test_branch_name);
        assert_eq!(&get_current_branch(), &setup.test_branch_name);

        checkout_branch(&original_branch);

        setup.delete_test_branch();
        let branches = get_branches();
        assert!(!branches.iter().any(|e| e == &setup.test_branch_name));
    }
}
