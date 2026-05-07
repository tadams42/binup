use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct AppVersion(pub u64, pub u64, pub u64);

impl AppVersion {
    pub fn parse(s: &str) -> Option<Self> {
        let s = s.trim();
        // Strip leading non-numeric prefix (e.g. 'v', 'V', letters)
        let s = s.trim_start_matches(|c: char| !c.is_ascii_digit());
        if s.is_empty() {
            return None;
        }
        // Replace hyphens with dots for versions like "1.2.3-4"
        let s = s.replace('-', ".");
        let parts: Vec<u64> = s
            .split('.')
            .take(3)
            .map(|p| {
                let num: String = p.chars().take_while(|c| c.is_ascii_digit()).collect();
                num.parse::<u64>().unwrap_or(0)
            })
            .collect();

        if parts.is_empty() {
            return None;
        }

        Some(AppVersion(
            *parts.first().unwrap_or(&0),
            *parts.get(1).unwrap_or(&0),
            *parts.get(2).unwrap_or(&0),
        ))
    }
}

impl fmt::Display for AppVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.0, self.1, self.2)
    }
}
