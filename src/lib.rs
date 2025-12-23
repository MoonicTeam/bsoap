/**
 * YES, I know this sucks.
 * -Cosmic
 */
use serde::Serialize;
use yaserde_derive::{YaDeserialize, YaSerialize};

pub const BSOAP_VER: &str = env!("CARGO_PKG_VERSION");

// ===================================================================================
// Top-level
// ===================================================================================

#[derive(YaSerialize, YaDeserialize)]
#[yaserde(rename = "Header")]
pub struct SoapHeader;

#[derive(YaSerialize, YaDeserialize)]
#[yaserde(prefix = "SOAP-ENV", rename = "Envelope")]
pub struct Envelope {
    #[yaserde(attribute = true, rename = "SOAP-ENV", prefix = "xmlns")]
    pub soapenv: String,

    #[yaserde(attribute = true, rename = "ns1", prefix = "xmlns")]
    pub ns1: String,

    #[yaserde(rename = "Header", prefix = "SOAP-ENV")]
    pub header: SoapHeader,

    #[yaserde(rename = "Body", prefix = "SOAP-ENV")]
    pub body: SoapBodyContent,
}

#[derive(YaDeserialize)]
#[yaserde(
    rename = "Envelope",
    prefix = "SOAP-ENV",
    namespaces = {
        "SOAP-ENV" = "http://schemas.xmlsoap.org/soap/envelope/",
        "SOAP-ENC" = "http://schemas.xmlsoap.org/soap/encoding/",
        "xsi" = "http://www.w3.org/2001/XMLSchema-instance",
        "xsd" = "http://www.w3.org/2001/XMLSchema",
        "ns2" = "http://roblox.com/RCCServiceSoap",
        "ns1" = "http://roblox.com/",
        "ns3" = "http://roblox.com/RCCServiceSoap12"
    }
)]
pub struct EnvelopeResponse {
    #[yaserde(rename = "Body", prefix = "SOAP-ENV")]
    pub body: SoapResponseBody,
}

#[derive(Debug, YaDeserialize)]
#[yaserde(
    rename = "Body",
    prefix = "SOAP-ENV",
    namespaces = {
        "SOAP-ENV" = "http://schemas.xmlsoap.org/soap/envelope/",
        "SOAP-ENC" = "http://schemas.xmlsoap.org/soap/encoding/",
        "xsi" = "http://www.w3.org/2001/XMLSchema-instance",
        "xsd" = "http://www.w3.org/2001/XMLSchema",
        "ns2" = "http://roblox.com/RCCServiceSoap",
        "ns1" = "http://roblox.com/",
        "ns3" = "http://roblox.com/RCCServiceSoap12"
    }
)]
pub enum SoapResponseBody {
    #[yaserde(rename = "HelloWorldResponse", prefix = "ns1")]
    HelloWorldResponse(HelloWorldResponse),
    #[yaserde(rename = "GetVersionResponse", prefix = "ns1")]
    GetVersionResponse(GetVersionResponse),
    #[yaserde(rename = "OpenJobResponse", prefix = "ns1")]
    OpenJobResponse(OpenJobResponse),
    #[yaserde(rename = "OpenJobExResponse", prefix = "ns1")]
    OpenJobExResponse(OpenJobExResponse),
    #[yaserde(rename = "BatchJobResponse", prefix = "ns1")]
    BatchJobResponse(BatchJobResponse),
    #[yaserde(rename = "BatchJobExResponse", prefix = "ns1")]
    BatchJobExResponse(BatchJobExResponse),
    #[yaserde(rename = "CloseJobResponse", prefix = "ns1")]
    CloseJobResponse(CloseJobResponse),
    #[yaserde(rename = "ExecuteResponse", prefix = "ns1")]
    ExecuteResponse(ExecuteResponse),
}

impl Default for SoapResponseBody {
    fn default() -> Self {
        SoapResponseBody::HelloWorldResponse(HelloWorldResponse { result: None })
    }
}

