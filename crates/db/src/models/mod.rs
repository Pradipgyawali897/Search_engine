pub mod crawl_target;
pub mod discovered_link;
pub mod document;
pub mod document_content;
pub mod document_term;
pub mod term;
pub mod url;

pub use crawl_target::{CrawlStatus, CrawlTarget};
pub use discovered_link::{DiscoveredLink, LinkCategory};
pub use document::Document;
pub use document_content::DocumentContent;
pub use document_term::DocumentTerm;
pub use term::Term;
pub use url::{UrlParts, parse_canonical_url};

pub type Posting = DocumentTerm;
