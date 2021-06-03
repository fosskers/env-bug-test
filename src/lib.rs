use seed::{prelude::*, *};
use tame_oauth::gcp::{ServiceAccountAccess, ServiceAccountInfo, TokenOrRequest};

pub const GMAIL_SCOPES: &[&str] = &["https://www.googleapis.com/auth/gmail.readonly"];

/// These are NOT real credentials, but they will at least parse into a `ServiceAccountInfo`.
pub const SERVICE_ACCOUNT: &str = r#"
{
  "type": "service_account",
  "project_id": "env-bug-test",
  "private_key_id": "0000000000000000000000000000000000000000",
  "private_key": "-----BEGIN PRIVATE KEY-----\nMIIEvAIBADANBgkqhkiG9w0BAQEFAASCBKYwggSiAgEAAoIBAQCyXa2NvvfbZNXx\nDqURZ2ojW050H7BqG2BcxbKuMu6uDjefsO/vaKJWVW8OfUknFk3hCf2xP0oE06ww\nTJWig7SjGAM7VH3qbFNNaIcTBUBzbPfK1V3chQETbbHWR6IALZ6uUy+KTnsUe6k6\njbaMQ5iVhlMZrZgbn4h6IfyzfuLla2kypbx6pmLUkdwz2DAARyjZYsFFb6r4UQ9v\n2P+EW5T3dhZ4hacAyeFN7pOIniXrV4CTK2qsgD8eZPUVK5v56iyHqBOeoxw60EOs\neK0fF6D4MPfsipIdOAoDQEBNp+TBwijk26OkEFl1nrWU5kF4EqXb9VlncFAACOV5\nmgux/lERAgMBAAECggEABN2OdGn3Kpk0IoU65A3J1NSAgDeQgT2cOL3qsP9/0YzM\nlxCTGvR2TcCWSGDmhjq3htjWEMNpfJ+mHMoHvA0lZbfQii6iavQMXj7D++v75Txw\nAD8CUeDEo5otkSHyhsYlg/ayqG5Qw3hGX348aWaEWrs+b1pPeMFtl1dLJIbksicR\nsjoq9sA+/q7gKogUk1UKNKwLVt9qUpg9k4KC3JAZ8RD5KdV+aaVDTzoJJlkES7nz\nz9HbOGTgIgsPRDaN9wkPV0C/TiwAzqm+XB3PCS5GmoCOAjuYAmM0w+pPAke7WyA8\nP4r+Jus+nlu9mE9O5p7RZ6EFnsXyDncNAEd6Enxz+QKBgQDWgXjOfo+GtAHB66ZN\nLYTjZQCGbP8ZHqDbWjSkrh7/zxIA1r31Wd5Kprs+7xx+JUiYgm2nlVFrjJRv2fTL\nicuEve7CJstrrGrPueHxYKsrRK6jPFAXZUKqf+HjFG9VOzDIJDAquwDn/pepSAnc\nn5N6897BIYMATcVpbkn5zgOJPwKBgQDU3ocjc2KrWOEdJijuUoxKM3VA2e1kME4G\nn6yGgIwWpcjdN+w8ZHsu6A+43cS0EHIRGBJONC1mmze5F4WMal4kKccbsrG+qHYp\nv9TCBpJ1ggqqX2XlQwPv5wEm7Yvfg0V9bJ44CHyrIQPx/DZlOWGAT/L7+Cugpco1\npR9fYyXBrwKBgDlbXWY5yT8HmwCzdkBkz4fOVhmbw4p/+hstP65ZMMjOIc4/nYOh\niCMF+kOVxFSmu4asOY36C09qw7Y03ZQfQGA6OUGJBhPAgr8QwS2934cuUlQrqG7Z\nEL5ue4QwzqRHs3+QzaBfaJqgJQWK0bhAoqrZQtL5lwHZydOrPfSh9agZAoGANBHm\n3p4X8bABCN9PvdRFatUhhmaIcix6uZYPXqtNGTEebDaEu9UcaMHtLpxYxG8NZ+8K\nc0TCIkovnEALs+hE9PWfNcBFSEzu8pFxGrD+3G2d7ET/qM/2w5UvK/WJdHgo5GBQ\nZ4Zz8GxYCbZXG/WoSvnDhy0Shu5nAy5L+UpIbxMCgYABU1cTqc0+3ViBqzrTLArk\nHHeCC+UG5/DloF20eKdLPNs8gWs5fmf3i0t01RZdodRuXMe6bjFXleA+xfbrIzIh\nhYJhHU+QID5Up5kzEPY3SRGdjvihzeBkxukjg3EU9tCkWE3l5BFVIysKbt+/qiln\nGmvUURBk9OTb6D8/+HKVvA==\n-----END PRIVATE KEY-----\n",
  "client_email": "env-bug-test-service-account@env-bug-test.iam.gserviceaccount.com",
  "client_id": "000000000000000000000",
  "auth_uri": "https://accounts.google.com/o/oauth2/auth",
  "token_uri": "https://oauth2.googleapis.com/token",
  "auth_provider_x509_cert_url": "https://www.googleapis.com/oauth2/v1/certs",
  "client_x509_cert_url": "https://www.googleapis.com/robot/v1/metadata/x509/env-bug-test-service-account%40env-bug-test.iam.gserviceaccount.com"
}
"#;

fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
    match account() {
        Err(e) => {
            log!("Couldn't parse the account info!");
            log!(e);
        }
        Ok(acc) => {
            log!("Parsed the account info just fine.");
            // (3) UNCOMMENT THIS TO TRIGGER THE `env` bug!
            // orders.perform_cmd(async { Msg::TokenReq(get_token(acc).await) });
        }
    }

    Model { message: None }
}

struct Model {
    message: Option<&'static str>,
}

enum Msg {
    TokenReq(Option<&'static str>),
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::TokenReq(None) => {
            model.message.replace("It failed!");
        }
        Msg::TokenReq(Some(m)) => {
            model.message.replace(m);
        }
    }
}

fn view(model: &Model) -> Node<Msg> {
    div![match model.message.as_deref() {
        None => "Nothing yet!",
        Some(m) => m,
    }]
}

pub fn account() -> Result<ServiceAccountAccess, tame_oauth::Error> {
    ServiceAccountInfo::deserialize(SERVICE_ACCOUNT).and_then(ServiceAccountAccess::new)
}

/// Mocks the token creation/fetching process.
async fn get_token(account: ServiceAccountAccess) -> Option<&'static str> {
    match account.get_token(GMAIL_SCOPES).ok()? {
        TokenOrRequest::Token(_) => Some("Preexisting token!"),
        TokenOrRequest::Request {
            request,
            scope_hash,
            ..
        } => Some("Got a new token!"),
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
