use {
    atom_syndication as atom,
    crate::{CONFIG, link::Link, content::Meta},
};

#[derive(Default)]
pub(crate) struct Post {
    pub meta: Meta,
    pub link: Link,
}

impl TryFrom<&Post> for atom::Entry {
    type Error = crate::Error;

    /// Generates an atom feed entry for this post
    fn try_from(post: &Post) -> Result<atom::Entry, Self::Error> {
        let mut link = atom::Link::default();
        link.set_href(&post.link.url);
        link.set_rel("alternate");
        let author = CONFIG.author.to_atom();
        let entry = atom::EntryBuilder::default()
            .title(post.meta.title.clone())
            .id(&post.link.url)
            .updated(post.meta.published.as_ref().unwrap().to_date_time()?)
            .authors(vec![author])
            .categories(post.meta.categories()?)
            .link(link)
            .published(post.meta.published.as_ref().unwrap().to_date_time()?)
            .rights(atom::Text::plain(format!(
                "© {} by {}",
                post.meta.published.as_ref().unwrap().year(),
                &CONFIG.author.name
            )))
            .summary(post.meta.summary.as_ref().map(atom::Text::plain))
            .build();
        Ok(entry)
    }
}

