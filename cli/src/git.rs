use git2::{Branch, Repository};

pub struct Stack<'a> {
    /// the core Git repository that this stack is associated with
    pub repository: Repository,

    /// an ordered list of branches in the stack
    pub branches: Vec<Branch<'a>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        // construct a repository from the GIT_DIR environment variable
        let repo = Repository::open_from_env().expect("Failed to open repository");

        // print repository content
        println!("Repostirory: {:?}", repo.path());

        #[allow(unused_variables)]
        let stack = Stack {
            repository: repo,
            branches: Vec::new(),
        };
    }
}
