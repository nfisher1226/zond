use {
    atom_syndication as atom,
    crate::{AsAtom, config::Config, link::Link, content::Meta},
};

#[derive(Default)]
pub(crate) struct Post {
    pub meta: Meta,
    pub link: Link,
}

impl AsAtom<atom::Entry> for Post {
    type Err = crate::Error;

    /// Generates an atom feed entry for this post
    fn as_atom(&self, cfg: &Config) -> Result<atom::Entry, Self::Err> {
        let mut link = atom::Link::default();
        link.set_href(&self.link.url);
        link.set_rel("alternate");
        let author = cfg.author.to_atom();
        let entry = atom::EntryBuilder::default()
            .title(self.meta.title.clone())
            .id(&self.link.url)
            .updated(self.meta.published.as_ref().unwrap().to_date_time()?)
            .authors(vec![author])
            .categories(self.meta.categories(cfg)?)
            .link(link)
            .published(self.meta.published.as_ref().unwrap().to_date_time()?)
            .rights(atom::Text::plain(format!(
                "© {} by {}",
                self.meta.published.as_ref().unwrap().year(),
                &cfg.author.name
            )))
            .summary(self.meta.summary.as_ref().map(atom::Text::plain))
            .build();
        Ok(entry)
    }
}

