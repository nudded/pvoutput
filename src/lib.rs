#[macro_use] extern crate hyper;
extern crate reqwest;

use reqwest::{Result, Response};

use hyper::header::Headers;
header! { (XPvoutputApikey, "X-Pvoutput-Apikey") => [String] }
header! { (XPvoutputSystemId, "X-Pvoutput-SystemId") => [String] }

use std::collections::HashMap;

pub struct PvOutput<'a> {
    api_key: &'a str,
    system_id: &'a str,
    client: reqwest::Client,
}

#[derive(Debug)]
pub struct Status {
    date: String,
    time: String,
    v1: Option<String>, // Energy generation
    v2: Option<String>, // Power generation
    v3: Option<String>, // Energy consumption
    v4: Option<String>, // Power consumption
    v5: Option<String>, // Temperature
    v6: Option<String>, // Voltage
    cumulative: bool // Cumulative
}

pub trait PvOutputRequest {
    fn request_url(&self) -> &str;
    fn parameters(self) -> HashMap<&'static str, String>;
}

impl<'a> PvOutput<'a> {

    pub fn new(api_key: &'a str, system_id: &'a str) -> PvOutput<'a> {
        PvOutput {
            api_key: api_key,
            system_id: system_id,
            client: reqwest::Client::new().unwrap()
        }
    }

    pub fn send_request<T: PvOutputRequest>(&self, request: T) -> Result<Response> {
        self.client.post(request.request_url()).
            headers(self.headers()).
            json(&request.parameters()).
            send()
    }

    fn headers(&self) -> Headers {
        let mut headers = Headers::new();
        headers.set(XPvoutputApikey(self.api_key.to_owned()));
        headers.set(XPvoutputSystemId(self.system_id.to_owned()));
        return headers;
    }
}

impl Status {

    pub fn simple_for_v2(date: String, time: String, v2: String) -> Status {
        Status {
            date: date,
            time: time,
            v1: None,
            v2: Some(v2),
            v3: None,
            v4: None,
            v5: None,
            v6: None,
            cumulative: true,
        }
    }
}

impl PvOutputRequest for Status {

    fn request_url(&self) -> &str {
        "http://pvoutput.org/service/r2/addstatus.jsp"
    }

    fn parameters(self) -> HashMap<&'static str,String> {
        let mut params = HashMap::new();
        params.insert("d", self.date);
        params.insert("t", self.time);
        self.v1.and_then(|v| params.insert("v1", v));
        self.v2.and_then(|v| params.insert("v2", v));
        self.v3.and_then(|v| params.insert("v3", v));
        self.v4.and_then(|v| params.insert("v4", v));
        self.v5.and_then(|v| params.insert("v5", v));
        self.v6.and_then(|v| params.insert("v6", v));
        params.insert("c1", if self.cumulative { "1".to_string() } else { "0".to_string() });
        return params;
    }
}

