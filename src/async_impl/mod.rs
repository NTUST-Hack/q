use std::collections::HashMap;
use std::time::Duration;

use crate::{
    CourseDetails, CourseInfo, Language, QueryError, SearchOptions, DEFAULT_TIMEOUT,
    DEFAULT_USER_AGENT,
};

#[derive(Default, Debug)]
pub struct ClientBuilder<'a> {
    reqwest_builder: reqwest::ClientBuilder,

    user_agent: &'a str,
    timeout: Duration,
}

impl<'a> ClientBuilder<'a> {
    pub fn new() -> Self {
        ClientBuilder {
            reqwest_builder: reqwest::ClientBuilder::new(),

            user_agent: DEFAULT_USER_AGENT,
            timeout: DEFAULT_TIMEOUT,
        }
    }

    pub fn user_agent(mut self, user_agent: &'a str) -> Self {
        self.user_agent = user_agent;
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn local_address(mut self, addr: std::net::IpAddr) -> Self {
        self.reqwest_builder = self.reqwest_builder.local_address(addr);
        self
    }

    pub fn build(self) -> Result<Q, Box<dyn std::error::Error>> {
        Ok(Q {
            http_client: self
                .reqwest_builder
                .user_agent(self.user_agent)
                .timeout(self.timeout)
                .build()?,
        })
    }
}

pub struct Q {
    http_client: reqwest::Client,
}

impl Q {
    pub fn new() -> Self {
        ClientBuilder::new().build().unwrap()
    }

    pub async fn search(&self, options: &SearchOptions) -> Result<Vec<CourseInfo>, QueryError> {
        let resp = match self
            .http_client
            .post("https://querycourse.ntust.edu.tw/querycourse/api/courses")
            .json(&options)
            .send()
            .await
        {
            Ok(resp) => resp,
            Err(e) => return Err(QueryError::HttpError(format!("{}", e))),
        };

        match resp.json::<Vec<CourseInfo>>().await {
            Ok(json) => Ok(json),
            Err(e) => return Err(QueryError::ParseError(format!("{}", e))),
        }
    }

    pub async fn query(
        &self,
        semester: &str,
        course_no: &str,
        language: Language,
    ) -> Result<CourseDetails, QueryError> {
        let mut params = HashMap::new();
        params.insert("semester", semester);
        params.insert("course_no", course_no);
        params.insert("language", language.as_str());

        let resp = match self
            .http_client
            .get("https://querycourse.ntust.edu.tw/querycourse/api/coursedetials")
            .query(&params)
            .send()
            .await
        {
            Ok(resp) => resp,
            Err(e) => return Err(QueryError::HttpError(format!("{}", e))),
        };

        let json = match resp.json::<Vec<CourseDetails>>().await {
            Ok(json) => json,
            Err(e) => return Err(QueryError::ParseError(format!("{}", e))),
        };

        if let Some(course_details) = json.get(0) {
            Ok(course_details.clone())
        } else {
            Err(QueryError::ParseError(format!("No course found")))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn new() {
        let _client = Q::new();
    }

    #[tokio::test]
    async fn search() {
        let client = Q::new();

        let mut options = SearchOptions::new("1131", Language::Zh);

        options.course_no = "cs".to_string();

        let _details = client
            .search(&options)
            .await
            .expect("failed to search courses");

        println!("{:#?}", _details);
    }

    #[tokio::test]
    async fn query() {
        let client = Q::new();

        let _details = client
            .query("1122", "AT2005701", Language::Zh)
            .await
            .expect("Failed to query");

        println!("{:#?}", _details)
    }
}
