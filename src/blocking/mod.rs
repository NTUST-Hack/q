use std::time::Duration;

pub use crate::CourseDetails;
pub use crate::QueryError;
use crate::{CourseInfo, Language, SearchOptions};

use crate::async_impl;

pub struct ClientBuilder<'a> {
    async_builder: async_impl::ClientBuilder<'a>,
}

impl<'a> ClientBuilder<'a> {
    pub fn new() -> Self {
        Self {
            async_builder: async_impl::ClientBuilder::new(),
        }
    }

    pub fn user_agent(mut self, user_agent: &'a str) -> Self {
        self.async_builder = self.async_builder.user_agent(&user_agent);
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.async_builder = self.async_builder.timeout(timeout);
        self
    }

    pub fn local_address(mut self, addr: std::net::IpAddr) -> Self {
        self.async_builder = self.async_builder.local_address(addr);
        self
    }

    pub fn build(self) -> Result<Q, Box<dyn std::error::Error>> {
        Ok(Q {
            async_q: self.async_builder.build()?,
            runtime: tokio::runtime::Runtime::new().unwrap(),
        })
    }
}

pub struct Q {
    async_q: async_impl::Q,
    runtime: tokio::runtime::Runtime,
}

impl Q {
    pub fn new() -> Self {
        ClientBuilder::new().build().unwrap()
    }

    pub fn search(
        &self,
        options: &SearchOptions,
        merge_courses: bool,
    ) -> Result<Vec<CourseInfo>, QueryError> {
        self.runtime
            .block_on(self.async_q.search(options, merge_courses))
    }

    pub fn query(
        &self,
        semester: &str,
        course_no: &str,
        language: Language,
    ) -> Result<CourseDetails, QueryError> {
        self.runtime
            .block_on(self.async_q.query(semester, course_no, language))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let _client = Q::new();
    }

    #[test]
    fn search() {
        let client = Q::new();

        let mut options = SearchOptions::new("1131", Language::Zh);

        options.course_no = "cs".to_string();

        let _details = client
            .search(&options, true)
            .expect("failed to search courses");

        println!("{:#?}", _details);
    }

    #[test]
    fn query() {
        let client = Q::new();

        let _details = client
            .query("1122", "AT2005701", Language::Zh)
            .expect("Failed to query");

        println!("{:#?}", _details)
    }

    #[test]
    fn query_all_cs() {
        let client = Q::new();

        let mut options = SearchOptions::new("1131", Language::Zh);

        options.course_no = "cs".to_string();

        let search_results = client
            .search(&options, true)
            .expect("failed to search courses");

        let _ = search_results.iter().for_each(|c| {
            let query_client = Q::new();

            let details = query_client
                .query(&c.semester, &c.course_no, Language::Zh)
                .expect("Failed to query");

            println!("{:#?}", details)
        });
    }
}
