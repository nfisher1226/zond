use {
    serde::{Deserialize, Serialize},
    std::fmt,
};
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
/// Represents one of several Creative Commons license types or a custom license
pub enum License {
    /// Credit must be given to the creator\
    /// [details](https://creativecommons.org/licenses/by/4.0/)
    CcBy,
    /// Credit must be given to the creator\
    /// Adaptations must be shared under the same terms\
    /// [details](https://creativecommons.org/licenses/by-sa/4.0/)
    CcBySa,
    /// Credit must be given to the creator\
    /// Only noncommercial uses of the work are permitted\
    /// [details](https://creativecommons.org/licenses/by-nc/4.0/)
    CcByNc,
    /// Credit must be given to the creator\
    /// Only noncommercial uses of the work are permitted\
    /// Adaptations must be shared under the same terms\
    /// [details](https://creativecommons.org/licenses/by-nc-sa/4.0/)
    CcByNcSa,
    /// Credit must be given to the creator\
    /// No derivatives or adaptations of the work are permitted\
    /// [details](https://creativecommons.org/licenses/by-nd/4.0/)
    CcByNd,
    /// Credit must be given to the creator\
    /// Only noncommercial uses of the work are permitted\
    /// No derivatives or adaptations of the work are permitted\
    /// [details](https://creativecommons.org/licenses/by-nc-nd/4.0/)
    CcByNcNd,
    /// Dedicated to public domain\
    /// [details](https://creativecommons.org/publicdomain/zero/1.0/)
    CcZero,
    /// A custom license of the author's choosing
    Other(
        /// Short text identifying the license
        String,
    ),
}

impl fmt::Display for License {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::CcBy => "CC BY",
                Self::CcBySa => "CC BY-SA",
                Self::CcByNc => "CC BY-NC",
                Self::CcByNcSa => "CC BY-NC-SA",
                Self::CcByNd => "CC BY-ND",
                Self::CcByNcNd => "CC BY-NC-ND",
                Self::CcZero => "CC Zero",
                Self::Other(s) => s,
            }
        )
    }
}

impl From<&str> for License {
    fn from(s: &str) -> Self {
        match s {
            "CcBy" | "ccby" => Self::CcBy,
            "CcBySa" | "ccbysa" => Self::CcBySa,
            "CcByNc" | "ccbync" => Self::CcByNc,
            "CcByNcSa" | "ccbyncsa" => Self::CcByNcSa,
            "CcByNd" | "ccbynd" => Self::CcByNd,
            "CcByNcNd" | "ccbyncnd" => Self::CcByNcNd,
            "CcZero" | "cczero" | "Cc0" | "cc0" => Self::CcZero,
            s => License::Other(s.to_string()),
        }
    }
}