// ===================================================================================
// SOAP body enum
// ===================================================================================
#[derive(YaSerialize, YaDeserialize)]
pub enum SoapBodyContent {
    #[yaserde(rename = "HelloWorld", prefix = "ns1")]
    HelloWorld(HelloWorld),
    #[yaserde(rename = "GetVersion", prefix = "ns1")]
    GetVersion(GetVersion),
    #[yaserde(rename = "OpenJob", prefix = "ns1")]
    OpenJob(OpenJob),
    #[yaserde(rename = "OpenJobEx", prefix = "ns1")]
    OpenJobEx(OpenJobEx),
    #[yaserde(rename = "BatchJob", prefix = "ns1")]
    BatchJob(BatchJob),
    #[yaserde(rename = "BatchJobEx", prefix = "ns1")]
    BatchJobEx(BatchJobEx),
    #[yaserde(rename = "CloseJob", prefix = "ns1")]
    CloseJob(CloseJob),
    #[yaserde(rename = "Execute", prefix = "ns1")]
    ExecuteScript(ExecuteScript),
    #[yaserde(rename = "RenewLease", prefix = "ns1")]
    RenewLease(RenewLease),
}

impl Default for SoapBodyContent {
    fn default() -> Self {
        SoapBodyContent::HelloWorld(HelloWorld { data: None })
    }
}

// ===================================================================================
// SOAP Actions
// ===================================================================================
macro_rules! define_simple_struct {
    ($name:ident, $rename:literal) => {
        #[derive(YaSerialize, YaDeserialize)]
        #[yaserde(rename = $rename, prefix = "ns1")]
        pub struct $name {
            pub data: Option<String>,
        }
    };
}

define_simple_struct!(HelloWorld, "HelloWorld");
define_simple_struct!(GetVersion, "GetVersion");

// ===================================================================================
// Construction for SOAP Actions: Jobs (OpenJob, BatchJob, CloseJob, etc)
// ===================================================================================
macro_rules! define_job_struct {
    ($name:ident, $rename:literal) => {
        #[derive(YaSerialize, YaDeserialize)]
        #[yaserde(rename = $rename, prefix = "ns1")]
        pub struct $name {
            #[yaserde(rename = "job", prefix = "ns1")]
            pub job: Job,
            #[yaserde(rename = "script", prefix = "ns1")]
            pub script: Script,
        }
    };
}

define_job_struct!(OpenJob, "OpenJob");
define_job_struct!(OpenJobEx, "OpenJobEx");
define_job_struct!(BatchJob, "BatchJob");
define_job_struct!(BatchJobEx, "BatchJobEx");

#[derive(YaSerialize, YaDeserialize)]
#[yaserde(rename = "CloseJob", prefix = "ns1")]
pub struct CloseJob {
    #[yaserde(rename = "jobID", prefix = "ns1")]
    pub job_id: String,
}

#[derive(YaSerialize, YaDeserialize)]
#[yaserde(rename = "Execute", prefix = "ns1")]
pub struct ExecuteScript {
    #[yaserde(rename = "jobID", prefix = "ns1")]
    pub job_id: String,
    #[yaserde(rename = "script", prefix = "ns1")]
    pub script: Script,
}

#[derive(YaSerialize, YaDeserialize)]
#[yaserde(rename = "RenewLease", prefix = "ns1")]
pub struct RenewLease {
    #[yaserde(rename = "jobID", prefix = "ns1")]
    pub job_id: String,

    #[yaserde(rename = "expirationInSeconds", prefix = "ns1")]
    pub expiration_in_seconds: u32,
}

// ===================================================================================
// Deserialization for SOAP Actions: Jobs (OpenJob, BatchJob, CloseJob, etc)
// ===================================================================================
macro_rules! define_response_struct {
    ($name:ident, $rename:literal, $result_rename:literal, $result_type:ty) => {
        #[derive(Debug, YaDeserialize)]
        #[yaserde(rename = $rename, prefix = "ns1")]
        pub struct $name {
            #[yaserde(rename = $result_rename, prefix = "ns1")]
            pub result: $result_type,
        }
    };
    ($name:ident, $rename:literal, $result_rename:literal, $result_type:ty, $namespaces:tt) => {
        #[derive(Debug, YaDeserialize)]
        #[yaserde(rename = $rename, prefix = "ns1", namespaces = $namespaces)]
        pub struct $name {
            #[yaserde(rename = $result_rename, prefix = "ns1")]
            pub result: $result_type,
        }
    };
}

