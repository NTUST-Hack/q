mod async_impl;
pub use self::async_impl::*;
pub mod blocking;

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, BoolFromInt};
use std::{fmt, str::FromStr, time::Duration};

pub const DEFAULT_USER_AGENT: &'static str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/121.0.0.0 Safari/537.36";
pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    Zh,
    En,
}

impl FromStr for Language {
    type Err = ();

    fn from_str(input: &str) -> Result<Language, Self::Err> {
        match input {
            "zh" => Ok(Language::Zh),
            "en" => Ok(Language::En),
            _ => Err(()),
        }
    }
}

impl Language {
    pub fn as_str(&self) -> &'static str {
        match self {
            Language::Zh => "zh",
            Language::En => "en",
        }
    }
}

#[serde_as]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CourseInfo {
    pub semester: String,
    pub course_no: String,
    pub course_name: String,
    pub course_teacher: String,
    pub dimension: String,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub credit_point: f32,
    pub require_option: String,
    pub all_year: String,
    pub choose_student: i32,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub restrict1: i32,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub restrict2: i32,
    pub three_student: i32,
    pub all_student: i32,
    #[serde(rename = "NTURestrict")]
    #[serde_as(deserialize_as = "serde_with::DefaultOnError")]
    pub nturestrict: i32,
    #[serde(rename = "NTNURestrict")]
    #[serde_as(deserialize_as = "serde_with::DefaultOnError")]
    pub ntnurestrict: i32,
    pub course_times: String,
    pub practical_times: String,
    #[serde(default)]
    pub class_room_no: Option<String>,
    #[serde(default)]
    pub three_node: Option<String>,
    #[serde(default)]
    pub node: Option<String>,
    pub contents: String,
    #[serde(rename = "NTU_People")]
    pub ntu_people: i32,
    #[serde(rename = "NTNU_People")]
    pub ntnu_people: i32,
    pub abroad_people: i32,
}

#[serde_as]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CourseDetails {
    pub semester: String,
    pub course_no: String,
    pub course_name: String,
    pub course_teacher: String,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub credit_point: f32,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub course_times: i32,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub practical_times: i32,
    pub require_option: String,
    pub all_year: String,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub choose_student: i32,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub three_student: i32,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub all_student: i32,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub restrict1: i32,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub restrict2: i32,
    #[serde(rename = "NTURestrict")]
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub nturestrict: i32,
    #[serde(rename = "NTNURestrict")]
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub ntnurestrict: i32,
    #[serde_as(as = "serde_with::NoneAsEmptyString")]
    pub class_room_no: Option<String>,
    #[serde_as(as = "serde_with::NoneAsEmptyString")]
    pub core_ability: Option<String>,
    #[serde(rename = "CourseURL")]
    #[serde_as(as = "serde_with::NoneAsEmptyString")]
    pub course_url: Option<String>,
    #[serde_as(as = "serde_with::NoneAsEmptyString")]
    pub course_object: Option<String>,
    #[serde_as(as = "serde_with::NoneAsEmptyString")]
    pub course_content: Option<String>,
    #[serde_as(as = "serde_with::NoneAsEmptyString")]
    pub course_textbook: Option<String>,
    #[serde_as(as = "serde_with::NoneAsEmptyString")]
    pub course_refbook: Option<String>,
    #[serde_as(as = "serde_with::NoneAsEmptyString")]
    pub course_note: Option<String>,
    #[serde_as(as = "serde_with::NoneAsEmptyString")]
    pub course_grading: Option<String>,
    #[serde_as(as = "serde_with::NoneAsEmptyString")]
    pub course_remark: Option<String>,
    #[serde(rename = "Instruction_1")]
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub instruction_1: Option<i32>,
    #[serde(rename = "Instruction_2")]
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub instruction_2: Option<i32>,
    #[serde(rename = "Instruction_3")]
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub instruction_3: Option<i32>,
    #[serde(rename = "Instruction_4")]
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub instruction_4: Option<i32>,
    #[serde(rename = "Instruction_other")]
    #[serde_as(as = "serde_with::NoneAsEmptyString")]
    pub instruction_other: Option<String>,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct SearchOptions {
    pub semester: String,
    pub course_no: String,
    pub course_name: String,
    pub course_teacher: String,
    pub dimension: String,
    pub course_notes: String,
    #[serde_as(as = "BoolFromInt")]
    pub foreign_language: bool,
    #[serde_as(as = "BoolFromInt")]
    pub only_general: bool,
    #[serde_as(as = "BoolFromInt")]
    #[serde(rename = "OnleyNTUST")] // looks like a low-level naming mistake in the API
    pub only_ntust: bool,
    #[serde_as(as = "BoolFromInt")]
    pub only_master: bool,
    #[serde_as(as = "BoolFromInt")]
    pub only_under_graduate: bool,
    #[serde_as(as = "BoolFromInt")]
    pub only_node: bool,
    pub language: Language,
}

impl SearchOptions {
    pub fn new(semester: &str, language: Language) -> Self {
        Self {
            semester: semester.to_string(),
            course_no: String::new(),
            course_name: String::new(),
            course_teacher: String::new(),
            dimension: String::new(),
            course_notes: String::new(),
            foreign_language: false,
            only_general: false,
            only_ntust: false,
            only_master: false,
            only_under_graduate: false,
            only_node: false,
            language,
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
