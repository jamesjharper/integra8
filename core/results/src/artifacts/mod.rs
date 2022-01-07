pub mod stdio;

#[derive(Clone, PartialEq, Debug)]
pub struct ComponentRunArtifacts {
    pub stdio: stdio::TestResultStdio,
    // Implement more here in the future
}
