use {
    crate::{config::Config, content::Meta},
    serde::{Deserialize, Serialize},
    std::{fmt, path::Path},
};

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
/// Represents both a url and the text to be displayed
pub struct Link {
    pub url: String,
    pub display: String,
}

impl fmt::Display for Link {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "=> {} {}", self.url, self.display)
    }
}

impl Link {
    pub fn get(origin: &Path, cfg: &Config, meta: &Meta) -> Result<Self, crate::Error> {
        let mut url = cfg.url()?;
        let mut current = std::env::current_dir()?;
        current.push("content");
        let path = origin.strip_prefix(current)?;
        url.set_path(&path.to_string_lossy());
        Ok(Self {
            url: url.to_string(),
            display: format!(
                "{} - {}",
                meta.published.as_ref().unwrap().date_string(),
                &meta.title,
            ),
        })
    }
}
