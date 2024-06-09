mod async_impl;
pub use self::async_impl::*;

pub mod blocking;

use std::{fmt, time::Duration};

use serde::{Deserialize, Serialize};
use serde_aux::prelude::*;

pub const DEFAULT_USER_AGENT: &'static str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/121.0.0.0 Safari/537.36";
pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(10);

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
