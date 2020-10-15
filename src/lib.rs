use reqwest::Client;
use std::collections::HashMap;
use thiserror::Error;
use url::Url;

/// BoundaryClient is used to connect to the Boundary server
/// The Default trait is implemented to cover the Boundary server in dev mode
/// Otherwise you can configure the client prior to authentication using builder pattern methods
#[derive(Debug, Clone)]
pub struct BoundaryClient {
    pub host: Url,
    pub auth_method_id: String,
    pub login_name: String,
    pub password: String,
    client: Client,
}

impl Default for BoundaryClient {
    /// These default values correspond to the defaults returned by running Boundary in dev mode
    fn default() -> Self {
        BoundaryClient::new("http://127.0.0.1:9200")
            .expect("Provided Boundary host url failed to parse")
            .auth_method_id("ampw_1234567890")
            .login_name("admin")
            .password("password")
    }
}

impl BoundaryClient {
    /// `new()` takes in any String or &str that returns Ok() from `Url::parse()`
    /// Need to configure other struct values before running `authenticate()` against Boundary server
    pub fn new<S>(host: S) -> Result<Self, BoundaryError>
    where
        S: AsRef<str>,
    {
        Ok(BoundaryClient {
            host: Url::parse(host.as_ref())?,
            auth_method_id: String::new(),
            login_name: String::new(),
            password: String::new(),
            client: Client::new(),
        })
    }

    /// `auth_method_id` configures the value passed to the Boundary auth method service
    pub fn auth_method_id<'a, S>(&'a mut self, id: S) -> Self
    where
        S: Into<String>,
    {
        self.auth_method_id = id.into();
        self.to_owned()
    }

    /// `login_name` configures the login name used for connecting to the Boundary auth method service
    pub fn login_name<'a, S>(&'a mut self, login_name: S) -> Self
    where
        S: Into<String>,
    {
        self.login_name = login_name.into();
        self.to_owned()
    }

    /// `password` configures the password used for connecting to the Boundary auth method service
    pub fn password<'a, S>(&'a mut self, password: S) -> Self
    where
        S: Into<String>,
    {
        self.password = password.into();
        self.to_owned()
    }

    /// `authenticate` will connect to the Boundary server using the current struct values
    pub async fn authenticate<'a>(&'a mut self) -> Result<reqwest::Response, BoundaryError> {
        let mut creds = HashMap::new();
        creds.insert("login_name", self.login_name.as_str());
        creds.insert("password", self.password.as_str());

        let mut auth_payload = HashMap::new();
        auth_payload.insert("credentials", creds);

        let auth_endpoint = format!(
            "/v1/auth-methods/{auth_method_id}:authenticate",
            auth_method_id = self.auth_method_id
        );
        let auth_method_service = self.host.join(&auth_endpoint)?;

        let req = self
            .client
            .post(auth_method_service)
            .json(&auth_payload)
            .build()?;

        let res = self.client.execute(req).await?;

        Ok(res)
    }
}

/// Internal error type managed by `thiserror` crate
#[derive(Error, Debug)]
pub enum BoundaryError {
    #[error(transparent)]
    InvalidHost(#[from] url::ParseError),
    #[error(transparent)]
    RequestError(#[from] reqwest::Error),
}
