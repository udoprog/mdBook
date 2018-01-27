use url::Url;
use relative_path::RelativePath;

/// Translate the given destination from a relative link with an '.md' extension, to a link with
/// a '.html' extension.
pub struct ChangeExtLinkFilter<'a, F> {
    is_dest: F,
    expected: &'a str,
    ext: &'a str,
}

impl<'a, F> ChangeExtLinkFilter<'a, F>
    where F: Fn(&RelativePath) -> bool
{
    pub fn new(is_dest: F, expected: &'a str, ext: &'a str) -> ChangeExtLinkFilter<'a, F> {
        ChangeExtLinkFilter {
            is_dest: is_dest,
            expected: expected,
            ext: ext,
        }
    }
}

impl<'a, F> LinkFilter for ChangeExtLinkFilter<'a, F>
    where F: Fn(&RelativePath) -> bool
{
    fn apply(&self, dest: &str) -> Option<String> {
        use url::ParseError;

        // Verify that specified URL is relative.
        if let Err(ParseError::RelativeUrlWithoutBase) = Url::parse(dest) {
            let dest = RelativePath::new(dest);

            if (self.is_dest)(dest) {
                let mut components = dest.components();

                if let Some(head) = components.next_back() {
                    let mut head = head.split('.');

                    if Some(self.expected) == head.next_back() {
                        let mut full_dest = components.map(str::to_string).collect::<Vec<_>>();

                        full_dest.push(
                            format!("{}.{}", head.collect::<Vec<_>>().join("."), self.ext)
                        );

                        return Some(full_dest.join("/"));
                    }
                }
            }
        }

        None
    }
}

/// A filter to optionally apply to links.
pub trait LinkFilter {
    /// Optionally translate the given destination, if applicable.
    fn apply(&self, dest: &str) -> Option<String>;
}
