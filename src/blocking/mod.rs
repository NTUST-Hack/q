use std::collections::HashMap;
use std::time::Duration;

use crate::CourseDetails;
use crate::DEFAULT_TIMEOUT;
use crate::DEFAULT_USER_AGENT;

pub struct ClientBuilder<'a> {
    reqwest_builder: reqwest::blocking::ClientBuilder,

    user_agent: &'a str,
    timeout: Duration,
}

impl<'a> ClientBuilder<'a> {
    pub fn new() -> Self {
        ClientBuilder {
            reqwest_builder: reqwest::blocking::ClientBuilder::new(),

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
    http_client: reqwest::blocking::Client,
}

impl Q {
    pub fn new() -> Self {
        ClientBuilder::new().build().unwrap()
    }

    pub fn query(
        &self,
        semester: &str,
        course_no: &str,
        language: &str,
    ) -> Result<CourseDetails, Box<dyn std::error::Error>> {
        let mut params = HashMap::new();
        params.insert("semester", semester);
        params.insert("course_no", course_no);
        params.insert("language", language);

        let resp = self
            .http_client
            .get("https://querycourse.ntust.edu.tw/querycourse/api/coursedetials")
            .query(&params)
            .send()?
            .json::<Vec<CourseDetails>>()?;

        if let Some(course_details) = resp.get(0) {
            Ok(course_details.clone())
        } else {
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Failed to parse response data",
            )))
        }
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
    fn query() {
        let client = Q::new();

        let _details = client
            .query("1122", "AT2005701", "zh")
            .expect("Failed to query");

        println!("{:#?}", _details)
    }
}
