use {
    crate::{
        content::{Kind, Page, Time},
        Error, ToDisk,
    },
    std::{fmt::Write, path::PathBuf},
};

pub fn edit() -> Result<(), Error> {
    Page::edit(Kind::Page(Some(PathBuf::from("content/tinylog"))), "")
}

pub fn tags(tags: &[String]) -> Result<(), Error> {
    let path = PathBuf::from("content/tinylog");
    if !path.exists() {
        init(None, None, None)?;
    }
    if let Some(mut page) = Page::from_path(&path) {
        for t in tags.iter() {
            if !page.meta.tags.contains(t) {
                page.meta.tags.push(t.clone());
            }
        }
        page.to_disk(&path)?;
    }
    Ok(())
}

pub fn init(
    title: Option<&str>,
    summary: Option<&str>,
    tags: Option<Vec<&str>>,
) -> Result<(), Error> {
    let path = PathBuf::from("content/tinylog");
    if !path.exists() {
        let _path = Page::create(
            Kind::Page(Some(PathBuf::from("tinylog.gmi"))),
            if let Some(t) = title { t } else { "Tinylog" },
            summary,
            tags.unwrap_or(vec![])
                .iter()
                .map(|&x| x.to_string())
                .collect(),
        )?;
    }
    Ok(())
}

pub fn update(text: &str, tags: Option<Vec<String>>) -> Result<(), Error> {
    let path = PathBuf::from("content/tinylog");
    if !path.exists() {
        init(None, None, None)?;
    }
    if let Some(mut page) = Page::from_path(&path) {
        let time = Time::now();
        let mut tiny = String::new();
        writeln!(
            tiny,
            "## {}-{}-{} {}:{} UTC",
            time.year(),
            time.month(),
            time.day(),
            time.hour(),
            time.minute(),
        )?;
        writeln!(tiny, "{text}")?;
        writeln!(tiny, "{}", page.content)?;
        page.content = tiny;
        page.meta.published = Some(time);
        if let Some(tags) = tags {
            for t in &tags {
                if !page.meta.tags.contains(t) {
                    page.meta.tags.push(t.clone());
                }
            }
        }
        page.to_disk(&path)?;
    }
    Ok(())
}
