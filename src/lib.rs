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
    v1: &'a str, // Energy generation
    v2: &'a str, // Power generation
    v3: &'a str, // Energy consumption
    v4: &'a str, // Power consumption
    v5: &'a str, // Temperature
    v6: &'a str, // Voltage
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

impl<'a> PvOutputRequest for Status<'a> {

    fn request_url(&self) -> &str {
        "http://pvoutput.org/service/r2/addstatus.jsp"
    }

    fn parameters(&self) -> HashMap<&str,&str> {
        let mut params = HashMap::new();
        params.insert("d", self.date);
        params.insert("t", self.time);
        params.insert("v1", self.v1);
        params.insert("v2", self.v2);
        params.insert("v3", self.v3);
        params.insert("v4", self.v4);
        params.insert("v5", self.v5);
        params.insert("v6", self.v6);
        params.insert("c1", if self.cumulative { "1" } else { "0" });
        return params;
    }
}