define_response_struct!(HelloWorldResponse, "HelloWorldResponse", "HelloWorldResult", Option<String>);
define_response_struct!(GetVersionResponse, "GetVersionResponse", "GetVersionResult", Option<String>);
define_response_struct!(OpenJobResponse, "OpenJobResponse", "OpenJobResult", Arguments);
define_response_struct!(OpenJobExResponse, "OpenJobExResponse", "OpenJobExResult", Arguments);
define_response_struct!(BatchJobResponse, "BatchJobResponse", "BatchJobResult", Arguments, { "ns1" = "http://roblox.com/" });
define_response_struct!(BatchJobExResponse, "BatchJobExResponse", "BatchJobExResult", Option<Arguments>, { "ns1" = "http://roblox.com/" });

#[derive(Debug, YaDeserialize)]
#[yaserde(rename = "CloseJobResponse", prefix = "ns1")]
pub struct CloseJobResponse {
    #[yaserde(rename = "CloseJobResult", prefix = "ns1")]
    pub result: Option<String>,
}

#[derive(Debug, YaDeserialize)]
#[yaserde(rename = "ExecuteResponse", prefix = "ns1")]
pub struct ExecuteResponse {
    #[yaserde(rename = "ExecuteResult", prefix = "ns1")]
    pub result: JobResult,
}

// reusable struct for job-like results containing LuaValue
#[derive(Debug, YaDeserialize, Clone, PartialEq)]
pub struct JobResult {
    #[yaserde(rename = "LuaValue", prefix = "ns1")]
    pub lua_values: Vec<LuaValue>,
}

// ===================================================================================
// Structs for opening Jobs
// ===================================================================================

#[derive(YaSerialize, Serialize, YaDeserialize)]
pub struct Job {
    #[yaserde(rename = "id", prefix = "ns1")]
    pub id: String,

    #[yaserde(rename = "expirationInSeconds", prefix = "ns1")]
    pub expiration_in_seconds: u32,

    #[yaserde(rename = "cores", prefix = "ns1")]
    pub cores: u8,
}

#[derive(YaSerialize, YaDeserialize)]
pub struct Script {
    #[yaserde(rename = "name", prefix = "ns1")]
    pub name: String,
    #[yaserde(rename = "script", prefix = "ns1")]
    #[yaserde(cdata = true)]
    pub script: String,

    #[yaserde(rename = "arguments", prefix = "ns1")]
    pub arguments: Arguments,
}

#[derive(YaSerialize, YaDeserialize, Debug, Clone, PartialEq)]
#[yaserde(namespaces = {
    "ns1" = "http://roblox.com/"
})]
pub struct Arguments {
    #[yaserde(namespaces = {
        "ns1" = "http://roblox.com/"
    }, rename = "LuaValue", prefix = "ns1")]
    pub lua_values: Vec<LuaValue>,
}

#[derive(YaSerialize, YaDeserialize, Clone, Debug, PartialEq)]
#[yaserde(rename = "LuaValue", prefix = "ns1",
    namespaces = {
        "ns1" = "http://roblox.com/"
    }
)]
pub struct LuaValue {
    #[yaserde(rename = "type", prefix = "ns1")]
    pub valuetype: String,

    #[yaserde(rename = "value", prefix = "ns1")]
    pub value: Option<String>,

    #[yaserde(rename = "table", prefix = "ns1")]
    pub table: Option<String>,
}

// ===================================================================================
// SOAP client
// ===================================================================================

#[derive(Debug)]
pub enum SuccessType {
    WithResult(String),
    NoResult,
}

#[derive(Debug)]
pub enum SoapResult {
    Success(SuccessType),
    InternalServerError,
    ConnectionError,
    BadRequest,
}

pub mod client {
    use super::*;
    use reqwest::Client;

