#[derive(clap::Args)]
pub struct Filter {
    filter: String,

    #[clap(long, default_value = "contains")]
    by: FilterBy,
}

#[derive(clap::ValueEnum, Clone)]
pub enum FilterBy {
    // Default value
    Contains,

    // Other values
    EndsWith,
    StartsWith,
}

impl Filter {
    pub fn run(self, lines: &mut Vec<String>) {
        match self.by {
            FilterBy::Contains => lines.retain(|line| line.contains(&self.filter)),
            FilterBy::EndsWith => lines.retain(|line| line.ends_with(&self.filter)),
            FilterBy::StartsWith => lines.retain(|line| line.starts_with(&self.filter)),
        }
    }
}
