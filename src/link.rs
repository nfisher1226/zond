use {
    crate::{content::Meta, CONFIG},
    serde::{Deserialize, Serialize},
    std::{env, fmt, path::Path},
};

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
/// Represents both a url and the text to be displayed
pub struct Link {
    pub url: String,
    /// The string which will be displayed in lieu of the full url
    pub display: String,
}

impl fmt::Display for Link {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "=> {} {}", self.url, self.display)
    }
}

impl Link {
    /// Takes the original path in the content tree and meta information and
    /// returns a `Link` struct
    pub fn get(origin: &Path, meta: &Meta) -> Result<Self, crate::Error> {
        let mut url = CONFIG.url()?;
        let mut current = env::current_dir()?;
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
