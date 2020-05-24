pub struct File {
    workspace: String,
}

impl File {
    fn new(workspace: &str) -> Self {
        File {
            workspace: workspace.to_string(),
        }
    }
}