    async fn send_request(port: i32, body: SoapBodyContent) -> SoapResult {
        let envelope = Envelope {
            soapenv: "http://schemas.xmlsoap.org/soap/envelope/".to_string(),
            ns1: "http://roblox.com/".to_string(),
            header: SoapHeader,
            body,
        };

        let xml = match yaserde::ser::to_string(&envelope) {
            Ok(xml) => xml,
            Err(_) => {
                return SoapResult::ConnectionError;
            }
        };

        let url = format!("http://localhost:{}/", port);
        let response = match Client::new()
            .post(&url)
            .header("Content-Type", "text/xml")
            .body(xml)
            .send()
            .await
        {
            Ok(resp) => resp,
            Err(_) => {
                return SoapResult::ConnectionError;
            }
        };

        if response.status().is_success() {
            match response.text().await {
                Ok(text) => {
                    if text.is_empty() {
                        SoapResult::Success(SuccessType::NoResult)
                    } else {
                        SoapResult::Success(SuccessType::WithResult(text))
                    }
                }
                Err(_) => SoapResult::ConnectionError,
            }
        } else if response.status().as_u16() == 500 {
            SoapResult::InternalServerError
        } else if response.status().as_u16() == 400 {
            SoapResult::BadRequest
        } else {
            SoapResult::ConnectionError
        }
    }

    macro_rules! define_simple_client_func {
        ($func_name:ident, $body_variant:ident, $struct_name:ident) => {
            #[allow(non_snake_case, dead_code)]
            pub async fn $func_name(port: i32) -> SoapResult {
                send_request(
                    port,
                    SoapBodyContent::$body_variant($struct_name {
                        data: Some("".to_string()),
                    }),
                )
                .await
            }
        };
    }

    define_simple_client_func!(HelloWorld, HelloWorld, HelloWorld);
    define_simple_client_func!(GetVersion, GetVersion, GetVersion);

    macro_rules! define_job_client_func {
        ($func_name:ident, $body_variant:ident, $struct_name:ident) => {
            #[allow(non_snake_case, dead_code)]
            pub async fn $func_name(
                port: i32,
                job_id: String,
                script_content: String,
                script_args: Option<Vec<LuaValue>>,
                expiration_in_seconds: u32,
            ) -> SoapResult {
                let body = SoapBodyContent::$body_variant($struct_name {
                    job: Job {
                        id: job_id,
                        expiration_in_seconds,
                        cores: 1,
                    },
                    script: Script {
                        name: "Script".to_string(),
                        script: script_content,
                        arguments: Arguments {
                            lua_values: script_args.unwrap_or_default(),
                        },
                    },
                });
                send_request(port, body).await
            }
        };
    }

    define_job_client_func!(OpenJobEx, OpenJobEx, OpenJobEx);
    define_job_client_func!(BatchJobEx, BatchJobEx, BatchJobEx);
    define_job_client_func!(OpenJob, OpenJob, OpenJob);

    #[allow(non_snake_case, dead_code)]
    pub async fn CloseJob(port: i32, job_id: String) -> SoapResult {
        let body = SoapBodyContent::CloseJob(CloseJob { job_id });
        send_request(port, body).await
    }

    define_job_client_func!(BatchJob, BatchJob, BatchJob);

    #[allow(non_snake_case, dead_code)]
    pub async fn ExecuteScript(
        port: i32,
        job_id: String,
        script_content: String,
        script_args: Option<Vec<LuaValue>>,
    ) -> SoapResult {
        let body = SoapBodyContent::ExecuteScript(ExecuteScript {
            job_id,
            script: Script {
                name: "Script".to_string(),
                script: script_content,
                arguments: Arguments {
                    lua_values: script_args.unwrap_or_default(),
                },
            },
        });
        send_request(port, body).await
    }

    #[allow(non_snake_case, dead_code)]
    pub async fn RenewLease(port: i32, job_id: String, expiration_in_seconds: u32) -> SoapResult {
        let body = SoapBodyContent::RenewLease(RenewLease {
            job_id,
            expiration_in_seconds,
        });
        send_request(port, body).await
    }
}