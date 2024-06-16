mod async_impl;
pub use self::async_impl::*;
pub mod blocking;

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, BoolFromInt};
use std::{collections::HashMap, fmt, str::FromStr, time::Duration};

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
    #[serde_as(deserialize_as = "serde_with::DefaultOnError")]
    pub nturestrict: i32,
    #[serde(rename = "NTNURestrict")]
    #[serde_as(deserialize_as = "serde_with::DefaultOnError")]
    pub ntnurestrict: Option<i32>,
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

fn merge_courses(courses: Vec<CourseInfo>) -> Vec<CourseInfo> {
    let mut course_map: HashMap<String, CourseInfo> = HashMap::new();

    for course in courses.into_iter() {
        course_map
            .entry(course.course_no.clone())
            .and_modify(|existing_course| {
                if let Some(new_nodes) = &course.node {
                    if let Some(existing_nodes) = &mut existing_course.node {
                        existing_nodes.push(',');
                        existing_nodes.push_str(new_nodes);
                    } else {
                        existing_course.node = Some(new_nodes.clone());
                    }
                }
            })
            .or_insert(course);
    }

    // ensure nodes are unique and in order
    for course in course_map.values_mut() {
        if let Some(nodes) = &mut course.node {
            let mut nodes_vec: Vec<&str> = nodes.split(',').collect();
            nodes_vec.sort();
            nodes_vec.dedup();
            *nodes = nodes_vec.join(",");
        }
    }

    course_map.into_values().collect()
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

    #[test]
    fn merge_courses() {
        let courses: Vec<CourseInfo> = vec![
            CourseInfo {
                semester: String::from("1131"),
                course_no: String::from("CS1003302"),
                course_name: String::from("計算機程式設計"),
                course_teacher: String::from("金台齡"),
                dimension: String::from(""),
                credit_point: 3.0,
                require_option: String::from("R"),
                all_year: String::from("H"),
                choose_student: 0,
                restrict1: 9999,
                restrict2: 53,
                three_student: 0,
                all_student: 0,
                nturestrict: 0,
                ntnurestrict: 0,
                course_times: String::from("3"),
                practical_times: String::from("0"),
                class_room_no: None,
                three_node: None,
                node: Some(String::from("R1")),
                contents: String::from("學號雙數／EMI課程／英語授課"),
                ntu_people: 0,
                ntnu_people: 0,
                abroad_people: 0,
            },
            CourseInfo {
                semester: String::from("1131"),
                course_no: String::from("CS1003302"),
                course_name: String::from("計算機程式設計"),
                course_teacher: String::from("金台齡"),
                dimension: String::from(""),
                credit_point: 3.0,
                require_option: String::from("R"),
                all_year: String::from("H"),
                choose_student: 0,
                restrict1: 9999,
                restrict2: 53,
                three_student: 0,
                all_student: 0,
                nturestrict: 0,
                ntnurestrict: 0,
                course_times: String::from("3"),
                practical_times: String::from("0"),
                class_room_no: None,
                three_node: None,
                node: Some(String::from("T1,T2")),
                contents: String::from("學號雙數／EMI課程／英語授課"),
                ntu_people: 0,
                ntnu_people: 0,
                abroad_people: 0,
            },
        ];

        let merged_courses = crate::merge_courses(courses);

        println!("{:#?}", merged_courses);
    }
}
