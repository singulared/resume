//! 2008-2010 [KayCom](https://kaycom.ru) is a web-studio in Vladimir city.
//!
//! KayCom is a web-studio with mobile development team and many projects in Russia and around
//! the world.
use resume_macro::Project;

/// Russian online [poster shop](https://web.archive.org/web/20100416090318/http://www.postershop.ru/).
#[deprecated]
#[derive(Project)]
#[project(parent = KayCom)]
#[role(Developer, langs(PHP, JS), param(Language))]
#[tech(languages::PHP, "")]
#[tech(languages::JS, "")]
#[tech(databases::MySQL, "")]
pub struct PosterShop;

/// US online [bookstore](https://web.archive.org/web/20101225030926/http://www.cyberread.com/).
#[deprecated]
#[derive(Project)]
#[project(parent = KayCom)]
#[role(Developer, langs(PHP, JS), param(Language))]
#[tech(languages::PHP, "")]
#[tech(languages::JS, "")]
#[tech(databases::MySQL, "")]
pub struct CyberRead;
