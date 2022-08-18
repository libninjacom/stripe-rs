use serde_json::json;
use crate::model;
use crate::model::*;
use crate::StripeClient;
pub struct GetAccountRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub expand: Option<Vec<String>>,
}
impl<'a> GetAccountRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Account> {
        let mut r = self.client.client.get("/v1/account");
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct PostAccountRequest<'a> {
    pub(crate) client: &'a StripeClient,
}
impl<'a> PostAccountRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Account> {
        let mut r = self.client.client.post("/v1/account");
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct DeleteAccountRequest<'a> {
    pub(crate) client: &'a StripeClient,
}
impl<'a> DeleteAccountRequest<'a> {
    pub async fn send(self) -> anyhow::Result<DeletedAccount> {
        let mut r = self.client.client.delete("/v1/account");
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct PostAccountBankAccountsRequest<'a> {
    pub(crate) client: &'a StripeClient,
}
impl<'a> PostAccountBankAccountsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ExternalAccount> {
        let mut r = self.client.client.post("/v1/account/bank_accounts");
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetAccountBankAccountsIdRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub expand: Option<Vec<String>>,
    pub id: String,
}
impl<'a> GetAccountBankAccountsIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ExternalAccount> {
        let mut r = self
            .client
            .client
            .get(&format!("/v1/account/bank_accounts/{id}", id = self.id));
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct PostAccountBankAccountsIdRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub id: String,
}
impl<'a> PostAccountBankAccountsIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ExternalAccount> {
        let mut r = self
            .client
            .client
            .post(&format!("/v1/account/bank_accounts/{id}", id = self.id));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct DeleteAccountBankAccountsIdRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub id: String,
}
impl<'a> DeleteAccountBankAccountsIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<DeletedExternalAccount> {
        let mut r = self
            .client
            .client
            .delete(&format!("/v1/account/bank_accounts/{id}", id = self.id));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetAccountCapabilitiesRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub expand: Option<Vec<String>>,
}
impl<'a> GetAccountCapabilitiesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v1/account/capabilities");
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct GetAccountCapabilitiesCapabilityRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub capability: String,
    pub expand: Option<Vec<String>>,
}
impl<'a> GetAccountCapabilitiesCapabilityRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Capability> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/v1/account/capabilities/{capability}", capability = self.capability
                ),
            );
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct PostAccountCapabilitiesCapabilityRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub capability: String,
}
impl<'a> PostAccountCapabilitiesCapabilityRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Capability> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/v1/account/capabilities/{capability}", capability = self.capability
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetAccountExternalAccountsRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
}
impl<'a> GetAccountExternalAccountsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v1/account/external_accounts");
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
}
pub struct PostAccountExternalAccountsRequest<'a> {
    pub(crate) client: &'a StripeClient,
}
impl<'a> PostAccountExternalAccountsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ExternalAccount> {
        let mut r = self.client.client.post("/v1/account/external_accounts");
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetAccountExternalAccountsIdRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub expand: Option<Vec<String>>,
    pub id: String,
}
impl<'a> GetAccountExternalAccountsIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ExternalAccount> {
        let mut r = self
            .client
            .client
            .get(&format!("/v1/account/external_accounts/{id}", id = self.id));
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct PostAccountExternalAccountsIdRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub id: String,
}
impl<'a> PostAccountExternalAccountsIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ExternalAccount> {
        let mut r = self
            .client
            .client
            .post(&format!("/v1/account/external_accounts/{id}", id = self.id));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct DeleteAccountExternalAccountsIdRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub id: String,
}
impl<'a> DeleteAccountExternalAccountsIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<DeletedExternalAccount> {
        let mut r = self
            .client
            .client
            .delete(&format!("/v1/account/external_accounts/{id}", id = self.id));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct PostAccountLoginLinksRequest<'a> {
    pub(crate) client: &'a StripeClient,
}
impl<'a> PostAccountLoginLinksRequest<'a> {
    pub async fn send(self) -> anyhow::Result<LoginLink> {
        let mut r = self.client.client.post("/v1/account/login_links");
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetAccountPeopleRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub relationship: Option<serde_json::Value>,
    pub starting_after: Option<String>,
}
impl<'a> GetAccountPeopleRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v1/account/people");
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.relationship {
            r = r.push_query("relationship", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn relationship(mut self, relationship: serde_json::Value) -> Self {
        self.relationship = Some(relationship);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
}
pub struct PostAccountPeopleRequest<'a> {
    pub(crate) client: &'a StripeClient,
}
impl<'a> PostAccountPeopleRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Person> {
        let mut r = self.client.client.post("/v1/account/people");
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetAccountPeoplePersonRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub expand: Option<Vec<String>>,
    pub person: String,
}
impl<'a> GetAccountPeoplePersonRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Person> {
        let mut r = self
            .client
            .client
            .get(&format!("/v1/account/people/{person}", person = self.person));
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct PostAccountPeoplePersonRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub person: String,
}
impl<'a> PostAccountPeoplePersonRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Person> {
        let mut r = self
            .client
            .client
            .post(&format!("/v1/account/people/{person}", person = self.person));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct DeleteAccountPeoplePersonRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub person: String,
}
impl<'a> DeleteAccountPeoplePersonRequest<'a> {
    pub async fn send(self) -> anyhow::Result<DeletedPerson> {
        let mut r = self
            .client
            .client
            .delete(&format!("/v1/account/people/{person}", person = self.person));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetAccountPersonsRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub relationship: Option<serde_json::Value>,
    pub starting_after: Option<String>,
}
impl<'a> GetAccountPersonsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v1/account/persons");
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.relationship {
            r = r.push_query("relationship", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn relationship(mut self, relationship: serde_json::Value) -> Self {
        self.relationship = Some(relationship);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
}
pub struct PostAccountPersonsRequest<'a> {
    pub(crate) client: &'a StripeClient,
}
impl<'a> PostAccountPersonsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Person> {
        let mut r = self.client.client.post("/v1/account/persons");
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetAccountPersonsPersonRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub expand: Option<Vec<String>>,
    pub person: String,
}
impl<'a> GetAccountPersonsPersonRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Person> {
        let mut r = self
            .client
            .client
            .get(&format!("/v1/account/persons/{person}", person = self.person));
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct PostAccountPersonsPersonRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub person: String,
}
impl<'a> PostAccountPersonsPersonRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Person> {
        let mut r = self
            .client
            .client
            .post(&format!("/v1/account/persons/{person}", person = self.person));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct DeleteAccountPersonsPersonRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub person: String,
}
impl<'a> DeleteAccountPersonsPersonRequest<'a> {
    pub async fn send(self) -> anyhow::Result<DeletedPerson> {
        let mut r = self
            .client
            .client
            .delete(&format!("/v1/account/persons/{person}", person = self.person));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct PostAccountLinksRequest<'a> {
    pub(crate) client: &'a StripeClient,
}
impl<'a> PostAccountLinksRequest<'a> {
    pub async fn send(self) -> anyhow::Result<AccountLink> {
        let mut r = self.client.client.post("/v1/account_links");
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetAccountsRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub created: Option<serde_json::Value>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
}
impl<'a> GetAccountsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v1/accounts");
        if let Some(ref unwrapped) = self.created {
            r = r.push_query("created", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn created(mut self, created: serde_json::Value) -> Self {
        self.created = Some(created);
        self
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
}
pub struct PostAccountsRequest<'a> {
    pub(crate) client: &'a StripeClient,
}
impl<'a> PostAccountsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Account> {
        let mut r = self.client.client.post("/v1/accounts");
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetAccountsAccountRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub account: String,
    pub expand: Option<Vec<String>>,
}
impl<'a> GetAccountsAccountRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Account> {
        let mut r = self
            .client
            .client
            .get(&format!("/v1/accounts/{account}", account = self.account));
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct PostAccountsAccountRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub account: String,
}
impl<'a> PostAccountsAccountRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Account> {
        let mut r = self
            .client
            .client
            .post(&format!("/v1/accounts/{account}", account = self.account));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct DeleteAccountsAccountRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub account: String,
}
impl<'a> DeleteAccountsAccountRequest<'a> {
    pub async fn send(self) -> anyhow::Result<DeletedAccount> {
        let mut r = self
            .client
            .client
            .delete(&format!("/v1/accounts/{account}", account = self.account));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct PostAccountsAccountBankAccountsRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub account: String,
}
impl<'a> PostAccountsAccountBankAccountsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ExternalAccount> {
        let mut r = self
            .client
            .client
            .post(
                &format!("/v1/accounts/{account}/bank_accounts", account = self.account),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetAccountsAccountBankAccountsIdRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub account: String,
    pub expand: Option<Vec<String>>,
    pub id: String,
}
impl<'a> GetAccountsAccountBankAccountsIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ExternalAccount> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/v1/accounts/{account}/bank_accounts/{id}", account = self.account,
                    id = self.id
                ),
            );
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct PostAccountsAccountBankAccountsIdRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub account: String,
    pub id: String,
}
impl<'a> PostAccountsAccountBankAccountsIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ExternalAccount> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/v1/accounts/{account}/bank_accounts/{id}", account = self.account,
                    id = self.id
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct DeleteAccountsAccountBankAccountsIdRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub account: String,
    pub id: String,
}
impl<'a> DeleteAccountsAccountBankAccountsIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<DeletedExternalAccount> {
        let mut r = self
            .client
            .client
            .delete(
                &format!(
                    "/v1/accounts/{account}/bank_accounts/{id}", account = self.account,
                    id = self.id
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetAccountsAccountCapabilitiesRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub account: String,
    pub expand: Option<Vec<String>>,
}
impl<'a> GetAccountsAccountCapabilitiesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .get(
                &format!("/v1/accounts/{account}/capabilities", account = self.account),
            );
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct GetAccountsAccountCapabilitiesCapabilityRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub account: String,
    pub capability: String,
    pub expand: Option<Vec<String>>,
}
impl<'a> GetAccountsAccountCapabilitiesCapabilityRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Capability> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/v1/accounts/{account}/capabilities/{capability}", account = self
                    .account, capability = self.capability
                ),
            );
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct PostAccountsAccountCapabilitiesCapabilityRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub account: String,
    pub capability: String,
}
impl<'a> PostAccountsAccountCapabilitiesCapabilityRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Capability> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/v1/accounts/{account}/capabilities/{capability}", account = self
                    .account, capability = self.capability
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetAccountsAccountExternalAccountsRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub account: String,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
}
impl<'a> GetAccountsAccountExternalAccountsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/v1/accounts/{account}/external_accounts", account = self.account
                ),
            );
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
}
pub struct PostAccountsAccountExternalAccountsRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub account: String,
}
impl<'a> PostAccountsAccountExternalAccountsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ExternalAccount> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/v1/accounts/{account}/external_accounts", account = self.account
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetAccountsAccountExternalAccountsIdRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub account: String,
    pub expand: Option<Vec<String>>,
    pub id: String,
}
impl<'a> GetAccountsAccountExternalAccountsIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ExternalAccount> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/v1/accounts/{account}/external_accounts/{id}", account = self
                    .account, id = self.id
                ),
            );
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct PostAccountsAccountExternalAccountsIdRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub account: String,
    pub id: String,
}
impl<'a> PostAccountsAccountExternalAccountsIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ExternalAccount> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/v1/accounts/{account}/external_accounts/{id}", account = self
                    .account, id = self.id
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct DeleteAccountsAccountExternalAccountsIdRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub account: String,
    pub id: String,
}
impl<'a> DeleteAccountsAccountExternalAccountsIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<DeletedExternalAccount> {
        let mut r = self
            .client
            .client
            .delete(
                &format!(
                    "/v1/accounts/{account}/external_accounts/{id}", account = self
                    .account, id = self.id
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct PostAccountsAccountLoginLinksRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub account: String,
}
impl<'a> PostAccountsAccountLoginLinksRequest<'a> {
    pub async fn send(self) -> anyhow::Result<LoginLink> {
        let mut r = self
            .client
            .client
            .post(
                &format!("/v1/accounts/{account}/login_links", account = self.account),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetAccountsAccountPeopleRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub account: String,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub relationship: Option<serde_json::Value>,
    pub starting_after: Option<String>,
}
impl<'a> GetAccountsAccountPeopleRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .get(&format!("/v1/accounts/{account}/people", account = self.account));
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.relationship {
            r = r.push_query("relationship", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn relationship(mut self, relationship: serde_json::Value) -> Self {
        self.relationship = Some(relationship);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
}
pub struct PostAccountsAccountPeopleRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub account: String,
}
impl<'a> PostAccountsAccountPeopleRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Person> {
        let mut r = self
            .client
            .client
            .post(&format!("/v1/accounts/{account}/people", account = self.account));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetAccountsAccountPeoplePersonRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub account: String,
    pub expand: Option<Vec<String>>,
    pub person: String,
}
impl<'a> GetAccountsAccountPeoplePersonRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Person> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/v1/accounts/{account}/people/{person}", account = self.account,
                    person = self.person
                ),
            );
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct PostAccountsAccountPeoplePersonRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub account: String,
    pub person: String,
}
impl<'a> PostAccountsAccountPeoplePersonRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Person> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/v1/accounts/{account}/people/{person}", account = self.account,
                    person = self.person
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct DeleteAccountsAccountPeoplePersonRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub account: String,
    pub person: String,
}
impl<'a> DeleteAccountsAccountPeoplePersonRequest<'a> {
    pub async fn send(self) -> anyhow::Result<DeletedPerson> {
        let mut r = self
            .client
            .client
            .delete(
                &format!(
                    "/v1/accounts/{account}/people/{person}", account = self.account,
                    person = self.person
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetAccountsAccountPersonsRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub account: String,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub relationship: Option<serde_json::Value>,
    pub starting_after: Option<String>,
}
impl<'a> GetAccountsAccountPersonsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .get(&format!("/v1/accounts/{account}/persons", account = self.account));
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.relationship {
            r = r.push_query("relationship", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn relationship(mut self, relationship: serde_json::Value) -> Self {
        self.relationship = Some(relationship);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
}
pub struct PostAccountsAccountPersonsRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub account: String,
}
impl<'a> PostAccountsAccountPersonsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Person> {
        let mut r = self
            .client
            .client
            .post(&format!("/v1/accounts/{account}/persons", account = self.account));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetAccountsAccountPersonsPersonRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub account: String,
    pub expand: Option<Vec<String>>,
    pub person: String,
}
impl<'a> GetAccountsAccountPersonsPersonRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Person> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/v1/accounts/{account}/persons/{person}", account = self.account,
                    person = self.person
                ),
            );
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct PostAccountsAccountPersonsPersonRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub account: String,
    pub person: String,
}
impl<'a> PostAccountsAccountPersonsPersonRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Person> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/v1/accounts/{account}/persons/{person}", account = self.account,
                    person = self.person
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct DeleteAccountsAccountPersonsPersonRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub account: String,
    pub person: String,
}
impl<'a> DeleteAccountsAccountPersonsPersonRequest<'a> {
    pub async fn send(self) -> anyhow::Result<DeletedPerson> {
        let mut r = self
            .client
            .client
            .delete(
                &format!(
                    "/v1/accounts/{account}/persons/{person}", account = self.account,
                    person = self.person
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct PostAccountsAccountRejectRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub account: String,
}
impl<'a> PostAccountsAccountRejectRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Account> {
        let mut r = self
            .client
            .client
            .post(&format!("/v1/accounts/{account}/reject", account = self.account));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetApplePayDomainsRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub domain_name: Option<String>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
}
impl<'a> GetApplePayDomainsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v1/apple_pay/domains");
        if let Some(ref unwrapped) = self.domain_name {
            r = r.push_query("domain_name", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn domain_name(mut self, domain_name: String) -> Self {
        self.domain_name = Some(domain_name);
        self
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
}
pub struct PostApplePayDomainsRequest<'a> {
    pub(crate) client: &'a StripeClient,
}
impl<'a> PostApplePayDomainsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ApplePayDomain> {
        let mut r = self.client.client.post("/v1/apple_pay/domains");
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetApplePayDomainsDomainRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub domain: String,
    pub expand: Option<Vec<String>>,
}
impl<'a> GetApplePayDomainsDomainRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ApplePayDomain> {
        let mut r = self
            .client
            .client
            .get(&format!("/v1/apple_pay/domains/{domain}", domain = self.domain));
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct DeleteApplePayDomainsDomainRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub domain: String,
}
impl<'a> DeleteApplePayDomainsDomainRequest<'a> {
    pub async fn send(self) -> anyhow::Result<DeletedApplePayDomain> {
        let mut r = self
            .client
            .client
            .delete(&format!("/v1/apple_pay/domains/{domain}", domain = self.domain));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetApplicationFeesRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub charge: Option<String>,
    pub created: Option<serde_json::Value>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
}
impl<'a> GetApplicationFeesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v1/application_fees");
        if let Some(ref unwrapped) = self.charge {
            r = r.push_query("charge", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.created {
            r = r.push_query("created", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn charge(mut self, charge: String) -> Self {
        self.charge = Some(charge);
        self
    }
    pub fn created(mut self, created: serde_json::Value) -> Self {
        self.created = Some(created);
        self
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
}
pub struct GetApplicationFeesFeeRefundsIdRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub expand: Option<Vec<String>>,
    pub fee: String,
    pub id: String,
}
impl<'a> GetApplicationFeesFeeRefundsIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<FeeRefund> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/v1/application_fees/{fee}/refunds/{id}", fee = self.fee, id = self
                    .id
                ),
            );
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct PostApplicationFeesFeeRefundsIdRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub fee: String,
    pub id: String,
}
impl<'a> PostApplicationFeesFeeRefundsIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<FeeRefund> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/v1/application_fees/{fee}/refunds/{id}", fee = self.fee, id = self
                    .id
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetApplicationFeesIdRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub expand: Option<Vec<String>>,
    pub id: String,
}
impl<'a> GetApplicationFeesIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ApplicationFee> {
        let mut r = self
            .client
            .client
            .get(&format!("/v1/application_fees/{id}", id = self.id));
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct PostApplicationFeesIdRefundRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub id: String,
}
impl<'a> PostApplicationFeesIdRefundRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ApplicationFee> {
        let mut r = self
            .client
            .client
            .post(&format!("/v1/application_fees/{id}/refund", id = self.id));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetApplicationFeesIdRefundsRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub id: String,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
}
impl<'a> GetApplicationFeesIdRefundsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .get(&format!("/v1/application_fees/{id}/refunds", id = self.id));
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
}
pub struct PostApplicationFeesIdRefundsRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub id: String,
}
impl<'a> PostApplicationFeesIdRefundsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<FeeRefund> {
        let mut r = self
            .client
            .client
            .post(&format!("/v1/application_fees/{id}/refunds", id = self.id));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetAppsSecretsRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub scope: Option<serde_json::Value>,
    pub starting_after: Option<String>,
}
impl<'a> GetAppsSecretsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v1/apps/secrets");
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.scope {
            r = r.push_query("scope", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn scope(mut self, scope: serde_json::Value) -> Self {
        self.scope = Some(scope);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
}
pub struct PostAppsSecretsRequest<'a> {
    pub(crate) client: &'a StripeClient,
}
impl<'a> PostAppsSecretsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<AppsSecret> {
        let mut r = self.client.client.post("/v1/apps/secrets");
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct PostAppsSecretsDeleteRequest<'a> {
    pub(crate) client: &'a StripeClient,
}
impl<'a> PostAppsSecretsDeleteRequest<'a> {
    pub async fn send(self) -> anyhow::Result<AppsSecret> {
        let mut r = self.client.client.post("/v1/apps/secrets/delete");
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetAppsSecretsFindRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub expand: Option<Vec<String>>,
    pub name: String,
    pub scope: Option<serde_json::Value>,
}
impl<'a> GetAppsSecretsFindRequest<'a> {
    pub async fn send(self) -> anyhow::Result<AppsSecret> {
        let mut r = self.client.client.get("/v1/apps/secrets/find");
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = r.push_query("name", &self.name.to_string());
        if let Some(ref unwrapped) = self.scope {
            r = r.push_query("scope", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn scope(mut self, scope: serde_json::Value) -> Self {
        self.scope = Some(scope);
        self
    }
}
pub struct GetBalanceRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub expand: Option<Vec<String>>,
}
impl<'a> GetBalanceRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Balance> {
        let mut r = self.client.client.get("/v1/balance");
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct GetBalanceHistoryRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub created: Option<serde_json::Value>,
    pub currency: Option<String>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub payout: Option<String>,
    pub source: Option<String>,
    pub starting_after: Option<String>,
    pub type_: Option<String>,
}
impl<'a> GetBalanceHistoryRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v1/balance/history");
        if let Some(ref unwrapped) = self.created {
            r = r.push_query("created", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.currency {
            r = r.push_query("currency", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.payout {
            r = r.push_query("payout", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.source {
            r = r.push_query("source", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.type_ {
            r = r.push_query("type", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn created(mut self, created: serde_json::Value) -> Self {
        self.created = Some(created);
        self
    }
    pub fn currency(mut self, currency: String) -> Self {
        self.currency = Some(currency);
        self
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn payout(mut self, payout: String) -> Self {
        self.payout = Some(payout);
        self
    }
    pub fn source(mut self, source: String) -> Self {
        self.source = Some(source);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
    pub fn type_(mut self, type_: String) -> Self {
        self.type_ = Some(type_);
        self
    }
}
pub struct GetBalanceHistoryIdRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub expand: Option<Vec<String>>,
    pub id: String,
}
impl<'a> GetBalanceHistoryIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<BalanceTransaction> {
        let mut r = self
            .client
            .client
            .get(&format!("/v1/balance/history/{id}", id = self.id));
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct GetBalanceTransactionsRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub created: Option<serde_json::Value>,
    pub currency: Option<String>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub payout: Option<String>,
    pub source: Option<String>,
    pub starting_after: Option<String>,
    pub type_: Option<String>,
}
impl<'a> GetBalanceTransactionsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v1/balance_transactions");
        if let Some(ref unwrapped) = self.created {
            r = r.push_query("created", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.currency {
            r = r.push_query("currency", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.payout {
            r = r.push_query("payout", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.source {
            r = r.push_query("source", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.type_ {
            r = r.push_query("type", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn created(mut self, created: serde_json::Value) -> Self {
        self.created = Some(created);
        self
    }
    pub fn currency(mut self, currency: String) -> Self {
        self.currency = Some(currency);
        self
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn payout(mut self, payout: String) -> Self {
        self.payout = Some(payout);
        self
    }
    pub fn source(mut self, source: String) -> Self {
        self.source = Some(source);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
    pub fn type_(mut self, type_: String) -> Self {
        self.type_ = Some(type_);
        self
    }
}
pub struct GetBalanceTransactionsIdRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub expand: Option<Vec<String>>,
    pub id: String,
}
impl<'a> GetBalanceTransactionsIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<BalanceTransaction> {
        let mut r = self
            .client
            .client
            .get(&format!("/v1/balance_transactions/{id}", id = self.id));
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct GetBillingPortalConfigurationsRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub active: Option<bool>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub is_default: Option<bool>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
}
impl<'a> GetBillingPortalConfigurationsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v1/billing_portal/configurations");
        if let Some(ref unwrapped) = self.active {
            r = r.push_query("active", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.is_default {
            r = r.push_query("is_default", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn active(mut self, active: bool) -> Self {
        self.active = Some(active);
        self
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn is_default(mut self, is_default: bool) -> Self {
        self.is_default = Some(is_default);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
}
pub struct PostBillingPortalConfigurationsRequest<'a> {
    pub(crate) client: &'a StripeClient,
}
impl<'a> PostBillingPortalConfigurationsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<BillingPortalConfiguration> {
        let mut r = self.client.client.post("/v1/billing_portal/configurations");
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetBillingPortalConfigurationsConfigurationRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub configuration: String,
    pub expand: Option<Vec<String>>,
}
impl<'a> GetBillingPortalConfigurationsConfigurationRequest<'a> {
    pub async fn send(self) -> anyhow::Result<BillingPortalConfiguration> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/v1/billing_portal/configurations/{configuration}", configuration =
                    self.configuration
                ),
            );
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct PostBillingPortalConfigurationsConfigurationRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub configuration: String,
}
impl<'a> PostBillingPortalConfigurationsConfigurationRequest<'a> {
    pub async fn send(self) -> anyhow::Result<BillingPortalConfiguration> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/v1/billing_portal/configurations/{configuration}", configuration =
                    self.configuration
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct PostBillingPortalSessionsRequest<'a> {
    pub(crate) client: &'a StripeClient,
}
impl<'a> PostBillingPortalSessionsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<BillingPortalSession> {
        let mut r = self.client.client.post("/v1/billing_portal/sessions");
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetChargesRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub created: Option<serde_json::Value>,
    pub customer: Option<String>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub payment_intent: Option<String>,
    pub starting_after: Option<String>,
    pub transfer_group: Option<String>,
}
impl<'a> GetChargesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v1/charges");
        if let Some(ref unwrapped) = self.created {
            r = r.push_query("created", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.customer {
            r = r.push_query("customer", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.payment_intent {
            r = r.push_query("payment_intent", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.transfer_group {
            r = r.push_query("transfer_group", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn created(mut self, created: serde_json::Value) -> Self {
        self.created = Some(created);
        self
    }
    pub fn customer(mut self, customer: String) -> Self {
        self.customer = Some(customer);
        self
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn payment_intent(mut self, payment_intent: String) -> Self {
        self.payment_intent = Some(payment_intent);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
    pub fn transfer_group(mut self, transfer_group: String) -> Self {
        self.transfer_group = Some(transfer_group);
        self
    }
}
pub struct PostChargesRequest<'a> {
    pub(crate) client: &'a StripeClient,
}
impl<'a> PostChargesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Charge> {
        let mut r = self.client.client.post("/v1/charges");
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetChargesSearchRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub page: Option<String>,
    pub query: String,
}
impl<'a> GetChargesSearchRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v1/charges/search");
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.page {
            r = r.push_query("page", &unwrapped.to_string());
        }
        r = r.push_query("query", &self.query.to_string());
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn page(mut self, page: String) -> Self {
        self.page = Some(page);
        self
    }
}
pub struct GetChargesChargeRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub charge: String,
    pub expand: Option<Vec<String>>,
}
impl<'a> GetChargesChargeRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Charge> {
        let mut r = self
            .client
            .client
            .get(&format!("/v1/charges/{charge}", charge = self.charge));
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct PostChargesChargeRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub charge: String,
}
impl<'a> PostChargesChargeRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Charge> {
        let mut r = self
            .client
            .client
            .post(&format!("/v1/charges/{charge}", charge = self.charge));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct PostChargesChargeCaptureRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub charge: String,
}
impl<'a> PostChargesChargeCaptureRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Charge> {
        let mut r = self
            .client
            .client
            .post(&format!("/v1/charges/{charge}/capture", charge = self.charge));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetChargesChargeDisputeRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub charge: String,
    pub expand: Option<Vec<String>>,
}
impl<'a> GetChargesChargeDisputeRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Dispute> {
        let mut r = self
            .client
            .client
            .get(&format!("/v1/charges/{charge}/dispute", charge = self.charge));
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct PostChargesChargeDisputeRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub charge: String,
}
impl<'a> PostChargesChargeDisputeRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Dispute> {
        let mut r = self
            .client
            .client
            .post(&format!("/v1/charges/{charge}/dispute", charge = self.charge));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct PostChargesChargeDisputeCloseRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub charge: String,
}
impl<'a> PostChargesChargeDisputeCloseRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Dispute> {
        let mut r = self
            .client
            .client
            .post(&format!("/v1/charges/{charge}/dispute/close", charge = self.charge));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct PostChargesChargeRefundRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub charge: String,
}
impl<'a> PostChargesChargeRefundRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Charge> {
        let mut r = self
            .client
            .client
            .post(&format!("/v1/charges/{charge}/refund", charge = self.charge));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetChargesChargeRefundsRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub charge: String,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
}
impl<'a> GetChargesChargeRefundsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .get(&format!("/v1/charges/{charge}/refunds", charge = self.charge));
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
}
pub struct PostChargesChargeRefundsRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub charge: String,
}
impl<'a> PostChargesChargeRefundsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Refund> {
        let mut r = self
            .client
            .client
            .post(&format!("/v1/charges/{charge}/refunds", charge = self.charge));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetChargesChargeRefundsRefundRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub charge: String,
    pub expand: Option<Vec<String>>,
    pub refund: String,
}
impl<'a> GetChargesChargeRefundsRefundRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Refund> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/v1/charges/{charge}/refunds/{refund}", charge = self.charge, refund
                    = self.refund
                ),
            );
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct PostChargesChargeRefundsRefundRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub charge: String,
    pub refund: String,
}
impl<'a> PostChargesChargeRefundsRefundRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Refund> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/v1/charges/{charge}/refunds/{refund}", charge = self.charge, refund
                    = self.refund
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetCheckoutSessionsRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub customer: Option<String>,
    pub customer_details: Option<serde_json::Value>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub payment_intent: Option<String>,
    pub starting_after: Option<String>,
    pub subscription: Option<String>,
}
impl<'a> GetCheckoutSessionsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v1/checkout/sessions");
        if let Some(ref unwrapped) = self.customer {
            r = r.push_query("customer", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.customer_details {
            r = r.push_query("customer_details", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.payment_intent {
            r = r.push_query("payment_intent", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.subscription {
            r = r.push_query("subscription", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn customer(mut self, customer: String) -> Self {
        self.customer = Some(customer);
        self
    }
    pub fn customer_details(mut self, customer_details: serde_json::Value) -> Self {
        self.customer_details = Some(customer_details);
        self
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn payment_intent(mut self, payment_intent: String) -> Self {
        self.payment_intent = Some(payment_intent);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
    pub fn subscription(mut self, subscription: String) -> Self {
        self.subscription = Some(subscription);
        self
    }
}
pub struct PostCheckoutSessionsRequest<'a> {
    pub(crate) client: &'a StripeClient,
}
impl<'a> PostCheckoutSessionsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<CheckoutSession> {
        let mut r = self.client.client.post("/v1/checkout/sessions");
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetCheckoutSessionsSessionRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub expand: Option<Vec<String>>,
    pub session: String,
}
impl<'a> GetCheckoutSessionsSessionRequest<'a> {
    pub async fn send(self) -> anyhow::Result<CheckoutSession> {
        let mut r = self
            .client
            .client
            .get(&format!("/v1/checkout/sessions/{session}", session = self.session));
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct PostCheckoutSessionsSessionExpireRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub session: String,
}
impl<'a> PostCheckoutSessionsSessionExpireRequest<'a> {
    pub async fn send(self) -> anyhow::Result<CheckoutSession> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/v1/checkout/sessions/{session}/expire", session = self.session
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetCheckoutSessionsSessionLineItemsRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub session: String,
    pub starting_after: Option<String>,
}
impl<'a> GetCheckoutSessionsSessionLineItemsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/v1/checkout/sessions/{session}/line_items", session = self.session
                ),
            );
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
}
pub struct GetCountrySpecsRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
}
impl<'a> GetCountrySpecsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v1/country_specs");
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
}
pub struct GetCountrySpecsCountryRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub country: String,
    pub expand: Option<Vec<String>>,
}
impl<'a> GetCountrySpecsCountryRequest<'a> {
    pub async fn send(self) -> anyhow::Result<CountrySpec> {
        let mut r = self
            .client
            .client
            .get(&format!("/v1/country_specs/{country}", country = self.country));
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct GetCouponsRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub created: Option<serde_json::Value>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
}
impl<'a> GetCouponsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v1/coupons");
        if let Some(ref unwrapped) = self.created {
            r = r.push_query("created", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn created(mut self, created: serde_json::Value) -> Self {
        self.created = Some(created);
        self
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
}
pub struct PostCouponsRequest<'a> {
    pub(crate) client: &'a StripeClient,
}
impl<'a> PostCouponsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Coupon> {
        let mut r = self.client.client.post("/v1/coupons");
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetCouponsCouponRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub coupon: String,
    pub expand: Option<Vec<String>>,
}
impl<'a> GetCouponsCouponRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Coupon> {
        let mut r = self
            .client
            .client
            .get(&format!("/v1/coupons/{coupon}", coupon = self.coupon));
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct PostCouponsCouponRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub coupon: String,
}
impl<'a> PostCouponsCouponRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Coupon> {
        let mut r = self
            .client
            .client
            .post(&format!("/v1/coupons/{coupon}", coupon = self.coupon));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct DeleteCouponsCouponRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub coupon: String,
}
impl<'a> DeleteCouponsCouponRequest<'a> {
    pub async fn send(self) -> anyhow::Result<DeletedCoupon> {
        let mut r = self
            .client
            .client
            .delete(&format!("/v1/coupons/{coupon}", coupon = self.coupon));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetCreditNotesRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub customer: Option<String>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub invoice: Option<String>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
}
impl<'a> GetCreditNotesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v1/credit_notes");
        if let Some(ref unwrapped) = self.customer {
            r = r.push_query("customer", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.invoice {
            r = r.push_query("invoice", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn customer(mut self, customer: String) -> Self {
        self.customer = Some(customer);
        self
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn invoice(mut self, invoice: String) -> Self {
        self.invoice = Some(invoice);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
}
pub struct PostCreditNotesRequest<'a> {
    pub(crate) client: &'a StripeClient,
}
impl<'a> PostCreditNotesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<CreditNote> {
        let mut r = self.client.client.post("/v1/credit_notes");
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetCreditNotesPreviewRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub amount: Option<i64>,
    pub credit_amount: Option<i64>,
    pub expand: Option<Vec<String>>,
    pub invoice: String,
    pub lines: Option<Vec<serde_json::Value>>,
    pub memo: Option<String>,
    pub metadata: Option<serde_json::Value>,
    pub out_of_band_amount: Option<i64>,
    pub reason: Option<String>,
    pub refund: Option<String>,
    pub refund_amount: Option<i64>,
}
impl<'a> GetCreditNotesPreviewRequest<'a> {
    pub async fn send(self) -> anyhow::Result<CreditNote> {
        let mut r = self.client.client.get("/v1/credit_notes/preview");
        if let Some(ref unwrapped) = self.amount {
            r = r.push_query("amount", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.credit_amount {
            r = r.push_query("credit_amount", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = r.push_query("invoice", &self.invoice.to_string());
        if let Some(ref unwrapped) = self.lines {
            for item in unwrapped {
                r = r.push_query("lines[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.memo {
            r = r.push_query("memo", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.metadata {
            r = r.push_query("metadata", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.out_of_band_amount {
            r = r.push_query("out_of_band_amount", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.reason {
            r = r.push_query("reason", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.refund {
            r = r.push_query("refund", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.refund_amount {
            r = r.push_query("refund_amount", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn amount(mut self, amount: i64) -> Self {
        self.amount = Some(amount);
        self
    }
    pub fn credit_amount(mut self, credit_amount: i64) -> Self {
        self.credit_amount = Some(credit_amount);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn lines(mut self, lines: Vec<serde_json::Value>) -> Self {
        self.lines = Some(lines);
        self
    }
    pub fn memo(mut self, memo: String) -> Self {
        self.memo = Some(memo);
        self
    }
    pub fn metadata(mut self, metadata: serde_json::Value) -> Self {
        self.metadata = Some(metadata);
        self
    }
    pub fn out_of_band_amount(mut self, out_of_band_amount: i64) -> Self {
        self.out_of_band_amount = Some(out_of_band_amount);
        self
    }
    pub fn reason(mut self, reason: String) -> Self {
        self.reason = Some(reason);
        self
    }
    pub fn refund(mut self, refund: String) -> Self {
        self.refund = Some(refund);
        self
    }
    pub fn refund_amount(mut self, refund_amount: i64) -> Self {
        self.refund_amount = Some(refund_amount);
        self
    }
}
pub struct GetCreditNotesPreviewLinesRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub amount: Option<i64>,
    pub credit_amount: Option<i64>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub invoice: String,
    pub limit: Option<i64>,
    pub lines: Option<Vec<serde_json::Value>>,
    pub memo: Option<String>,
    pub metadata: Option<serde_json::Value>,
    pub out_of_band_amount: Option<i64>,
    pub reason: Option<String>,
    pub refund: Option<String>,
    pub refund_amount: Option<i64>,
    pub starting_after: Option<String>,
}
impl<'a> GetCreditNotesPreviewLinesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v1/credit_notes/preview/lines");
        if let Some(ref unwrapped) = self.amount {
            r = r.push_query("amount", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.credit_amount {
            r = r.push_query("credit_amount", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = r.push_query("invoice", &self.invoice.to_string());
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.lines {
            for item in unwrapped {
                r = r.push_query("lines[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.memo {
            r = r.push_query("memo", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.metadata {
            r = r.push_query("metadata", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.out_of_band_amount {
            r = r.push_query("out_of_band_amount", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.reason {
            r = r.push_query("reason", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.refund {
            r = r.push_query("refund", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.refund_amount {
            r = r.push_query("refund_amount", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn amount(mut self, amount: i64) -> Self {
        self.amount = Some(amount);
        self
    }
    pub fn credit_amount(mut self, credit_amount: i64) -> Self {
        self.credit_amount = Some(credit_amount);
        self
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn lines(mut self, lines: Vec<serde_json::Value>) -> Self {
        self.lines = Some(lines);
        self
    }
    pub fn memo(mut self, memo: String) -> Self {
        self.memo = Some(memo);
        self
    }
    pub fn metadata(mut self, metadata: serde_json::Value) -> Self {
        self.metadata = Some(metadata);
        self
    }
    pub fn out_of_band_amount(mut self, out_of_band_amount: i64) -> Self {
        self.out_of_band_amount = Some(out_of_band_amount);
        self
    }
    pub fn reason(mut self, reason: String) -> Self {
        self.reason = Some(reason);
        self
    }
    pub fn refund(mut self, refund: String) -> Self {
        self.refund = Some(refund);
        self
    }
    pub fn refund_amount(mut self, refund_amount: i64) -> Self {
        self.refund_amount = Some(refund_amount);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
}
pub struct GetCreditNotesCreditNoteLinesRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub credit_note: String,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
}
impl<'a> GetCreditNotesCreditNoteLinesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/v1/credit_notes/{credit_note}/lines", credit_note = self
                    .credit_note
                ),
            );
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
}
pub struct GetCreditNotesIdRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub expand: Option<Vec<String>>,
    pub id: String,
}
impl<'a> GetCreditNotesIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<CreditNote> {
        let mut r = self
            .client
            .client
            .get(&format!("/v1/credit_notes/{id}", id = self.id));
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct PostCreditNotesIdRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub id: String,
}
impl<'a> PostCreditNotesIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<CreditNote> {
        let mut r = self
            .client
            .client
            .post(&format!("/v1/credit_notes/{id}", id = self.id));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct PostCreditNotesIdVoidRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub id: String,
}
impl<'a> PostCreditNotesIdVoidRequest<'a> {
    pub async fn send(self) -> anyhow::Result<CreditNote> {
        let mut r = self
            .client
            .client
            .post(&format!("/v1/credit_notes/{id}/void", id = self.id));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetCustomersRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub created: Option<serde_json::Value>,
    pub email: Option<String>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
    pub test_clock: Option<String>,
}
impl<'a> GetCustomersRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v1/customers");
        if let Some(ref unwrapped) = self.created {
            r = r.push_query("created", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.email {
            r = r.push_query("email", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.test_clock {
            r = r.push_query("test_clock", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn created(mut self, created: serde_json::Value) -> Self {
        self.created = Some(created);
        self
    }
    pub fn email(mut self, email: String) -> Self {
        self.email = Some(email);
        self
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
    pub fn test_clock(mut self, test_clock: String) -> Self {
        self.test_clock = Some(test_clock);
        self
    }
}
pub struct PostCustomersRequest<'a> {
    pub(crate) client: &'a StripeClient,
}
impl<'a> PostCustomersRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Customer> {
        let mut r = self.client.client.post("/v1/customers");
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetCustomersSearchRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub page: Option<String>,
    pub query: String,
}
impl<'a> GetCustomersSearchRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v1/customers/search");
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.page {
            r = r.push_query("page", &unwrapped.to_string());
        }
        r = r.push_query("query", &self.query.to_string());
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn page(mut self, page: String) -> Self {
        self.page = Some(page);
        self
    }
}
pub struct GetCustomersCustomerRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub customer: String,
    pub expand: Option<Vec<String>>,
}
impl<'a> GetCustomersCustomerRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .get(&format!("/v1/customers/{customer}", customer = self.customer));
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct PostCustomersCustomerRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub customer: String,
}
impl<'a> PostCustomersCustomerRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Customer> {
        let mut r = self
            .client
            .client
            .post(&format!("/v1/customers/{customer}", customer = self.customer));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct DeleteCustomersCustomerRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub customer: String,
}
impl<'a> DeleteCustomersCustomerRequest<'a> {
    pub async fn send(self) -> anyhow::Result<DeletedCustomer> {
        let mut r = self
            .client
            .client
            .delete(&format!("/v1/customers/{customer}", customer = self.customer));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetCustomersCustomerBalanceTransactionsRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub customer: String,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
}
impl<'a> GetCustomersCustomerBalanceTransactionsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/v1/customers/{customer}/balance_transactions", customer = self
                    .customer
                ),
            );
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
}
pub struct PostCustomersCustomerBalanceTransactionsRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub customer: String,
}
impl<'a> PostCustomersCustomerBalanceTransactionsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<CustomerBalanceTransaction> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/v1/customers/{customer}/balance_transactions", customer = self
                    .customer
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetCustomersCustomerBalanceTransactionsTransactionRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub customer: String,
    pub expand: Option<Vec<String>>,
    pub transaction: String,
}
impl<'a> GetCustomersCustomerBalanceTransactionsTransactionRequest<'a> {
    pub async fn send(self) -> anyhow::Result<CustomerBalanceTransaction> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/v1/customers/{customer}/balance_transactions/{transaction}",
                    customer = self.customer, transaction = self.transaction
                ),
            );
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct PostCustomersCustomerBalanceTransactionsTransactionRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub customer: String,
    pub transaction: String,
}
impl<'a> PostCustomersCustomerBalanceTransactionsTransactionRequest<'a> {
    pub async fn send(self) -> anyhow::Result<CustomerBalanceTransaction> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/v1/customers/{customer}/balance_transactions/{transaction}",
                    customer = self.customer, transaction = self.transaction
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetCustomersCustomerBankAccountsRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub customer: String,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
}
impl<'a> GetCustomersCustomerBankAccountsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/v1/customers/{customer}/bank_accounts", customer = self.customer
                ),
            );
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
}
pub struct PostCustomersCustomerBankAccountsRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub customer: String,
}
impl<'a> PostCustomersCustomerBankAccountsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<PaymentSource> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/v1/customers/{customer}/bank_accounts", customer = self.customer
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetCustomersCustomerBankAccountsIdRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub customer: String,
    pub expand: Option<Vec<String>>,
    pub id: String,
}
impl<'a> GetCustomersCustomerBankAccountsIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<BankAccount> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/v1/customers/{customer}/bank_accounts/{id}", customer = self
                    .customer, id = self.id
                ),
            );
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct PostCustomersCustomerBankAccountsIdRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub customer: String,
    pub id: String,
}
impl<'a> PostCustomersCustomerBankAccountsIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/v1/customers/{customer}/bank_accounts/{id}", customer = self
                    .customer, id = self.id
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct DeleteCustomersCustomerBankAccountsIdRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub customer: String,
    pub id: String,
}
impl<'a> DeleteCustomersCustomerBankAccountsIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .delete(
                &format!(
                    "/v1/customers/{customer}/bank_accounts/{id}", customer = self
                    .customer, id = self.id
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct PostCustomersCustomerBankAccountsIdVerifyRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub customer: String,
    pub id: String,
}
impl<'a> PostCustomersCustomerBankAccountsIdVerifyRequest<'a> {
    pub async fn send(self) -> anyhow::Result<BankAccount> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/v1/customers/{customer}/bank_accounts/{id}/verify", customer = self
                    .customer, id = self.id
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetCustomersCustomerCardsRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub customer: String,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
}
impl<'a> GetCustomersCustomerCardsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .get(&format!("/v1/customers/{customer}/cards", customer = self.customer));
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
}
pub struct PostCustomersCustomerCardsRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub customer: String,
}
impl<'a> PostCustomersCustomerCardsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<PaymentSource> {
        let mut r = self
            .client
            .client
            .post(&format!("/v1/customers/{customer}/cards", customer = self.customer));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetCustomersCustomerCardsIdRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub customer: String,
    pub expand: Option<Vec<String>>,
    pub id: String,
}
impl<'a> GetCustomersCustomerCardsIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Card> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/v1/customers/{customer}/cards/{id}", customer = self.customer, id =
                    self.id
                ),
            );
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct PostCustomersCustomerCardsIdRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub customer: String,
    pub id: String,
}
impl<'a> PostCustomersCustomerCardsIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/v1/customers/{customer}/cards/{id}", customer = self.customer, id =
                    self.id
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct DeleteCustomersCustomerCardsIdRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub customer: String,
    pub id: String,
}
impl<'a> DeleteCustomersCustomerCardsIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .delete(
                &format!(
                    "/v1/customers/{customer}/cards/{id}", customer = self.customer, id =
                    self.id
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetCustomersCustomerCashBalanceRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub customer: String,
    pub expand: Option<Vec<String>>,
}
impl<'a> GetCustomersCustomerCashBalanceRequest<'a> {
    pub async fn send(self) -> anyhow::Result<CashBalance> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/v1/customers/{customer}/cash_balance", customer = self.customer
                ),
            );
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct PostCustomersCustomerCashBalanceRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub customer: String,
}
impl<'a> PostCustomersCustomerCashBalanceRequest<'a> {
    pub async fn send(self) -> anyhow::Result<CashBalance> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/v1/customers/{customer}/cash_balance", customer = self.customer
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetCustomersCustomerCashBalanceTransactionsRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub customer: String,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
}
impl<'a> GetCustomersCustomerCashBalanceTransactionsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/v1/customers/{customer}/cash_balance_transactions", customer = self
                    .customer
                ),
            );
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
}
pub struct GetCustomersCustomerCashBalanceTransactionsTransactionRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub customer: String,
    pub expand: Option<Vec<String>>,
    pub transaction: String,
}
impl<'a> GetCustomersCustomerCashBalanceTransactionsTransactionRequest<'a> {
    pub async fn send(self) -> anyhow::Result<CustomerCashBalanceTransaction> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/v1/customers/{customer}/cash_balance_transactions/{transaction}",
                    customer = self.customer, transaction = self.transaction
                ),
            );
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct GetCustomersCustomerDiscountRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub customer: String,
    pub expand: Option<Vec<String>>,
}
impl<'a> GetCustomersCustomerDiscountRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Discount> {
        let mut r = self
            .client
            .client
            .get(
                &format!("/v1/customers/{customer}/discount", customer = self.customer),
            );
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct DeleteCustomersCustomerDiscountRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub customer: String,
}
impl<'a> DeleteCustomersCustomerDiscountRequest<'a> {
    pub async fn send(self) -> anyhow::Result<DeletedDiscount> {
        let mut r = self
            .client
            .client
            .delete(
                &format!("/v1/customers/{customer}/discount", customer = self.customer),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct PostCustomersCustomerFundingInstructionsRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub customer: String,
}
impl<'a> PostCustomersCustomerFundingInstructionsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<FundingInstructions> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/v1/customers/{customer}/funding_instructions", customer = self
                    .customer
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetCustomersCustomerPaymentMethodsRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub customer: String,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
    pub type_: String,
}
impl<'a> GetCustomersCustomerPaymentMethodsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/v1/customers/{customer}/payment_methods", customer = self.customer
                ),
            );
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        r = r.push_query("type", &self.type_.to_string());
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
}
pub struct GetCustomersCustomerPaymentMethodsPaymentMethodRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub customer: String,
    pub expand: Option<Vec<String>>,
    pub payment_method: String,
}
impl<'a> GetCustomersCustomerPaymentMethodsPaymentMethodRequest<'a> {
    pub async fn send(self) -> anyhow::Result<PaymentMethod> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/v1/customers/{customer}/payment_methods/{payment_method}", customer
                    = self.customer, payment_method = self.payment_method
                ),
            );
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct GetCustomersCustomerSourcesRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub customer: String,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub object: Option<String>,
    pub starting_after: Option<String>,
}
impl<'a> GetCustomersCustomerSourcesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .get(&format!("/v1/customers/{customer}/sources", customer = self.customer));
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.object {
            r = r.push_query("object", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn object(mut self, object: String) -> Self {
        self.object = Some(object);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
}
pub struct PostCustomersCustomerSourcesRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub customer: String,
}
impl<'a> PostCustomersCustomerSourcesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<PaymentSource> {
        let mut r = self
            .client
            .client
            .post(
                &format!("/v1/customers/{customer}/sources", customer = self.customer),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetCustomersCustomerSourcesIdRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub customer: String,
    pub expand: Option<Vec<String>>,
    pub id: String,
}
impl<'a> GetCustomersCustomerSourcesIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<PaymentSource> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/v1/customers/{customer}/sources/{id}", customer = self.customer, id
                    = self.id
                ),
            );
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct PostCustomersCustomerSourcesIdRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub customer: String,
    pub id: String,
}
impl<'a> PostCustomersCustomerSourcesIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/v1/customers/{customer}/sources/{id}", customer = self.customer, id
                    = self.id
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct DeleteCustomersCustomerSourcesIdRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub customer: String,
    pub id: String,
}
impl<'a> DeleteCustomersCustomerSourcesIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .delete(
                &format!(
                    "/v1/customers/{customer}/sources/{id}", customer = self.customer, id
                    = self.id
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct PostCustomersCustomerSourcesIdVerifyRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub customer: String,
    pub id: String,
}
impl<'a> PostCustomersCustomerSourcesIdVerifyRequest<'a> {
    pub async fn send(self) -> anyhow::Result<BankAccount> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/v1/customers/{customer}/sources/{id}/verify", customer = self
                    .customer, id = self.id
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetCustomersCustomerSubscriptionsRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub customer: String,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
}
impl<'a> GetCustomersCustomerSubscriptionsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/v1/customers/{customer}/subscriptions", customer = self.customer
                ),
            );
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
}
pub struct PostCustomersCustomerSubscriptionsRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub customer: String,
}
impl<'a> PostCustomersCustomerSubscriptionsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Subscription> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/v1/customers/{customer}/subscriptions", customer = self.customer
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetCustomersCustomerSubscriptionsSubscriptionExposedIdRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub customer: String,
    pub expand: Option<Vec<String>>,
    pub subscription_exposed_id: String,
}
impl<'a> GetCustomersCustomerSubscriptionsSubscriptionExposedIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Subscription> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/v1/customers/{customer}/subscriptions/{subscription_exposed_id}",
                    customer = self.customer, subscription_exposed_id = self
                    .subscription_exposed_id
                ),
            );
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct PostCustomersCustomerSubscriptionsSubscriptionExposedIdRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub customer: String,
    pub subscription_exposed_id: String,
}
impl<'a> PostCustomersCustomerSubscriptionsSubscriptionExposedIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Subscription> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/v1/customers/{customer}/subscriptions/{subscription_exposed_id}",
                    customer = self.customer, subscription_exposed_id = self
                    .subscription_exposed_id
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct DeleteCustomersCustomerSubscriptionsSubscriptionExposedIdRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub customer: String,
    pub subscription_exposed_id: String,
}
impl<'a> DeleteCustomersCustomerSubscriptionsSubscriptionExposedIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Subscription> {
        let mut r = self
            .client
            .client
            .delete(
                &format!(
                    "/v1/customers/{customer}/subscriptions/{subscription_exposed_id}",
                    customer = self.customer, subscription_exposed_id = self
                    .subscription_exposed_id
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetCustomersCustomerSubscriptionsSubscriptionExposedIdDiscountRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub customer: String,
    pub expand: Option<Vec<String>>,
    pub subscription_exposed_id: String,
}
impl<'a> GetCustomersCustomerSubscriptionsSubscriptionExposedIdDiscountRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Discount> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/v1/customers/{customer}/subscriptions/{subscription_exposed_id}/discount",
                    customer = self.customer, subscription_exposed_id = self
                    .subscription_exposed_id
                ),
            );
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct DeleteCustomersCustomerSubscriptionsSubscriptionExposedIdDiscountRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub customer: String,
    pub subscription_exposed_id: String,
}
impl<'a> DeleteCustomersCustomerSubscriptionsSubscriptionExposedIdDiscountRequest<'a> {
    pub async fn send(self) -> anyhow::Result<DeletedDiscount> {
        let mut r = self
            .client
            .client
            .delete(
                &format!(
                    "/v1/customers/{customer}/subscriptions/{subscription_exposed_id}/discount",
                    customer = self.customer, subscription_exposed_id = self
                    .subscription_exposed_id
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetCustomersCustomerTaxIdsRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub customer: String,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
}
impl<'a> GetCustomersCustomerTaxIdsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .get(&format!("/v1/customers/{customer}/tax_ids", customer = self.customer));
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
}
pub struct PostCustomersCustomerTaxIdsRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub customer: String,
}
impl<'a> PostCustomersCustomerTaxIdsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TaxId> {
        let mut r = self
            .client
            .client
            .post(
                &format!("/v1/customers/{customer}/tax_ids", customer = self.customer),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetCustomersCustomerTaxIdsIdRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub customer: String,
    pub expand: Option<Vec<String>>,
    pub id: String,
}
impl<'a> GetCustomersCustomerTaxIdsIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TaxId> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/v1/customers/{customer}/tax_ids/{id}", customer = self.customer, id
                    = self.id
                ),
            );
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct DeleteCustomersCustomerTaxIdsIdRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub customer: String,
    pub id: String,
}
impl<'a> DeleteCustomersCustomerTaxIdsIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<DeletedTaxId> {
        let mut r = self
            .client
            .client
            .delete(
                &format!(
                    "/v1/customers/{customer}/tax_ids/{id}", customer = self.customer, id
                    = self.id
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetDisputesRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub charge: Option<String>,
    pub created: Option<serde_json::Value>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub payment_intent: Option<String>,
    pub starting_after: Option<String>,
}
impl<'a> GetDisputesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v1/disputes");
        if let Some(ref unwrapped) = self.charge {
            r = r.push_query("charge", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.created {
            r = r.push_query("created", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.payment_intent {
            r = r.push_query("payment_intent", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn charge(mut self, charge: String) -> Self {
        self.charge = Some(charge);
        self
    }
    pub fn created(mut self, created: serde_json::Value) -> Self {
        self.created = Some(created);
        self
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn payment_intent(mut self, payment_intent: String) -> Self {
        self.payment_intent = Some(payment_intent);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
}
pub struct GetDisputesDisputeRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub dispute: String,
    pub expand: Option<Vec<String>>,
}
impl<'a> GetDisputesDisputeRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Dispute> {
        let mut r = self
            .client
            .client
            .get(&format!("/v1/disputes/{dispute}", dispute = self.dispute));
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct PostDisputesDisputeRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub dispute: String,
}
impl<'a> PostDisputesDisputeRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Dispute> {
        let mut r = self
            .client
            .client
            .post(&format!("/v1/disputes/{dispute}", dispute = self.dispute));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct PostDisputesDisputeCloseRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub dispute: String,
}
impl<'a> PostDisputesDisputeCloseRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Dispute> {
        let mut r = self
            .client
            .client
            .post(&format!("/v1/disputes/{dispute}/close", dispute = self.dispute));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct PostEphemeralKeysRequest<'a> {
    pub(crate) client: &'a StripeClient,
}
impl<'a> PostEphemeralKeysRequest<'a> {
    pub async fn send(self) -> anyhow::Result<EphemeralKey> {
        let mut r = self.client.client.post("/v1/ephemeral_keys");
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct DeleteEphemeralKeysKeyRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub key: String,
}
impl<'a> DeleteEphemeralKeysKeyRequest<'a> {
    pub async fn send(self) -> anyhow::Result<EphemeralKey> {
        let mut r = self
            .client
            .client
            .delete(&format!("/v1/ephemeral_keys/{key}", key = self.key));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetEventsRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub created: Option<serde_json::Value>,
    pub delivery_success: Option<bool>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
    pub type_: Option<String>,
    pub types: Option<Vec<String>>,
}
impl<'a> GetEventsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v1/events");
        if let Some(ref unwrapped) = self.created {
            r = r.push_query("created", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.delivery_success {
            r = r.push_query("delivery_success", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.type_ {
            r = r.push_query("type", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.types {
            for item in unwrapped {
                r = r.push_query("types[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn created(mut self, created: serde_json::Value) -> Self {
        self.created = Some(created);
        self
    }
    pub fn delivery_success(mut self, delivery_success: bool) -> Self {
        self.delivery_success = Some(delivery_success);
        self
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
    pub fn type_(mut self, type_: String) -> Self {
        self.type_ = Some(type_);
        self
    }
    pub fn types(mut self, types: Vec<String>) -> Self {
        self.types = Some(types);
        self
    }
}
pub struct GetEventsIdRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub expand: Option<Vec<String>>,
    pub id: String,
}
impl<'a> GetEventsIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Event> {
        let mut r = self.client.client.get(&format!("/v1/events/{id}", id = self.id));
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct GetExchangeRatesRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
}
impl<'a> GetExchangeRatesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v1/exchange_rates");
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
}
pub struct GetExchangeRatesRateIdRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub expand: Option<Vec<String>>,
    pub rate_id: String,
}
impl<'a> GetExchangeRatesRateIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ExchangeRate> {
        let mut r = self
            .client
            .client
            .get(&format!("/v1/exchange_rates/{rate_id}", rate_id = self.rate_id));
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct GetFileLinksRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub created: Option<serde_json::Value>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub expired: Option<bool>,
    pub file: Option<String>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
}
impl<'a> GetFileLinksRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v1/file_links");
        if let Some(ref unwrapped) = self.created {
            r = r.push_query("created", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.expired {
            r = r.push_query("expired", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.file {
            r = r.push_query("file", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn created(mut self, created: serde_json::Value) -> Self {
        self.created = Some(created);
        self
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn expired(mut self, expired: bool) -> Self {
        self.expired = Some(expired);
        self
    }
    pub fn file(mut self, file: String) -> Self {
        self.file = Some(file);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
}
pub struct PostFileLinksRequest<'a> {
    pub(crate) client: &'a StripeClient,
}
impl<'a> PostFileLinksRequest<'a> {
    pub async fn send(self) -> anyhow::Result<FileLink> {
        let mut r = self.client.client.post("/v1/file_links");
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetFileLinksLinkRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub expand: Option<Vec<String>>,
    pub link: String,
}
impl<'a> GetFileLinksLinkRequest<'a> {
    pub async fn send(self) -> anyhow::Result<FileLink> {
        let mut r = self
            .client
            .client
            .get(&format!("/v1/file_links/{link}", link = self.link));
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct PostFileLinksLinkRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub link: String,
}
impl<'a> PostFileLinksLinkRequest<'a> {
    pub async fn send(self) -> anyhow::Result<FileLink> {
        let mut r = self
            .client
            .client
            .post(&format!("/v1/file_links/{link}", link = self.link));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetFilesRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub created: Option<serde_json::Value>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub purpose: Option<String>,
    pub starting_after: Option<String>,
}
impl<'a> GetFilesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v1/files");
        if let Some(ref unwrapped) = self.created {
            r = r.push_query("created", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.purpose {
            r = r.push_query("purpose", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn created(mut self, created: serde_json::Value) -> Self {
        self.created = Some(created);
        self
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn purpose(mut self, purpose: String) -> Self {
        self.purpose = Some(purpose);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
}
pub struct PostFilesRequest<'a> {
    pub(crate) client: &'a StripeClient,
}
impl<'a> PostFilesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<File> {
        let mut r = self.client.client.post("/v1/files");
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetFilesFileRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub expand: Option<Vec<String>>,
    pub file: String,
}
impl<'a> GetFilesFileRequest<'a> {
    pub async fn send(self) -> anyhow::Result<File> {
        let mut r = self
            .client
            .client
            .get(&format!("/v1/files/{file}", file = self.file));
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct GetFinancialConnectionsAccountsRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub account_holder: Option<serde_json::Value>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub session: Option<String>,
    pub starting_after: Option<String>,
}
impl<'a> GetFinancialConnectionsAccountsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v1/financial_connections/accounts");
        if let Some(ref unwrapped) = self.account_holder {
            r = r.push_query("account_holder", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.session {
            r = r.push_query("session", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn account_holder(mut self, account_holder: serde_json::Value) -> Self {
        self.account_holder = Some(account_holder);
        self
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn session(mut self, session: String) -> Self {
        self.session = Some(session);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
}
pub struct GetFinancialConnectionsAccountsAccountRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub account: String,
    pub expand: Option<Vec<String>>,
}
impl<'a> GetFinancialConnectionsAccountsAccountRequest<'a> {
    pub async fn send(self) -> anyhow::Result<FinancialConnectionsAccount> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/v1/financial_connections/accounts/{account}", account = self
                    .account
                ),
            );
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct PostFinancialConnectionsAccountsAccountDisconnectRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub account: String,
}
impl<'a> PostFinancialConnectionsAccountsAccountDisconnectRequest<'a> {
    pub async fn send(self) -> anyhow::Result<FinancialConnectionsAccount> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/v1/financial_connections/accounts/{account}/disconnect", account =
                    self.account
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetFinancialConnectionsAccountsAccountOwnersRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub account: String,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub ownership: String,
    pub starting_after: Option<String>,
}
impl<'a> GetFinancialConnectionsAccountsAccountOwnersRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/v1/financial_connections/accounts/{account}/owners", account = self
                    .account
                ),
            );
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        r = r.push_query("ownership", &self.ownership.to_string());
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
}
pub struct PostFinancialConnectionsAccountsAccountRefreshRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub account: String,
}
impl<'a> PostFinancialConnectionsAccountsAccountRefreshRequest<'a> {
    pub async fn send(self) -> anyhow::Result<FinancialConnectionsAccount> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/v1/financial_connections/accounts/{account}/refresh", account =
                    self.account
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct PostFinancialConnectionsSessionsRequest<'a> {
    pub(crate) client: &'a StripeClient,
}
impl<'a> PostFinancialConnectionsSessionsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<FinancialConnectionsSession> {
        let mut r = self.client.client.post("/v1/financial_connections/sessions");
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetFinancialConnectionsSessionsSessionRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub expand: Option<Vec<String>>,
    pub session: String,
}
impl<'a> GetFinancialConnectionsSessionsSessionRequest<'a> {
    pub async fn send(self) -> anyhow::Result<FinancialConnectionsSession> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/v1/financial_connections/sessions/{session}", session = self
                    .session
                ),
            );
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct GetIdentityVerificationReportsRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub created: Option<serde_json::Value>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
    pub type_: Option<String>,
    pub verification_session: Option<String>,
}
impl<'a> GetIdentityVerificationReportsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v1/identity/verification_reports");
        if let Some(ref unwrapped) = self.created {
            r = r.push_query("created", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.type_ {
            r = r.push_query("type", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.verification_session {
            r = r.push_query("verification_session", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn created(mut self, created: serde_json::Value) -> Self {
        self.created = Some(created);
        self
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
    pub fn type_(mut self, type_: String) -> Self {
        self.type_ = Some(type_);
        self
    }
    pub fn verification_session(mut self, verification_session: String) -> Self {
        self.verification_session = Some(verification_session);
        self
    }
}
pub struct GetIdentityVerificationReportsReportRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub expand: Option<Vec<String>>,
    pub report: String,
}
impl<'a> GetIdentityVerificationReportsReportRequest<'a> {
    pub async fn send(self) -> anyhow::Result<IdentityVerificationReport> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/v1/identity/verification_reports/{report}", report = self.report
                ),
            );
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct GetIdentityVerificationSessionsRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub created: Option<serde_json::Value>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
    pub status: Option<String>,
}
impl<'a> GetIdentityVerificationSessionsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v1/identity/verification_sessions");
        if let Some(ref unwrapped) = self.created {
            r = r.push_query("created", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.status {
            r = r.push_query("status", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn created(mut self, created: serde_json::Value) -> Self {
        self.created = Some(created);
        self
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
    pub fn status(mut self, status: String) -> Self {
        self.status = Some(status);
        self
    }
}
pub struct PostIdentityVerificationSessionsRequest<'a> {
    pub(crate) client: &'a StripeClient,
}
impl<'a> PostIdentityVerificationSessionsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<IdentityVerificationSession> {
        let mut r = self.client.client.post("/v1/identity/verification_sessions");
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetIdentityVerificationSessionsSessionRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub expand: Option<Vec<String>>,
    pub session: String,
}
impl<'a> GetIdentityVerificationSessionsSessionRequest<'a> {
    pub async fn send(self) -> anyhow::Result<IdentityVerificationSession> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/v1/identity/verification_sessions/{session}", session = self
                    .session
                ),
            );
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct PostIdentityVerificationSessionsSessionRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub session: String,
}
impl<'a> PostIdentityVerificationSessionsSessionRequest<'a> {
    pub async fn send(self) -> anyhow::Result<IdentityVerificationSession> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/v1/identity/verification_sessions/{session}", session = self
                    .session
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct PostIdentityVerificationSessionsSessionCancelRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub session: String,
}
impl<'a> PostIdentityVerificationSessionsSessionCancelRequest<'a> {
    pub async fn send(self) -> anyhow::Result<IdentityVerificationSession> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/v1/identity/verification_sessions/{session}/cancel", session = self
                    .session
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct PostIdentityVerificationSessionsSessionRedactRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub session: String,
}
impl<'a> PostIdentityVerificationSessionsSessionRedactRequest<'a> {
    pub async fn send(self) -> anyhow::Result<IdentityVerificationSession> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/v1/identity/verification_sessions/{session}/redact", session = self
                    .session
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetInvoiceitemsRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub created: Option<serde_json::Value>,
    pub customer: Option<String>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub invoice: Option<String>,
    pub limit: Option<i64>,
    pub pending: Option<bool>,
    pub starting_after: Option<String>,
}
impl<'a> GetInvoiceitemsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v1/invoiceitems");
        if let Some(ref unwrapped) = self.created {
            r = r.push_query("created", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.customer {
            r = r.push_query("customer", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.invoice {
            r = r.push_query("invoice", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.pending {
            r = r.push_query("pending", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn created(mut self, created: serde_json::Value) -> Self {
        self.created = Some(created);
        self
    }
    pub fn customer(mut self, customer: String) -> Self {
        self.customer = Some(customer);
        self
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn invoice(mut self, invoice: String) -> Self {
        self.invoice = Some(invoice);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn pending(mut self, pending: bool) -> Self {
        self.pending = Some(pending);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
}
pub struct PostInvoiceitemsRequest<'a> {
    pub(crate) client: &'a StripeClient,
}
impl<'a> PostInvoiceitemsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Invoiceitem> {
        let mut r = self.client.client.post("/v1/invoiceitems");
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetInvoiceitemsInvoiceitemRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub expand: Option<Vec<String>>,
    pub invoiceitem: String,
}
impl<'a> GetInvoiceitemsInvoiceitemRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Invoiceitem> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/v1/invoiceitems/{invoiceitem}", invoiceitem = self.invoiceitem
                ),
            );
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct PostInvoiceitemsInvoiceitemRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub invoiceitem: String,
}
impl<'a> PostInvoiceitemsInvoiceitemRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Invoiceitem> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/v1/invoiceitems/{invoiceitem}", invoiceitem = self.invoiceitem
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct DeleteInvoiceitemsInvoiceitemRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub invoiceitem: String,
}
impl<'a> DeleteInvoiceitemsInvoiceitemRequest<'a> {
    pub async fn send(self) -> anyhow::Result<DeletedInvoiceitem> {
        let mut r = self
            .client
            .client
            .delete(
                &format!(
                    "/v1/invoiceitems/{invoiceitem}", invoiceitem = self.invoiceitem
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetInvoicesRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub collection_method: Option<String>,
    pub created: Option<serde_json::Value>,
    pub customer: Option<String>,
    pub due_date: Option<serde_json::Value>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
    pub status: Option<String>,
    pub subscription: Option<String>,
}
impl<'a> GetInvoicesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v1/invoices");
        if let Some(ref unwrapped) = self.collection_method {
            r = r.push_query("collection_method", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.created {
            r = r.push_query("created", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.customer {
            r = r.push_query("customer", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.due_date {
            r = r.push_query("due_date", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.status {
            r = r.push_query("status", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.subscription {
            r = r.push_query("subscription", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn collection_method(mut self, collection_method: String) -> Self {
        self.collection_method = Some(collection_method);
        self
    }
    pub fn created(mut self, created: serde_json::Value) -> Self {
        self.created = Some(created);
        self
    }
    pub fn customer(mut self, customer: String) -> Self {
        self.customer = Some(customer);
        self
    }
    pub fn due_date(mut self, due_date: serde_json::Value) -> Self {
        self.due_date = Some(due_date);
        self
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
    pub fn status(mut self, status: String) -> Self {
        self.status = Some(status);
        self
    }
    pub fn subscription(mut self, subscription: String) -> Self {
        self.subscription = Some(subscription);
        self
    }
}
pub struct PostInvoicesRequest<'a> {
    pub(crate) client: &'a StripeClient,
}
impl<'a> PostInvoicesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Invoice> {
        let mut r = self.client.client.post("/v1/invoices");
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetInvoicesSearchRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub page: Option<String>,
    pub query: String,
}
impl<'a> GetInvoicesSearchRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v1/invoices/search");
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.page {
            r = r.push_query("page", &unwrapped.to_string());
        }
        r = r.push_query("query", &self.query.to_string());
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn page(mut self, page: String) -> Self {
        self.page = Some(page);
        self
    }
}
pub struct GetInvoicesUpcomingRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub automatic_tax: Option<serde_json::Value>,
    pub coupon: Option<String>,
    pub currency: Option<String>,
    pub customer: Option<String>,
    pub customer_details: Option<serde_json::Value>,
    pub discounts: Option<serde_json::Value>,
    pub expand: Option<Vec<String>>,
    pub invoice_items: Option<Vec<serde_json::Value>>,
    pub schedule: Option<String>,
    pub subscription: Option<String>,
    pub subscription_billing_cycle_anchor: Option<serde_json::Value>,
    pub subscription_cancel_at: Option<serde_json::Value>,
    pub subscription_cancel_at_period_end: Option<bool>,
    pub subscription_cancel_now: Option<bool>,
    pub subscription_default_tax_rates: Option<serde_json::Value>,
    pub subscription_items: Option<Vec<serde_json::Value>>,
    pub subscription_proration_behavior: Option<String>,
    pub subscription_proration_date: Option<i64>,
    pub subscription_start_date: Option<i64>,
    pub subscription_trial_end: Option<serde_json::Value>,
    pub subscription_trial_from_plan: Option<bool>,
}
impl<'a> GetInvoicesUpcomingRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Invoice> {
        let mut r = self.client.client.get("/v1/invoices/upcoming");
        if let Some(ref unwrapped) = self.automatic_tax {
            r = r.push_query("automatic_tax", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.coupon {
            r = r.push_query("coupon", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.currency {
            r = r.push_query("currency", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.customer {
            r = r.push_query("customer", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.customer_details {
            r = r.push_query("customer_details", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.discounts {
            r = r.push_query("discounts", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.invoice_items {
            for item in unwrapped {
                r = r.push_query("invoice_items[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.schedule {
            r = r.push_query("schedule", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.subscription {
            r = r.push_query("subscription", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.subscription_billing_cycle_anchor {
            r = r
                .push_query("subscription_billing_cycle_anchor", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.subscription_cancel_at {
            r = r.push_query("subscription_cancel_at", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.subscription_cancel_at_period_end {
            r = r
                .push_query("subscription_cancel_at_period_end", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.subscription_cancel_now {
            r = r.push_query("subscription_cancel_now", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.subscription_default_tax_rates {
            r = r.push_query("subscription_default_tax_rates", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.subscription_items {
            for item in unwrapped {
                r = r.push_query("subscription_items[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.subscription_proration_behavior {
            r = r.push_query("subscription_proration_behavior", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.subscription_proration_date {
            r = r.push_query("subscription_proration_date", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.subscription_start_date {
            r = r.push_query("subscription_start_date", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.subscription_trial_end {
            r = r.push_query("subscription_trial_end", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.subscription_trial_from_plan {
            r = r.push_query("subscription_trial_from_plan", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn automatic_tax(mut self, automatic_tax: serde_json::Value) -> Self {
        self.automatic_tax = Some(automatic_tax);
        self
    }
    pub fn coupon(mut self, coupon: String) -> Self {
        self.coupon = Some(coupon);
        self
    }
    pub fn currency(mut self, currency: String) -> Self {
        self.currency = Some(currency);
        self
    }
    pub fn customer(mut self, customer: String) -> Self {
        self.customer = Some(customer);
        self
    }
    pub fn customer_details(mut self, customer_details: serde_json::Value) -> Self {
        self.customer_details = Some(customer_details);
        self
    }
    pub fn discounts(mut self, discounts: serde_json::Value) -> Self {
        self.discounts = Some(discounts);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn invoice_items(mut self, invoice_items: Vec<serde_json::Value>) -> Self {
        self.invoice_items = Some(invoice_items);
        self
    }
    pub fn schedule(mut self, schedule: String) -> Self {
        self.schedule = Some(schedule);
        self
    }
    pub fn subscription(mut self, subscription: String) -> Self {
        self.subscription = Some(subscription);
        self
    }
    pub fn subscription_billing_cycle_anchor(
        mut self,
        subscription_billing_cycle_anchor: serde_json::Value,
    ) -> Self {
        self.subscription_billing_cycle_anchor = Some(subscription_billing_cycle_anchor);
        self
    }
    pub fn subscription_cancel_at(
        mut self,
        subscription_cancel_at: serde_json::Value,
    ) -> Self {
        self.subscription_cancel_at = Some(subscription_cancel_at);
        self
    }
    pub fn subscription_cancel_at_period_end(
        mut self,
        subscription_cancel_at_period_end: bool,
    ) -> Self {
        self.subscription_cancel_at_period_end = Some(subscription_cancel_at_period_end);
        self
    }
    pub fn subscription_cancel_now(mut self, subscription_cancel_now: bool) -> Self {
        self.subscription_cancel_now = Some(subscription_cancel_now);
        self
    }
    pub fn subscription_default_tax_rates(
        mut self,
        subscription_default_tax_rates: serde_json::Value,
    ) -> Self {
        self.subscription_default_tax_rates = Some(subscription_default_tax_rates);
        self
    }
    pub fn subscription_items(
        mut self,
        subscription_items: Vec<serde_json::Value>,
    ) -> Self {
        self.subscription_items = Some(subscription_items);
        self
    }
    pub fn subscription_proration_behavior(
        mut self,
        subscription_proration_behavior: String,
    ) -> Self {
        self.subscription_proration_behavior = Some(subscription_proration_behavior);
        self
    }
    pub fn subscription_proration_date(
        mut self,
        subscription_proration_date: i64,
    ) -> Self {
        self.subscription_proration_date = Some(subscription_proration_date);
        self
    }
    pub fn subscription_start_date(mut self, subscription_start_date: i64) -> Self {
        self.subscription_start_date = Some(subscription_start_date);
        self
    }
    pub fn subscription_trial_end(
        mut self,
        subscription_trial_end: serde_json::Value,
    ) -> Self {
        self.subscription_trial_end = Some(subscription_trial_end);
        self
    }
    pub fn subscription_trial_from_plan(
        mut self,
        subscription_trial_from_plan: bool,
    ) -> Self {
        self.subscription_trial_from_plan = Some(subscription_trial_from_plan);
        self
    }
}
pub struct GetInvoicesUpcomingLinesRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub automatic_tax: Option<serde_json::Value>,
    pub coupon: Option<String>,
    pub currency: Option<String>,
    pub customer: Option<String>,
    pub customer_details: Option<serde_json::Value>,
    pub discounts: Option<serde_json::Value>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub invoice_items: Option<Vec<serde_json::Value>>,
    pub limit: Option<i64>,
    pub schedule: Option<String>,
    pub starting_after: Option<String>,
    pub subscription: Option<String>,
    pub subscription_billing_cycle_anchor: Option<serde_json::Value>,
    pub subscription_cancel_at: Option<serde_json::Value>,
    pub subscription_cancel_at_period_end: Option<bool>,
    pub subscription_cancel_now: Option<bool>,
    pub subscription_default_tax_rates: Option<serde_json::Value>,
    pub subscription_items: Option<Vec<serde_json::Value>>,
    pub subscription_proration_behavior: Option<String>,
    pub subscription_proration_date: Option<i64>,
    pub subscription_start_date: Option<i64>,
    pub subscription_trial_end: Option<serde_json::Value>,
    pub subscription_trial_from_plan: Option<bool>,
}
impl<'a> GetInvoicesUpcomingLinesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v1/invoices/upcoming/lines");
        if let Some(ref unwrapped) = self.automatic_tax {
            r = r.push_query("automatic_tax", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.coupon {
            r = r.push_query("coupon", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.currency {
            r = r.push_query("currency", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.customer {
            r = r.push_query("customer", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.customer_details {
            r = r.push_query("customer_details", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.discounts {
            r = r.push_query("discounts", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.invoice_items {
            for item in unwrapped {
                r = r.push_query("invoice_items[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.schedule {
            r = r.push_query("schedule", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.subscription {
            r = r.push_query("subscription", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.subscription_billing_cycle_anchor {
            r = r
                .push_query("subscription_billing_cycle_anchor", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.subscription_cancel_at {
            r = r.push_query("subscription_cancel_at", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.subscription_cancel_at_period_end {
            r = r
                .push_query("subscription_cancel_at_period_end", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.subscription_cancel_now {
            r = r.push_query("subscription_cancel_now", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.subscription_default_tax_rates {
            r = r.push_query("subscription_default_tax_rates", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.subscription_items {
            for item in unwrapped {
                r = r.push_query("subscription_items[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.subscription_proration_behavior {
            r = r.push_query("subscription_proration_behavior", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.subscription_proration_date {
            r = r.push_query("subscription_proration_date", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.subscription_start_date {
            r = r.push_query("subscription_start_date", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.subscription_trial_end {
            r = r.push_query("subscription_trial_end", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.subscription_trial_from_plan {
            r = r.push_query("subscription_trial_from_plan", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn automatic_tax(mut self, automatic_tax: serde_json::Value) -> Self {
        self.automatic_tax = Some(automatic_tax);
        self
    }
    pub fn coupon(mut self, coupon: String) -> Self {
        self.coupon = Some(coupon);
        self
    }
    pub fn currency(mut self, currency: String) -> Self {
        self.currency = Some(currency);
        self
    }
    pub fn customer(mut self, customer: String) -> Self {
        self.customer = Some(customer);
        self
    }
    pub fn customer_details(mut self, customer_details: serde_json::Value) -> Self {
        self.customer_details = Some(customer_details);
        self
    }
    pub fn discounts(mut self, discounts: serde_json::Value) -> Self {
        self.discounts = Some(discounts);
        self
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn invoice_items(mut self, invoice_items: Vec<serde_json::Value>) -> Self {
        self.invoice_items = Some(invoice_items);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn schedule(mut self, schedule: String) -> Self {
        self.schedule = Some(schedule);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
    pub fn subscription(mut self, subscription: String) -> Self {
        self.subscription = Some(subscription);
        self
    }
    pub fn subscription_billing_cycle_anchor(
        mut self,
        subscription_billing_cycle_anchor: serde_json::Value,
    ) -> Self {
        self.subscription_billing_cycle_anchor = Some(subscription_billing_cycle_anchor);
        self
    }
    pub fn subscription_cancel_at(
        mut self,
        subscription_cancel_at: serde_json::Value,
    ) -> Self {
        self.subscription_cancel_at = Some(subscription_cancel_at);
        self
    }
    pub fn subscription_cancel_at_period_end(
        mut self,
        subscription_cancel_at_period_end: bool,
    ) -> Self {
        self.subscription_cancel_at_period_end = Some(subscription_cancel_at_period_end);
        self
    }
    pub fn subscription_cancel_now(mut self, subscription_cancel_now: bool) -> Self {
        self.subscription_cancel_now = Some(subscription_cancel_now);
        self
    }
    pub fn subscription_default_tax_rates(
        mut self,
        subscription_default_tax_rates: serde_json::Value,
    ) -> Self {
        self.subscription_default_tax_rates = Some(subscription_default_tax_rates);
        self
    }
    pub fn subscription_items(
        mut self,
        subscription_items: Vec<serde_json::Value>,
    ) -> Self {
        self.subscription_items = Some(subscription_items);
        self
    }
    pub fn subscription_proration_behavior(
        mut self,
        subscription_proration_behavior: String,
    ) -> Self {
        self.subscription_proration_behavior = Some(subscription_proration_behavior);
        self
    }
    pub fn subscription_proration_date(
        mut self,
        subscription_proration_date: i64,
    ) -> Self {
        self.subscription_proration_date = Some(subscription_proration_date);
        self
    }
    pub fn subscription_start_date(mut self, subscription_start_date: i64) -> Self {
        self.subscription_start_date = Some(subscription_start_date);
        self
    }
    pub fn subscription_trial_end(
        mut self,
        subscription_trial_end: serde_json::Value,
    ) -> Self {
        self.subscription_trial_end = Some(subscription_trial_end);
        self
    }
    pub fn subscription_trial_from_plan(
        mut self,
        subscription_trial_from_plan: bool,
    ) -> Self {
        self.subscription_trial_from_plan = Some(subscription_trial_from_plan);
        self
    }
}
pub struct GetInvoicesInvoiceRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub expand: Option<Vec<String>>,
    pub invoice: String,
}
impl<'a> GetInvoicesInvoiceRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Invoice> {
        let mut r = self
            .client
            .client
            .get(&format!("/v1/invoices/{invoice}", invoice = self.invoice));
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct PostInvoicesInvoiceRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub invoice: String,
}
impl<'a> PostInvoicesInvoiceRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Invoice> {
        let mut r = self
            .client
            .client
            .post(&format!("/v1/invoices/{invoice}", invoice = self.invoice));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct DeleteInvoicesInvoiceRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub invoice: String,
}
impl<'a> DeleteInvoicesInvoiceRequest<'a> {
    pub async fn send(self) -> anyhow::Result<DeletedInvoice> {
        let mut r = self
            .client
            .client
            .delete(&format!("/v1/invoices/{invoice}", invoice = self.invoice));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct PostInvoicesInvoiceFinalizeRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub invoice: String,
}
impl<'a> PostInvoicesInvoiceFinalizeRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Invoice> {
        let mut r = self
            .client
            .client
            .post(&format!("/v1/invoices/{invoice}/finalize", invoice = self.invoice));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetInvoicesInvoiceLinesRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub invoice: String,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
}
impl<'a> GetInvoicesInvoiceLinesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .get(&format!("/v1/invoices/{invoice}/lines", invoice = self.invoice));
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
}
pub struct PostInvoicesInvoiceMarkUncollectibleRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub invoice: String,
}
impl<'a> PostInvoicesInvoiceMarkUncollectibleRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Invoice> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/v1/invoices/{invoice}/mark_uncollectible", invoice = self.invoice
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct PostInvoicesInvoicePayRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub invoice: String,
}
impl<'a> PostInvoicesInvoicePayRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Invoice> {
        let mut r = self
            .client
            .client
            .post(&format!("/v1/invoices/{invoice}/pay", invoice = self.invoice));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct PostInvoicesInvoiceSendRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub invoice: String,
}
impl<'a> PostInvoicesInvoiceSendRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Invoice> {
        let mut r = self
            .client
            .client
            .post(&format!("/v1/invoices/{invoice}/send", invoice = self.invoice));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct PostInvoicesInvoiceVoidRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub invoice: String,
}
impl<'a> PostInvoicesInvoiceVoidRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Invoice> {
        let mut r = self
            .client
            .client
            .post(&format!("/v1/invoices/{invoice}/void", invoice = self.invoice));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetIssuingAuthorizationsRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub card: Option<String>,
    pub cardholder: Option<String>,
    pub created: Option<serde_json::Value>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
    pub status: Option<String>,
}
impl<'a> GetIssuingAuthorizationsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v1/issuing/authorizations");
        if let Some(ref unwrapped) = self.card {
            r = r.push_query("card", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.cardholder {
            r = r.push_query("cardholder", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.created {
            r = r.push_query("created", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.status {
            r = r.push_query("status", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn card(mut self, card: String) -> Self {
        self.card = Some(card);
        self
    }
    pub fn cardholder(mut self, cardholder: String) -> Self {
        self.cardholder = Some(cardholder);
        self
    }
    pub fn created(mut self, created: serde_json::Value) -> Self {
        self.created = Some(created);
        self
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
    pub fn status(mut self, status: String) -> Self {
        self.status = Some(status);
        self
    }
}
pub struct GetIssuingAuthorizationsAuthorizationRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub authorization: String,
    pub expand: Option<Vec<String>>,
}
impl<'a> GetIssuingAuthorizationsAuthorizationRequest<'a> {
    pub async fn send(self) -> anyhow::Result<IssuingAuthorization> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/v1/issuing/authorizations/{authorization}", authorization = self
                    .authorization
                ),
            );
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct PostIssuingAuthorizationsAuthorizationRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub authorization: String,
}
impl<'a> PostIssuingAuthorizationsAuthorizationRequest<'a> {
    pub async fn send(self) -> anyhow::Result<IssuingAuthorization> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/v1/issuing/authorizations/{authorization}", authorization = self
                    .authorization
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct PostIssuingAuthorizationsAuthorizationApproveRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub authorization: String,
}
impl<'a> PostIssuingAuthorizationsAuthorizationApproveRequest<'a> {
    pub async fn send(self) -> anyhow::Result<IssuingAuthorization> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/v1/issuing/authorizations/{authorization}/approve", authorization =
                    self.authorization
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct PostIssuingAuthorizationsAuthorizationDeclineRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub authorization: String,
}
impl<'a> PostIssuingAuthorizationsAuthorizationDeclineRequest<'a> {
    pub async fn send(self) -> anyhow::Result<IssuingAuthorization> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/v1/issuing/authorizations/{authorization}/decline", authorization =
                    self.authorization
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetIssuingCardholdersRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub created: Option<serde_json::Value>,
    pub email: Option<String>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub phone_number: Option<String>,
    pub starting_after: Option<String>,
    pub status: Option<String>,
    pub type_: Option<String>,
}
impl<'a> GetIssuingCardholdersRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v1/issuing/cardholders");
        if let Some(ref unwrapped) = self.created {
            r = r.push_query("created", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.email {
            r = r.push_query("email", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.phone_number {
            r = r.push_query("phone_number", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.status {
            r = r.push_query("status", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.type_ {
            r = r.push_query("type", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn created(mut self, created: serde_json::Value) -> Self {
        self.created = Some(created);
        self
    }
    pub fn email(mut self, email: String) -> Self {
        self.email = Some(email);
        self
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn phone_number(mut self, phone_number: String) -> Self {
        self.phone_number = Some(phone_number);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
    pub fn status(mut self, status: String) -> Self {
        self.status = Some(status);
        self
    }
    pub fn type_(mut self, type_: String) -> Self {
        self.type_ = Some(type_);
        self
    }
}
pub struct PostIssuingCardholdersRequest<'a> {
    pub(crate) client: &'a StripeClient,
}
impl<'a> PostIssuingCardholdersRequest<'a> {
    pub async fn send(self) -> anyhow::Result<IssuingCardholder> {
        let mut r = self.client.client.post("/v1/issuing/cardholders");
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetIssuingCardholdersCardholderRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub cardholder: String,
    pub expand: Option<Vec<String>>,
}
impl<'a> GetIssuingCardholdersCardholderRequest<'a> {
    pub async fn send(self) -> anyhow::Result<IssuingCardholder> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/v1/issuing/cardholders/{cardholder}", cardholder = self.cardholder
                ),
            );
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct PostIssuingCardholdersCardholderRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub cardholder: String,
}
impl<'a> PostIssuingCardholdersCardholderRequest<'a> {
    pub async fn send(self) -> anyhow::Result<IssuingCardholder> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/v1/issuing/cardholders/{cardholder}", cardholder = self.cardholder
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetIssuingCardsRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub cardholder: Option<String>,
    pub created: Option<serde_json::Value>,
    pub ending_before: Option<String>,
    pub exp_month: Option<i64>,
    pub exp_year: Option<i64>,
    pub expand: Option<Vec<String>>,
    pub last4: Option<String>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
    pub status: Option<String>,
    pub type_: Option<String>,
}
impl<'a> GetIssuingCardsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v1/issuing/cards");
        if let Some(ref unwrapped) = self.cardholder {
            r = r.push_query("cardholder", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.created {
            r = r.push_query("created", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.exp_month {
            r = r.push_query("exp_month", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.exp_year {
            r = r.push_query("exp_year", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.last4 {
            r = r.push_query("last4", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.status {
            r = r.push_query("status", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.type_ {
            r = r.push_query("type", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn cardholder(mut self, cardholder: String) -> Self {
        self.cardholder = Some(cardholder);
        self
    }
    pub fn created(mut self, created: serde_json::Value) -> Self {
        self.created = Some(created);
        self
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn exp_month(mut self, exp_month: i64) -> Self {
        self.exp_month = Some(exp_month);
        self
    }
    pub fn exp_year(mut self, exp_year: i64) -> Self {
        self.exp_year = Some(exp_year);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn last4(mut self, last4: String) -> Self {
        self.last4 = Some(last4);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
    pub fn status(mut self, status: String) -> Self {
        self.status = Some(status);
        self
    }
    pub fn type_(mut self, type_: String) -> Self {
        self.type_ = Some(type_);
        self
    }
}
pub struct PostIssuingCardsRequest<'a> {
    pub(crate) client: &'a StripeClient,
}
impl<'a> PostIssuingCardsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<IssuingCard> {
        let mut r = self.client.client.post("/v1/issuing/cards");
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetIssuingCardsCardRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub card: String,
    pub expand: Option<Vec<String>>,
}
impl<'a> GetIssuingCardsCardRequest<'a> {
    pub async fn send(self) -> anyhow::Result<IssuingCard> {
        let mut r = self
            .client
            .client
            .get(&format!("/v1/issuing/cards/{card}", card = self.card));
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct PostIssuingCardsCardRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub card: String,
}
impl<'a> PostIssuingCardsCardRequest<'a> {
    pub async fn send(self) -> anyhow::Result<IssuingCard> {
        let mut r = self
            .client
            .client
            .post(&format!("/v1/issuing/cards/{card}", card = self.card));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetIssuingDisputesRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub created: Option<serde_json::Value>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
    pub status: Option<String>,
    pub transaction: Option<String>,
}
impl<'a> GetIssuingDisputesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v1/issuing/disputes");
        if let Some(ref unwrapped) = self.created {
            r = r.push_query("created", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.status {
            r = r.push_query("status", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.transaction {
            r = r.push_query("transaction", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn created(mut self, created: serde_json::Value) -> Self {
        self.created = Some(created);
        self
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
    pub fn status(mut self, status: String) -> Self {
        self.status = Some(status);
        self
    }
    pub fn transaction(mut self, transaction: String) -> Self {
        self.transaction = Some(transaction);
        self
    }
}
pub struct PostIssuingDisputesRequest<'a> {
    pub(crate) client: &'a StripeClient,
}
impl<'a> PostIssuingDisputesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<IssuingDispute> {
        let mut r = self.client.client.post("/v1/issuing/disputes");
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetIssuingDisputesDisputeRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub dispute: String,
    pub expand: Option<Vec<String>>,
}
impl<'a> GetIssuingDisputesDisputeRequest<'a> {
    pub async fn send(self) -> anyhow::Result<IssuingDispute> {
        let mut r = self
            .client
            .client
            .get(&format!("/v1/issuing/disputes/{dispute}", dispute = self.dispute));
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct PostIssuingDisputesDisputeRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub dispute: String,
}
impl<'a> PostIssuingDisputesDisputeRequest<'a> {
    pub async fn send(self) -> anyhow::Result<IssuingDispute> {
        let mut r = self
            .client
            .client
            .post(&format!("/v1/issuing/disputes/{dispute}", dispute = self.dispute));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct PostIssuingDisputesDisputeSubmitRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub dispute: String,
}
impl<'a> PostIssuingDisputesDisputeSubmitRequest<'a> {
    pub async fn send(self) -> anyhow::Result<IssuingDispute> {
        let mut r = self
            .client
            .client
            .post(
                &format!("/v1/issuing/disputes/{dispute}/submit", dispute = self.dispute),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetIssuingSettlementsRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub created: Option<serde_json::Value>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
}
impl<'a> GetIssuingSettlementsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v1/issuing/settlements");
        if let Some(ref unwrapped) = self.created {
            r = r.push_query("created", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn created(mut self, created: serde_json::Value) -> Self {
        self.created = Some(created);
        self
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
}
pub struct GetIssuingSettlementsSettlementRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub expand: Option<Vec<String>>,
    pub settlement: String,
}
impl<'a> GetIssuingSettlementsSettlementRequest<'a> {
    pub async fn send(self) -> anyhow::Result<IssuingSettlement> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/v1/issuing/settlements/{settlement}", settlement = self.settlement
                ),
            );
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct PostIssuingSettlementsSettlementRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub settlement: String,
}
impl<'a> PostIssuingSettlementsSettlementRequest<'a> {
    pub async fn send(self) -> anyhow::Result<IssuingSettlement> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/v1/issuing/settlements/{settlement}", settlement = self.settlement
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetIssuingTransactionsRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub card: Option<String>,
    pub cardholder: Option<String>,
    pub created: Option<serde_json::Value>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
    pub type_: Option<String>,
}
impl<'a> GetIssuingTransactionsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v1/issuing/transactions");
        if let Some(ref unwrapped) = self.card {
            r = r.push_query("card", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.cardholder {
            r = r.push_query("cardholder", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.created {
            r = r.push_query("created", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.type_ {
            r = r.push_query("type", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn card(mut self, card: String) -> Self {
        self.card = Some(card);
        self
    }
    pub fn cardholder(mut self, cardholder: String) -> Self {
        self.cardholder = Some(cardholder);
        self
    }
    pub fn created(mut self, created: serde_json::Value) -> Self {
        self.created = Some(created);
        self
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
    pub fn type_(mut self, type_: String) -> Self {
        self.type_ = Some(type_);
        self
    }
}
pub struct GetIssuingTransactionsTransactionRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub expand: Option<Vec<String>>,
    pub transaction: String,
}
impl<'a> GetIssuingTransactionsTransactionRequest<'a> {
    pub async fn send(self) -> anyhow::Result<IssuingTransaction> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/v1/issuing/transactions/{transaction}", transaction = self
                    .transaction
                ),
            );
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct PostIssuingTransactionsTransactionRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub transaction: String,
}
impl<'a> PostIssuingTransactionsTransactionRequest<'a> {
    pub async fn send(self) -> anyhow::Result<IssuingTransaction> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/v1/issuing/transactions/{transaction}", transaction = self
                    .transaction
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct PostLinkAccountSessionsRequest<'a> {
    pub(crate) client: &'a StripeClient,
}
impl<'a> PostLinkAccountSessionsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<FinancialConnectionsSession> {
        let mut r = self.client.client.post("/v1/link_account_sessions");
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetLinkAccountSessionsSessionRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub expand: Option<Vec<String>>,
    pub session: String,
}
impl<'a> GetLinkAccountSessionsSessionRequest<'a> {
    pub async fn send(self) -> anyhow::Result<FinancialConnectionsSession> {
        let mut r = self
            .client
            .client
            .get(
                &format!("/v1/link_account_sessions/{session}", session = self.session),
            );
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct GetLinkedAccountsRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub account_holder: Option<serde_json::Value>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub session: Option<String>,
    pub starting_after: Option<String>,
}
impl<'a> GetLinkedAccountsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v1/linked_accounts");
        if let Some(ref unwrapped) = self.account_holder {
            r = r.push_query("account_holder", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.session {
            r = r.push_query("session", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn account_holder(mut self, account_holder: serde_json::Value) -> Self {
        self.account_holder = Some(account_holder);
        self
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn session(mut self, session: String) -> Self {
        self.session = Some(session);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
}
pub struct GetLinkedAccountsAccountRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub account: String,
    pub expand: Option<Vec<String>>,
}
impl<'a> GetLinkedAccountsAccountRequest<'a> {
    pub async fn send(self) -> anyhow::Result<FinancialConnectionsAccount> {
        let mut r = self
            .client
            .client
            .get(&format!("/v1/linked_accounts/{account}", account = self.account));
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct PostLinkedAccountsAccountDisconnectRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub account: String,
}
impl<'a> PostLinkedAccountsAccountDisconnectRequest<'a> {
    pub async fn send(self) -> anyhow::Result<FinancialConnectionsAccount> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/v1/linked_accounts/{account}/disconnect", account = self.account
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetLinkedAccountsAccountOwnersRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub account: String,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub ownership: String,
    pub starting_after: Option<String>,
}
impl<'a> GetLinkedAccountsAccountOwnersRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .get(
                &format!("/v1/linked_accounts/{account}/owners", account = self.account),
            );
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        r = r.push_query("ownership", &self.ownership.to_string());
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
}
pub struct PostLinkedAccountsAccountRefreshRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub account: String,
}
impl<'a> PostLinkedAccountsAccountRefreshRequest<'a> {
    pub async fn send(self) -> anyhow::Result<FinancialConnectionsAccount> {
        let mut r = self
            .client
            .client
            .post(
                &format!("/v1/linked_accounts/{account}/refresh", account = self.account),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetMandatesMandateRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub expand: Option<Vec<String>>,
    pub mandate: String,
}
impl<'a> GetMandatesMandateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Mandate> {
        let mut r = self
            .client
            .client
            .get(&format!("/v1/mandates/{mandate}", mandate = self.mandate));
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct GetOrdersRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub customer: Option<String>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
}
impl<'a> GetOrdersRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v1/orders");
        if let Some(ref unwrapped) = self.customer {
            r = r.push_query("customer", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn customer(mut self, customer: String) -> Self {
        self.customer = Some(customer);
        self
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
}
pub struct PostOrdersRequest<'a> {
    pub(crate) client: &'a StripeClient,
}
impl<'a> PostOrdersRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Order> {
        let mut r = self.client.client.post("/v1/orders");
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetOrdersIdRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub expand: Option<Vec<String>>,
    pub id: String,
}
impl<'a> GetOrdersIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Order> {
        let mut r = self.client.client.get(&format!("/v1/orders/{id}", id = self.id));
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct PostOrdersIdRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub id: String,
}
impl<'a> PostOrdersIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Order> {
        let mut r = self.client.client.post(&format!("/v1/orders/{id}", id = self.id));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct PostOrdersIdCancelRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub id: String,
}
impl<'a> PostOrdersIdCancelRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Order> {
        let mut r = self
            .client
            .client
            .post(&format!("/v1/orders/{id}/cancel", id = self.id));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetOrdersIdLineItemsRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub id: String,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
}
impl<'a> GetOrdersIdLineItemsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .get(&format!("/v1/orders/{id}/line_items", id = self.id));
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
}
pub struct PostOrdersIdReopenRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub id: String,
}
impl<'a> PostOrdersIdReopenRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Order> {
        let mut r = self
            .client
            .client
            .post(&format!("/v1/orders/{id}/reopen", id = self.id));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct PostOrdersIdSubmitRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub id: String,
}
impl<'a> PostOrdersIdSubmitRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Order> {
        let mut r = self
            .client
            .client
            .post(&format!("/v1/orders/{id}/submit", id = self.id));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetPaymentIntentsRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub created: Option<serde_json::Value>,
    pub customer: Option<String>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
}
impl<'a> GetPaymentIntentsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v1/payment_intents");
        if let Some(ref unwrapped) = self.created {
            r = r.push_query("created", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.customer {
            r = r.push_query("customer", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn created(mut self, created: serde_json::Value) -> Self {
        self.created = Some(created);
        self
    }
    pub fn customer(mut self, customer: String) -> Self {
        self.customer = Some(customer);
        self
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
}
pub struct PostPaymentIntentsRequest<'a> {
    pub(crate) client: &'a StripeClient,
}
impl<'a> PostPaymentIntentsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<PaymentIntent> {
        let mut r = self.client.client.post("/v1/payment_intents");
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetPaymentIntentsSearchRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub page: Option<String>,
    pub query: String,
}
impl<'a> GetPaymentIntentsSearchRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v1/payment_intents/search");
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.page {
            r = r.push_query("page", &unwrapped.to_string());
        }
        r = r.push_query("query", &self.query.to_string());
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn page(mut self, page: String) -> Self {
        self.page = Some(page);
        self
    }
}
pub struct GetPaymentIntentsIntentRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub client_secret: Option<String>,
    pub expand: Option<Vec<String>>,
    pub intent: String,
}
impl<'a> GetPaymentIntentsIntentRequest<'a> {
    pub async fn send(self) -> anyhow::Result<PaymentIntent> {
        let mut r = self
            .client
            .client
            .get(&format!("/v1/payment_intents/{intent}", intent = self.intent));
        if let Some(ref unwrapped) = self.client_secret {
            r = r.push_query("client_secret", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn client_secret(mut self, client_secret: String) -> Self {
        self.client_secret = Some(client_secret);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct PostPaymentIntentsIntentRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub intent: String,
}
impl<'a> PostPaymentIntentsIntentRequest<'a> {
    pub async fn send(self) -> anyhow::Result<PaymentIntent> {
        let mut r = self
            .client
            .client
            .post(&format!("/v1/payment_intents/{intent}", intent = self.intent));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct PostPaymentIntentsIntentApplyCustomerBalanceRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub intent: String,
}
impl<'a> PostPaymentIntentsIntentApplyCustomerBalanceRequest<'a> {
    pub async fn send(self) -> anyhow::Result<PaymentIntent> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/v1/payment_intents/{intent}/apply_customer_balance", intent = self
                    .intent
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct PostPaymentIntentsIntentCancelRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub intent: String,
}
impl<'a> PostPaymentIntentsIntentCancelRequest<'a> {
    pub async fn send(self) -> anyhow::Result<PaymentIntent> {
        let mut r = self
            .client
            .client
            .post(&format!("/v1/payment_intents/{intent}/cancel", intent = self.intent));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct PostPaymentIntentsIntentCaptureRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub intent: String,
}
impl<'a> PostPaymentIntentsIntentCaptureRequest<'a> {
    pub async fn send(self) -> anyhow::Result<PaymentIntent> {
        let mut r = self
            .client
            .client
            .post(
                &format!("/v1/payment_intents/{intent}/capture", intent = self.intent),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct PostPaymentIntentsIntentConfirmRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub intent: String,
}
impl<'a> PostPaymentIntentsIntentConfirmRequest<'a> {
    pub async fn send(self) -> anyhow::Result<PaymentIntent> {
        let mut r = self
            .client
            .client
            .post(
                &format!("/v1/payment_intents/{intent}/confirm", intent = self.intent),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct PostPaymentIntentsIntentIncrementAuthorizationRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub intent: String,
}
impl<'a> PostPaymentIntentsIntentIncrementAuthorizationRequest<'a> {
    pub async fn send(self) -> anyhow::Result<PaymentIntent> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/v1/payment_intents/{intent}/increment_authorization", intent = self
                    .intent
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct PostPaymentIntentsIntentVerifyMicrodepositsRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub intent: String,
}
impl<'a> PostPaymentIntentsIntentVerifyMicrodepositsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<PaymentIntent> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/v1/payment_intents/{intent}/verify_microdeposits", intent = self
                    .intent
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetPaymentLinksRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub active: Option<bool>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
}
impl<'a> GetPaymentLinksRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v1/payment_links");
        if let Some(ref unwrapped) = self.active {
            r = r.push_query("active", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn active(mut self, active: bool) -> Self {
        self.active = Some(active);
        self
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
}
pub struct PostPaymentLinksRequest<'a> {
    pub(crate) client: &'a StripeClient,
}
impl<'a> PostPaymentLinksRequest<'a> {
    pub async fn send(self) -> anyhow::Result<PaymentLink> {
        let mut r = self.client.client.post("/v1/payment_links");
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetPaymentLinksPaymentLinkRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub expand: Option<Vec<String>>,
    pub payment_link: String,
}
impl<'a> GetPaymentLinksPaymentLinkRequest<'a> {
    pub async fn send(self) -> anyhow::Result<PaymentLink> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/v1/payment_links/{payment_link}", payment_link = self.payment_link
                ),
            );
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct PostPaymentLinksPaymentLinkRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub payment_link: String,
}
impl<'a> PostPaymentLinksPaymentLinkRequest<'a> {
    pub async fn send(self) -> anyhow::Result<PaymentLink> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/v1/payment_links/{payment_link}", payment_link = self.payment_link
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetPaymentLinksPaymentLinkLineItemsRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub payment_link: String,
    pub starting_after: Option<String>,
}
impl<'a> GetPaymentLinksPaymentLinkLineItemsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/v1/payment_links/{payment_link}/line_items", payment_link = self
                    .payment_link
                ),
            );
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
}
pub struct GetPaymentMethodsRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub customer: Option<String>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
    pub type_: String,
}
impl<'a> GetPaymentMethodsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v1/payment_methods");
        if let Some(ref unwrapped) = self.customer {
            r = r.push_query("customer", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        r = r.push_query("type", &self.type_.to_string());
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn customer(mut self, customer: String) -> Self {
        self.customer = Some(customer);
        self
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
}
pub struct PostPaymentMethodsRequest<'a> {
    pub(crate) client: &'a StripeClient,
}
impl<'a> PostPaymentMethodsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<PaymentMethod> {
        let mut r = self.client.client.post("/v1/payment_methods");
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetPaymentMethodsPaymentMethodRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub expand: Option<Vec<String>>,
    pub payment_method: String,
}
impl<'a> GetPaymentMethodsPaymentMethodRequest<'a> {
    pub async fn send(self) -> anyhow::Result<PaymentMethod> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/v1/payment_methods/{payment_method}", payment_method = self
                    .payment_method
                ),
            );
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct PostPaymentMethodsPaymentMethodRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub payment_method: String,
}
impl<'a> PostPaymentMethodsPaymentMethodRequest<'a> {
    pub async fn send(self) -> anyhow::Result<PaymentMethod> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/v1/payment_methods/{payment_method}", payment_method = self
                    .payment_method
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct PostPaymentMethodsPaymentMethodAttachRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub payment_method: String,
}
impl<'a> PostPaymentMethodsPaymentMethodAttachRequest<'a> {
    pub async fn send(self) -> anyhow::Result<PaymentMethod> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/v1/payment_methods/{payment_method}/attach", payment_method = self
                    .payment_method
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct PostPaymentMethodsPaymentMethodDetachRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub payment_method: String,
}
impl<'a> PostPaymentMethodsPaymentMethodDetachRequest<'a> {
    pub async fn send(self) -> anyhow::Result<PaymentMethod> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/v1/payment_methods/{payment_method}/detach", payment_method = self
                    .payment_method
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetPayoutsRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub arrival_date: Option<serde_json::Value>,
    pub created: Option<serde_json::Value>,
    pub destination: Option<String>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
    pub status: Option<String>,
}
impl<'a> GetPayoutsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v1/payouts");
        if let Some(ref unwrapped) = self.arrival_date {
            r = r.push_query("arrival_date", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.created {
            r = r.push_query("created", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.destination {
            r = r.push_query("destination", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.status {
            r = r.push_query("status", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn arrival_date(mut self, arrival_date: serde_json::Value) -> Self {
        self.arrival_date = Some(arrival_date);
        self
    }
    pub fn created(mut self, created: serde_json::Value) -> Self {
        self.created = Some(created);
        self
    }
    pub fn destination(mut self, destination: String) -> Self {
        self.destination = Some(destination);
        self
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
    pub fn status(mut self, status: String) -> Self {
        self.status = Some(status);
        self
    }
}
pub struct PostPayoutsRequest<'a> {
    pub(crate) client: &'a StripeClient,
}
impl<'a> PostPayoutsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Payout> {
        let mut r = self.client.client.post("/v1/payouts");
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetPayoutsPayoutRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub expand: Option<Vec<String>>,
    pub payout: String,
}
impl<'a> GetPayoutsPayoutRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Payout> {
        let mut r = self
            .client
            .client
            .get(&format!("/v1/payouts/{payout}", payout = self.payout));
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct PostPayoutsPayoutRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub payout: String,
}
impl<'a> PostPayoutsPayoutRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Payout> {
        let mut r = self
            .client
            .client
            .post(&format!("/v1/payouts/{payout}", payout = self.payout));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct PostPayoutsPayoutCancelRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub payout: String,
}
impl<'a> PostPayoutsPayoutCancelRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Payout> {
        let mut r = self
            .client
            .client
            .post(&format!("/v1/payouts/{payout}/cancel", payout = self.payout));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct PostPayoutsPayoutReverseRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub payout: String,
}
impl<'a> PostPayoutsPayoutReverseRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Payout> {
        let mut r = self
            .client
            .client
            .post(&format!("/v1/payouts/{payout}/reverse", payout = self.payout));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetPlansRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub active: Option<bool>,
    pub created: Option<serde_json::Value>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub product: Option<String>,
    pub starting_after: Option<String>,
}
impl<'a> GetPlansRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v1/plans");
        if let Some(ref unwrapped) = self.active {
            r = r.push_query("active", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.created {
            r = r.push_query("created", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.product {
            r = r.push_query("product", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn active(mut self, active: bool) -> Self {
        self.active = Some(active);
        self
    }
    pub fn created(mut self, created: serde_json::Value) -> Self {
        self.created = Some(created);
        self
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn product(mut self, product: String) -> Self {
        self.product = Some(product);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
}
pub struct PostPlansRequest<'a> {
    pub(crate) client: &'a StripeClient,
}
impl<'a> PostPlansRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Plan> {
        let mut r = self.client.client.post("/v1/plans");
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetPlansPlanRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub expand: Option<Vec<String>>,
    pub plan: String,
}
impl<'a> GetPlansPlanRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Plan> {
        let mut r = self
            .client
            .client
            .get(&format!("/v1/plans/{plan}", plan = self.plan));
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct PostPlansPlanRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub plan: String,
}
impl<'a> PostPlansPlanRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Plan> {
        let mut r = self
            .client
            .client
            .post(&format!("/v1/plans/{plan}", plan = self.plan));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct DeletePlansPlanRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub plan: String,
}
impl<'a> DeletePlansPlanRequest<'a> {
    pub async fn send(self) -> anyhow::Result<DeletedPlan> {
        let mut r = self
            .client
            .client
            .delete(&format!("/v1/plans/{plan}", plan = self.plan));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetPricesRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub active: Option<bool>,
    pub created: Option<serde_json::Value>,
    pub currency: Option<String>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub lookup_keys: Option<Vec<String>>,
    pub product: Option<String>,
    pub recurring: Option<serde_json::Value>,
    pub starting_after: Option<String>,
    pub type_: Option<String>,
}
impl<'a> GetPricesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v1/prices");
        if let Some(ref unwrapped) = self.active {
            r = r.push_query("active", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.created {
            r = r.push_query("created", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.currency {
            r = r.push_query("currency", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.lookup_keys {
            for item in unwrapped {
                r = r.push_query("lookup_keys[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.product {
            r = r.push_query("product", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.recurring {
            r = r.push_query("recurring", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.type_ {
            r = r.push_query("type", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn active(mut self, active: bool) -> Self {
        self.active = Some(active);
        self
    }
    pub fn created(mut self, created: serde_json::Value) -> Self {
        self.created = Some(created);
        self
    }
    pub fn currency(mut self, currency: String) -> Self {
        self.currency = Some(currency);
        self
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn lookup_keys(mut self, lookup_keys: Vec<String>) -> Self {
        self.lookup_keys = Some(lookup_keys);
        self
    }
    pub fn product(mut self, product: String) -> Self {
        self.product = Some(product);
        self
    }
    pub fn recurring(mut self, recurring: serde_json::Value) -> Self {
        self.recurring = Some(recurring);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
    pub fn type_(mut self, type_: String) -> Self {
        self.type_ = Some(type_);
        self
    }
}
pub struct PostPricesRequest<'a> {
    pub(crate) client: &'a StripeClient,
}
impl<'a> PostPricesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Price> {
        let mut r = self.client.client.post("/v1/prices");
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetPricesSearchRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub page: Option<String>,
    pub query: String,
}
impl<'a> GetPricesSearchRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v1/prices/search");
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.page {
            r = r.push_query("page", &unwrapped.to_string());
        }
        r = r.push_query("query", &self.query.to_string());
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn page(mut self, page: String) -> Self {
        self.page = Some(page);
        self
    }
}
pub struct GetPricesPriceRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub expand: Option<Vec<String>>,
    pub price: String,
}
impl<'a> GetPricesPriceRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Price> {
        let mut r = self
            .client
            .client
            .get(&format!("/v1/prices/{price}", price = self.price));
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct PostPricesPriceRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub price: String,
}
impl<'a> PostPricesPriceRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Price> {
        let mut r = self
            .client
            .client
            .post(&format!("/v1/prices/{price}", price = self.price));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetProductsRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub active: Option<bool>,
    pub created: Option<serde_json::Value>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub ids: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub shippable: Option<bool>,
    pub starting_after: Option<String>,
    pub url: Option<String>,
}
impl<'a> GetProductsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v1/products");
        if let Some(ref unwrapped) = self.active {
            r = r.push_query("active", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.created {
            r = r.push_query("created", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.ids {
            for item in unwrapped {
                r = r.push_query("ids[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.shippable {
            r = r.push_query("shippable", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.url {
            r = r.push_query("url", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn active(mut self, active: bool) -> Self {
        self.active = Some(active);
        self
    }
    pub fn created(mut self, created: serde_json::Value) -> Self {
        self.created = Some(created);
        self
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn ids(mut self, ids: Vec<String>) -> Self {
        self.ids = Some(ids);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn shippable(mut self, shippable: bool) -> Self {
        self.shippable = Some(shippable);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
    pub fn url(mut self, url: String) -> Self {
        self.url = Some(url);
        self
    }
}
pub struct PostProductsRequest<'a> {
    pub(crate) client: &'a StripeClient,
}
impl<'a> PostProductsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Product> {
        let mut r = self.client.client.post("/v1/products");
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetProductsSearchRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub page: Option<String>,
    pub query: String,
}
impl<'a> GetProductsSearchRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v1/products/search");
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.page {
            r = r.push_query("page", &unwrapped.to_string());
        }
        r = r.push_query("query", &self.query.to_string());
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn page(mut self, page: String) -> Self {
        self.page = Some(page);
        self
    }
}
pub struct GetProductsIdRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub expand: Option<Vec<String>>,
    pub id: String,
}
impl<'a> GetProductsIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Product> {
        let mut r = self.client.client.get(&format!("/v1/products/{id}", id = self.id));
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct PostProductsIdRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub id: String,
}
impl<'a> PostProductsIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Product> {
        let mut r = self.client.client.post(&format!("/v1/products/{id}", id = self.id));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct DeleteProductsIdRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub id: String,
}
impl<'a> DeleteProductsIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<DeletedProduct> {
        let mut r = self
            .client
            .client
            .delete(&format!("/v1/products/{id}", id = self.id));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetPromotionCodesRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub active: Option<bool>,
    pub code: Option<String>,
    pub coupon: Option<String>,
    pub created: Option<serde_json::Value>,
    pub customer: Option<String>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
}
impl<'a> GetPromotionCodesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v1/promotion_codes");
        if let Some(ref unwrapped) = self.active {
            r = r.push_query("active", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.code {
            r = r.push_query("code", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.coupon {
            r = r.push_query("coupon", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.created {
            r = r.push_query("created", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.customer {
            r = r.push_query("customer", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn active(mut self, active: bool) -> Self {
        self.active = Some(active);
        self
    }
    pub fn code(mut self, code: String) -> Self {
        self.code = Some(code);
        self
    }
    pub fn coupon(mut self, coupon: String) -> Self {
        self.coupon = Some(coupon);
        self
    }
    pub fn created(mut self, created: serde_json::Value) -> Self {
        self.created = Some(created);
        self
    }
    pub fn customer(mut self, customer: String) -> Self {
        self.customer = Some(customer);
        self
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
}
pub struct PostPromotionCodesRequest<'a> {
    pub(crate) client: &'a StripeClient,
}
impl<'a> PostPromotionCodesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<PromotionCode> {
        let mut r = self.client.client.post("/v1/promotion_codes");
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetPromotionCodesPromotionCodeRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub expand: Option<Vec<String>>,
    pub promotion_code: String,
}
impl<'a> GetPromotionCodesPromotionCodeRequest<'a> {
    pub async fn send(self) -> anyhow::Result<PromotionCode> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/v1/promotion_codes/{promotion_code}", promotion_code = self
                    .promotion_code
                ),
            );
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct PostPromotionCodesPromotionCodeRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub promotion_code: String,
}
impl<'a> PostPromotionCodesPromotionCodeRequest<'a> {
    pub async fn send(self) -> anyhow::Result<PromotionCode> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/v1/promotion_codes/{promotion_code}", promotion_code = self
                    .promotion_code
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetQuotesRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub customer: Option<String>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
    pub status: Option<String>,
    pub test_clock: Option<String>,
}
impl<'a> GetQuotesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v1/quotes");
        if let Some(ref unwrapped) = self.customer {
            r = r.push_query("customer", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.status {
            r = r.push_query("status", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.test_clock {
            r = r.push_query("test_clock", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn customer(mut self, customer: String) -> Self {
        self.customer = Some(customer);
        self
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
    pub fn status(mut self, status: String) -> Self {
        self.status = Some(status);
        self
    }
    pub fn test_clock(mut self, test_clock: String) -> Self {
        self.test_clock = Some(test_clock);
        self
    }
}
pub struct PostQuotesRequest<'a> {
    pub(crate) client: &'a StripeClient,
}
impl<'a> PostQuotesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Quote> {
        let mut r = self.client.client.post("/v1/quotes");
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetQuotesQuoteRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub expand: Option<Vec<String>>,
    pub quote: String,
}
impl<'a> GetQuotesQuoteRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Quote> {
        let mut r = self
            .client
            .client
            .get(&format!("/v1/quotes/{quote}", quote = self.quote));
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct PostQuotesQuoteRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub quote: String,
}
impl<'a> PostQuotesQuoteRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Quote> {
        let mut r = self
            .client
            .client
            .post(&format!("/v1/quotes/{quote}", quote = self.quote));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct PostQuotesQuoteAcceptRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub quote: String,
}
impl<'a> PostQuotesQuoteAcceptRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Quote> {
        let mut r = self
            .client
            .client
            .post(&format!("/v1/quotes/{quote}/accept", quote = self.quote));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct PostQuotesQuoteCancelRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub quote: String,
}
impl<'a> PostQuotesQuoteCancelRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Quote> {
        let mut r = self
            .client
            .client
            .post(&format!("/v1/quotes/{quote}/cancel", quote = self.quote));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetQuotesQuoteComputedUpfrontLineItemsRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub quote: String,
    pub starting_after: Option<String>,
}
impl<'a> GetQuotesQuoteComputedUpfrontLineItemsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/v1/quotes/{quote}/computed_upfront_line_items", quote = self.quote
                ),
            );
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
}
pub struct PostQuotesQuoteFinalizeRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub quote: String,
}
impl<'a> PostQuotesQuoteFinalizeRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Quote> {
        let mut r = self
            .client
            .client
            .post(&format!("/v1/quotes/{quote}/finalize", quote = self.quote));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetQuotesQuoteLineItemsRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub quote: String,
    pub starting_after: Option<String>,
}
impl<'a> GetQuotesQuoteLineItemsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .get(&format!("/v1/quotes/{quote}/line_items", quote = self.quote));
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
}
pub struct GetRadarEarlyFraudWarningsRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub charge: Option<String>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub payment_intent: Option<String>,
    pub starting_after: Option<String>,
}
impl<'a> GetRadarEarlyFraudWarningsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v1/radar/early_fraud_warnings");
        if let Some(ref unwrapped) = self.charge {
            r = r.push_query("charge", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.payment_intent {
            r = r.push_query("payment_intent", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn charge(mut self, charge: String) -> Self {
        self.charge = Some(charge);
        self
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn payment_intent(mut self, payment_intent: String) -> Self {
        self.payment_intent = Some(payment_intent);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
}
pub struct GetRadarEarlyFraudWarningsEarlyFraudWarningRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub early_fraud_warning: String,
    pub expand: Option<Vec<String>>,
}
impl<'a> GetRadarEarlyFraudWarningsEarlyFraudWarningRequest<'a> {
    pub async fn send(self) -> anyhow::Result<RadarEarlyFraudWarning> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/v1/radar/early_fraud_warnings/{early_fraud_warning}",
                    early_fraud_warning = self.early_fraud_warning
                ),
            );
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct GetRadarValueListItemsRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub created: Option<serde_json::Value>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
    pub value: Option<String>,
    pub value_list: String,
}
impl<'a> GetRadarValueListItemsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v1/radar/value_list_items");
        if let Some(ref unwrapped) = self.created {
            r = r.push_query("created", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.value {
            r = r.push_query("value", &unwrapped.to_string());
        }
        r = r.push_query("value_list", &self.value_list.to_string());
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn created(mut self, created: serde_json::Value) -> Self {
        self.created = Some(created);
        self
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
    pub fn value(mut self, value: String) -> Self {
        self.value = Some(value);
        self
    }
}
pub struct PostRadarValueListItemsRequest<'a> {
    pub(crate) client: &'a StripeClient,
}
impl<'a> PostRadarValueListItemsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<RadarValueListItem> {
        let mut r = self.client.client.post("/v1/radar/value_list_items");
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetRadarValueListItemsItemRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub expand: Option<Vec<String>>,
    pub item: String,
}
impl<'a> GetRadarValueListItemsItemRequest<'a> {
    pub async fn send(self) -> anyhow::Result<RadarValueListItem> {
        let mut r = self
            .client
            .client
            .get(&format!("/v1/radar/value_list_items/{item}", item = self.item));
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct DeleteRadarValueListItemsItemRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub item: String,
}
impl<'a> DeleteRadarValueListItemsItemRequest<'a> {
    pub async fn send(self) -> anyhow::Result<DeletedRadarValueListItem> {
        let mut r = self
            .client
            .client
            .delete(&format!("/v1/radar/value_list_items/{item}", item = self.item));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetRadarValueListsRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub alias: Option<String>,
    pub contains: Option<String>,
    pub created: Option<serde_json::Value>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
}
impl<'a> GetRadarValueListsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v1/radar/value_lists");
        if let Some(ref unwrapped) = self.alias {
            r = r.push_query("alias", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.contains {
            r = r.push_query("contains", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.created {
            r = r.push_query("created", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn alias(mut self, alias: String) -> Self {
        self.alias = Some(alias);
        self
    }
    pub fn contains(mut self, contains: String) -> Self {
        self.contains = Some(contains);
        self
    }
    pub fn created(mut self, created: serde_json::Value) -> Self {
        self.created = Some(created);
        self
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
}
pub struct PostRadarValueListsRequest<'a> {
    pub(crate) client: &'a StripeClient,
}
impl<'a> PostRadarValueListsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<RadarValueList> {
        let mut r = self.client.client.post("/v1/radar/value_lists");
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetRadarValueListsValueListRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub expand: Option<Vec<String>>,
    pub value_list: String,
}
impl<'a> GetRadarValueListsValueListRequest<'a> {
    pub async fn send(self) -> anyhow::Result<RadarValueList> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/v1/radar/value_lists/{value_list}", value_list = self.value_list
                ),
            );
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct PostRadarValueListsValueListRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub value_list: String,
}
impl<'a> PostRadarValueListsValueListRequest<'a> {
    pub async fn send(self) -> anyhow::Result<RadarValueList> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/v1/radar/value_lists/{value_list}", value_list = self.value_list
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct DeleteRadarValueListsValueListRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub value_list: String,
}
impl<'a> DeleteRadarValueListsValueListRequest<'a> {
    pub async fn send(self) -> anyhow::Result<DeletedRadarValueList> {
        let mut r = self
            .client
            .client
            .delete(
                &format!(
                    "/v1/radar/value_lists/{value_list}", value_list = self.value_list
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetRefundsRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub charge: Option<String>,
    pub created: Option<serde_json::Value>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub payment_intent: Option<String>,
    pub starting_after: Option<String>,
}
impl<'a> GetRefundsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v1/refunds");
        if let Some(ref unwrapped) = self.charge {
            r = r.push_query("charge", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.created {
            r = r.push_query("created", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.payment_intent {
            r = r.push_query("payment_intent", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn charge(mut self, charge: String) -> Self {
        self.charge = Some(charge);
        self
    }
    pub fn created(mut self, created: serde_json::Value) -> Self {
        self.created = Some(created);
        self
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn payment_intent(mut self, payment_intent: String) -> Self {
        self.payment_intent = Some(payment_intent);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
}
pub struct PostRefundsRequest<'a> {
    pub(crate) client: &'a StripeClient,
}
impl<'a> PostRefundsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Refund> {
        let mut r = self.client.client.post("/v1/refunds");
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetRefundsRefundRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub expand: Option<Vec<String>>,
    pub refund: String,
}
impl<'a> GetRefundsRefundRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Refund> {
        let mut r = self
            .client
            .client
            .get(&format!("/v1/refunds/{refund}", refund = self.refund));
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct PostRefundsRefundRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub refund: String,
}
impl<'a> PostRefundsRefundRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Refund> {
        let mut r = self
            .client
            .client
            .post(&format!("/v1/refunds/{refund}", refund = self.refund));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct PostRefundsRefundCancelRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub refund: String,
}
impl<'a> PostRefundsRefundCancelRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Refund> {
        let mut r = self
            .client
            .client
            .post(&format!("/v1/refunds/{refund}/cancel", refund = self.refund));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetReportingReportRunsRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub created: Option<serde_json::Value>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
}
impl<'a> GetReportingReportRunsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v1/reporting/report_runs");
        if let Some(ref unwrapped) = self.created {
            r = r.push_query("created", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn created(mut self, created: serde_json::Value) -> Self {
        self.created = Some(created);
        self
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
}
pub struct PostReportingReportRunsRequest<'a> {
    pub(crate) client: &'a StripeClient,
}
impl<'a> PostReportingReportRunsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ReportingReportRun> {
        let mut r = self.client.client.post("/v1/reporting/report_runs");
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetReportingReportRunsReportRunRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub expand: Option<Vec<String>>,
    pub report_run: String,
}
impl<'a> GetReportingReportRunsReportRunRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ReportingReportRun> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/v1/reporting/report_runs/{report_run}", report_run = self
                    .report_run
                ),
            );
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct GetReportingReportTypesRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub expand: Option<Vec<String>>,
}
impl<'a> GetReportingReportTypesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v1/reporting/report_types");
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct GetReportingReportTypesReportTypeRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub expand: Option<Vec<String>>,
    pub report_type: String,
}
impl<'a> GetReportingReportTypesReportTypeRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ReportingReportType> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/v1/reporting/report_types/{report_type}", report_type = self
                    .report_type
                ),
            );
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct GetReviewsRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub created: Option<serde_json::Value>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
}
impl<'a> GetReviewsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v1/reviews");
        if let Some(ref unwrapped) = self.created {
            r = r.push_query("created", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn created(mut self, created: serde_json::Value) -> Self {
        self.created = Some(created);
        self
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
}
pub struct GetReviewsReviewRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub expand: Option<Vec<String>>,
    pub review: String,
}
impl<'a> GetReviewsReviewRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Review> {
        let mut r = self
            .client
            .client
            .get(&format!("/v1/reviews/{review}", review = self.review));
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct PostReviewsReviewApproveRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub review: String,
}
impl<'a> PostReviewsReviewApproveRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Review> {
        let mut r = self
            .client
            .client
            .post(&format!("/v1/reviews/{review}/approve", review = self.review));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetSetupAttemptsRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub created: Option<serde_json::Value>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub setup_intent: String,
    pub starting_after: Option<String>,
}
impl<'a> GetSetupAttemptsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v1/setup_attempts");
        if let Some(ref unwrapped) = self.created {
            r = r.push_query("created", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        r = r.push_query("setup_intent", &self.setup_intent.to_string());
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn created(mut self, created: serde_json::Value) -> Self {
        self.created = Some(created);
        self
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
}
pub struct GetSetupIntentsRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub attach_to_self: Option<bool>,
    pub created: Option<serde_json::Value>,
    pub customer: Option<String>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub payment_method: Option<String>,
    pub starting_after: Option<String>,
}
impl<'a> GetSetupIntentsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v1/setup_intents");
        if let Some(ref unwrapped) = self.attach_to_self {
            r = r.push_query("attach_to_self", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.created {
            r = r.push_query("created", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.customer {
            r = r.push_query("customer", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.payment_method {
            r = r.push_query("payment_method", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn attach_to_self(mut self, attach_to_self: bool) -> Self {
        self.attach_to_self = Some(attach_to_self);
        self
    }
    pub fn created(mut self, created: serde_json::Value) -> Self {
        self.created = Some(created);
        self
    }
    pub fn customer(mut self, customer: String) -> Self {
        self.customer = Some(customer);
        self
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn payment_method(mut self, payment_method: String) -> Self {
        self.payment_method = Some(payment_method);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
}
pub struct PostSetupIntentsRequest<'a> {
    pub(crate) client: &'a StripeClient,
}
impl<'a> PostSetupIntentsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<SetupIntent> {
        let mut r = self.client.client.post("/v1/setup_intents");
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetSetupIntentsIntentRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub client_secret: Option<String>,
    pub expand: Option<Vec<String>>,
    pub intent: String,
}
impl<'a> GetSetupIntentsIntentRequest<'a> {
    pub async fn send(self) -> anyhow::Result<SetupIntent> {
        let mut r = self
            .client
            .client
            .get(&format!("/v1/setup_intents/{intent}", intent = self.intent));
        if let Some(ref unwrapped) = self.client_secret {
            r = r.push_query("client_secret", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn client_secret(mut self, client_secret: String) -> Self {
        self.client_secret = Some(client_secret);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct PostSetupIntentsIntentRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub intent: String,
}
impl<'a> PostSetupIntentsIntentRequest<'a> {
    pub async fn send(self) -> anyhow::Result<SetupIntent> {
        let mut r = self
            .client
            .client
            .post(&format!("/v1/setup_intents/{intent}", intent = self.intent));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct PostSetupIntentsIntentCancelRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub intent: String,
}
impl<'a> PostSetupIntentsIntentCancelRequest<'a> {
    pub async fn send(self) -> anyhow::Result<SetupIntent> {
        let mut r = self
            .client
            .client
            .post(&format!("/v1/setup_intents/{intent}/cancel", intent = self.intent));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct PostSetupIntentsIntentConfirmRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub intent: String,
}
impl<'a> PostSetupIntentsIntentConfirmRequest<'a> {
    pub async fn send(self) -> anyhow::Result<SetupIntent> {
        let mut r = self
            .client
            .client
            .post(&format!("/v1/setup_intents/{intent}/confirm", intent = self.intent));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct PostSetupIntentsIntentVerifyMicrodepositsRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub intent: String,
}
impl<'a> PostSetupIntentsIntentVerifyMicrodepositsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<SetupIntent> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/v1/setup_intents/{intent}/verify_microdeposits", intent = self
                    .intent
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetShippingRatesRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub active: Option<bool>,
    pub created: Option<serde_json::Value>,
    pub currency: Option<String>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
}
impl<'a> GetShippingRatesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v1/shipping_rates");
        if let Some(ref unwrapped) = self.active {
            r = r.push_query("active", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.created {
            r = r.push_query("created", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.currency {
            r = r.push_query("currency", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn active(mut self, active: bool) -> Self {
        self.active = Some(active);
        self
    }
    pub fn created(mut self, created: serde_json::Value) -> Self {
        self.created = Some(created);
        self
    }
    pub fn currency(mut self, currency: String) -> Self {
        self.currency = Some(currency);
        self
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
}
pub struct PostShippingRatesRequest<'a> {
    pub(crate) client: &'a StripeClient,
}
impl<'a> PostShippingRatesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ShippingRate> {
        let mut r = self.client.client.post("/v1/shipping_rates");
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetShippingRatesShippingRateTokenRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub expand: Option<Vec<String>>,
    pub shipping_rate_token: String,
}
impl<'a> GetShippingRatesShippingRateTokenRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ShippingRate> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/v1/shipping_rates/{shipping_rate_token}", shipping_rate_token =
                    self.shipping_rate_token
                ),
            );
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct PostShippingRatesShippingRateTokenRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub shipping_rate_token: String,
}
impl<'a> PostShippingRatesShippingRateTokenRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ShippingRate> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/v1/shipping_rates/{shipping_rate_token}", shipping_rate_token =
                    self.shipping_rate_token
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetSigmaScheduledQueryRunsRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
}
impl<'a> GetSigmaScheduledQueryRunsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v1/sigma/scheduled_query_runs");
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
}
pub struct GetSigmaScheduledQueryRunsScheduledQueryRunRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub expand: Option<Vec<String>>,
    pub scheduled_query_run: String,
}
impl<'a> GetSigmaScheduledQueryRunsScheduledQueryRunRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ScheduledQueryRun> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/v1/sigma/scheduled_query_runs/{scheduled_query_run}",
                    scheduled_query_run = self.scheduled_query_run
                ),
            );
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct GetSkusRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub active: Option<bool>,
    pub attributes: Option<serde_json::Value>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub ids: Option<Vec<String>>,
    pub in_stock: Option<bool>,
    pub limit: Option<i64>,
    pub product: Option<String>,
    pub starting_after: Option<String>,
}
impl<'a> GetSkusRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v1/skus");
        if let Some(ref unwrapped) = self.active {
            r = r.push_query("active", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.attributes {
            r = r.push_query("attributes", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.ids {
            for item in unwrapped {
                r = r.push_query("ids[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.in_stock {
            r = r.push_query("in_stock", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.product {
            r = r.push_query("product", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn active(mut self, active: bool) -> Self {
        self.active = Some(active);
        self
    }
    pub fn attributes(mut self, attributes: serde_json::Value) -> Self {
        self.attributes = Some(attributes);
        self
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn ids(mut self, ids: Vec<String>) -> Self {
        self.ids = Some(ids);
        self
    }
    pub fn in_stock(mut self, in_stock: bool) -> Self {
        self.in_stock = Some(in_stock);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn product(mut self, product: String) -> Self {
        self.product = Some(product);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
}
pub struct PostSkusRequest<'a> {
    pub(crate) client: &'a StripeClient,
}
impl<'a> PostSkusRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Sku> {
        let mut r = self.client.client.post("/v1/skus");
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetSkusIdRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub expand: Option<Vec<String>>,
    pub id: String,
}
impl<'a> GetSkusIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get(&format!("/v1/skus/{id}", id = self.id));
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct PostSkusIdRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub id: String,
}
impl<'a> PostSkusIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Sku> {
        let mut r = self.client.client.post(&format!("/v1/skus/{id}", id = self.id));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct DeleteSkusIdRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub id: String,
}
impl<'a> DeleteSkusIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<DeletedSku> {
        let mut r = self.client.client.delete(&format!("/v1/skus/{id}", id = self.id));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct PostSourcesRequest<'a> {
    pub(crate) client: &'a StripeClient,
}
impl<'a> PostSourcesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Source> {
        let mut r = self.client.client.post("/v1/sources");
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetSourcesSourceRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub client_secret: Option<String>,
    pub expand: Option<Vec<String>>,
    pub source: String,
}
impl<'a> GetSourcesSourceRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Source> {
        let mut r = self
            .client
            .client
            .get(&format!("/v1/sources/{source}", source = self.source));
        if let Some(ref unwrapped) = self.client_secret {
            r = r.push_query("client_secret", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn client_secret(mut self, client_secret: String) -> Self {
        self.client_secret = Some(client_secret);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct PostSourcesSourceRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub source: String,
}
impl<'a> PostSourcesSourceRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Source> {
        let mut r = self
            .client
            .client
            .post(&format!("/v1/sources/{source}", source = self.source));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetSourcesSourceMandateNotificationsMandateNotificationRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub expand: Option<Vec<String>>,
    pub mandate_notification: String,
    pub source: String,
}
impl<'a> GetSourcesSourceMandateNotificationsMandateNotificationRequest<'a> {
    pub async fn send(self) -> anyhow::Result<SourceMandateNotification> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/v1/sources/{source}/mandate_notifications/{mandate_notification}",
                    mandate_notification = self.mandate_notification, source = self
                    .source
                ),
            );
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct GetSourcesSourceSourceTransactionsRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub source: String,
    pub starting_after: Option<String>,
}
impl<'a> GetSourcesSourceSourceTransactionsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/v1/sources/{source}/source_transactions", source = self.source
                ),
            );
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
}
pub struct GetSourcesSourceSourceTransactionsSourceTransactionRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub expand: Option<Vec<String>>,
    pub source: String,
    pub source_transaction: String,
}
impl<'a> GetSourcesSourceSourceTransactionsSourceTransactionRequest<'a> {
    pub async fn send(self) -> anyhow::Result<SourceTransaction> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/v1/sources/{source}/source_transactions/{source_transaction}",
                    source = self.source, source_transaction = self.source_transaction
                ),
            );
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct PostSourcesSourceVerifyRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub source: String,
}
impl<'a> PostSourcesSourceVerifyRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Source> {
        let mut r = self
            .client
            .client
            .post(&format!("/v1/sources/{source}/verify", source = self.source));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetSubscriptionItemsRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
    pub subscription: String,
}
impl<'a> GetSubscriptionItemsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v1/subscription_items");
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        r = r.push_query("subscription", &self.subscription.to_string());
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
}
pub struct PostSubscriptionItemsRequest<'a> {
    pub(crate) client: &'a StripeClient,
}
impl<'a> PostSubscriptionItemsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<SubscriptionItem> {
        let mut r = self.client.client.post("/v1/subscription_items");
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetSubscriptionItemsItemRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub expand: Option<Vec<String>>,
    pub item: String,
}
impl<'a> GetSubscriptionItemsItemRequest<'a> {
    pub async fn send(self) -> anyhow::Result<SubscriptionItem> {
        let mut r = self
            .client
            .client
            .get(&format!("/v1/subscription_items/{item}", item = self.item));
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct PostSubscriptionItemsItemRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub item: String,
}
impl<'a> PostSubscriptionItemsItemRequest<'a> {
    pub async fn send(self) -> anyhow::Result<SubscriptionItem> {
        let mut r = self
            .client
            .client
            .post(&format!("/v1/subscription_items/{item}", item = self.item));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct DeleteSubscriptionItemsItemRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub item: String,
}
impl<'a> DeleteSubscriptionItemsItemRequest<'a> {
    pub async fn send(self) -> anyhow::Result<DeletedSubscriptionItem> {
        let mut r = self
            .client
            .client
            .delete(&format!("/v1/subscription_items/{item}", item = self.item));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetSubscriptionItemsSubscriptionItemUsageRecordSummariesRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
    pub subscription_item: String,
}
impl<'a> GetSubscriptionItemsSubscriptionItemUsageRecordSummariesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/v1/subscription_items/{subscription_item}/usage_record_summaries",
                    subscription_item = self.subscription_item
                ),
            );
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
}
pub struct PostSubscriptionItemsSubscriptionItemUsageRecordsRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub subscription_item: String,
}
impl<'a> PostSubscriptionItemsSubscriptionItemUsageRecordsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<UsageRecord> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/v1/subscription_items/{subscription_item}/usage_records",
                    subscription_item = self.subscription_item
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetSubscriptionSchedulesRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub canceled_at: Option<serde_json::Value>,
    pub completed_at: Option<serde_json::Value>,
    pub created: Option<serde_json::Value>,
    pub customer: Option<String>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub released_at: Option<serde_json::Value>,
    pub scheduled: Option<bool>,
    pub starting_after: Option<String>,
}
impl<'a> GetSubscriptionSchedulesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v1/subscription_schedules");
        if let Some(ref unwrapped) = self.canceled_at {
            r = r.push_query("canceled_at", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.completed_at {
            r = r.push_query("completed_at", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.created {
            r = r.push_query("created", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.customer {
            r = r.push_query("customer", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.released_at {
            r = r.push_query("released_at", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.scheduled {
            r = r.push_query("scheduled", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn canceled_at(mut self, canceled_at: serde_json::Value) -> Self {
        self.canceled_at = Some(canceled_at);
        self
    }
    pub fn completed_at(mut self, completed_at: serde_json::Value) -> Self {
        self.completed_at = Some(completed_at);
        self
    }
    pub fn created(mut self, created: serde_json::Value) -> Self {
        self.created = Some(created);
        self
    }
    pub fn customer(mut self, customer: String) -> Self {
        self.customer = Some(customer);
        self
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn released_at(mut self, released_at: serde_json::Value) -> Self {
        self.released_at = Some(released_at);
        self
    }
    pub fn scheduled(mut self, scheduled: bool) -> Self {
        self.scheduled = Some(scheduled);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
}
pub struct PostSubscriptionSchedulesRequest<'a> {
    pub(crate) client: &'a StripeClient,
}
impl<'a> PostSubscriptionSchedulesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<SubscriptionSchedule> {
        let mut r = self.client.client.post("/v1/subscription_schedules");
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetSubscriptionSchedulesScheduleRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub expand: Option<Vec<String>>,
    pub schedule: String,
}
impl<'a> GetSubscriptionSchedulesScheduleRequest<'a> {
    pub async fn send(self) -> anyhow::Result<SubscriptionSchedule> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/v1/subscription_schedules/{schedule}", schedule = self.schedule
                ),
            );
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct PostSubscriptionSchedulesScheduleRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub schedule: String,
}
impl<'a> PostSubscriptionSchedulesScheduleRequest<'a> {
    pub async fn send(self) -> anyhow::Result<SubscriptionSchedule> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/v1/subscription_schedules/{schedule}", schedule = self.schedule
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct PostSubscriptionSchedulesScheduleCancelRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub schedule: String,
}
impl<'a> PostSubscriptionSchedulesScheduleCancelRequest<'a> {
    pub async fn send(self) -> anyhow::Result<SubscriptionSchedule> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/v1/subscription_schedules/{schedule}/cancel", schedule = self
                    .schedule
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct PostSubscriptionSchedulesScheduleReleaseRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub schedule: String,
}
impl<'a> PostSubscriptionSchedulesScheduleReleaseRequest<'a> {
    pub async fn send(self) -> anyhow::Result<SubscriptionSchedule> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/v1/subscription_schedules/{schedule}/release", schedule = self
                    .schedule
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetSubscriptionsRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub collection_method: Option<String>,
    pub created: Option<serde_json::Value>,
    pub current_period_end: Option<serde_json::Value>,
    pub current_period_start: Option<serde_json::Value>,
    pub customer: Option<String>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub price: Option<String>,
    pub starting_after: Option<String>,
    pub status: Option<String>,
    pub test_clock: Option<String>,
}
impl<'a> GetSubscriptionsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v1/subscriptions");
        if let Some(ref unwrapped) = self.collection_method {
            r = r.push_query("collection_method", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.created {
            r = r.push_query("created", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.current_period_end {
            r = r.push_query("current_period_end", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.current_period_start {
            r = r.push_query("current_period_start", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.customer {
            r = r.push_query("customer", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.price {
            r = r.push_query("price", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.status {
            r = r.push_query("status", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.test_clock {
            r = r.push_query("test_clock", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn collection_method(mut self, collection_method: String) -> Self {
        self.collection_method = Some(collection_method);
        self
    }
    pub fn created(mut self, created: serde_json::Value) -> Self {
        self.created = Some(created);
        self
    }
    pub fn current_period_end(mut self, current_period_end: serde_json::Value) -> Self {
        self.current_period_end = Some(current_period_end);
        self
    }
    pub fn current_period_start(
        mut self,
        current_period_start: serde_json::Value,
    ) -> Self {
        self.current_period_start = Some(current_period_start);
        self
    }
    pub fn customer(mut self, customer: String) -> Self {
        self.customer = Some(customer);
        self
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn price(mut self, price: String) -> Self {
        self.price = Some(price);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
    pub fn status(mut self, status: String) -> Self {
        self.status = Some(status);
        self
    }
    pub fn test_clock(mut self, test_clock: String) -> Self {
        self.test_clock = Some(test_clock);
        self
    }
}
pub struct PostSubscriptionsRequest<'a> {
    pub(crate) client: &'a StripeClient,
}
impl<'a> PostSubscriptionsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Subscription> {
        let mut r = self.client.client.post("/v1/subscriptions");
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetSubscriptionsSearchRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub page: Option<String>,
    pub query: String,
}
impl<'a> GetSubscriptionsSearchRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v1/subscriptions/search");
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.page {
            r = r.push_query("page", &unwrapped.to_string());
        }
        r = r.push_query("query", &self.query.to_string());
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn page(mut self, page: String) -> Self {
        self.page = Some(page);
        self
    }
}
pub struct GetSubscriptionsSubscriptionExposedIdRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub expand: Option<Vec<String>>,
    pub subscription_exposed_id: String,
}
impl<'a> GetSubscriptionsSubscriptionExposedIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Subscription> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/v1/subscriptions/{subscription_exposed_id}",
                    subscription_exposed_id = self.subscription_exposed_id
                ),
            );
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct PostSubscriptionsSubscriptionExposedIdRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub subscription_exposed_id: String,
}
impl<'a> PostSubscriptionsSubscriptionExposedIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Subscription> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/v1/subscriptions/{subscription_exposed_id}",
                    subscription_exposed_id = self.subscription_exposed_id
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct DeleteSubscriptionsSubscriptionExposedIdRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub subscription_exposed_id: String,
}
impl<'a> DeleteSubscriptionsSubscriptionExposedIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Subscription> {
        let mut r = self
            .client
            .client
            .delete(
                &format!(
                    "/v1/subscriptions/{subscription_exposed_id}",
                    subscription_exposed_id = self.subscription_exposed_id
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct DeleteSubscriptionsSubscriptionExposedIdDiscountRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub subscription_exposed_id: String,
}
impl<'a> DeleteSubscriptionsSubscriptionExposedIdDiscountRequest<'a> {
    pub async fn send(self) -> anyhow::Result<DeletedDiscount> {
        let mut r = self
            .client
            .client
            .delete(
                &format!(
                    "/v1/subscriptions/{subscription_exposed_id}/discount",
                    subscription_exposed_id = self.subscription_exposed_id
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetTaxCodesRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
}
impl<'a> GetTaxCodesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v1/tax_codes");
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
}
pub struct GetTaxCodesIdRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub expand: Option<Vec<String>>,
    pub id: String,
}
impl<'a> GetTaxCodesIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TaxCode> {
        let mut r = self.client.client.get(&format!("/v1/tax_codes/{id}", id = self.id));
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct GetTaxRatesRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub active: Option<bool>,
    pub created: Option<serde_json::Value>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub inclusive: Option<bool>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
}
impl<'a> GetTaxRatesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v1/tax_rates");
        if let Some(ref unwrapped) = self.active {
            r = r.push_query("active", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.created {
            r = r.push_query("created", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.inclusive {
            r = r.push_query("inclusive", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn active(mut self, active: bool) -> Self {
        self.active = Some(active);
        self
    }
    pub fn created(mut self, created: serde_json::Value) -> Self {
        self.created = Some(created);
        self
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn inclusive(mut self, inclusive: bool) -> Self {
        self.inclusive = Some(inclusive);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
}
pub struct PostTaxRatesRequest<'a> {
    pub(crate) client: &'a StripeClient,
}
impl<'a> PostTaxRatesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TaxRate> {
        let mut r = self.client.client.post("/v1/tax_rates");
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetTaxRatesTaxRateRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub expand: Option<Vec<String>>,
    pub tax_rate: String,
}
impl<'a> GetTaxRatesTaxRateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TaxRate> {
        let mut r = self
            .client
            .client
            .get(&format!("/v1/tax_rates/{tax_rate}", tax_rate = self.tax_rate));
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct PostTaxRatesTaxRateRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub tax_rate: String,
}
impl<'a> PostTaxRatesTaxRateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TaxRate> {
        let mut r = self
            .client
            .client
            .post(&format!("/v1/tax_rates/{tax_rate}", tax_rate = self.tax_rate));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetTerminalConfigurationsRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub is_account_default: Option<bool>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
}
impl<'a> GetTerminalConfigurationsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v1/terminal/configurations");
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.is_account_default {
            r = r.push_query("is_account_default", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn is_account_default(mut self, is_account_default: bool) -> Self {
        self.is_account_default = Some(is_account_default);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
}
pub struct PostTerminalConfigurationsRequest<'a> {
    pub(crate) client: &'a StripeClient,
}
impl<'a> PostTerminalConfigurationsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TerminalConfiguration> {
        let mut r = self.client.client.post("/v1/terminal/configurations");
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetTerminalConfigurationsConfigurationRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub configuration: String,
    pub expand: Option<Vec<String>>,
}
impl<'a> GetTerminalConfigurationsConfigurationRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/v1/terminal/configurations/{configuration}", configuration = self
                    .configuration
                ),
            );
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct PostTerminalConfigurationsConfigurationRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub configuration: String,
}
impl<'a> PostTerminalConfigurationsConfigurationRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/v1/terminal/configurations/{configuration}", configuration = self
                    .configuration
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct DeleteTerminalConfigurationsConfigurationRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub configuration: String,
}
impl<'a> DeleteTerminalConfigurationsConfigurationRequest<'a> {
    pub async fn send(self) -> anyhow::Result<DeletedTerminalConfiguration> {
        let mut r = self
            .client
            .client
            .delete(
                &format!(
                    "/v1/terminal/configurations/{configuration}", configuration = self
                    .configuration
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct PostTerminalConnectionTokensRequest<'a> {
    pub(crate) client: &'a StripeClient,
}
impl<'a> PostTerminalConnectionTokensRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TerminalConnectionToken> {
        let mut r = self.client.client.post("/v1/terminal/connection_tokens");
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetTerminalLocationsRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
}
impl<'a> GetTerminalLocationsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v1/terminal/locations");
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
}
pub struct PostTerminalLocationsRequest<'a> {
    pub(crate) client: &'a StripeClient,
}
impl<'a> PostTerminalLocationsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TerminalLocation> {
        let mut r = self.client.client.post("/v1/terminal/locations");
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetTerminalLocationsLocationRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub expand: Option<Vec<String>>,
    pub location: String,
}
impl<'a> GetTerminalLocationsLocationRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .get(
                &format!("/v1/terminal/locations/{location}", location = self.location),
            );
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct PostTerminalLocationsLocationRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub location: String,
}
impl<'a> PostTerminalLocationsLocationRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .post(
                &format!("/v1/terminal/locations/{location}", location = self.location),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct DeleteTerminalLocationsLocationRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub location: String,
}
impl<'a> DeleteTerminalLocationsLocationRequest<'a> {
    pub async fn send(self) -> anyhow::Result<DeletedTerminalLocation> {
        let mut r = self
            .client
            .client
            .delete(
                &format!("/v1/terminal/locations/{location}", location = self.location),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetTerminalReadersRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub device_type: Option<String>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub location: Option<String>,
    pub starting_after: Option<String>,
    pub status: Option<String>,
}
impl<'a> GetTerminalReadersRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v1/terminal/readers");
        if let Some(ref unwrapped) = self.device_type {
            r = r.push_query("device_type", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.location {
            r = r.push_query("location", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.status {
            r = r.push_query("status", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn device_type(mut self, device_type: String) -> Self {
        self.device_type = Some(device_type);
        self
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn location(mut self, location: String) -> Self {
        self.location = Some(location);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
    pub fn status(mut self, status: String) -> Self {
        self.status = Some(status);
        self
    }
}
pub struct PostTerminalReadersRequest<'a> {
    pub(crate) client: &'a StripeClient,
}
impl<'a> PostTerminalReadersRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TerminalReader> {
        let mut r = self.client.client.post("/v1/terminal/readers");
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetTerminalReadersReaderRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub expand: Option<Vec<String>>,
    pub reader: String,
}
impl<'a> GetTerminalReadersReaderRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .get(&format!("/v1/terminal/readers/{reader}", reader = self.reader));
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct PostTerminalReadersReaderRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub reader: String,
}
impl<'a> PostTerminalReadersReaderRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .post(&format!("/v1/terminal/readers/{reader}", reader = self.reader));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct DeleteTerminalReadersReaderRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub reader: String,
}
impl<'a> DeleteTerminalReadersReaderRequest<'a> {
    pub async fn send(self) -> anyhow::Result<DeletedTerminalReader> {
        let mut r = self
            .client
            .client
            .delete(&format!("/v1/terminal/readers/{reader}", reader = self.reader));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct PostTerminalReadersReaderCancelActionRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub reader: String,
}
impl<'a> PostTerminalReadersReaderCancelActionRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TerminalReader> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/v1/terminal/readers/{reader}/cancel_action", reader = self.reader
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct PostTerminalReadersReaderProcessPaymentIntentRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub reader: String,
}
impl<'a> PostTerminalReadersReaderProcessPaymentIntentRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TerminalReader> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/v1/terminal/readers/{reader}/process_payment_intent", reader = self
                    .reader
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct PostTerminalReadersReaderProcessSetupIntentRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub reader: String,
}
impl<'a> PostTerminalReadersReaderProcessSetupIntentRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TerminalReader> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/v1/terminal/readers/{reader}/process_setup_intent", reader = self
                    .reader
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct PostTerminalReadersReaderSetReaderDisplayRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub reader: String,
}
impl<'a> PostTerminalReadersReaderSetReaderDisplayRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TerminalReader> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/v1/terminal/readers/{reader}/set_reader_display", reader = self
                    .reader
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct PostTestHelpersCustomersCustomerFundCashBalanceRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub customer: String,
}
impl<'a> PostTestHelpersCustomersCustomerFundCashBalanceRequest<'a> {
    pub async fn send(self) -> anyhow::Result<CustomerBalanceTransaction> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/v1/test_helpers/customers/{customer}/fund_cash_balance", customer =
                    self.customer
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct PostTestHelpersIssuingCardsCardShippingDeliverRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub card: String,
}
impl<'a> PostTestHelpersIssuingCardsCardShippingDeliverRequest<'a> {
    pub async fn send(self) -> anyhow::Result<IssuingCard> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/v1/test_helpers/issuing/cards/{card}/shipping/deliver", card = self
                    .card
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct PostTestHelpersIssuingCardsCardShippingFailRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub card: String,
}
impl<'a> PostTestHelpersIssuingCardsCardShippingFailRequest<'a> {
    pub async fn send(self) -> anyhow::Result<IssuingCard> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/v1/test_helpers/issuing/cards/{card}/shipping/fail", card = self
                    .card
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct PostTestHelpersIssuingCardsCardShippingReturnRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub card: String,
}
impl<'a> PostTestHelpersIssuingCardsCardShippingReturnRequest<'a> {
    pub async fn send(self) -> anyhow::Result<IssuingCard> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/v1/test_helpers/issuing/cards/{card}/shipping/return", card = self
                    .card
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct PostTestHelpersIssuingCardsCardShippingShipRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub card: String,
}
impl<'a> PostTestHelpersIssuingCardsCardShippingShipRequest<'a> {
    pub async fn send(self) -> anyhow::Result<IssuingCard> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/v1/test_helpers/issuing/cards/{card}/shipping/ship", card = self
                    .card
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct PostTestHelpersRefundsRefundExpireRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub refund: String,
}
impl<'a> PostTestHelpersRefundsRefundExpireRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Refund> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/v1/test_helpers/refunds/{refund}/expire", refund = self.refund
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct PostTestHelpersTerminalReadersReaderPresentPaymentMethodRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub reader: String,
}
impl<'a> PostTestHelpersTerminalReadersReaderPresentPaymentMethodRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TerminalReader> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/v1/test_helpers/terminal/readers/{reader}/present_payment_method",
                    reader = self.reader
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetTestHelpersTestClocksRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
}
impl<'a> GetTestHelpersTestClocksRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v1/test_helpers/test_clocks");
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
}
pub struct PostTestHelpersTestClocksRequest<'a> {
    pub(crate) client: &'a StripeClient,
}
impl<'a> PostTestHelpersTestClocksRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TestHelpersTestClock> {
        let mut r = self.client.client.post("/v1/test_helpers/test_clocks");
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetTestHelpersTestClocksTestClockRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub expand: Option<Vec<String>>,
    pub test_clock: String,
}
impl<'a> GetTestHelpersTestClocksTestClockRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TestHelpersTestClock> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/v1/test_helpers/test_clocks/{test_clock}", test_clock = self
                    .test_clock
                ),
            );
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct DeleteTestHelpersTestClocksTestClockRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub test_clock: String,
}
impl<'a> DeleteTestHelpersTestClocksTestClockRequest<'a> {
    pub async fn send(self) -> anyhow::Result<DeletedTestHelpersTestClock> {
        let mut r = self
            .client
            .client
            .delete(
                &format!(
                    "/v1/test_helpers/test_clocks/{test_clock}", test_clock = self
                    .test_clock
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct PostTestHelpersTestClocksTestClockAdvanceRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub test_clock: String,
}
impl<'a> PostTestHelpersTestClocksTestClockAdvanceRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TestHelpersTestClock> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/v1/test_helpers/test_clocks/{test_clock}/advance", test_clock =
                    self.test_clock
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct PostTestHelpersTreasuryInboundTransfersIdFailRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub id: String,
}
impl<'a> PostTestHelpersTreasuryInboundTransfersIdFailRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TreasuryInboundTransfer> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/v1/test_helpers/treasury/inbound_transfers/{id}/fail", id = self.id
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct PostTestHelpersTreasuryInboundTransfersIdReturnRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub id: String,
}
impl<'a> PostTestHelpersTreasuryInboundTransfersIdReturnRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TreasuryInboundTransfer> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/v1/test_helpers/treasury/inbound_transfers/{id}/return", id = self
                    .id
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct PostTestHelpersTreasuryInboundTransfersIdSucceedRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub id: String,
}
impl<'a> PostTestHelpersTreasuryInboundTransfersIdSucceedRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TreasuryInboundTransfer> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/v1/test_helpers/treasury/inbound_transfers/{id}/succeed", id = self
                    .id
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct PostTestHelpersTreasuryOutboundPaymentsIdFailRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub id: String,
}
impl<'a> PostTestHelpersTreasuryOutboundPaymentsIdFailRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TreasuryOutboundPayment> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/v1/test_helpers/treasury/outbound_payments/{id}/fail", id = self.id
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct PostTestHelpersTreasuryOutboundPaymentsIdPostRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub id: String,
}
impl<'a> PostTestHelpersTreasuryOutboundPaymentsIdPostRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TreasuryOutboundPayment> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/v1/test_helpers/treasury/outbound_payments/{id}/post", id = self.id
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct PostTestHelpersTreasuryOutboundPaymentsIdReturnRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub id: String,
}
impl<'a> PostTestHelpersTreasuryOutboundPaymentsIdReturnRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TreasuryOutboundPayment> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/v1/test_helpers/treasury/outbound_payments/{id}/return", id = self
                    .id
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct PostTestHelpersTreasuryOutboundTransfersOutboundTransferFailRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub outbound_transfer: String,
}
impl<'a> PostTestHelpersTreasuryOutboundTransfersOutboundTransferFailRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TreasuryOutboundTransfer> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/v1/test_helpers/treasury/outbound_transfers/{outbound_transfer}/fail",
                    outbound_transfer = self.outbound_transfer
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct PostTestHelpersTreasuryOutboundTransfersOutboundTransferPostRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub outbound_transfer: String,
}
impl<'a> PostTestHelpersTreasuryOutboundTransfersOutboundTransferPostRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TreasuryOutboundTransfer> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/v1/test_helpers/treasury/outbound_transfers/{outbound_transfer}/post",
                    outbound_transfer = self.outbound_transfer
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct PostTestHelpersTreasuryOutboundTransfersOutboundTransferReturnRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub outbound_transfer: String,
}
impl<'a> PostTestHelpersTreasuryOutboundTransfersOutboundTransferReturnRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TreasuryOutboundTransfer> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/v1/test_helpers/treasury/outbound_transfers/{outbound_transfer}/return",
                    outbound_transfer = self.outbound_transfer
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct PostTestHelpersTreasuryReceivedCreditsRequest<'a> {
    pub(crate) client: &'a StripeClient,
}
impl<'a> PostTestHelpersTreasuryReceivedCreditsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TreasuryReceivedCredit> {
        let mut r = self
            .client
            .client
            .post("/v1/test_helpers/treasury/received_credits");
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct PostTestHelpersTreasuryReceivedDebitsRequest<'a> {
    pub(crate) client: &'a StripeClient,
}
impl<'a> PostTestHelpersTreasuryReceivedDebitsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TreasuryReceivedDebit> {
        let mut r = self.client.client.post("/v1/test_helpers/treasury/received_debits");
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct PostTokensRequest<'a> {
    pub(crate) client: &'a StripeClient,
}
impl<'a> PostTokensRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Token> {
        let mut r = self.client.client.post("/v1/tokens");
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetTokensTokenRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub expand: Option<Vec<String>>,
    pub token: String,
}
impl<'a> GetTokensTokenRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Token> {
        let mut r = self
            .client
            .client
            .get(&format!("/v1/tokens/{token}", token = self.token));
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct GetTopupsRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub amount: Option<serde_json::Value>,
    pub created: Option<serde_json::Value>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
    pub status: Option<String>,
}
impl<'a> GetTopupsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v1/topups");
        if let Some(ref unwrapped) = self.amount {
            r = r.push_query("amount", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.created {
            r = r.push_query("created", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.status {
            r = r.push_query("status", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn amount(mut self, amount: serde_json::Value) -> Self {
        self.amount = Some(amount);
        self
    }
    pub fn created(mut self, created: serde_json::Value) -> Self {
        self.created = Some(created);
        self
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
    pub fn status(mut self, status: String) -> Self {
        self.status = Some(status);
        self
    }
}
pub struct PostTopupsRequest<'a> {
    pub(crate) client: &'a StripeClient,
}
impl<'a> PostTopupsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Topup> {
        let mut r = self.client.client.post("/v1/topups");
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetTopupsTopupRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub expand: Option<Vec<String>>,
    pub topup: String,
}
impl<'a> GetTopupsTopupRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Topup> {
        let mut r = self
            .client
            .client
            .get(&format!("/v1/topups/{topup}", topup = self.topup));
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct PostTopupsTopupRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub topup: String,
}
impl<'a> PostTopupsTopupRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Topup> {
        let mut r = self
            .client
            .client
            .post(&format!("/v1/topups/{topup}", topup = self.topup));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct PostTopupsTopupCancelRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub topup: String,
}
impl<'a> PostTopupsTopupCancelRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Topup> {
        let mut r = self
            .client
            .client
            .post(&format!("/v1/topups/{topup}/cancel", topup = self.topup));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetTransfersRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub created: Option<serde_json::Value>,
    pub destination: Option<String>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
    pub transfer_group: Option<String>,
}
impl<'a> GetTransfersRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v1/transfers");
        if let Some(ref unwrapped) = self.created {
            r = r.push_query("created", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.destination {
            r = r.push_query("destination", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.transfer_group {
            r = r.push_query("transfer_group", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn created(mut self, created: serde_json::Value) -> Self {
        self.created = Some(created);
        self
    }
    pub fn destination(mut self, destination: String) -> Self {
        self.destination = Some(destination);
        self
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
    pub fn transfer_group(mut self, transfer_group: String) -> Self {
        self.transfer_group = Some(transfer_group);
        self
    }
}
pub struct PostTransfersRequest<'a> {
    pub(crate) client: &'a StripeClient,
}
impl<'a> PostTransfersRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Transfer> {
        let mut r = self.client.client.post("/v1/transfers");
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetTransfersIdReversalsRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub id: String,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
}
impl<'a> GetTransfersIdReversalsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .get(&format!("/v1/transfers/{id}/reversals", id = self.id));
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
}
pub struct PostTransfersIdReversalsRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub id: String,
}
impl<'a> PostTransfersIdReversalsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TransferReversal> {
        let mut r = self
            .client
            .client
            .post(&format!("/v1/transfers/{id}/reversals", id = self.id));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetTransfersTransferRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub expand: Option<Vec<String>>,
    pub transfer: String,
}
impl<'a> GetTransfersTransferRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Transfer> {
        let mut r = self
            .client
            .client
            .get(&format!("/v1/transfers/{transfer}", transfer = self.transfer));
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct PostTransfersTransferRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub transfer: String,
}
impl<'a> PostTransfersTransferRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Transfer> {
        let mut r = self
            .client
            .client
            .post(&format!("/v1/transfers/{transfer}", transfer = self.transfer));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetTransfersTransferReversalsIdRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub expand: Option<Vec<String>>,
    pub id: String,
    pub transfer: String,
}
impl<'a> GetTransfersTransferReversalsIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TransferReversal> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/v1/transfers/{transfer}/reversals/{id}", id = self.id, transfer =
                    self.transfer
                ),
            );
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct PostTransfersTransferReversalsIdRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub id: String,
    pub transfer: String,
}
impl<'a> PostTransfersTransferReversalsIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TransferReversal> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/v1/transfers/{transfer}/reversals/{id}", id = self.id, transfer =
                    self.transfer
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetTreasuryCreditReversalsRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub financial_account: String,
    pub limit: Option<i64>,
    pub received_credit: Option<String>,
    pub starting_after: Option<String>,
    pub status: Option<String>,
}
impl<'a> GetTreasuryCreditReversalsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v1/treasury/credit_reversals");
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = r.push_query("financial_account", &self.financial_account.to_string());
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.received_credit {
            r = r.push_query("received_credit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.status {
            r = r.push_query("status", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn received_credit(mut self, received_credit: String) -> Self {
        self.received_credit = Some(received_credit);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
    pub fn status(mut self, status: String) -> Self {
        self.status = Some(status);
        self
    }
}
pub struct PostTreasuryCreditReversalsRequest<'a> {
    pub(crate) client: &'a StripeClient,
}
impl<'a> PostTreasuryCreditReversalsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TreasuryCreditReversal> {
        let mut r = self.client.client.post("/v1/treasury/credit_reversals");
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetTreasuryCreditReversalsCreditReversalRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub credit_reversal: String,
    pub expand: Option<Vec<String>>,
}
impl<'a> GetTreasuryCreditReversalsCreditReversalRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TreasuryCreditReversal> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/v1/treasury/credit_reversals/{credit_reversal}", credit_reversal =
                    self.credit_reversal
                ),
            );
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct GetTreasuryDebitReversalsRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub financial_account: String,
    pub limit: Option<i64>,
    pub received_debit: Option<String>,
    pub resolution: Option<String>,
    pub starting_after: Option<String>,
    pub status: Option<String>,
}
impl<'a> GetTreasuryDebitReversalsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v1/treasury/debit_reversals");
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = r.push_query("financial_account", &self.financial_account.to_string());
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.received_debit {
            r = r.push_query("received_debit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.resolution {
            r = r.push_query("resolution", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.status {
            r = r.push_query("status", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn received_debit(mut self, received_debit: String) -> Self {
        self.received_debit = Some(received_debit);
        self
    }
    pub fn resolution(mut self, resolution: String) -> Self {
        self.resolution = Some(resolution);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
    pub fn status(mut self, status: String) -> Self {
        self.status = Some(status);
        self
    }
}
pub struct PostTreasuryDebitReversalsRequest<'a> {
    pub(crate) client: &'a StripeClient,
}
impl<'a> PostTreasuryDebitReversalsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TreasuryDebitReversal> {
        let mut r = self.client.client.post("/v1/treasury/debit_reversals");
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetTreasuryDebitReversalsDebitReversalRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub debit_reversal: String,
    pub expand: Option<Vec<String>>,
}
impl<'a> GetTreasuryDebitReversalsDebitReversalRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TreasuryDebitReversal> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/v1/treasury/debit_reversals/{debit_reversal}", debit_reversal =
                    self.debit_reversal
                ),
            );
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct GetTreasuryFinancialAccountsRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub created: Option<serde_json::Value>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
}
impl<'a> GetTreasuryFinancialAccountsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v1/treasury/financial_accounts");
        if let Some(ref unwrapped) = self.created {
            r = r.push_query("created", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn created(mut self, created: serde_json::Value) -> Self {
        self.created = Some(created);
        self
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
}
pub struct PostTreasuryFinancialAccountsRequest<'a> {
    pub(crate) client: &'a StripeClient,
}
impl<'a> PostTreasuryFinancialAccountsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TreasuryFinancialAccount> {
        let mut r = self.client.client.post("/v1/treasury/financial_accounts");
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetTreasuryFinancialAccountsFinancialAccountRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub expand: Option<Vec<String>>,
    pub financial_account: String,
}
impl<'a> GetTreasuryFinancialAccountsFinancialAccountRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TreasuryFinancialAccount> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/v1/treasury/financial_accounts/{financial_account}",
                    financial_account = self.financial_account
                ),
            );
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct PostTreasuryFinancialAccountsFinancialAccountRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub financial_account: String,
}
impl<'a> PostTreasuryFinancialAccountsFinancialAccountRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TreasuryFinancialAccount> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/v1/treasury/financial_accounts/{financial_account}",
                    financial_account = self.financial_account
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetTreasuryFinancialAccountsFinancialAccountFeaturesRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub expand: Option<Vec<String>>,
    pub financial_account: String,
}
impl<'a> GetTreasuryFinancialAccountsFinancialAccountFeaturesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TreasuryFinancialAccountFeatures> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/v1/treasury/financial_accounts/{financial_account}/features",
                    financial_account = self.financial_account
                ),
            );
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct PostTreasuryFinancialAccountsFinancialAccountFeaturesRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub financial_account: String,
}
impl<'a> PostTreasuryFinancialAccountsFinancialAccountFeaturesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TreasuryFinancialAccountFeatures> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/v1/treasury/financial_accounts/{financial_account}/features",
                    financial_account = self.financial_account
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetTreasuryInboundTransfersRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub financial_account: String,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
    pub status: Option<String>,
}
impl<'a> GetTreasuryInboundTransfersRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v1/treasury/inbound_transfers");
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = r.push_query("financial_account", &self.financial_account.to_string());
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.status {
            r = r.push_query("status", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
    pub fn status(mut self, status: String) -> Self {
        self.status = Some(status);
        self
    }
}
pub struct PostTreasuryInboundTransfersRequest<'a> {
    pub(crate) client: &'a StripeClient,
}
impl<'a> PostTreasuryInboundTransfersRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TreasuryInboundTransfer> {
        let mut r = self.client.client.post("/v1/treasury/inbound_transfers");
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetTreasuryInboundTransfersIdRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub expand: Option<Vec<String>>,
    pub id: String,
}
impl<'a> GetTreasuryInboundTransfersIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TreasuryInboundTransfer> {
        let mut r = self
            .client
            .client
            .get(&format!("/v1/treasury/inbound_transfers/{id}", id = self.id));
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct PostTreasuryInboundTransfersInboundTransferCancelRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub inbound_transfer: String,
}
impl<'a> PostTreasuryInboundTransfersInboundTransferCancelRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TreasuryInboundTransfer> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/v1/treasury/inbound_transfers/{inbound_transfer}/cancel",
                    inbound_transfer = self.inbound_transfer
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetTreasuryOutboundPaymentsRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub customer: Option<String>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub financial_account: String,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
    pub status: Option<String>,
}
impl<'a> GetTreasuryOutboundPaymentsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v1/treasury/outbound_payments");
        if let Some(ref unwrapped) = self.customer {
            r = r.push_query("customer", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = r.push_query("financial_account", &self.financial_account.to_string());
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.status {
            r = r.push_query("status", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn customer(mut self, customer: String) -> Self {
        self.customer = Some(customer);
        self
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
    pub fn status(mut self, status: String) -> Self {
        self.status = Some(status);
        self
    }
}
pub struct PostTreasuryOutboundPaymentsRequest<'a> {
    pub(crate) client: &'a StripeClient,
}
impl<'a> PostTreasuryOutboundPaymentsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TreasuryOutboundPayment> {
        let mut r = self.client.client.post("/v1/treasury/outbound_payments");
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetTreasuryOutboundPaymentsIdRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub expand: Option<Vec<String>>,
    pub id: String,
}
impl<'a> GetTreasuryOutboundPaymentsIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TreasuryOutboundPayment> {
        let mut r = self
            .client
            .client
            .get(&format!("/v1/treasury/outbound_payments/{id}", id = self.id));
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct PostTreasuryOutboundPaymentsIdCancelRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub id: String,
}
impl<'a> PostTreasuryOutboundPaymentsIdCancelRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TreasuryOutboundPayment> {
        let mut r = self
            .client
            .client
            .post(&format!("/v1/treasury/outbound_payments/{id}/cancel", id = self.id));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetTreasuryOutboundTransfersRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub financial_account: String,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
    pub status: Option<String>,
}
impl<'a> GetTreasuryOutboundTransfersRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v1/treasury/outbound_transfers");
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = r.push_query("financial_account", &self.financial_account.to_string());
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.status {
            r = r.push_query("status", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
    pub fn status(mut self, status: String) -> Self {
        self.status = Some(status);
        self
    }
}
pub struct PostTreasuryOutboundTransfersRequest<'a> {
    pub(crate) client: &'a StripeClient,
}
impl<'a> PostTreasuryOutboundTransfersRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TreasuryOutboundTransfer> {
        let mut r = self.client.client.post("/v1/treasury/outbound_transfers");
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetTreasuryOutboundTransfersOutboundTransferRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub expand: Option<Vec<String>>,
    pub outbound_transfer: String,
}
impl<'a> GetTreasuryOutboundTransfersOutboundTransferRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TreasuryOutboundTransfer> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/v1/treasury/outbound_transfers/{outbound_transfer}",
                    outbound_transfer = self.outbound_transfer
                ),
            );
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct PostTreasuryOutboundTransfersOutboundTransferCancelRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub outbound_transfer: String,
}
impl<'a> PostTreasuryOutboundTransfersOutboundTransferCancelRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TreasuryOutboundTransfer> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/v1/treasury/outbound_transfers/{outbound_transfer}/cancel",
                    outbound_transfer = self.outbound_transfer
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetTreasuryReceivedCreditsRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub financial_account: String,
    pub limit: Option<i64>,
    pub linked_flows: Option<serde_json::Value>,
    pub starting_after: Option<String>,
    pub status: Option<String>,
}
impl<'a> GetTreasuryReceivedCreditsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v1/treasury/received_credits");
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = r.push_query("financial_account", &self.financial_account.to_string());
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.linked_flows {
            r = r.push_query("linked_flows", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.status {
            r = r.push_query("status", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn linked_flows(mut self, linked_flows: serde_json::Value) -> Self {
        self.linked_flows = Some(linked_flows);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
    pub fn status(mut self, status: String) -> Self {
        self.status = Some(status);
        self
    }
}
pub struct GetTreasuryReceivedCreditsIdRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub expand: Option<Vec<String>>,
    pub id: String,
}
impl<'a> GetTreasuryReceivedCreditsIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TreasuryReceivedCredit> {
        let mut r = self
            .client
            .client
            .get(&format!("/v1/treasury/received_credits/{id}", id = self.id));
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct GetTreasuryReceivedDebitsRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub financial_account: String,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
    pub status: Option<String>,
}
impl<'a> GetTreasuryReceivedDebitsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v1/treasury/received_debits");
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = r.push_query("financial_account", &self.financial_account.to_string());
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.status {
            r = r.push_query("status", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
    pub fn status(mut self, status: String) -> Self {
        self.status = Some(status);
        self
    }
}
pub struct GetTreasuryReceivedDebitsIdRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub expand: Option<Vec<String>>,
    pub id: String,
}
impl<'a> GetTreasuryReceivedDebitsIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TreasuryReceivedDebit> {
        let mut r = self
            .client
            .client
            .get(&format!("/v1/treasury/received_debits/{id}", id = self.id));
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct GetTreasuryTransactionEntriesRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub created: Option<serde_json::Value>,
    pub effective_at: Option<serde_json::Value>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub financial_account: String,
    pub limit: Option<i64>,
    pub order_by: Option<String>,
    pub starting_after: Option<String>,
    pub transaction: Option<String>,
}
impl<'a> GetTreasuryTransactionEntriesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v1/treasury/transaction_entries");
        if let Some(ref unwrapped) = self.created {
            r = r.push_query("created", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.effective_at {
            r = r.push_query("effective_at", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = r.push_query("financial_account", &self.financial_account.to_string());
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.order_by {
            r = r.push_query("order_by", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.transaction {
            r = r.push_query("transaction", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn created(mut self, created: serde_json::Value) -> Self {
        self.created = Some(created);
        self
    }
    pub fn effective_at(mut self, effective_at: serde_json::Value) -> Self {
        self.effective_at = Some(effective_at);
        self
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn order_by(mut self, order_by: String) -> Self {
        self.order_by = Some(order_by);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
    pub fn transaction(mut self, transaction: String) -> Self {
        self.transaction = Some(transaction);
        self
    }
}
pub struct GetTreasuryTransactionEntriesIdRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub expand: Option<Vec<String>>,
    pub id: String,
}
impl<'a> GetTreasuryTransactionEntriesIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TreasuryTransactionEntry> {
        let mut r = self
            .client
            .client
            .get(&format!("/v1/treasury/transaction_entries/{id}", id = self.id));
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct GetTreasuryTransactionsRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub created: Option<serde_json::Value>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub financial_account: String,
    pub limit: Option<i64>,
    pub order_by: Option<String>,
    pub starting_after: Option<String>,
    pub status: Option<String>,
    pub status_transitions: Option<serde_json::Value>,
}
impl<'a> GetTreasuryTransactionsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v1/treasury/transactions");
        if let Some(ref unwrapped) = self.created {
            r = r.push_query("created", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = r.push_query("financial_account", &self.financial_account.to_string());
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.order_by {
            r = r.push_query("order_by", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.status {
            r = r.push_query("status", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.status_transitions {
            r = r.push_query("status_transitions", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn created(mut self, created: serde_json::Value) -> Self {
        self.created = Some(created);
        self
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn order_by(mut self, order_by: String) -> Self {
        self.order_by = Some(order_by);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
    pub fn status(mut self, status: String) -> Self {
        self.status = Some(status);
        self
    }
    pub fn status_transitions(mut self, status_transitions: serde_json::Value) -> Self {
        self.status_transitions = Some(status_transitions);
        self
    }
}
pub struct GetTreasuryTransactionsIdRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub expand: Option<Vec<String>>,
    pub id: String,
}
impl<'a> GetTreasuryTransactionsIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TreasuryTransaction> {
        let mut r = self
            .client
            .client
            .get(&format!("/v1/treasury/transactions/{id}", id = self.id));
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct GetWebhookEndpointsRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
}
impl<'a> GetWebhookEndpointsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v1/webhook_endpoints");
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.ending_before = Some(ending_before);
        self
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.starting_after = Some(starting_after);
        self
    }
}
pub struct PostWebhookEndpointsRequest<'a> {
    pub(crate) client: &'a StripeClient,
}
impl<'a> PostWebhookEndpointsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<WebhookEndpoint> {
        let mut r = self.client.client.post("/v1/webhook_endpoints");
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct GetWebhookEndpointsWebhookEndpointRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub expand: Option<Vec<String>>,
    pub webhook_endpoint: String,
}
impl<'a> GetWebhookEndpointsWebhookEndpointRequest<'a> {
    pub async fn send(self) -> anyhow::Result<WebhookEndpoint> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/v1/webhook_endpoints/{webhook_endpoint}", webhook_endpoint = self
                    .webhook_endpoint
                ),
            );
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn expand(mut self, expand: Vec<String>) -> Self {
        self.expand = Some(expand);
        self
    }
}
pub struct PostWebhookEndpointsWebhookEndpointRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub webhook_endpoint: String,
}
impl<'a> PostWebhookEndpointsWebhookEndpointRequest<'a> {
    pub async fn send(self) -> anyhow::Result<WebhookEndpoint> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/v1/webhook_endpoints/{webhook_endpoint}", webhook_endpoint = self
                    .webhook_endpoint
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct DeleteWebhookEndpointsWebhookEndpointRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub webhook_endpoint: String,
}
impl<'a> DeleteWebhookEndpointsWebhookEndpointRequest<'a> {
    pub async fn send(self) -> anyhow::Result<DeletedWebhookEndpoint> {
        let mut r = self
            .client
            .client
            .delete(
                &format!(
                    "/v1/webhook_endpoints/{webhook_endpoint}", webhook_endpoint = self
                    .webhook_endpoint
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
