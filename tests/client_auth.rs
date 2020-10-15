use boundary_api::BoundaryClient;
use url::Url;

#[test]
fn client_auth_host_default() {
    let client = BoundaryClient::default();
    assert_eq!(client.host, Url::parse("http://127.0.0.1:9200").unwrap());
}

#[test]
fn client_auth_auth_method_id_default() {
    let client = BoundaryClient::default();
    assert_eq!(&client.auth_method_id, "ampw_1234567890");
}

#[test]
fn client_auth_login_name_default() {
    let client = BoundaryClient::default();
    assert_eq!(&client.login_name, "admin");
}

#[test]
fn client_auth_password_default() {
    let client = BoundaryClient::default();
    assert_eq!(&client.password, "password");
}

#[test]
fn client_auth_host_user_supplied() {
    let client = BoundaryClient::new("https://fake.url.fyi").unwrap();
    assert_eq!(client.host, Url::parse("https://fake.url.fyi").unwrap());
}

#[test]
fn client_auth_auth_method_id_user_supplied() {
    let client = BoundaryClient::default().auth_method_id("auth_qwertyuiop");
    assert_eq!(&client.auth_method_id, "auth_qwertyuiop");
}

#[test]
fn client_auth_login_name_user_supplied() {
    let client = BoundaryClient::default().login_name("tron");
    assert_eq!(&client.login_name, "tron");
}

#[test]
fn client_auth_password_user_supplied() {
    let client = BoundaryClient::default().password("SecurePassword9000");
    assert_eq!(&client.password, "SecurePassword9000");
}