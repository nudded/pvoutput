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

pub struct Status<'a> {
    date: &'a str,
    time: &'a str,
    v1: Option<&'a str>, // Energy generation
    v2: Option<&'a str>, // Power generation
    v3: Option<&'a str>, // Energy consumption
    v4: Option<&'a str>, // Power consumption
    v5: Option<&'a str>, // Temperature
    v6: Option<&'a str>, // Voltage
    cumulative: bool // Cumulative
}

pub trait PvOutputRequest {
    fn request_url(&self) -> &str;
    fn parameters(&self) -> HashMap<&str, &str>;
}

impl<'a> PvOutput<'a> {

    pub fn new(api_key: &'a str, system_id: &'a str) -> PvOutput<'a> {
        PvOutput {
            api_key: api_key,
            system_id: system_id,
            client: reqwest::Client::new().unwrap()
        }
    }

    pub fn send_request(&self, request: &PvOutputRequest) -> Result<Response> {
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

impl<'a> Status<'a> {

    pub fn simple_for_v1(date: &'a str, time: &'a str, v1: &'a str) -> Status<'a> {
        Status {
            date: date,
            time: time,
            v1: Some(v1),
            v2: None,
            v3: None,
            v4: None,
            v5: None,
            v6: None,
            cumulative: true,
        }
    }
}

impl<'a> PvOutputRequest for Status<'a> {

    fn request_url(&self) -> &str {
        "http://pvoutput.org/service/r2/addstatus.jsp"
    }

    fn parameters(&self) -> HashMap<&str,&str> {
        let mut params = HashMap::new();
        params.insert("d", self.date);
        params.insert("t", self.time);
        self.v1.and_then(|v| params.insert("v1", v));
        self.v2.and_then(|v| params.insert("v2", v));
        self.v3.and_then(|v| params.insert("v3", v));
        self.v4.and_then(|v| params.insert("v4", v));
        self.v5.and_then(|v| params.insert("v5", v));
        self.v6.and_then(|v| params.insert("v6", v));
        params.insert("c1", if self.cumulative { "1" } else { "0" });
        return params;
    }
}

