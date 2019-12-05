// -----------------------------------------------
// ███╗   ██╗ ██████╗ ████████╗██╗ ██████╗███████╗
// ████╗  ██║██╔═══██╗╚══██╔══╝██║██╔════╝██╔════╝
// ██╔██╗ ██║██║   ██║   ██║   ██║██║     █████╗
// ██║╚██╗██║██║   ██║   ██║   ██║██║     ██╔══╝
// ██║ ╚████║╚██████╔╝   ██║   ██║╚██████╗███████╗
// ╚═╝  ╚═══╝ ╚═════╝    ╚═╝   ╚═╝ ╚═════╝╚══════╝
// -----------------------------------------------
//
// This file is generated,
// Please do not edit it manually.
// Run the following in the root of the repo:
//
// cargo run -p api_generator
//
// -----------------------------------------------
use crate::{
    client::Elasticsearch,
    enums::*,
    error::ElasticsearchError,
    http_method::HttpMethod,
    request::{Body, JsonBody, NdBody},
    response::ElasticsearchResponse,
};
use reqwest::{header::HeaderMap, Error, Request, Response, StatusCode};
use serde::{de::DeserializeOwned, Serialize};
use serde_with;
use std::borrow::Cow;
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Monitoring Bulk API"]
pub enum MonitoringBulkUrlParts<'a> {
    None,
    Type(&'a str),
}
impl<'a> MonitoringBulkUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            MonitoringBulkUrlParts::None => "/_monitoring/bulk".into(),
            MonitoringBulkUrlParts::Type(ref ty) => {
                let mut p = String::with_capacity(18usize + ty.len());
                p.push_str("/_monitoring/");
                p.push_str(ty.as_ref());
                p.push_str("/bulk");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Monitoring Bulk API"]
pub struct MonitoringBulk<'a, B> {
    client: Elasticsearch,
    parts: MonitoringBulkUrlParts<'a>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    interval: Option<&'a str>,
    pretty: Option<bool>,
    source: Option<&'a str>,
    system_api_version: Option<&'a str>,
    system_id: Option<&'a str>,
}
impl<'a, B> MonitoringBulk<'a, B>
where
    B: Body,
{
    pub fn new(client: Elasticsearch, parts: MonitoringBulkUrlParts<'a>) -> Self {
        MonitoringBulk {
            client,
            parts,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            interval: None,
            pretty: None,
            source: None,
            system_api_version: None,
            system_id: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: Vec<T>) -> MonitoringBulk<'a, NdBody<T>>
    where
        T: Body,
    {
        MonitoringBulk {
            client: self.client,
            parts: self.parts,
            body: Some(NdBody(body)),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            human: self.human,
            interval: self.interval,
            pretty: self.pretty,
            source: self.source,
            system_api_version: self.system_api_version,
            system_id: self.system_id,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Collection interval (e.g., '10s' or '10000ms') of the payload"]
    pub fn interval(mut self, interval: &'a str) -> Self {
        self.interval = Some(interval);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "API Version of the monitored system"]
    pub fn system_api_version(mut self, system_api_version: &'a str) -> Self {
        self.system_api_version = Some(system_api_version);
        self
    }
    #[doc = "Identifier of the monitored system"]
    pub fn system_id(mut self, system_id: &'a str) -> Self {
        self.system_id = Some(system_id);
        self
    }
    #[doc = "Creates an asynchronous request to the Monitoring Bulk API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "interval")]
                interval: Option<&'a str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
                #[serde(rename = "system_api_version")]
                system_api_version: Option<&'a str>,
                #[serde(rename = "system_id")]
                system_id: Option<&'a str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                interval: self.interval,
                pretty: self.pretty,
                source: self.source,
                system_api_version: self.system_api_version,
                system_id: self.system_id,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[doc = "Monitoring APIs"]
pub struct Monitoring {
    client: Elasticsearch,
}
impl Monitoring {
    pub fn new(client: Elasticsearch) -> Self {
        Monitoring { client }
    }
    pub fn bulk<'a>(&self, parts: MonitoringBulkUrlParts<'a>) -> MonitoringBulk<'a, ()> {
        MonitoringBulk::new(self.client.clone(), parts)
    }
}
impl Elasticsearch {
    #[doc = "Monitoring APIs"]
    pub fn monitoring(&self) -> Monitoring {
        Monitoring::new(self.clone())
    }
}
