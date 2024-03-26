use crate::error_and_exit;

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
    Regex,
    StartsWith,
}

impl Filter {
    pub fn run(self, lines: &mut Vec<String>) {
        match self.by {
            FilterBy::Contains => lines.retain(|line| line.contains(&self.filter)),
            FilterBy::EndsWith => lines.retain(|line| line.ends_with(&self.filter)),
            FilterBy::Regex => {
                let regex = match regex::Regex::new(&self.filter) {
                    Ok(regex) => regex,
                    Err(_) => error_and_exit!("Regex error"),
                };
                lines.retain(|line| regex.is_match(line));
            }
            FilterBy::StartsWith => lines.retain(|line| line.starts_with(&self.filter)),
        }
    }
}
