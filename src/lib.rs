pub mod blocking;

use serde_aux::prelude::*;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::collections::HashMap;
use std::fmt;
use std::time::Duration;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CourseDetails {
    #[serde(rename = "Semester")]
    pub semester: String,
    #[serde(rename = "CourseNo")]
    pub course_no: String,
    #[serde(rename = "CourseName")]
    pub course_name: String,
    #[serde(rename = "CourseTeacher")]
    pub course_teacher: String,
    #[serde(
        rename = "CreditPoint",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub credit_point: f32,
    #[serde(rename = "CourseTimes")]
    pub course_times: String,
    #[serde(rename = "PracticalTimes")]
    pub practical_times: String,
    #[serde(rename = "RequireOption")]
    pub require_option: String,
    #[serde(rename = "AllYear")]
    pub all_year: String,
    #[serde(
        rename = "ChooseStudent",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub choose_student: i32,
    #[serde(
        rename = "ThreeStudent",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub three_student: i32,
    #[serde(
        rename = "AllStudent",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub all_student: i32,
    #[serde(
        rename = "Restrict1",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub restrict1: i32,
    #[serde(
        rename = "Restrict2",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub restrict2: i32,
    #[serde(
        rename = "NTURestrict",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub nturestrict: i32,
    #[serde(
        rename = "NTNURestrict",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub ntnurestrict: i32,
    #[serde(rename = "ClassRoomNo")]
    pub class_room_no: String,
    #[serde(rename = "CoreAbility")]
    pub core_ability: String,
    #[serde(rename = "CourseURL")]
    pub course_url: String,
    #[serde(rename = "CourseObject")]
    pub course_object: String,
    #[serde(rename = "CourseContent")]
    pub course_content: String,
}

const DEFAULT_USER_AGENT: &'static str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/121.0.0.0 Safari/537.36";
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(10);

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

    pub async fn query(
        &self,
        semester: &str,
        course_no: &str,
        language: &str,
    ) -> Result<CourseDetails, QueryError> {
        let mut params = HashMap::new();
        params.insert("semester", semester);
        params.insert("course_no", course_no);
        params.insert("language", language);

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

#[derive(Debug, Clone)]
pub enum QueryError {
    HttpError(String),
    ParseError(String),
}

impl std::error::Error for QueryError {}

impl fmt::Display for QueryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            QueryError::HttpError(msg) => write!(f, "HTTP Error: {}", msg),
            QueryError::ParseError(msg) => write!(f, "Parse Error: {}", msg),
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
    async fn query() {
        let client = Q::new();

        let _details = client
            .query("1122", "AT2005701", "zh")
            .await
            .expect("Failed to query");

        println!("{:#?}", _details)
    }
}
