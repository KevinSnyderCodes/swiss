use std::collections::HashSet;
use std::net::Ipv4Addr;

use clap::Args;
use log::info;
use strum::IntoEnumIterator;

#[derive(Args)]
pub struct Sort {
    #[clap(short, long, default_value = "auto")]
    format: SortFormat,
}

#[derive(clap::ValueEnum, strum::EnumIter, Clone, Debug, Eq, Hash, PartialEq)]
pub enum SortFormat {
    // Special format
    Auto,

    // Default format
    Alphanumeric,

    // Custom formats
    IPv4,
}

fn get_formats(lines: &Vec<String>) -> HashSet<SortFormat> {
    // Create set with all formats
    let mut formats = HashSet::new();
    for format in SortFormat::iter() {
        formats.insert(format);
    }

    // Remove special and default formats
    formats.remove(&SortFormat::Auto);
    formats.remove(&SortFormat::Alphanumeric);

    // Remove formats that don't match
    for line in lines {
        if let Err(_) = line.parse::<Ipv4Addr>() {
            formats.remove(&SortFormat::IPv4);
        }
    }

    formats
}

impl Sort {
    pub fn run(self, lines: &mut Vec<String>) {
        let mut format = self.format;
        if format == SortFormat::Auto {
            let formats = get_formats(lines);
            match formats.len() {
                0 => {
                    info!("No matching formats found, defaulting to alphanumeric");
                    format = SortFormat::Alphanumeric;
                }
                1 => {
                    info!("One matching format found");
                    format = formats.iter().next().unwrap().clone();
                }
                _ => {
                    info!("Multiple matching formats found, defaulting to alphanumeric");
                    format = SortFormat::Alphanumeric;
                }
            }
        }

        match format {
            SortFormat::Alphanumeric => lines.sort(),
            SortFormat::IPv4 => lines.sort_by(|a, b| {
                let a = a.parse::<Ipv4Addr>().unwrap();
                let b = b.parse::<Ipv4Addr>().unwrap();
                a.cmp(&b)
            }),
            _ => unreachable!(),
        }
    }
}
