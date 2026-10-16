//! 2008-2010 [KayCom](https://kaycom.ru) is a web studio in Vladimir.
//!
//! KayCom is a web studio with a mobile development team and many projects in Russia and around
//! the world.
use resume_macro::Project;

/// Russian online [poster shop](https://web.archive.org/web/20100416090318/http://www.postershop.ru/).
#[deprecated]
#[derive(Project)]
#[role(Developer, langs(PHP, JS), param(Language))]
#[tech(languages::PHP, "")]
#[tech(languages::JS, "")]
#[tech(databases::MySQL, "")]
pub struct PosterShop;

/// US online [bookstore](https://web.archive.org/web/20101225030926/http://www.cyberread.com/).
#[deprecated]
#[derive(Project)]
#[role(Developer, langs(PHP, JS), param(Language))]
#[tech(languages::PHP, "")]
#[tech(languages::JS, "")]
#[tech(databases::MySQL, "")]
pub struct CyberRead;
