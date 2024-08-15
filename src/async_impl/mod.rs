use std::collections::HashMap;

use url::Url;

use crate::{
    default_reqwest_builder, CourseDetails, CourseInfo, Language, QueryError, SearchOptions,
    DEFAULT_API_URL,
};

#[derive(Debug)]
pub struct ClientBuilder {
    reqwest_client: reqwest::Client,
    base_url: Url,
}

impl ClientBuilder {
    pub fn new() -> Self {
        ClientBuilder {
            reqwest_client: default_reqwest_builder().build().unwrap(),
            base_url: Url::parse(&DEFAULT_API_URL).unwrap(),
        }
    }

    pub fn reqwest_client(mut self, client: reqwest::Client) -> Self {
        self.reqwest_client = client;
        self
    }

    pub fn api_url(mut self, url: Url) -> Self {
        self.base_url = url;
        self
    }

    pub fn build(self) -> Q {
        Q {
            http_client: self.reqwest_client,
            base_url: self.base_url,
        }
    }
}

pub struct Q {
    http_client: reqwest::Client,
    base_url: Url,
}

impl Q {
    pub fn new() -> Self {
        ClientBuilder::new().build()
    }

    pub async fn search(
        &self,
        options: &SearchOptions,
        merge_courses: bool,
    ) -> Result<Vec<CourseInfo>, QueryError> {
        let url = self.base_url.join("courses").unwrap();

        let resp = match self.http_client.post(url).json(&options).send().await {
            Ok(resp) => resp,
            Err(e) => return Err(QueryError::HttpError(format!("{}", e))),
        };

        match resp.json::<Vec<CourseInfo>>().await {
            Ok(json) => Ok(if merge_courses {
                crate::merge_courses(json)
            } else {
                json
            }),
            Err(e) => return Err(QueryError::ParseError(format!("{}", e))),
        }
    }

    pub async fn query(
        &self,
        semester: &str,
        course_no: &str,
        language: Language,
    ) -> Result<CourseDetails, QueryError> {
        // is "coursedetials" not "coursedetails"
        // looks like an idiotic typo in the API
        let url = self.base_url.join("coursedetials").unwrap();

        let mut params = HashMap::new();
        params.insert("semester", semester);
        params.insert("course_no", course_no);
        params.insert("language", language.as_str());

        let resp = match self.http_client.get(url).query(&params).send().await {
            Ok(resp) => resp,
            Err(e) => return Err(QueryError::HttpError(format!("{}", e))),
        };

        let full = match resp.bytes().await {
            Ok(bytes) => bytes,
            Err(e) => return Err(QueryError::ParseError(format!("{}", e))),
        };

        // # debug
        // let text = match String::from_utf8(full.to_vec()) {
        //     Ok(bytes) => bytes,
        //     Err(e) => return Err(QueryError::ParseError(format!("{}", e))),
        // };
        // println!("{}", text);

        let json = match serde_json::from_slice::<Vec<CourseDetails>>(&full) {
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
    use futures::future::join_all;

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
            .search(&options, true)
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

    #[tokio::test]
    async fn query_all_cs() {
        let client = Q::new();

        let mut options = SearchOptions::new("1131", Language::Zh);

        options.course_no = "cs".to_string();

        let search_results = client
            .search(&options, true)
            .await
            .expect("failed to search courses");

        let futures = search_results.into_iter().map(|c| async move {
            let query_client = Q::new();
            let details = query_client
                .query(&c.semester, &c.course_no, Language::Zh)
                .await
                .expect("Failed to query");

            println!("{:#?}", details);
        });

        join_all(futures).await;
    }
}
