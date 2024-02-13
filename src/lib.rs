use serde_aux::prelude::*;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::collections::HashMap;
use std::net::IpAddr;
use std::time::Duration;

pub struct Q {
    client: reqwest::Client,
}

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

impl Q {
    pub fn new() -> Self {
        Q::build(None, None, None)
    }

    pub fn build(
        user_agent: Option<&str>,
        timeout: Option<Duration>,
        local_address: Option<IpAddr>,
    ) -> Self {
        let mut builder = reqwest::Client::builder()
            .user_agent(user_agent.unwrap_or(DEFAULT_USER_AGENT))
            .timeout(timeout.unwrap_or(DEFAULT_TIMEOUT));

        if local_address.is_some() {
            builder = builder.local_address(local_address.unwrap());
        }

        Q {
            client: builder.build().unwrap(),
        }
    }

    pub async fn query(
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
            .client
            .get("https://querycourse.ntust.edu.tw/querycourse/api/coursedetials")
            .query(&params)
            .send()
            .await?
            .json::<Vec<CourseDetails>>()
            .await?;

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
