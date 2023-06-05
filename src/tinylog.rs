use std::fs;

use {
    crate::{
        content::{Kind, Page, Time},
        Error, ToDisk,
    },
    std::{fs::File, path::PathBuf},
    tinylog::{Entry, Tinylog},
    tinyrand::{Rand, RandRange, Seeded, StdRand},
    tinyrand_std::clock_seed::ClockSeed,
};

pub fn edit() -> Result<(), Error> {
    Page::edit(Kind::Page(Some(PathBuf::from("content/tinylog.gmi"))), "")
}

pub fn tags(tags: &[String]) -> Result<(), Error> {
    let path = PathBuf::from("content/tinylog.gmi");
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
    let path = PathBuf::from("content/tinylog.gmi");
    if !path.exists() {
        let _path = Page::create(
            Kind::Page(Some(path)),
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
    let path = PathBuf::from("content/tinylog.gmi");
    if !path.exists() {
        init(None, None, None)?;
    }
    if let Some(mut page) = Page::from_path(&path) {
        let mut log: Tinylog<Time> = page.content.parse()?;
        let time = Time::now();
        let entry = Entry {
            datetime: time,
            body: text.to_string(),
        };
        log.insert(entry);
        page.content = log.to_string();
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

pub fn create_post() -> Result<(), Error> {
    let seed = ClockSeed::default().next_u64();
    let mut rand = StdRand::seed(seed);
    let mut s = "/tmp/zond-".to_string();
    for _n in 0..9 {
        let c = char::from(rand.next_range(97_u32..123) as u8);
        s.push(c);
    }
    let fd = File::create(&s);
    drop(fd);
    crate::edit(&s)?;
    let post = fs::read_to_string(&s)?;
    if !post.is_empty() {
        update(&post, None)?;
    }
    fs::remove_file(&s)?;
    Ok(())
}
