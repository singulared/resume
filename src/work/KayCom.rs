//! 2008-2010 [KayCom](https://kaycom.ru) is a web-studio in Vladimir city.
//!
//! KayCom is a web-studio with mobile development team and many projects in Russia and around
//! the world.
use crate::{
    roles::Developer,
    technologies::{
        databases::MySQL,
        languages::{JS, PHP},
    },
    CleanupDocs,
};

/// Russian online [poster shop](https://web.archive.org/web/20100416090318/http://www.postershop.ru/).
#[deprecated]
#[derive(CleanupDocs)]
pub struct PosterShop;
impl<Language> Developer<Language> for PosterShop where Language: PHP + JS {}
impl PHP for PosterShop {}
impl JS for PosterShop {}
impl MySQL for PosterShop {}

/// US online [bookstore](https://web.archive.org/web/20101225030926/http://www.cyberread.com/).
#[deprecated]
#[derive(CleanupDocs)]
pub struct CyberRead;
impl<Language> Developer<Language> for CyberRead where Language: PHP + JS {}
impl PHP for CyberRead {}
impl JS for CyberRead {}
impl MySQL for CyberRead {}
