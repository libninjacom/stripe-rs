//! [`StripeClient`](struct.StripeClient.html) is the main entry point for this library.
//!
//! Library created with [`libninja`](https://www.libninja.com).
#![allow(non_camel_case_types)]
pub mod model;
pub mod request_model;
use crate::model::*;

pub struct StripeClient {
    pub(crate) client: httpclient::Client,
    authentication: Option<StripeAuthentication>,
}
impl StripeClient {}
impl StripeClient {
    pub fn new(url: &str) -> Self {
        let client = httpclient::Client::new(Some(url.to_string()));
        let authentication = None;
        Self { client, authentication }
    }
    pub fn with_authentication(mut self, authentication: StripeAuthentication) -> Self {
        self.authentication = Some(authentication);
        self
    }
    pub fn authenticate<'a>(
        &self,
        mut r: httpclient::RequestBuilder<'a>,
    ) -> httpclient::RequestBuilder<'a> {
        if let Some(ref authentication) = self.authentication {
            match authentication {
                StripeAuthentication::BasicAuth { basic_auth } => {
                    r = r.basic_auth(basic_auth);
                }
                StripeAuthentication::BearerAuth { bearer_auth } => {
                    r = r.bearer_auth(bearer_auth);
                }
            }
        }
        r
    }
    pub fn with_middleware<M: httpclient::Middleware + 'static>(
        mut self,
        middleware: M,
    ) -> Self {
        self.client = self.client.with_middleware(middleware);
        self
    }
    ///<p>Retrieves the details of an account.</p>
    pub fn get_account(&self) -> request_model::GetAccountRequest {
        request_model::GetAccountRequest {
            client: &self,
            expand: None,
        }
    }
    /**<p>Updates a <a href="/docs/connect/accounts">connected account</a> by setting the values of the parameters passed. Any parameters not provided are left unchanged. Most parameters can be changed only for Custom accounts. (These are marked <strong>Custom Only</strong> below.) Parameters marked <strong>Custom and Express</strong> are not supported for Standard accounts.</p>

<p>To update your own account, use the <a href="https://dashboard.stripe.com/account">Dashboard</a>. Refer to our <a href="/docs/connect/updating-accounts">Connect</a> documentation to learn more about updating accounts.</p>*/
    pub fn post_account(&self) -> request_model::PostAccountRequest {
        request_model::PostAccountRequest {
            client: &self,
        }
    }
    /**<p>With <a href="/docs/connect">Connect</a>, you can delete accounts you manage.</p>

<p>Accounts created using test-mode keys can be deleted at any time. Standard accounts created using live-mode keys cannot be deleted. Custom or Express accounts created using live-mode keys can only be deleted once all balances are zero.</p>

<p>If you want to delete your own account, use the <a href="https://dashboard.stripe.com/account">account information tab in your account settings</a> instead.</p>*/
    pub fn delete_account(&self) -> request_model::DeleteAccountRequest {
        request_model::DeleteAccountRequest {
            client: &self,
        }
    }
    ///<p>Create an external account for a given account.</p>
    pub fn post_account_bank_accounts(
        &self,
    ) -> request_model::PostAccountBankAccountsRequest {
        request_model::PostAccountBankAccountsRequest {
            client: &self,
        }
    }
    ///<p>Retrieve a specified external account for a given account.</p>
    pub fn get_account_bank_accounts_id(
        &self,
        id: String,
    ) -> request_model::GetAccountBankAccountsIdRequest {
        request_model::GetAccountBankAccountsIdRequest {
            client: &self,
            expand: None,
            id,
        }
    }
    /**<p>Updates the metadata, account holder name, account holder type of a bank account belonging to a <a href="/docs/connect/custom-accounts">Custom account</a>, and optionally sets it as the default for its currency. Other bank account details are not editable by design.</p>

<p>You can re-enable a disabled bank account by performing an update call without providing any arguments or changes.</p>*/
    pub fn post_account_bank_accounts_id(
        &self,
        id: String,
    ) -> request_model::PostAccountBankAccountsIdRequest {
        request_model::PostAccountBankAccountsIdRequest {
            client: &self,
            id,
        }
    }
    ///<p>Delete a specified external account for a given account.</p>
    pub fn delete_account_bank_accounts_id(
        &self,
        id: String,
    ) -> request_model::DeleteAccountBankAccountsIdRequest {
        request_model::DeleteAccountBankAccountsIdRequest {
            client: &self,
            id,
        }
    }
    ///<p>Returns a list of capabilities associated with the account. The capabilities are returned sorted by creation date, with the most recent capability appearing first.</p>
    pub fn get_account_capabilities(
        &self,
    ) -> request_model::GetAccountCapabilitiesRequest {
        request_model::GetAccountCapabilitiesRequest {
            client: &self,
            expand: None,
        }
    }
    ///<p>Retrieves information about the specified Account Capability.</p>
    pub fn get_account_capabilities_capability(
        &self,
        capability: String,
    ) -> request_model::GetAccountCapabilitiesCapabilityRequest {
        request_model::GetAccountCapabilitiesCapabilityRequest {
            client: &self,
            capability,
            expand: None,
        }
    }
    ///<p>Updates an existing Account Capability.</p>
    pub fn post_account_capabilities_capability(
        &self,
        capability: String,
    ) -> request_model::PostAccountCapabilitiesCapabilityRequest {
        request_model::PostAccountCapabilitiesCapabilityRequest {
            client: &self,
            capability,
        }
    }
    ///<p>List external accounts for an account.</p>
    pub fn get_account_external_accounts(
        &self,
    ) -> request_model::GetAccountExternalAccountsRequest {
        request_model::GetAccountExternalAccountsRequest {
            client: &self,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
        }
    }
    ///<p>Create an external account for a given account.</p>
    pub fn post_account_external_accounts(
        &self,
    ) -> request_model::PostAccountExternalAccountsRequest {
        request_model::PostAccountExternalAccountsRequest {
            client: &self,
        }
    }
    ///<p>Retrieve a specified external account for a given account.</p>
    pub fn get_account_external_accounts_id(
        &self,
        id: String,
    ) -> request_model::GetAccountExternalAccountsIdRequest {
        request_model::GetAccountExternalAccountsIdRequest {
            client: &self,
            expand: None,
            id,
        }
    }
    /**<p>Updates the metadata, account holder name, account holder type of a bank account belonging to a <a href="/docs/connect/custom-accounts">Custom account</a>, and optionally sets it as the default for its currency. Other bank account details are not editable by design.</p>

<p>You can re-enable a disabled bank account by performing an update call without providing any arguments or changes.</p>*/
    pub fn post_account_external_accounts_id(
        &self,
        id: String,
    ) -> request_model::PostAccountExternalAccountsIdRequest {
        request_model::PostAccountExternalAccountsIdRequest {
            client: &self,
            id,
        }
    }
    ///<p>Delete a specified external account for a given account.</p>
    pub fn delete_account_external_accounts_id(
        &self,
        id: String,
    ) -> request_model::DeleteAccountExternalAccountsIdRequest {
        request_model::DeleteAccountExternalAccountsIdRequest {
            client: &self,
            id,
        }
    }
    /**<p>Creates a single-use login link for an Express account to access their Stripe dashboard.</p>

<p><strong>You may only create login links for <a href="/docs/connect/express-accounts">Express accounts</a> connected to your platform</strong>.</p>*/
    pub fn post_account_login_links(
        &self,
    ) -> request_model::PostAccountLoginLinksRequest {
        request_model::PostAccountLoginLinksRequest {
            client: &self,
        }
    }
    ///<p>Returns a list of people associated with the account’s legal entity. The people are returned sorted by creation date, with the most recent people appearing first.</p>
    pub fn get_account_people(&self) -> request_model::GetAccountPeopleRequest {
        request_model::GetAccountPeopleRequest {
            client: &self,
            ending_before: None,
            expand: None,
            limit: None,
            relationship: None,
            starting_after: None,
        }
    }
    ///<p>Creates a new person.</p>
    pub fn post_account_people(&self) -> request_model::PostAccountPeopleRequest {
        request_model::PostAccountPeopleRequest {
            client: &self,
        }
    }
    ///<p>Retrieves an existing person.</p>
    pub fn get_account_people_person(
        &self,
        person: String,
    ) -> request_model::GetAccountPeoplePersonRequest {
        request_model::GetAccountPeoplePersonRequest {
            client: &self,
            expand: None,
            person,
        }
    }
    ///<p>Updates an existing person.</p>
    pub fn post_account_people_person(
        &self,
        person: String,
    ) -> request_model::PostAccountPeoplePersonRequest {
        request_model::PostAccountPeoplePersonRequest {
            client: &self,
            person,
        }
    }
    ///<p>Deletes an existing person’s relationship to the account’s legal entity. Any person with a relationship for an account can be deleted through the API, except if the person is the <code>account_opener</code>. If your integration is using the <code>executive</code> parameter, you cannot delete the only verified <code>executive</code> on file.</p>
    pub fn delete_account_people_person(
        &self,
        person: String,
    ) -> request_model::DeleteAccountPeoplePersonRequest {
        request_model::DeleteAccountPeoplePersonRequest {
            client: &self,
            person,
        }
    }
    ///<p>Returns a list of people associated with the account’s legal entity. The people are returned sorted by creation date, with the most recent people appearing first.</p>
    pub fn get_account_persons(&self) -> request_model::GetAccountPersonsRequest {
        request_model::GetAccountPersonsRequest {
            client: &self,
            ending_before: None,
            expand: None,
            limit: None,
            relationship: None,
            starting_after: None,
        }
    }
    ///<p>Creates a new person.</p>
    pub fn post_account_persons(&self) -> request_model::PostAccountPersonsRequest {
        request_model::PostAccountPersonsRequest {
            client: &self,
        }
    }
    ///<p>Retrieves an existing person.</p>
    pub fn get_account_persons_person(
        &self,
        person: String,
    ) -> request_model::GetAccountPersonsPersonRequest {
        request_model::GetAccountPersonsPersonRequest {
            client: &self,
            expand: None,
            person,
        }
    }
    ///<p>Updates an existing person.</p>
    pub fn post_account_persons_person(
        &self,
        person: String,
    ) -> request_model::PostAccountPersonsPersonRequest {
        request_model::PostAccountPersonsPersonRequest {
            client: &self,
            person,
        }
    }
    ///<p>Deletes an existing person’s relationship to the account’s legal entity. Any person with a relationship for an account can be deleted through the API, except if the person is the <code>account_opener</code>. If your integration is using the <code>executive</code> parameter, you cannot delete the only verified <code>executive</code> on file.</p>
    pub fn delete_account_persons_person(
        &self,
        person: String,
    ) -> request_model::DeleteAccountPersonsPersonRequest {
        request_model::DeleteAccountPersonsPersonRequest {
            client: &self,
            person,
        }
    }
    ///<p>Creates an AccountLink object that includes a single-use Stripe URL that the platform can redirect their user to in order to take them through the Connect Onboarding flow.</p>
    pub fn post_account_links(&self) -> request_model::PostAccountLinksRequest {
        request_model::PostAccountLinksRequest {
            client: &self,
        }
    }
    ///<p>Returns a list of accounts connected to your platform via <a href="/docs/connect">Connect</a>. If you’re not a platform, the list is empty.</p>
    pub fn get_accounts(&self) -> request_model::GetAccountsRequest {
        request_model::GetAccountsRequest {
            client: &self,
            created: None,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
        }
    }
    /**<p>With <a href="/docs/connect">Connect</a>, you can create Stripe accounts for your users.
To do this, you’ll first need to <a href="https://dashboard.stripe.com/account/applications/settings">register your platform</a>.</p>*/
    pub fn post_accounts(&self) -> request_model::PostAccountsRequest {
        request_model::PostAccountsRequest {
            client: &self,
        }
    }
    ///<p>Retrieves the details of an account.</p>
    pub fn get_accounts_account(
        &self,
        account: String,
    ) -> request_model::GetAccountsAccountRequest {
        request_model::GetAccountsAccountRequest {
            client: &self,
            account,
            expand: None,
        }
    }
    /**<p>Updates a <a href="/docs/connect/accounts">connected account</a> by setting the values of the parameters passed. Any parameters not provided are left unchanged. Most parameters can be changed only for Custom accounts. (These are marked <strong>Custom Only</strong> below.) Parameters marked <strong>Custom and Express</strong> are not supported for Standard accounts.</p>

<p>To update your own account, use the <a href="https://dashboard.stripe.com/account">Dashboard</a>. Refer to our <a href="/docs/connect/updating-accounts">Connect</a> documentation to learn more about updating accounts.</p>*/
    pub fn post_accounts_account(
        &self,
        account: String,
    ) -> request_model::PostAccountsAccountRequest {
        request_model::PostAccountsAccountRequest {
            client: &self,
            account,
        }
    }
    /**<p>With <a href="/docs/connect">Connect</a>, you can delete accounts you manage.</p>

<p>Accounts created using test-mode keys can be deleted at any time. Standard accounts created using live-mode keys cannot be deleted. Custom or Express accounts created using live-mode keys can only be deleted once all balances are zero.</p>

<p>If you want to delete your own account, use the <a href="https://dashboard.stripe.com/account">account information tab in your account settings</a> instead.</p>*/
    pub fn delete_accounts_account(
        &self,
        account: String,
    ) -> request_model::DeleteAccountsAccountRequest {
        request_model::DeleteAccountsAccountRequest {
            client: &self,
            account,
        }
    }
    ///<p>Create an external account for a given account.</p>
    pub fn post_accounts_account_bank_accounts(
        &self,
        account: String,
    ) -> request_model::PostAccountsAccountBankAccountsRequest {
        request_model::PostAccountsAccountBankAccountsRequest {
            client: &self,
            account,
        }
    }
    ///<p>Retrieve a specified external account for a given account.</p>
    pub fn get_accounts_account_bank_accounts_id(
        &self,
        account: String,
        id: String,
    ) -> request_model::GetAccountsAccountBankAccountsIdRequest {
        request_model::GetAccountsAccountBankAccountsIdRequest {
            client: &self,
            account,
            expand: None,
            id,
        }
    }
    /**<p>Updates the metadata, account holder name, account holder type of a bank account belonging to a <a href="/docs/connect/custom-accounts">Custom account</a>, and optionally sets it as the default for its currency. Other bank account details are not editable by design.</p>

<p>You can re-enable a disabled bank account by performing an update call without providing any arguments or changes.</p>*/
    pub fn post_accounts_account_bank_accounts_id(
        &self,
        account: String,
        id: String,
    ) -> request_model::PostAccountsAccountBankAccountsIdRequest {
        request_model::PostAccountsAccountBankAccountsIdRequest {
            client: &self,
            account,
            id,
        }
    }
    ///<p>Delete a specified external account for a given account.</p>
    pub fn delete_accounts_account_bank_accounts_id(
        &self,
        account: String,
        id: String,
    ) -> request_model::DeleteAccountsAccountBankAccountsIdRequest {
        request_model::DeleteAccountsAccountBankAccountsIdRequest {
            client: &self,
            account,
            id,
        }
    }
    ///<p>Returns a list of capabilities associated with the account. The capabilities are returned sorted by creation date, with the most recent capability appearing first.</p>
    pub fn get_accounts_account_capabilities(
        &self,
        account: String,
    ) -> request_model::GetAccountsAccountCapabilitiesRequest {
        request_model::GetAccountsAccountCapabilitiesRequest {
            client: &self,
            account,
            expand: None,
        }
    }
    ///<p>Retrieves information about the specified Account Capability.</p>
    pub fn get_accounts_account_capabilities_capability(
        &self,
        account: String,
        capability: String,
    ) -> request_model::GetAccountsAccountCapabilitiesCapabilityRequest {
        request_model::GetAccountsAccountCapabilitiesCapabilityRequest {
            client: &self,
            account,
            capability,
            expand: None,
        }
    }
    ///<p>Updates an existing Account Capability.</p>
    pub fn post_accounts_account_capabilities_capability(
        &self,
        account: String,
        capability: String,
    ) -> request_model::PostAccountsAccountCapabilitiesCapabilityRequest {
        request_model::PostAccountsAccountCapabilitiesCapabilityRequest {
            client: &self,
            account,
            capability,
        }
    }
    ///<p>List external accounts for an account.</p>
    pub fn get_accounts_account_external_accounts(
        &self,
        account: String,
    ) -> request_model::GetAccountsAccountExternalAccountsRequest {
        request_model::GetAccountsAccountExternalAccountsRequest {
            client: &self,
            account,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
        }
    }
    ///<p>Create an external account for a given account.</p>
    pub fn post_accounts_account_external_accounts(
        &self,
        account: String,
    ) -> request_model::PostAccountsAccountExternalAccountsRequest {
        request_model::PostAccountsAccountExternalAccountsRequest {
            client: &self,
            account,
        }
    }
    ///<p>Retrieve a specified external account for a given account.</p>
    pub fn get_accounts_account_external_accounts_id(
        &self,
        account: String,
        id: String,
    ) -> request_model::GetAccountsAccountExternalAccountsIdRequest {
        request_model::GetAccountsAccountExternalAccountsIdRequest {
            client: &self,
            account,
            expand: None,
            id,
        }
    }
    /**<p>Updates the metadata, account holder name, account holder type of a bank account belonging to a <a href="/docs/connect/custom-accounts">Custom account</a>, and optionally sets it as the default for its currency. Other bank account details are not editable by design.</p>

<p>You can re-enable a disabled bank account by performing an update call without providing any arguments or changes.</p>*/
    pub fn post_accounts_account_external_accounts_id(
        &self,
        account: String,
        id: String,
    ) -> request_model::PostAccountsAccountExternalAccountsIdRequest {
        request_model::PostAccountsAccountExternalAccountsIdRequest {
            client: &self,
            account,
            id,
        }
    }
    ///<p>Delete a specified external account for a given account.</p>
    pub fn delete_accounts_account_external_accounts_id(
        &self,
        account: String,
        id: String,
    ) -> request_model::DeleteAccountsAccountExternalAccountsIdRequest {
        request_model::DeleteAccountsAccountExternalAccountsIdRequest {
            client: &self,
            account,
            id,
        }
    }
    /**<p>Creates a single-use login link for an Express account to access their Stripe dashboard.</p>

<p><strong>You may only create login links for <a href="/docs/connect/express-accounts">Express accounts</a> connected to your platform</strong>.</p>*/
    pub fn post_accounts_account_login_links(
        &self,
        account: String,
    ) -> request_model::PostAccountsAccountLoginLinksRequest {
        request_model::PostAccountsAccountLoginLinksRequest {
            client: &self,
            account,
        }
    }
    ///<p>Returns a list of people associated with the account’s legal entity. The people are returned sorted by creation date, with the most recent people appearing first.</p>
    pub fn get_accounts_account_people(
        &self,
        account: String,
    ) -> request_model::GetAccountsAccountPeopleRequest {
        request_model::GetAccountsAccountPeopleRequest {
            client: &self,
            account,
            ending_before: None,
            expand: None,
            limit: None,
            relationship: None,
            starting_after: None,
        }
    }
    ///<p>Creates a new person.</p>
    pub fn post_accounts_account_people(
        &self,
        account: String,
    ) -> request_model::PostAccountsAccountPeopleRequest {
        request_model::PostAccountsAccountPeopleRequest {
            client: &self,
            account,
        }
    }
    ///<p>Retrieves an existing person.</p>
    pub fn get_accounts_account_people_person(
        &self,
        account: String,
        person: String,
    ) -> request_model::GetAccountsAccountPeoplePersonRequest {
        request_model::GetAccountsAccountPeoplePersonRequest {
            client: &self,
            account,
            expand: None,
            person,
        }
    }
    ///<p>Updates an existing person.</p>
    pub fn post_accounts_account_people_person(
        &self,
        account: String,
        person: String,
    ) -> request_model::PostAccountsAccountPeoplePersonRequest {
        request_model::PostAccountsAccountPeoplePersonRequest {
            client: &self,
            account,
            person,
        }
    }
    ///<p>Deletes an existing person’s relationship to the account’s legal entity. Any person with a relationship for an account can be deleted through the API, except if the person is the <code>account_opener</code>. If your integration is using the <code>executive</code> parameter, you cannot delete the only verified <code>executive</code> on file.</p>
    pub fn delete_accounts_account_people_person(
        &self,
        account: String,
        person: String,
    ) -> request_model::DeleteAccountsAccountPeoplePersonRequest {
        request_model::DeleteAccountsAccountPeoplePersonRequest {
            client: &self,
            account,
            person,
        }
    }
    ///<p>Returns a list of people associated with the account’s legal entity. The people are returned sorted by creation date, with the most recent people appearing first.</p>
    pub fn get_accounts_account_persons(
        &self,
        account: String,
    ) -> request_model::GetAccountsAccountPersonsRequest {
        request_model::GetAccountsAccountPersonsRequest {
            client: &self,
            account,
            ending_before: None,
            expand: None,
            limit: None,
            relationship: None,
            starting_after: None,
        }
    }
    ///<p>Creates a new person.</p>
    pub fn post_accounts_account_persons(
        &self,
        account: String,
    ) -> request_model::PostAccountsAccountPersonsRequest {
        request_model::PostAccountsAccountPersonsRequest {
            client: &self,
            account,
        }
    }
    ///<p>Retrieves an existing person.</p>
    pub fn get_accounts_account_persons_person(
        &self,
        account: String,
        person: String,
    ) -> request_model::GetAccountsAccountPersonsPersonRequest {
        request_model::GetAccountsAccountPersonsPersonRequest {
            client: &self,
            account,
            expand: None,
            person,
        }
    }
    ///<p>Updates an existing person.</p>
    pub fn post_accounts_account_persons_person(
        &self,
        account: String,
        person: String,
    ) -> request_model::PostAccountsAccountPersonsPersonRequest {
        request_model::PostAccountsAccountPersonsPersonRequest {
            client: &self,
            account,
            person,
        }
    }
    ///<p>Deletes an existing person’s relationship to the account’s legal entity. Any person with a relationship for an account can be deleted through the API, except if the person is the <code>account_opener</code>. If your integration is using the <code>executive</code> parameter, you cannot delete the only verified <code>executive</code> on file.</p>
    pub fn delete_accounts_account_persons_person(
        &self,
        account: String,
        person: String,
    ) -> request_model::DeleteAccountsAccountPersonsPersonRequest {
        request_model::DeleteAccountsAccountPersonsPersonRequest {
            client: &self,
            account,
            person,
        }
    }
    /**<p>With <a href="/docs/connect">Connect</a>, you may flag accounts as suspicious.</p>

<p>Test-mode Custom and Express accounts can be rejected at any time. Accounts created using live-mode keys may only be rejected once all balances are zero.</p>*/
    pub fn post_accounts_account_reject(
        &self,
        account: String,
    ) -> request_model::PostAccountsAccountRejectRequest {
        request_model::PostAccountsAccountRejectRequest {
            client: &self,
            account,
        }
    }
    ///<p>List apple pay domains.</p>
    pub fn get_apple_pay_domains(&self) -> request_model::GetApplePayDomainsRequest {
        request_model::GetApplePayDomainsRequest {
            client: &self,
            domain_name: None,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
        }
    }
    ///<p>Create an apple pay domain.</p>
    pub fn post_apple_pay_domains(&self) -> request_model::PostApplePayDomainsRequest {
        request_model::PostApplePayDomainsRequest {
            client: &self,
        }
    }
    ///<p>Retrieve an apple pay domain.</p>
    pub fn get_apple_pay_domains_domain(
        &self,
        domain: String,
    ) -> request_model::GetApplePayDomainsDomainRequest {
        request_model::GetApplePayDomainsDomainRequest {
            client: &self,
            domain,
            expand: None,
        }
    }
    ///<p>Delete an apple pay domain.</p>
    pub fn delete_apple_pay_domains_domain(
        &self,
        domain: String,
    ) -> request_model::DeleteApplePayDomainsDomainRequest {
        request_model::DeleteApplePayDomainsDomainRequest {
            client: &self,
            domain,
        }
    }
    ///<p>Returns a list of application fees you’ve previously collected. The application fees are returned in sorted order, with the most recent fees appearing first.</p>
    pub fn get_application_fees(&self) -> request_model::GetApplicationFeesRequest {
        request_model::GetApplicationFeesRequest {
            client: &self,
            charge: None,
            created: None,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
        }
    }
    ///<p>By default, you can see the 10 most recent refunds stored directly on the application fee object, but you can also retrieve details about a specific refund stored on the application fee.</p>
    pub fn get_application_fees_fee_refunds_id(
        &self,
        fee: String,
        id: String,
    ) -> request_model::GetApplicationFeesFeeRefundsIdRequest {
        request_model::GetApplicationFeesFeeRefundsIdRequest {
            client: &self,
            expand: None,
            fee,
            id,
        }
    }
    /**<p>Updates the specified application fee refund by setting the values of the parameters passed. Any parameters not provided will be left unchanged.</p>

<p>This request only accepts metadata as an argument.</p>*/
    pub fn post_application_fees_fee_refunds_id(
        &self,
        fee: String,
        id: String,
    ) -> request_model::PostApplicationFeesFeeRefundsIdRequest {
        request_model::PostApplicationFeesFeeRefundsIdRequest {
            client: &self,
            fee,
            id,
        }
    }
    ///<p>Retrieves the details of an application fee that your account has collected. The same information is returned when refunding the application fee.</p>
    pub fn get_application_fees_id(
        &self,
        id: String,
    ) -> request_model::GetApplicationFeesIdRequest {
        request_model::GetApplicationFeesIdRequest {
            client: &self,
            expand: None,
            id,
        }
    }
    ///
    pub fn post_application_fees_id_refund(
        &self,
        id: String,
    ) -> request_model::PostApplicationFeesIdRefundRequest {
        request_model::PostApplicationFeesIdRefundRequest {
            client: &self,
            id,
        }
    }
    ///<p>You can see a list of the refunds belonging to a specific application fee. Note that the 10 most recent refunds are always available by default on the application fee object. If you need more than those 10, you can use this API method and the <code>limit</code> and <code>starting_after</code> parameters to page through additional refunds.</p>
    pub fn get_application_fees_id_refunds(
        &self,
        id: String,
    ) -> request_model::GetApplicationFeesIdRefundsRequest {
        request_model::GetApplicationFeesIdRefundsRequest {
            client: &self,
            ending_before: None,
            expand: None,
            id,
            limit: None,
            starting_after: None,
        }
    }
    /**<p>Refunds an application fee that has previously been collected but not yet refunded.
Funds will be refunded to the Stripe account from which the fee was originally collected.</p>

<p>You can optionally refund only part of an application fee.
You can do so multiple times, until the entire fee has been refunded.</p>

<p>Once entirely refunded, an application fee can’t be refunded again.
This method will raise an error when called on an already-refunded application fee,
or when trying to refund more money than is left on an application fee.</p>*/
    pub fn post_application_fees_id_refunds(
        &self,
        id: String,
    ) -> request_model::PostApplicationFeesIdRefundsRequest {
        request_model::PostApplicationFeesIdRefundsRequest {
            client: &self,
            id,
        }
    }
    ///<p>List all secrets stored on the given scope.</p>
    pub fn get_apps_secrets(&self) -> request_model::GetAppsSecretsRequest {
        request_model::GetAppsSecretsRequest {
            client: &self,
            ending_before: None,
            expand: None,
            limit: None,
            scope: None,
            starting_after: None,
        }
    }
    ///<p>Create or replace a secret in the secret store.</p>
    pub fn post_apps_secrets(&self) -> request_model::PostAppsSecretsRequest {
        request_model::PostAppsSecretsRequest {
            client: &self,
        }
    }
    ///<p>Deletes a secret from the secret store by name and scope.</p>
    pub fn post_apps_secrets_delete(
        &self,
    ) -> request_model::PostAppsSecretsDeleteRequest {
        request_model::PostAppsSecretsDeleteRequest {
            client: &self,
        }
    }
    ///<p>Finds a secret in the secret store by name and scope.</p>
    pub fn get_apps_secrets_find(
        &self,
        name: String,
    ) -> request_model::GetAppsSecretsFindRequest {
        request_model::GetAppsSecretsFindRequest {
            client: &self,
            expand: None,
            name,
            scope: None,
        }
    }
    /**<p>Retrieves the current account balance, based on the authentication that was used to make the request.
 For a sample request, see <a href="/docs/connect/account-balances#accounting-for-negative-balances">Accounting for negative balances</a>.</p>*/
    pub fn get_balance(&self) -> request_model::GetBalanceRequest {
        request_model::GetBalanceRequest {
            client: &self,
            expand: None,
        }
    }
    /**<p>Returns a list of transactions that have contributed to the Stripe account balance (e.g., charges, transfers, and so forth). The transactions are returned in sorted order, with the most recent transactions appearing first.</p>

<p>Note that this endpoint was previously called “Balance history” and used the path <code>/v1/balance/history</code>.</p>*/
    pub fn get_balance_history(&self) -> request_model::GetBalanceHistoryRequest {
        request_model::GetBalanceHistoryRequest {
            client: &self,
            created: None,
            currency: None,
            ending_before: None,
            expand: None,
            limit: None,
            payout: None,
            source: None,
            starting_after: None,
            type_: None,
        }
    }
    /**<p>Retrieves the balance transaction with the given ID.</p>

<p>Note that this endpoint previously used the path <code>/v1/balance/history/:id</code>.</p>*/
    pub fn get_balance_history_id(
        &self,
        id: String,
    ) -> request_model::GetBalanceHistoryIdRequest {
        request_model::GetBalanceHistoryIdRequest {
            client: &self,
            expand: None,
            id,
        }
    }
    /**<p>Returns a list of transactions that have contributed to the Stripe account balance (e.g., charges, transfers, and so forth). The transactions are returned in sorted order, with the most recent transactions appearing first.</p>

<p>Note that this endpoint was previously called “Balance history” and used the path <code>/v1/balance/history</code>.</p>*/
    pub fn get_balance_transactions(
        &self,
    ) -> request_model::GetBalanceTransactionsRequest {
        request_model::GetBalanceTransactionsRequest {
            client: &self,
            created: None,
            currency: None,
            ending_before: None,
            expand: None,
            limit: None,
            payout: None,
            source: None,
            starting_after: None,
            type_: None,
        }
    }
    /**<p>Retrieves the balance transaction with the given ID.</p>

<p>Note that this endpoint previously used the path <code>/v1/balance/history/:id</code>.</p>*/
    pub fn get_balance_transactions_id(
        &self,
        id: String,
    ) -> request_model::GetBalanceTransactionsIdRequest {
        request_model::GetBalanceTransactionsIdRequest {
            client: &self,
            expand: None,
            id,
        }
    }
    ///<p>Returns a list of configurations that describe the functionality of the customer portal.</p>
    pub fn get_billing_portal_configurations(
        &self,
    ) -> request_model::GetBillingPortalConfigurationsRequest {
        request_model::GetBillingPortalConfigurationsRequest {
            client: &self,
            active: None,
            ending_before: None,
            expand: None,
            is_default: None,
            limit: None,
            starting_after: None,
        }
    }
    ///<p>Creates a configuration that describes the functionality and behavior of a PortalSession</p>
    pub fn post_billing_portal_configurations(
        &self,
    ) -> request_model::PostBillingPortalConfigurationsRequest {
        request_model::PostBillingPortalConfigurationsRequest {
            client: &self,
        }
    }
    ///<p>Retrieves a configuration that describes the functionality of the customer portal.</p>
    pub fn get_billing_portal_configurations_configuration(
        &self,
        configuration: String,
    ) -> request_model::GetBillingPortalConfigurationsConfigurationRequest {
        request_model::GetBillingPortalConfigurationsConfigurationRequest {
            client: &self,
            configuration,
            expand: None,
        }
    }
    ///<p>Updates a configuration that describes the functionality of the customer portal.</p>
    pub fn post_billing_portal_configurations_configuration(
        &self,
        configuration: String,
    ) -> request_model::PostBillingPortalConfigurationsConfigurationRequest {
        request_model::PostBillingPortalConfigurationsConfigurationRequest {
            client: &self,
            configuration,
        }
    }
    ///<p>Creates a session of the customer portal.</p>
    pub fn post_billing_portal_sessions(
        &self,
    ) -> request_model::PostBillingPortalSessionsRequest {
        request_model::PostBillingPortalSessionsRequest {
            client: &self,
        }
    }
    ///<p>Returns a list of charges you’ve previously created. The charges are returned in sorted order, with the most recent charges appearing first.</p>
    pub fn get_charges(&self) -> request_model::GetChargesRequest {
        request_model::GetChargesRequest {
            client: &self,
            created: None,
            customer: None,
            ending_before: None,
            expand: None,
            limit: None,
            payment_intent: None,
            starting_after: None,
            transfer_group: None,
        }
    }
    ///<p>To charge a credit card or other payment source, you create a <code>Charge</code> object. If your API key is in test mode, the supplied payment source (e.g., card) won’t actually be charged, although everything else will occur as if in live mode. (Stripe assumes that the charge would have completed successfully).</p>
    pub fn post_charges(&self) -> request_model::PostChargesRequest {
        request_model::PostChargesRequest {
            client: &self,
        }
    }
    /**<p>Search for charges you’ve previously created using Stripe’s <a href="/docs/search#search-query-language">Search Query Language</a>.
Don’t use search in read-after-write flows where strict consistency is necessary. Under normal operating
conditions, data is searchable in less than a minute. Occasionally, propagation of new or updated data can be up
to an hour behind during outages. Search functionality is not available to merchants in India.</p>*/
    pub fn get_charges_search(
        &self,
        query: String,
    ) -> request_model::GetChargesSearchRequest {
        request_model::GetChargesSearchRequest {
            client: &self,
            expand: None,
            limit: None,
            page: None,
            query,
        }
    }
    ///<p>Retrieves the details of a charge that has previously been created. Supply the unique charge ID that was returned from your previous request, and Stripe will return the corresponding charge information. The same information is returned when creating or refunding the charge.</p>
    pub fn get_charges_charge(
        &self,
        charge: String,
    ) -> request_model::GetChargesChargeRequest {
        request_model::GetChargesChargeRequest {
            client: &self,
            charge,
            expand: None,
        }
    }
    ///<p>Updates the specified charge by setting the values of the parameters passed. Any parameters not provided will be left unchanged.</p>
    pub fn post_charges_charge(
        &self,
        charge: String,
    ) -> request_model::PostChargesChargeRequest {
        request_model::PostChargesChargeRequest {
            client: &self,
            charge,
        }
    }
    /**<p>Capture the payment of an existing, uncaptured, charge. This is the second half of the two-step payment flow, where first you <a href="#create_charge">created a charge</a> with the capture option set to false.</p>

<p>Uncaptured payments expire a set number of days after they are created (<a href="/docs/charges/placing-a-hold">7 by default</a>). If they are not captured by that point in time, they will be marked as refunded and will no longer be capturable.</p>*/
    pub fn post_charges_charge_capture(
        &self,
        charge: String,
    ) -> request_model::PostChargesChargeCaptureRequest {
        request_model::PostChargesChargeCaptureRequest {
            client: &self,
            charge,
        }
    }
    ///<p>Retrieve a dispute for a specified charge.</p>
    pub fn get_charges_charge_dispute(
        &self,
        charge: String,
    ) -> request_model::GetChargesChargeDisputeRequest {
        request_model::GetChargesChargeDisputeRequest {
            client: &self,
            charge,
            expand: None,
        }
    }
    ///
    pub fn post_charges_charge_dispute(
        &self,
        charge: String,
    ) -> request_model::PostChargesChargeDisputeRequest {
        request_model::PostChargesChargeDisputeRequest {
            client: &self,
            charge,
        }
    }
    ///
    pub fn post_charges_charge_dispute_close(
        &self,
        charge: String,
    ) -> request_model::PostChargesChargeDisputeCloseRequest {
        request_model::PostChargesChargeDisputeCloseRequest {
            client: &self,
            charge,
        }
    }
    /**<p>When you create a new refund, you must specify a Charge or a PaymentIntent object on which to create it.</p>

<p>Creating a new refund will refund a charge that has previously been created but not yet refunded.
Funds will be refunded to the credit or debit card that was originally charged.</p>

<p>You can optionally refund only part of a charge.
You can do so multiple times, until the entire charge has been refunded.</p>

<p>Once entirely refunded, a charge can’t be refunded again.
This method will raise an error when called on an already-refunded charge,
or when trying to refund more money than is left on a charge.</p>*/
    pub fn post_charges_charge_refund(
        &self,
        charge: String,
    ) -> request_model::PostChargesChargeRefundRequest {
        request_model::PostChargesChargeRefundRequest {
            client: &self,
            charge,
        }
    }
    ///<p>You can see a list of the refunds belonging to a specific charge. Note that the 10 most recent refunds are always available by default on the charge object. If you need more than those 10, you can use this API method and the <code>limit</code> and <code>starting_after</code> parameters to page through additional refunds.</p>
    pub fn get_charges_charge_refunds(
        &self,
        charge: String,
    ) -> request_model::GetChargesChargeRefundsRequest {
        request_model::GetChargesChargeRefundsRequest {
            client: &self,
            charge,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
        }
    }
    ///<p>Create a refund.</p>
    pub fn post_charges_charge_refunds(
        &self,
        charge: String,
    ) -> request_model::PostChargesChargeRefundsRequest {
        request_model::PostChargesChargeRefundsRequest {
            client: &self,
            charge,
        }
    }
    ///<p>Retrieves the details of an existing refund.</p>
    pub fn get_charges_charge_refunds_refund(
        &self,
        charge: String,
        refund: String,
    ) -> request_model::GetChargesChargeRefundsRefundRequest {
        request_model::GetChargesChargeRefundsRefundRequest {
            client: &self,
            charge,
            expand: None,
            refund,
        }
    }
    ///<p>Update a specified refund.</p>
    pub fn post_charges_charge_refunds_refund(
        &self,
        charge: String,
        refund: String,
    ) -> request_model::PostChargesChargeRefundsRefundRequest {
        request_model::PostChargesChargeRefundsRefundRequest {
            client: &self,
            charge,
            refund,
        }
    }
    ///<p>Returns a list of Checkout Sessions.</p>
    pub fn get_checkout_sessions(&self) -> request_model::GetCheckoutSessionsRequest {
        request_model::GetCheckoutSessionsRequest {
            client: &self,
            customer: None,
            customer_details: None,
            ending_before: None,
            expand: None,
            limit: None,
            payment_intent: None,
            starting_after: None,
            subscription: None,
        }
    }
    ///<p>Creates a Session object.</p>
    pub fn post_checkout_sessions(&self) -> request_model::PostCheckoutSessionsRequest {
        request_model::PostCheckoutSessionsRequest {
            client: &self,
        }
    }
    ///<p>Retrieves a Session object.</p>
    pub fn get_checkout_sessions_session(
        &self,
        session: String,
    ) -> request_model::GetCheckoutSessionsSessionRequest {
        request_model::GetCheckoutSessionsSessionRequest {
            client: &self,
            expand: None,
            session,
        }
    }
    /**<p>A Session can be expired when it is in one of these statuses: <code>open</code> </p>

<p>After it expires, a customer can’t complete a Session and customers loading the Session see a message saying the Session is expired.</p>*/
    pub fn post_checkout_sessions_session_expire(
        &self,
        session: String,
    ) -> request_model::PostCheckoutSessionsSessionExpireRequest {
        request_model::PostCheckoutSessionsSessionExpireRequest {
            client: &self,
            session,
        }
    }
    ///<p>When retrieving a Checkout Session, there is an includable <strong>line_items</strong> property containing the first handful of those items. There is also a URL where you can retrieve the full (paginated) list of line items.</p>
    pub fn get_checkout_sessions_session_line_items(
        &self,
        session: String,
    ) -> request_model::GetCheckoutSessionsSessionLineItemsRequest {
        request_model::GetCheckoutSessionsSessionLineItemsRequest {
            client: &self,
            ending_before: None,
            expand: None,
            limit: None,
            session,
            starting_after: None,
        }
    }
    ///<p>Lists all Country Spec objects available in the API.</p>
    pub fn get_country_specs(&self) -> request_model::GetCountrySpecsRequest {
        request_model::GetCountrySpecsRequest {
            client: &self,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
        }
    }
    ///<p>Returns a Country Spec for a given Country code.</p>
    pub fn get_country_specs_country(
        &self,
        country: String,
    ) -> request_model::GetCountrySpecsCountryRequest {
        request_model::GetCountrySpecsCountryRequest {
            client: &self,
            country,
            expand: None,
        }
    }
    ///<p>Returns a list of your coupons.</p>
    pub fn get_coupons(&self) -> request_model::GetCouponsRequest {
        request_model::GetCouponsRequest {
            client: &self,
            created: None,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
        }
    }
    /**<p>You can create coupons easily via the <a href="https://dashboard.stripe.com/coupons">coupon management</a> page of the Stripe dashboard. Coupon creation is also accessible via the API if you need to create coupons on the fly.</p>

<p>A coupon has either a <code>percent_off</code> or an <code>amount_off</code> and <code>currency</code>. If you set an <code>amount_off</code>, that amount will be subtracted from any invoice’s subtotal. For example, an invoice with a subtotal of <currency>100</currency> will have a final total of <currency>0</currency> if a coupon with an <code>amount_off</code> of <amount>200</amount> is applied to it and an invoice with a subtotal of <currency>300</currency> will have a final total of <currency>100</currency> if a coupon with an <code>amount_off</code> of <amount>200</amount> is applied to it.</p>*/
    pub fn post_coupons(&self) -> request_model::PostCouponsRequest {
        request_model::PostCouponsRequest {
            client: &self,
        }
    }
    ///<p>Retrieves the coupon with the given ID.</p>
    pub fn get_coupons_coupon(
        &self,
        coupon: String,
    ) -> request_model::GetCouponsCouponRequest {
        request_model::GetCouponsCouponRequest {
            client: &self,
            coupon,
            expand: None,
        }
    }
    ///<p>Updates the metadata of a coupon. Other coupon details (currency, duration, amount_off) are, by design, not editable.</p>
    pub fn post_coupons_coupon(
        &self,
        coupon: String,
    ) -> request_model::PostCouponsCouponRequest {
        request_model::PostCouponsCouponRequest {
            client: &self,
            coupon,
        }
    }
    ///<p>You can delete coupons via the <a href="https://dashboard.stripe.com/coupons">coupon management</a> page of the Stripe dashboard. However, deleting a coupon does not affect any customers who have already applied the coupon; it means that new customers can’t redeem the coupon. You can also delete coupons via the API.</p>
    pub fn delete_coupons_coupon(
        &self,
        coupon: String,
    ) -> request_model::DeleteCouponsCouponRequest {
        request_model::DeleteCouponsCouponRequest {
            client: &self,
            coupon,
        }
    }
    ///<p>Returns a list of credit notes.</p>
    pub fn get_credit_notes(&self) -> request_model::GetCreditNotesRequest {
        request_model::GetCreditNotesRequest {
            client: &self,
            customer: None,
            ending_before: None,
            expand: None,
            invoice: None,
            limit: None,
            starting_after: None,
        }
    }
    /**<p>Issue a credit note to adjust the amount of a finalized invoice. For a <code>status=open</code> invoice, a credit note reduces
its <code>amount_due</code>. For a <code>status=paid</code> invoice, a credit note does not affect its <code>amount_due</code>. Instead, it can result
in any combination of the following:</p>

<ul>
<li>Refund: create a new refund (using <code>refund_amount</code>) or link an existing refund (using <code>refund</code>).</li>
<li>Customer balance credit: credit the customer’s balance (using <code>credit_amount</code>) which will be automatically applied to their next invoice when it’s finalized.</li>
<li>Outside of Stripe credit: record the amount that is or will be credited outside of Stripe (using <code>out_of_band_amount</code>).</li>
</ul>

<p>For post-payment credit notes the sum of the refund, credit and outside of Stripe amounts must equal the credit note total.</p>

<p>You may issue multiple credit notes for an invoice. Each credit note will increment the invoice’s <code>pre_payment_credit_notes_amount</code>
or <code>post_payment_credit_notes_amount</code> depending on its <code>status</code> at the time of credit note creation.</p>*/
    pub fn post_credit_notes(&self) -> request_model::PostCreditNotesRequest {
        request_model::PostCreditNotesRequest {
            client: &self,
        }
    }
    ///<p>Get a preview of a credit note without creating it.</p>
    pub fn get_credit_notes_preview(
        &self,
        invoice: String,
    ) -> request_model::GetCreditNotesPreviewRequest {
        request_model::GetCreditNotesPreviewRequest {
            client: &self,
            amount: None,
            credit_amount: None,
            expand: None,
            invoice,
            lines: None,
            memo: None,
            metadata: None,
            out_of_band_amount: None,
            reason: None,
            refund: None,
            refund_amount: None,
        }
    }
    ///<p>When retrieving a credit note preview, you’ll get a <strong>lines</strong> property containing the first handful of those items. This URL you can retrieve the full (paginated) list of line items.</p>
    pub fn get_credit_notes_preview_lines(
        &self,
        invoice: String,
    ) -> request_model::GetCreditNotesPreviewLinesRequest {
        request_model::GetCreditNotesPreviewLinesRequest {
            client: &self,
            amount: None,
            credit_amount: None,
            ending_before: None,
            expand: None,
            invoice,
            limit: None,
            lines: None,
            memo: None,
            metadata: None,
            out_of_band_amount: None,
            reason: None,
            refund: None,
            refund_amount: None,
            starting_after: None,
        }
    }
    ///<p>When retrieving a credit note, you’ll get a <strong>lines</strong> property containing the the first handful of those items. There is also a URL where you can retrieve the full (paginated) list of line items.</p>
    pub fn get_credit_notes_credit_note_lines(
        &self,
        credit_note: String,
    ) -> request_model::GetCreditNotesCreditNoteLinesRequest {
        request_model::GetCreditNotesCreditNoteLinesRequest {
            client: &self,
            credit_note,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
        }
    }
    ///<p>Retrieves the credit note object with the given identifier.</p>
    pub fn get_credit_notes_id(
        &self,
        id: String,
    ) -> request_model::GetCreditNotesIdRequest {
        request_model::GetCreditNotesIdRequest {
            client: &self,
            expand: None,
            id,
        }
    }
    ///<p>Updates an existing credit note.</p>
    pub fn post_credit_notes_id(
        &self,
        id: String,
    ) -> request_model::PostCreditNotesIdRequest {
        request_model::PostCreditNotesIdRequest {
            client: &self,
            id,
        }
    }
    ///<p>Marks a credit note as void. Learn more about <a href="/docs/billing/invoices/credit-notes#voiding">voiding credit notes</a>.</p>
    pub fn post_credit_notes_id_void(
        &self,
        id: String,
    ) -> request_model::PostCreditNotesIdVoidRequest {
        request_model::PostCreditNotesIdVoidRequest {
            client: &self,
            id,
        }
    }
    ///<p>Returns a list of your customers. The customers are returned sorted by creation date, with the most recent customers appearing first.</p>
    pub fn get_customers(&self) -> request_model::GetCustomersRequest {
        request_model::GetCustomersRequest {
            client: &self,
            created: None,
            email: None,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
            test_clock: None,
        }
    }
    ///<p>Creates a new customer object.</p>
    pub fn post_customers(&self) -> request_model::PostCustomersRequest {
        request_model::PostCustomersRequest {
            client: &self,
        }
    }
    /**<p>Search for customers you’ve previously created using Stripe’s <a href="/docs/search#search-query-language">Search Query Language</a>.
Don’t use search in read-after-write flows where strict consistency is necessary. Under normal operating
conditions, data is searchable in less than a minute. Occasionally, propagation of new or updated data can be up
to an hour behind during outages. Search functionality is not available to merchants in India.</p>*/
    pub fn get_customers_search(
        &self,
        query: String,
    ) -> request_model::GetCustomersSearchRequest {
        request_model::GetCustomersSearchRequest {
            client: &self,
            expand: None,
            limit: None,
            page: None,
            query,
        }
    }
    ///<p>Retrieves a Customer object.</p>
    pub fn get_customers_customer(
        &self,
        customer: String,
    ) -> request_model::GetCustomersCustomerRequest {
        request_model::GetCustomersCustomerRequest {
            client: &self,
            customer,
            expand: None,
        }
    }
    /**<p>Updates the specified customer by setting the values of the parameters passed. Any parameters not provided will be left unchanged. For example, if you pass the <strong>source</strong> parameter, that becomes the customer’s active source (e.g., a card) to be used for all charges in the future. When you update a customer to a new valid card source by passing the <strong>source</strong> parameter: for each of the customer’s current subscriptions, if the subscription bills automatically and is in the <code>past_due</code> state, then the latest open invoice for the subscription with automatic collection enabled will be retried. This retry will not count as an automatic retry, and will not affect the next regularly scheduled payment for the invoice. Changing the <strong>default_source</strong> for a customer will not trigger this behavior.</p>

<p>This request accepts mostly the same arguments as the customer creation call.</p>*/
    pub fn post_customers_customer(
        &self,
        customer: String,
    ) -> request_model::PostCustomersCustomerRequest {
        request_model::PostCustomersCustomerRequest {
            client: &self,
            customer,
        }
    }
    ///<p>Permanently deletes a customer. It cannot be undone. Also immediately cancels any active subscriptions on the customer.</p>
    pub fn delete_customers_customer(
        &self,
        customer: String,
    ) -> request_model::DeleteCustomersCustomerRequest {
        request_model::DeleteCustomersCustomerRequest {
            client: &self,
            customer,
        }
    }
    ///<p>Returns a list of transactions that updated the customer’s <a href="/docs/billing/customer/balance">balances</a>.</p>
    pub fn get_customers_customer_balance_transactions(
        &self,
        customer: String,
    ) -> request_model::GetCustomersCustomerBalanceTransactionsRequest {
        request_model::GetCustomersCustomerBalanceTransactionsRequest {
            client: &self,
            customer,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
        }
    }
    ///<p>Creates an immutable transaction that updates the customer’s credit <a href="/docs/billing/customer/balance">balance</a>.</p>
    pub fn post_customers_customer_balance_transactions(
        &self,
        customer: String,
    ) -> request_model::PostCustomersCustomerBalanceTransactionsRequest {
        request_model::PostCustomersCustomerBalanceTransactionsRequest {
            client: &self,
            customer,
        }
    }
    ///<p>Retrieves a specific customer balance transaction that updated the customer’s <a href="/docs/billing/customer/balance">balances</a>.</p>
    pub fn get_customers_customer_balance_transactions_transaction(
        &self,
        customer: String,
        transaction: String,
    ) -> request_model::GetCustomersCustomerBalanceTransactionsTransactionRequest {
        request_model::GetCustomersCustomerBalanceTransactionsTransactionRequest {
            client: &self,
            customer,
            expand: None,
            transaction,
        }
    }
    ///<p>Most credit balance transaction fields are immutable, but you may update its <code>description</code> and <code>metadata</code>.</p>
    pub fn post_customers_customer_balance_transactions_transaction(
        &self,
        customer: String,
        transaction: String,
    ) -> request_model::PostCustomersCustomerBalanceTransactionsTransactionRequest {
        request_model::PostCustomersCustomerBalanceTransactionsTransactionRequest {
            client: &self,
            customer,
            transaction,
        }
    }
    ///<p>You can see a list of the bank accounts belonging to a Customer. Note that the 10 most recent sources are always available by default on the Customer. If you need more than those 10, you can use this API method and the <code>limit</code> and <code>starting_after</code> parameters to page through additional bank accounts.</p>
    pub fn get_customers_customer_bank_accounts(
        &self,
        customer: String,
    ) -> request_model::GetCustomersCustomerBankAccountsRequest {
        request_model::GetCustomersCustomerBankAccountsRequest {
            client: &self,
            customer,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
        }
    }
    /**<p>When you create a new credit card, you must specify a customer or recipient on which to create it.</p>

<p>If the card’s owner has no default card, then the new card will become the default.
However, if the owner already has a default, then it will not change.
To change the default, you should <a href="/docs/api#update_customer">update the customer</a> to have a new <code>default_source</code>.</p>*/
    pub fn post_customers_customer_bank_accounts(
        &self,
        customer: String,
    ) -> request_model::PostCustomersCustomerBankAccountsRequest {
        request_model::PostCustomersCustomerBankAccountsRequest {
            client: &self,
            customer,
        }
    }
    ///<p>By default, you can see the 10 most recent sources stored on a Customer directly on the object, but you can also retrieve details about a specific bank account stored on the Stripe account.</p>
    pub fn get_customers_customer_bank_accounts_id(
        &self,
        customer: String,
        id: String,
    ) -> request_model::GetCustomersCustomerBankAccountsIdRequest {
        request_model::GetCustomersCustomerBankAccountsIdRequest {
            client: &self,
            customer,
            expand: None,
            id,
        }
    }
    ///<p>Update a specified source for a given customer.</p>
    pub fn post_customers_customer_bank_accounts_id(
        &self,
        customer: String,
        id: String,
    ) -> request_model::PostCustomersCustomerBankAccountsIdRequest {
        request_model::PostCustomersCustomerBankAccountsIdRequest {
            client: &self,
            customer,
            id,
        }
    }
    ///<p>Delete a specified source for a given customer.</p>
    pub fn delete_customers_customer_bank_accounts_id(
        &self,
        customer: String,
        id: String,
    ) -> request_model::DeleteCustomersCustomerBankAccountsIdRequest {
        request_model::DeleteCustomersCustomerBankAccountsIdRequest {
            client: &self,
            customer,
            id,
        }
    }
    ///<p>Verify a specified bank account for a given customer.</p>
    pub fn post_customers_customer_bank_accounts_id_verify(
        &self,
        customer: String,
        id: String,
    ) -> request_model::PostCustomersCustomerBankAccountsIdVerifyRequest {
        request_model::PostCustomersCustomerBankAccountsIdVerifyRequest {
            client: &self,
            customer,
            id,
        }
    }
    /**<p>You can see a list of the cards belonging to a customer.
Note that the 10 most recent sources are always available on the <code>Customer</code> object.
If you need more than those 10, you can use this API method and the <code>limit</code> and <code>starting_after</code> parameters to page through additional cards.</p>*/
    pub fn get_customers_customer_cards(
        &self,
        customer: String,
    ) -> request_model::GetCustomersCustomerCardsRequest {
        request_model::GetCustomersCustomerCardsRequest {
            client: &self,
            customer,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
        }
    }
    /**<p>When you create a new credit card, you must specify a customer or recipient on which to create it.</p>

<p>If the card’s owner has no default card, then the new card will become the default.
However, if the owner already has a default, then it will not change.
To change the default, you should <a href="/docs/api#update_customer">update the customer</a> to have a new <code>default_source</code>.</p>*/
    pub fn post_customers_customer_cards(
        &self,
        customer: String,
    ) -> request_model::PostCustomersCustomerCardsRequest {
        request_model::PostCustomersCustomerCardsRequest {
            client: &self,
            customer,
        }
    }
    ///<p>You can always see the 10 most recent cards directly on a customer; this method lets you retrieve details about a specific card stored on the customer.</p>
    pub fn get_customers_customer_cards_id(
        &self,
        customer: String,
        id: String,
    ) -> request_model::GetCustomersCustomerCardsIdRequest {
        request_model::GetCustomersCustomerCardsIdRequest {
            client: &self,
            customer,
            expand: None,
            id,
        }
    }
    ///<p>Update a specified source for a given customer.</p>
    pub fn post_customers_customer_cards_id(
        &self,
        customer: String,
        id: String,
    ) -> request_model::PostCustomersCustomerCardsIdRequest {
        request_model::PostCustomersCustomerCardsIdRequest {
            client: &self,
            customer,
            id,
        }
    }
    ///<p>Delete a specified source for a given customer.</p>
    pub fn delete_customers_customer_cards_id(
        &self,
        customer: String,
        id: String,
    ) -> request_model::DeleteCustomersCustomerCardsIdRequest {
        request_model::DeleteCustomersCustomerCardsIdRequest {
            client: &self,
            customer,
            id,
        }
    }
    ///<p>Retrieves a customer’s cash balance.</p>
    pub fn get_customers_customer_cash_balance(
        &self,
        customer: String,
    ) -> request_model::GetCustomersCustomerCashBalanceRequest {
        request_model::GetCustomersCustomerCashBalanceRequest {
            client: &self,
            customer,
            expand: None,
        }
    }
    ///<p>Changes the settings on a customer’s cash balance.</p>
    pub fn post_customers_customer_cash_balance(
        &self,
        customer: String,
    ) -> request_model::PostCustomersCustomerCashBalanceRequest {
        request_model::PostCustomersCustomerCashBalanceRequest {
            client: &self,
            customer,
        }
    }
    ///<p>Returns a list of transactions that modified the customer’s <a href="/docs/payments/customer-balance">cash balance</a>.</p>
    pub fn get_customers_customer_cash_balance_transactions(
        &self,
        customer: String,
    ) -> request_model::GetCustomersCustomerCashBalanceTransactionsRequest {
        request_model::GetCustomersCustomerCashBalanceTransactionsRequest {
            client: &self,
            customer,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
        }
    }
    ///<p>Retrieves a specific cash balance transaction, which updated the customer’s <a href="/docs/payments/customer-balance">cash balance</a>.</p>
    pub fn get_customers_customer_cash_balance_transactions_transaction(
        &self,
        customer: String,
        transaction: String,
    ) -> request_model::GetCustomersCustomerCashBalanceTransactionsTransactionRequest {
        request_model::GetCustomersCustomerCashBalanceTransactionsTransactionRequest {
            client: &self,
            customer,
            expand: None,
            transaction,
        }
    }
    ///
    pub fn get_customers_customer_discount(
        &self,
        customer: String,
    ) -> request_model::GetCustomersCustomerDiscountRequest {
        request_model::GetCustomersCustomerDiscountRequest {
            client: &self,
            customer,
            expand: None,
        }
    }
    ///<p>Removes the currently applied discount on a customer.</p>
    pub fn delete_customers_customer_discount(
        &self,
        customer: String,
    ) -> request_model::DeleteCustomersCustomerDiscountRequest {
        request_model::DeleteCustomersCustomerDiscountRequest {
            client: &self,
            customer,
        }
    }
    /**<p>Retrieve funding instructions for a customer cash balance. If funding instructions do not yet exist for the customer, new
funding instructions will be created. If funding instructions have already been created for a given customer, the same
funding instructions will be retrieved. In other words, we will return the same funding instructions each time.</p>*/
    pub fn post_customers_customer_funding_instructions(
        &self,
        customer: String,
    ) -> request_model::PostCustomersCustomerFundingInstructionsRequest {
        request_model::PostCustomersCustomerFundingInstructionsRequest {
            client: &self,
            customer,
        }
    }
    ///<p>Returns a list of PaymentMethods for a given Customer</p>
    pub fn get_customers_customer_payment_methods(
        &self,
        customer: String,
        type_: String,
    ) -> request_model::GetCustomersCustomerPaymentMethodsRequest {
        request_model::GetCustomersCustomerPaymentMethodsRequest {
            client: &self,
            customer,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
            type_,
        }
    }
    ///<p>Retrieves a PaymentMethod object for a given Customer.</p>
    pub fn get_customers_customer_payment_methods_payment_method(
        &self,
        customer: String,
        payment_method: String,
    ) -> request_model::GetCustomersCustomerPaymentMethodsPaymentMethodRequest {
        request_model::GetCustomersCustomerPaymentMethodsPaymentMethodRequest {
            client: &self,
            customer,
            expand: None,
            payment_method,
        }
    }
    ///<p>List sources for a specified customer.</p>
    pub fn get_customers_customer_sources(
        &self,
        customer: String,
    ) -> request_model::GetCustomersCustomerSourcesRequest {
        request_model::GetCustomersCustomerSourcesRequest {
            client: &self,
            customer,
            ending_before: None,
            expand: None,
            limit: None,
            object: None,
            starting_after: None,
        }
    }
    /**<p>When you create a new credit card, you must specify a customer or recipient on which to create it.</p>

<p>If the card’s owner has no default card, then the new card will become the default.
However, if the owner already has a default, then it will not change.
To change the default, you should <a href="/docs/api#update_customer">update the customer</a> to have a new <code>default_source</code>.</p>*/
    pub fn post_customers_customer_sources(
        &self,
        customer: String,
    ) -> request_model::PostCustomersCustomerSourcesRequest {
        request_model::PostCustomersCustomerSourcesRequest {
            client: &self,
            customer,
        }
    }
    ///<p>Retrieve a specified source for a given customer.</p>
    pub fn get_customers_customer_sources_id(
        &self,
        customer: String,
        id: String,
    ) -> request_model::GetCustomersCustomerSourcesIdRequest {
        request_model::GetCustomersCustomerSourcesIdRequest {
            client: &self,
            customer,
            expand: None,
            id,
        }
    }
    ///<p>Update a specified source for a given customer.</p>
    pub fn post_customers_customer_sources_id(
        &self,
        customer: String,
        id: String,
    ) -> request_model::PostCustomersCustomerSourcesIdRequest {
        request_model::PostCustomersCustomerSourcesIdRequest {
            client: &self,
            customer,
            id,
        }
    }
    ///<p>Delete a specified source for a given customer.</p>
    pub fn delete_customers_customer_sources_id(
        &self,
        customer: String,
        id: String,
    ) -> request_model::DeleteCustomersCustomerSourcesIdRequest {
        request_model::DeleteCustomersCustomerSourcesIdRequest {
            client: &self,
            customer,
            id,
        }
    }
    ///<p>Verify a specified bank account for a given customer.</p>
    pub fn post_customers_customer_sources_id_verify(
        &self,
        customer: String,
        id: String,
    ) -> request_model::PostCustomersCustomerSourcesIdVerifyRequest {
        request_model::PostCustomersCustomerSourcesIdVerifyRequest {
            client: &self,
            customer,
            id,
        }
    }
    ///<p>You can see a list of the customer’s active subscriptions. Note that the 10 most recent active subscriptions are always available by default on the customer object. If you need more than those 10, you can use the limit and starting_after parameters to page through additional subscriptions.</p>
    pub fn get_customers_customer_subscriptions(
        &self,
        customer: String,
    ) -> request_model::GetCustomersCustomerSubscriptionsRequest {
        request_model::GetCustomersCustomerSubscriptionsRequest {
            client: &self,
            customer,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
        }
    }
    ///<p>Creates a new subscription on an existing customer.</p>
    pub fn post_customers_customer_subscriptions(
        &self,
        customer: String,
    ) -> request_model::PostCustomersCustomerSubscriptionsRequest {
        request_model::PostCustomersCustomerSubscriptionsRequest {
            client: &self,
            customer,
        }
    }
    ///<p>Retrieves the subscription with the given ID.</p>
    pub fn get_customers_customer_subscriptions_subscription_exposed_id(
        &self,
        customer: String,
        subscription_exposed_id: String,
    ) -> request_model::GetCustomersCustomerSubscriptionsSubscriptionExposedIdRequest {
        request_model::GetCustomersCustomerSubscriptionsSubscriptionExposedIdRequest {
            client: &self,
            customer,
            expand: None,
            subscription_exposed_id,
        }
    }
    ///<p>Updates an existing subscription on a customer to match the specified parameters. When changing plans or quantities, we will optionally prorate the price we charge next month to make up for any price changes. To preview how the proration will be calculated, use the <a href="#upcoming_invoice">upcoming invoice</a> endpoint.</p>
    pub fn post_customers_customer_subscriptions_subscription_exposed_id(
        &self,
        customer: String,
        subscription_exposed_id: String,
    ) -> request_model::PostCustomersCustomerSubscriptionsSubscriptionExposedIdRequest {
        request_model::PostCustomersCustomerSubscriptionsSubscriptionExposedIdRequest {
            client: &self,
            customer,
            subscription_exposed_id,
        }
    }
    /**<p>Cancels a customer’s subscription. If you set the <code>at_period_end</code> parameter to <code>true</code>, the subscription will remain active until the end of the period, at which point it will be canceled and not renewed. Otherwise, with the default <code>false</code> value, the subscription is terminated immediately. In either case, the customer will not be charged again for the subscription.</p>

<p>Note, however, that any pending invoice items that you’ve created will still be charged for at the end of the period, unless manually <a href="#delete_invoiceitem">deleted</a>. If you’ve set the subscription to cancel at the end of the period, any pending prorations will also be left in place and collected at the end of the period. But if the subscription is set to cancel immediately, pending prorations will be removed.</p>

<p>By default, upon subscription cancellation, Stripe will stop automatic collection of all finalized invoices for the customer. This is intended to prevent unexpected payment attempts after the customer has canceled a subscription. However, you can resume automatic collection of the invoices manually after subscription cancellation to have us proceed. Or, you could check for unpaid invoices before allowing the customer to cancel the subscription at all.</p>*/
    pub fn delete_customers_customer_subscriptions_subscription_exposed_id(
        &self,
        customer: String,
        subscription_exposed_id: String,
    ) -> request_model::DeleteCustomersCustomerSubscriptionsSubscriptionExposedIdRequest {
        request_model::DeleteCustomersCustomerSubscriptionsSubscriptionExposedIdRequest {
            client: &self,
            customer,
            subscription_exposed_id,
        }
    }
    ///
    pub fn get_customers_customer_subscriptions_subscription_exposed_id_discount(
        &self,
        customer: String,
        subscription_exposed_id: String,
    ) -> request_model::GetCustomersCustomerSubscriptionsSubscriptionExposedIdDiscountRequest {
        request_model::GetCustomersCustomerSubscriptionsSubscriptionExposedIdDiscountRequest {
            client: &self,
            customer,
            expand: None,
            subscription_exposed_id,
        }
    }
    ///<p>Removes the currently applied discount on a customer.</p>
    pub fn delete_customers_customer_subscriptions_subscription_exposed_id_discount(
        &self,
        customer: String,
        subscription_exposed_id: String,
    ) -> request_model::DeleteCustomersCustomerSubscriptionsSubscriptionExposedIdDiscountRequest {
        request_model::DeleteCustomersCustomerSubscriptionsSubscriptionExposedIdDiscountRequest {
            client: &self,
            customer,
            subscription_exposed_id,
        }
    }
    ///<p>Returns a list of tax IDs for a customer.</p>
    pub fn get_customers_customer_tax_ids(
        &self,
        customer: String,
    ) -> request_model::GetCustomersCustomerTaxIdsRequest {
        request_model::GetCustomersCustomerTaxIdsRequest {
            client: &self,
            customer,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
        }
    }
    ///<p>Creates a new <code>TaxID</code> object for a customer.</p>
    pub fn post_customers_customer_tax_ids(
        &self,
        customer: String,
    ) -> request_model::PostCustomersCustomerTaxIdsRequest {
        request_model::PostCustomersCustomerTaxIdsRequest {
            client: &self,
            customer,
        }
    }
    ///<p>Retrieves the <code>TaxID</code> object with the given identifier.</p>
    pub fn get_customers_customer_tax_ids_id(
        &self,
        customer: String,
        id: String,
    ) -> request_model::GetCustomersCustomerTaxIdsIdRequest {
        request_model::GetCustomersCustomerTaxIdsIdRequest {
            client: &self,
            customer,
            expand: None,
            id,
        }
    }
    ///<p>Deletes an existing <code>TaxID</code> object.</p>
    pub fn delete_customers_customer_tax_ids_id(
        &self,
        customer: String,
        id: String,
    ) -> request_model::DeleteCustomersCustomerTaxIdsIdRequest {
        request_model::DeleteCustomersCustomerTaxIdsIdRequest {
            client: &self,
            customer,
            id,
        }
    }
    ///<p>Returns a list of your disputes.</p>
    pub fn get_disputes(&self) -> request_model::GetDisputesRequest {
        request_model::GetDisputesRequest {
            client: &self,
            charge: None,
            created: None,
            ending_before: None,
            expand: None,
            limit: None,
            payment_intent: None,
            starting_after: None,
        }
    }
    ///<p>Retrieves the dispute with the given ID.</p>
    pub fn get_disputes_dispute(
        &self,
        dispute: String,
    ) -> request_model::GetDisputesDisputeRequest {
        request_model::GetDisputesDisputeRequest {
            client: &self,
            dispute,
            expand: None,
        }
    }
    /**<p>When you get a dispute, contacting your customer is always the best first step. If that doesn’t work, you can submit evidence to help us resolve the dispute in your favor. You can do this in your <a href="https://dashboard.stripe.com/disputes">dashboard</a>, but if you prefer, you can use the API to submit evidence programmatically.</p>

<p>Depending on your dispute type, different evidence fields will give you a better chance of winning your dispute. To figure out which evidence fields to provide, see our <a href="/docs/disputes/categories">guide to dispute types</a>.</p>*/
    pub fn post_disputes_dispute(
        &self,
        dispute: String,
    ) -> request_model::PostDisputesDisputeRequest {
        request_model::PostDisputesDisputeRequest {
            client: &self,
            dispute,
        }
    }
    /**<p>Closing the dispute for a charge indicates that you do not have any evidence to submit and are essentially dismissing the dispute, acknowledging it as lost.</p>

<p>The status of the dispute will change from <code>needs_response</code> to <code>lost</code>. <em>Closing a dispute is irreversible</em>.</p>*/
    pub fn post_disputes_dispute_close(
        &self,
        dispute: String,
    ) -> request_model::PostDisputesDisputeCloseRequest {
        request_model::PostDisputesDisputeCloseRequest {
            client: &self,
            dispute,
        }
    }
    ///<p>Creates a short-lived API key for a given resource.</p>
    pub fn post_ephemeral_keys(&self) -> request_model::PostEphemeralKeysRequest {
        request_model::PostEphemeralKeysRequest {
            client: &self,
        }
    }
    ///<p>Invalidates a short-lived API key for a given resource.</p>
    pub fn delete_ephemeral_keys_key(
        &self,
        key: String,
    ) -> request_model::DeleteEphemeralKeysKeyRequest {
        request_model::DeleteEphemeralKeysKeyRequest {
            client: &self,
            key,
        }
    }
    ///<p>List events, going back up to 30 days. Each event data is rendered according to Stripe API version at its creation time, specified in <a href="/docs/api/events/object">event object</a> <code>api_version</code> attribute (not according to your current Stripe API version or <code>Stripe-Version</code> header).</p>
    pub fn get_events(&self) -> request_model::GetEventsRequest {
        request_model::GetEventsRequest {
            client: &self,
            created: None,
            delivery_success: None,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
            type_: None,
            types: None,
        }
    }
    ///<p>Retrieves the details of an event. Supply the unique identifier of the event, which you might have received in a webhook.</p>
    pub fn get_events_id(&self, id: String) -> request_model::GetEventsIdRequest {
        request_model::GetEventsIdRequest {
            client: &self,
            expand: None,
            id,
        }
    }
    ///<p>Returns a list of objects that contain the rates at which foreign currencies are converted to one another. Only shows the currencies for which Stripe supports.</p>
    pub fn get_exchange_rates(&self) -> request_model::GetExchangeRatesRequest {
        request_model::GetExchangeRatesRequest {
            client: &self,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
        }
    }
    ///<p>Retrieves the exchange rates from the given currency to every supported currency.</p>
    pub fn get_exchange_rates_rate_id(
        &self,
        rate_id: String,
    ) -> request_model::GetExchangeRatesRateIdRequest {
        request_model::GetExchangeRatesRateIdRequest {
            client: &self,
            expand: None,
            rate_id,
        }
    }
    ///<p>Returns a list of file links.</p>
    pub fn get_file_links(&self) -> request_model::GetFileLinksRequest {
        request_model::GetFileLinksRequest {
            client: &self,
            created: None,
            ending_before: None,
            expand: None,
            expired: None,
            file: None,
            limit: None,
            starting_after: None,
        }
    }
    ///<p>Creates a new file link object.</p>
    pub fn post_file_links(&self) -> request_model::PostFileLinksRequest {
        request_model::PostFileLinksRequest {
            client: &self,
        }
    }
    ///<p>Retrieves the file link with the given ID.</p>
    pub fn get_file_links_link(
        &self,
        link: String,
    ) -> request_model::GetFileLinksLinkRequest {
        request_model::GetFileLinksLinkRequest {
            client: &self,
            expand: None,
            link,
        }
    }
    ///<p>Updates an existing file link object. Expired links can no longer be updated.</p>
    pub fn post_file_links_link(
        &self,
        link: String,
    ) -> request_model::PostFileLinksLinkRequest {
        request_model::PostFileLinksLinkRequest {
            client: &self,
            link,
        }
    }
    ///<p>Returns a list of the files that your account has access to. The files are returned sorted by creation date, with the most recently created files appearing first.</p>
    pub fn get_files(&self) -> request_model::GetFilesRequest {
        request_model::GetFilesRequest {
            client: &self,
            created: None,
            ending_before: None,
            expand: None,
            limit: None,
            purpose: None,
            starting_after: None,
        }
    }
    /**<p>To upload a file to Stripe, you’ll need to send a request of type <code>multipart/form-data</code>. The request should contain the file you would like to upload, as well as the parameters for creating a file.</p>

<p>All of Stripe’s officially supported Client libraries should have support for sending <code>multipart/form-data</code>.</p>*/
    pub fn post_files(&self) -> request_model::PostFilesRequest {
        request_model::PostFilesRequest {
            client: &self,
        }
    }
    ///<p>Retrieves the details of an existing file object. Supply the unique file ID from a file, and Stripe will return the corresponding file object. To access file contents, see the <a href="/docs/file-upload#download-file-contents">File Upload Guide</a>.</p>
    pub fn get_files_file(&self, file: String) -> request_model::GetFilesFileRequest {
        request_model::GetFilesFileRequest {
            client: &self,
            expand: None,
            file,
        }
    }
    ///<p>Returns a list of Financial Connections <code>Account</code> objects.</p>
    pub fn get_financial_connections_accounts(
        &self,
    ) -> request_model::GetFinancialConnectionsAccountsRequest {
        request_model::GetFinancialConnectionsAccountsRequest {
            client: &self,
            account_holder: None,
            ending_before: None,
            expand: None,
            limit: None,
            session: None,
            starting_after: None,
        }
    }
    ///<p>Retrieves the details of an Financial Connections <code>Account</code>.</p>
    pub fn get_financial_connections_accounts_account(
        &self,
        account: String,
    ) -> request_model::GetFinancialConnectionsAccountsAccountRequest {
        request_model::GetFinancialConnectionsAccountsAccountRequest {
            client: &self,
            account,
            expand: None,
        }
    }
    ///<p>Disables your access to a Financial Connections <code>Account</code>. You will no longer be able to access data associated with the account (e.g. balances, transactions).</p>
    pub fn post_financial_connections_accounts_account_disconnect(
        &self,
        account: String,
    ) -> request_model::PostFinancialConnectionsAccountsAccountDisconnectRequest {
        request_model::PostFinancialConnectionsAccountsAccountDisconnectRequest {
            client: &self,
            account,
        }
    }
    ///<p>Lists all owners for a given <code>Account</code></p>
    pub fn get_financial_connections_accounts_account_owners(
        &self,
        account: String,
        ownership: String,
    ) -> request_model::GetFinancialConnectionsAccountsAccountOwnersRequest {
        request_model::GetFinancialConnectionsAccountsAccountOwnersRequest {
            client: &self,
            account,
            ending_before: None,
            expand: None,
            limit: None,
            ownership,
            starting_after: None,
        }
    }
    ///<p>Refreshes the data associated with a Financial Connections <code>Account</code>.</p>
    pub fn post_financial_connections_accounts_account_refresh(
        &self,
        account: String,
    ) -> request_model::PostFinancialConnectionsAccountsAccountRefreshRequest {
        request_model::PostFinancialConnectionsAccountsAccountRefreshRequest {
            client: &self,
            account,
        }
    }
    ///<p>To launch the Financial Connections authorization flow, create a <code>Session</code>. The session’s <code>client_secret</code> can be used to launch the flow using Stripe.js.</p>
    pub fn post_financial_connections_sessions(
        &self,
    ) -> request_model::PostFinancialConnectionsSessionsRequest {
        request_model::PostFinancialConnectionsSessionsRequest {
            client: &self,
        }
    }
    ///<p>Retrieves the details of a Financial Connections <code>Session</code></p>
    pub fn get_financial_connections_sessions_session(
        &self,
        session: String,
    ) -> request_model::GetFinancialConnectionsSessionsSessionRequest {
        request_model::GetFinancialConnectionsSessionsSessionRequest {
            client: &self,
            expand: None,
            session,
        }
    }
    ///<p>List all verification reports.</p>
    pub fn get_identity_verification_reports(
        &self,
    ) -> request_model::GetIdentityVerificationReportsRequest {
        request_model::GetIdentityVerificationReportsRequest {
            client: &self,
            created: None,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
            type_: None,
            verification_session: None,
        }
    }
    ///<p>Retrieves an existing VerificationReport</p>
    pub fn get_identity_verification_reports_report(
        &self,
        report: String,
    ) -> request_model::GetIdentityVerificationReportsReportRequest {
        request_model::GetIdentityVerificationReportsReportRequest {
            client: &self,
            expand: None,
            report,
        }
    }
    ///<p>Returns a list of VerificationSessions</p>
    pub fn get_identity_verification_sessions(
        &self,
    ) -> request_model::GetIdentityVerificationSessionsRequest {
        request_model::GetIdentityVerificationSessionsRequest {
            client: &self,
            created: None,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
            status: None,
        }
    }
    /**<p>Creates a VerificationSession object.</p>

<p>After the VerificationSession is created, display a verification modal using the session <code>client_secret</code> or send your users to the session’s <code>url</code>.</p>

<p>If your API key is in test mode, verification checks won’t actually process, though everything else will occur as if in live mode.</p>

<p>Related guide: <a href="/docs/identity/verify-identity-documents">Verify your users’ identity documents</a>.</p>*/
    pub fn post_identity_verification_sessions(
        &self,
    ) -> request_model::PostIdentityVerificationSessionsRequest {
        request_model::PostIdentityVerificationSessionsRequest {
            client: &self,
        }
    }
    /**<p>Retrieves the details of a VerificationSession that was previously created.</p>

<p>When the session status is <code>requires_input</code>, you can use this method to retrieve a valid
<code>client_secret</code> or <code>url</code> to allow re-submission.</p>*/
    pub fn get_identity_verification_sessions_session(
        &self,
        session: String,
    ) -> request_model::GetIdentityVerificationSessionsSessionRequest {
        request_model::GetIdentityVerificationSessionsSessionRequest {
            client: &self,
            expand: None,
            session,
        }
    }
    /**<p>Updates a VerificationSession object.</p>

<p>When the session status is <code>requires_input</code>, you can use this method to update the
verification check and options.</p>*/
    pub fn post_identity_verification_sessions_session(
        &self,
        session: String,
    ) -> request_model::PostIdentityVerificationSessionsSessionRequest {
        request_model::PostIdentityVerificationSessionsSessionRequest {
            client: &self,
            session,
        }
    }
    /**<p>A VerificationSession object can be canceled when it is in <code>requires_input</code> <a href="/docs/identity/how-sessions-work">status</a>.</p>

<p>Once canceled, future submission attempts are disabled. This cannot be undone. <a href="/docs/identity/verification-sessions#cancel">Learn more</a>.</p>*/
    pub fn post_identity_verification_sessions_session_cancel(
        &self,
        session: String,
    ) -> request_model::PostIdentityVerificationSessionsSessionCancelRequest {
        request_model::PostIdentityVerificationSessionsSessionCancelRequest {
            client: &self,
            session,
        }
    }
    /**<p>Redact a VerificationSession to remove all collected information from Stripe. This will redact
the VerificationSession and all objects related to it, including VerificationReports, Events,
request logs, etc.</p>

<p>A VerificationSession object can be redacted when it is in <code>requires_input</code> or <code>verified</code>
<a href="/docs/identity/how-sessions-work">status</a>. Redacting a VerificationSession in <code>requires_action</code>
state will automatically cancel it.</p>

<p>The redaction process may take up to four days. When the redaction process is in progress, the
VerificationSession’s <code>redaction.status</code> field will be set to <code>processing</code>; when the process is
finished, it will change to <code>redacted</code> and an <code>identity.verification_session.redacted</code> event
will be emitted.</p>

<p>Redaction is irreversible. Redacted objects are still accessible in the Stripe API, but all the
fields that contain personal data will be replaced by the string <code>[redacted]</code> or a similar
placeholder. The <code>metadata</code> field will also be erased. Redacted objects cannot be updated or
used for any purpose.</p>

<p><a href="/docs/identity/verification-sessions#redact">Learn more</a>.</p>*/
    pub fn post_identity_verification_sessions_session_redact(
        &self,
        session: String,
    ) -> request_model::PostIdentityVerificationSessionsSessionRedactRequest {
        request_model::PostIdentityVerificationSessionsSessionRedactRequest {
            client: &self,
            session,
        }
    }
    ///<p>Returns a list of your invoice items. Invoice items are returned sorted by creation date, with the most recently created invoice items appearing first.</p>
    pub fn get_invoiceitems(&self) -> request_model::GetInvoiceitemsRequest {
        request_model::GetInvoiceitemsRequest {
            client: &self,
            created: None,
            customer: None,
            ending_before: None,
            expand: None,
            invoice: None,
            limit: None,
            pending: None,
            starting_after: None,
        }
    }
    ///<p>Creates an item to be added to a draft invoice (up to 250 items per invoice). If no invoice is specified, the item will be on the next invoice created for the customer specified.</p>
    pub fn post_invoiceitems(&self) -> request_model::PostInvoiceitemsRequest {
        request_model::PostInvoiceitemsRequest {
            client: &self,
        }
    }
    ///<p>Retrieves the invoice item with the given ID.</p>
    pub fn get_invoiceitems_invoiceitem(
        &self,
        invoiceitem: String,
    ) -> request_model::GetInvoiceitemsInvoiceitemRequest {
        request_model::GetInvoiceitemsInvoiceitemRequest {
            client: &self,
            expand: None,
            invoiceitem,
        }
    }
    ///<p>Updates the amount or description of an invoice item on an upcoming invoice. Updating an invoice item is only possible before the invoice it’s attached to is closed.</p>
    pub fn post_invoiceitems_invoiceitem(
        &self,
        invoiceitem: String,
    ) -> request_model::PostInvoiceitemsInvoiceitemRequest {
        request_model::PostInvoiceitemsInvoiceitemRequest {
            client: &self,
            invoiceitem,
        }
    }
    ///<p>Deletes an invoice item, removing it from an invoice. Deleting invoice items is only possible when they’re not attached to invoices, or if it’s attached to a draft invoice.</p>
    pub fn delete_invoiceitems_invoiceitem(
        &self,
        invoiceitem: String,
    ) -> request_model::DeleteInvoiceitemsInvoiceitemRequest {
        request_model::DeleteInvoiceitemsInvoiceitemRequest {
            client: &self,
            invoiceitem,
        }
    }
    ///<p>You can list all invoices, or list the invoices for a specific customer. The invoices are returned sorted by creation date, with the most recently created invoices appearing first.</p>
    pub fn get_invoices(&self) -> request_model::GetInvoicesRequest {
        request_model::GetInvoicesRequest {
            client: &self,
            collection_method: None,
            created: None,
            customer: None,
            due_date: None,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
            status: None,
            subscription: None,
        }
    }
    ///<p>This endpoint creates a draft invoice for a given customer. The draft invoice created pulls in all pending invoice items on that customer, including prorations. The invoice remains a draft until you <a href="#finalize_invoice">finalize</a> the invoice, which allows you to <a href="#pay_invoice">pay</a> or <a href="#send_invoice">send</a> the invoice to your customers.</p>
    pub fn post_invoices(&self) -> request_model::PostInvoicesRequest {
        request_model::PostInvoicesRequest {
            client: &self,
        }
    }
    /**<p>Search for invoices you’ve previously created using Stripe’s <a href="/docs/search#search-query-language">Search Query Language</a>.
Don’t use search in read-after-write flows where strict consistency is necessary. Under normal operating
conditions, data is searchable in less than a minute. Occasionally, propagation of new or updated data can be up
to an hour behind during outages. Search functionality is not available to merchants in India.</p>*/
    pub fn get_invoices_search(
        &self,
        query: String,
    ) -> request_model::GetInvoicesSearchRequest {
        request_model::GetInvoicesSearchRequest {
            client: &self,
            expand: None,
            limit: None,
            page: None,
            query,
        }
    }
    /**<p>At any time, you can preview the upcoming invoice for a customer. This will show you all the charges that are pending, including subscription renewal charges, invoice item charges, etc. It will also show you any discounts that are applicable to the invoice.</p>

<p>Note that when you are viewing an upcoming invoice, you are simply viewing a preview – the invoice has not yet been created. As such, the upcoming invoice will not show up in invoice listing calls, and you cannot use the API to pay or edit the invoice. If you want to change the amount that your customer will be billed, you can add, remove, or update pending invoice items, or update the customer’s discount.</p>

<p>You can preview the effects of updating a subscription, including a preview of what proration will take place. To ensure that the actual proration is calculated exactly the same as the previewed proration, you should pass a <code>proration_date</code> parameter when doing the actual subscription update. The value passed in should be the same as the <code>subscription_proration_date</code> returned on the upcoming invoice resource. The recommended way to get only the prorations being previewed is to consider only proration line items where <code>period[start]</code> is equal to the <code>subscription_proration_date</code> on the upcoming invoice resource.</p>*/
    pub fn get_invoices_upcoming(&self) -> request_model::GetInvoicesUpcomingRequest {
        request_model::GetInvoicesUpcomingRequest {
            client: &self,
            automatic_tax: None,
            coupon: None,
            currency: None,
            customer: None,
            customer_details: None,
            discounts: None,
            expand: None,
            invoice_items: None,
            schedule: None,
            subscription: None,
            subscription_billing_cycle_anchor: None,
            subscription_cancel_at: None,
            subscription_cancel_at_period_end: None,
            subscription_cancel_now: None,
            subscription_default_tax_rates: None,
            subscription_items: None,
            subscription_proration_behavior: None,
            subscription_proration_date: None,
            subscription_start_date: None,
            subscription_trial_end: None,
            subscription_trial_from_plan: None,
        }
    }
    ///<p>When retrieving an upcoming invoice, you’ll get a <strong>lines</strong> property containing the total count of line items and the first handful of those items. There is also a URL where you can retrieve the full (paginated) list of line items.</p>
    pub fn get_invoices_upcoming_lines(
        &self,
    ) -> request_model::GetInvoicesUpcomingLinesRequest {
        request_model::GetInvoicesUpcomingLinesRequest {
            client: &self,
            automatic_tax: None,
            coupon: None,
            currency: None,
            customer: None,
            customer_details: None,
            discounts: None,
            ending_before: None,
            expand: None,
            invoice_items: None,
            limit: None,
            schedule: None,
            starting_after: None,
            subscription: None,
            subscription_billing_cycle_anchor: None,
            subscription_cancel_at: None,
            subscription_cancel_at_period_end: None,
            subscription_cancel_now: None,
            subscription_default_tax_rates: None,
            subscription_items: None,
            subscription_proration_behavior: None,
            subscription_proration_date: None,
            subscription_start_date: None,
            subscription_trial_end: None,
            subscription_trial_from_plan: None,
        }
    }
    ///<p>Retrieves the invoice with the given ID.</p>
    pub fn get_invoices_invoice(
        &self,
        invoice: String,
    ) -> request_model::GetInvoicesInvoiceRequest {
        request_model::GetInvoicesInvoiceRequest {
            client: &self,
            expand: None,
            invoice,
        }
    }
    /**<p>Draft invoices are fully editable. Once an invoice is <a href="/docs/billing/invoices/workflow#finalized">finalized</a>,
monetary values, as well as <code>collection_method</code>, become uneditable.</p>

<p>If you would like to stop the Stripe Billing engine from automatically finalizing, reattempting payments on,
sending reminders for, or <a href="/docs/billing/invoices/reconciliation">automatically reconciling</a> invoices, pass
<code>auto_advance=false</code>.</p>*/
    pub fn post_invoices_invoice(
        &self,
        invoice: String,
    ) -> request_model::PostInvoicesInvoiceRequest {
        request_model::PostInvoicesInvoiceRequest {
            client: &self,
            invoice,
        }
    }
    ///<p>Permanently deletes a one-off invoice draft. This cannot be undone. Attempts to delete invoices that are no longer in a draft state will fail; once an invoice has been finalized or if an invoice is for a subscription, it must be <a href="#void_invoice">voided</a>.</p>
    pub fn delete_invoices_invoice(
        &self,
        invoice: String,
    ) -> request_model::DeleteInvoicesInvoiceRequest {
        request_model::DeleteInvoicesInvoiceRequest {
            client: &self,
            invoice,
        }
    }
    ///<p>Stripe automatically finalizes drafts before sending and attempting payment on invoices. However, if you’d like to finalize a draft invoice manually, you can do so using this method.</p>
    pub fn post_invoices_invoice_finalize(
        &self,
        invoice: String,
    ) -> request_model::PostInvoicesInvoiceFinalizeRequest {
        request_model::PostInvoicesInvoiceFinalizeRequest {
            client: &self,
            invoice,
        }
    }
    ///<p>When retrieving an invoice, you’ll get a <strong>lines</strong> property containing the total count of line items and the first handful of those items. There is also a URL where you can retrieve the full (paginated) list of line items.</p>
    pub fn get_invoices_invoice_lines(
        &self,
        invoice: String,
    ) -> request_model::GetInvoicesInvoiceLinesRequest {
        request_model::GetInvoicesInvoiceLinesRequest {
            client: &self,
            ending_before: None,
            expand: None,
            invoice,
            limit: None,
            starting_after: None,
        }
    }
    ///<p>Marking an invoice as uncollectible is useful for keeping track of bad debts that can be written off for accounting purposes.</p>
    pub fn post_invoices_invoice_mark_uncollectible(
        &self,
        invoice: String,
    ) -> request_model::PostInvoicesInvoiceMarkUncollectibleRequest {
        request_model::PostInvoicesInvoiceMarkUncollectibleRequest {
            client: &self,
            invoice,
        }
    }
    ///<p>Stripe automatically creates and then attempts to collect payment on invoices for customers on subscriptions according to your <a href="https://dashboard.stripe.com/account/billing/automatic">subscriptions settings</a>. However, if you’d like to attempt payment on an invoice out of the normal collection schedule or for some other reason, you can do so.</p>
    pub fn post_invoices_invoice_pay(
        &self,
        invoice: String,
    ) -> request_model::PostInvoicesInvoicePayRequest {
        request_model::PostInvoicesInvoicePayRequest {
            client: &self,
            invoice,
        }
    }
    /**<p>Stripe will automatically send invoices to customers according to your <a href="https://dashboard.stripe.com/account/billing/automatic">subscriptions settings</a>. However, if you’d like to manually send an invoice to your customer out of the normal schedule, you can do so. When sending invoices that have already been paid, there will be no reference to the payment in the email.</p>

<p>Requests made in test-mode result in no emails being sent, despite sending an <code>invoice.sent</code> event.</p>*/
    pub fn post_invoices_invoice_send(
        &self,
        invoice: String,
    ) -> request_model::PostInvoicesInvoiceSendRequest {
        request_model::PostInvoicesInvoiceSendRequest {
            client: &self,
            invoice,
        }
    }
    ///<p>Mark a finalized invoice as void. This cannot be undone. Voiding an invoice is similar to <a href="#delete_invoice">deletion</a>, however it only applies to finalized invoices and maintains a papertrail where the invoice can still be found.</p>
    pub fn post_invoices_invoice_void(
        &self,
        invoice: String,
    ) -> request_model::PostInvoicesInvoiceVoidRequest {
        request_model::PostInvoicesInvoiceVoidRequest {
            client: &self,
            invoice,
        }
    }
    ///<p>Returns a list of Issuing <code>Authorization</code> objects. The objects are sorted in descending order by creation date, with the most recently created object appearing first.</p>
    pub fn get_issuing_authorizations(
        &self,
    ) -> request_model::GetIssuingAuthorizationsRequest {
        request_model::GetIssuingAuthorizationsRequest {
            client: &self,
            card: None,
            cardholder: None,
            created: None,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
            status: None,
        }
    }
    ///<p>Retrieves an Issuing <code>Authorization</code> object.</p>
    pub fn get_issuing_authorizations_authorization(
        &self,
        authorization: String,
    ) -> request_model::GetIssuingAuthorizationsAuthorizationRequest {
        request_model::GetIssuingAuthorizationsAuthorizationRequest {
            client: &self,
            authorization,
            expand: None,
        }
    }
    ///<p>Updates the specified Issuing <code>Authorization</code> object by setting the values of the parameters passed. Any parameters not provided will be left unchanged.</p>
    pub fn post_issuing_authorizations_authorization(
        &self,
        authorization: String,
    ) -> request_model::PostIssuingAuthorizationsAuthorizationRequest {
        request_model::PostIssuingAuthorizationsAuthorizationRequest {
            client: &self,
            authorization,
        }
    }
    ///<p>Approves a pending Issuing <code>Authorization</code> object. This request should be made within the timeout window of the <a href="/docs/issuing/controls/real-time-authorizations">real-time authorization</a> flow.</p>
    pub fn post_issuing_authorizations_authorization_approve(
        &self,
        authorization: String,
    ) -> request_model::PostIssuingAuthorizationsAuthorizationApproveRequest {
        request_model::PostIssuingAuthorizationsAuthorizationApproveRequest {
            client: &self,
            authorization,
        }
    }
    ///<p>Declines a pending Issuing <code>Authorization</code> object. This request should be made within the timeout window of the <a href="/docs/issuing/controls/real-time-authorizations">real time authorization</a> flow.</p>
    pub fn post_issuing_authorizations_authorization_decline(
        &self,
        authorization: String,
    ) -> request_model::PostIssuingAuthorizationsAuthorizationDeclineRequest {
        request_model::PostIssuingAuthorizationsAuthorizationDeclineRequest {
            client: &self,
            authorization,
        }
    }
    ///<p>Returns a list of Issuing <code>Cardholder</code> objects. The objects are sorted in descending order by creation date, with the most recently created object appearing first.</p>
    pub fn get_issuing_cardholders(
        &self,
    ) -> request_model::GetIssuingCardholdersRequest {
        request_model::GetIssuingCardholdersRequest {
            client: &self,
            created: None,
            email: None,
            ending_before: None,
            expand: None,
            limit: None,
            phone_number: None,
            starting_after: None,
            status: None,
            type_: None,
        }
    }
    ///<p>Creates a new Issuing <code>Cardholder</code> object that can be issued cards.</p>
    pub fn post_issuing_cardholders(
        &self,
    ) -> request_model::PostIssuingCardholdersRequest {
        request_model::PostIssuingCardholdersRequest {
            client: &self,
        }
    }
    ///<p>Retrieves an Issuing <code>Cardholder</code> object.</p>
    pub fn get_issuing_cardholders_cardholder(
        &self,
        cardholder: String,
    ) -> request_model::GetIssuingCardholdersCardholderRequest {
        request_model::GetIssuingCardholdersCardholderRequest {
            client: &self,
            cardholder,
            expand: None,
        }
    }
    ///<p>Updates the specified Issuing <code>Cardholder</code> object by setting the values of the parameters passed. Any parameters not provided will be left unchanged.</p>
    pub fn post_issuing_cardholders_cardholder(
        &self,
        cardholder: String,
    ) -> request_model::PostIssuingCardholdersCardholderRequest {
        request_model::PostIssuingCardholdersCardholderRequest {
            client: &self,
            cardholder,
        }
    }
    ///<p>Returns a list of Issuing <code>Card</code> objects. The objects are sorted in descending order by creation date, with the most recently created object appearing first.</p>
    pub fn get_issuing_cards(&self) -> request_model::GetIssuingCardsRequest {
        request_model::GetIssuingCardsRequest {
            client: &self,
            cardholder: None,
            created: None,
            ending_before: None,
            exp_month: None,
            exp_year: None,
            expand: None,
            last4: None,
            limit: None,
            starting_after: None,
            status: None,
            type_: None,
        }
    }
    ///<p>Creates an Issuing <code>Card</code> object.</p>
    pub fn post_issuing_cards(&self) -> request_model::PostIssuingCardsRequest {
        request_model::PostIssuingCardsRequest {
            client: &self,
        }
    }
    ///<p>Retrieves an Issuing <code>Card</code> object.</p>
    pub fn get_issuing_cards_card(
        &self,
        card: String,
    ) -> request_model::GetIssuingCardsCardRequest {
        request_model::GetIssuingCardsCardRequest {
            client: &self,
            card,
            expand: None,
        }
    }
    ///<p>Updates the specified Issuing <code>Card</code> object by setting the values of the parameters passed. Any parameters not provided will be left unchanged.</p>
    pub fn post_issuing_cards_card(
        &self,
        card: String,
    ) -> request_model::PostIssuingCardsCardRequest {
        request_model::PostIssuingCardsCardRequest {
            client: &self,
            card,
        }
    }
    ///<p>Returns a list of Issuing <code>Dispute</code> objects. The objects are sorted in descending order by creation date, with the most recently created object appearing first.</p>
    pub fn get_issuing_disputes(&self) -> request_model::GetIssuingDisputesRequest {
        request_model::GetIssuingDisputesRequest {
            client: &self,
            created: None,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
            status: None,
            transaction: None,
        }
    }
    ///<p>Creates an Issuing <code>Dispute</code> object. Individual pieces of evidence within the <code>evidence</code> object are optional at this point. Stripe only validates that required evidence is present during submission. Refer to <a href="/docs/issuing/purchases/disputes#dispute-reasons-and-evidence">Dispute reasons and evidence</a> for more details about evidence requirements.</p>
    pub fn post_issuing_disputes(&self) -> request_model::PostIssuingDisputesRequest {
        request_model::PostIssuingDisputesRequest {
            client: &self,
        }
    }
    ///<p>Retrieves an Issuing <code>Dispute</code> object.</p>
    pub fn get_issuing_disputes_dispute(
        &self,
        dispute: String,
    ) -> request_model::GetIssuingDisputesDisputeRequest {
        request_model::GetIssuingDisputesDisputeRequest {
            client: &self,
            dispute,
            expand: None,
        }
    }
    ///<p>Updates the specified Issuing <code>Dispute</code> object by setting the values of the parameters passed. Any parameters not provided will be left unchanged. Properties on the <code>evidence</code> object can be unset by passing in an empty string.</p>
    pub fn post_issuing_disputes_dispute(
        &self,
        dispute: String,
    ) -> request_model::PostIssuingDisputesDisputeRequest {
        request_model::PostIssuingDisputesDisputeRequest {
            client: &self,
            dispute,
        }
    }
    ///<p>Submits an Issuing <code>Dispute</code> to the card network. Stripe validates that all evidence fields required for the dispute’s reason are present. For more details, see <a href="/docs/issuing/purchases/disputes#dispute-reasons-and-evidence">Dispute reasons and evidence</a>.</p>
    pub fn post_issuing_disputes_dispute_submit(
        &self,
        dispute: String,
    ) -> request_model::PostIssuingDisputesDisputeSubmitRequest {
        request_model::PostIssuingDisputesDisputeSubmitRequest {
            client: &self,
            dispute,
        }
    }
    ///<p>Returns a list of Issuing <code>Settlement</code> objects. The objects are sorted in descending order by creation date, with the most recently created object appearing first.</p>
    pub fn get_issuing_settlements(
        &self,
    ) -> request_model::GetIssuingSettlementsRequest {
        request_model::GetIssuingSettlementsRequest {
            client: &self,
            created: None,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
        }
    }
    ///<p>Retrieves an Issuing <code>Settlement</code> object.</p>
    pub fn get_issuing_settlements_settlement(
        &self,
        settlement: String,
    ) -> request_model::GetIssuingSettlementsSettlementRequest {
        request_model::GetIssuingSettlementsSettlementRequest {
            client: &self,
            expand: None,
            settlement,
        }
    }
    ///<p>Updates the specified Issuing <code>Settlement</code> object by setting the values of the parameters passed. Any parameters not provided will be left unchanged.</p>
    pub fn post_issuing_settlements_settlement(
        &self,
        settlement: String,
    ) -> request_model::PostIssuingSettlementsSettlementRequest {
        request_model::PostIssuingSettlementsSettlementRequest {
            client: &self,
            settlement,
        }
    }
    ///<p>Returns a list of Issuing <code>Transaction</code> objects. The objects are sorted in descending order by creation date, with the most recently created object appearing first.</p>
    pub fn get_issuing_transactions(
        &self,
    ) -> request_model::GetIssuingTransactionsRequest {
        request_model::GetIssuingTransactionsRequest {
            client: &self,
            card: None,
            cardholder: None,
            created: None,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
            type_: None,
        }
    }
    ///<p>Retrieves an Issuing <code>Transaction</code> object.</p>
    pub fn get_issuing_transactions_transaction(
        &self,
        transaction: String,
    ) -> request_model::GetIssuingTransactionsTransactionRequest {
        request_model::GetIssuingTransactionsTransactionRequest {
            client: &self,
            expand: None,
            transaction,
        }
    }
    ///<p>Updates the specified Issuing <code>Transaction</code> object by setting the values of the parameters passed. Any parameters not provided will be left unchanged.</p>
    pub fn post_issuing_transactions_transaction(
        &self,
        transaction: String,
    ) -> request_model::PostIssuingTransactionsTransactionRequest {
        request_model::PostIssuingTransactionsTransactionRequest {
            client: &self,
            transaction,
        }
    }
    ///<p>To launch the Financial Connections authorization flow, create a <code>Session</code>. The session’s <code>client_secret</code> can be used to launch the flow using Stripe.js.</p>
    pub fn post_link_account_sessions(
        &self,
    ) -> request_model::PostLinkAccountSessionsRequest {
        request_model::PostLinkAccountSessionsRequest {
            client: &self,
        }
    }
    ///<p>Retrieves the details of a Financial Connections <code>Session</code></p>
    pub fn get_link_account_sessions_session(
        &self,
        session: String,
    ) -> request_model::GetLinkAccountSessionsSessionRequest {
        request_model::GetLinkAccountSessionsSessionRequest {
            client: &self,
            expand: None,
            session,
        }
    }
    ///<p>Returns a list of Financial Connections <code>Account</code> objects.</p>
    pub fn get_linked_accounts(&self) -> request_model::GetLinkedAccountsRequest {
        request_model::GetLinkedAccountsRequest {
            client: &self,
            account_holder: None,
            ending_before: None,
            expand: None,
            limit: None,
            session: None,
            starting_after: None,
        }
    }
    ///<p>Retrieves the details of an Financial Connections <code>Account</code>.</p>
    pub fn get_linked_accounts_account(
        &self,
        account: String,
    ) -> request_model::GetLinkedAccountsAccountRequest {
        request_model::GetLinkedAccountsAccountRequest {
            client: &self,
            account,
            expand: None,
        }
    }
    ///<p>Disables your access to a Financial Connections <code>Account</code>. You will no longer be able to access data associated with the account (e.g. balances, transactions).</p>
    pub fn post_linked_accounts_account_disconnect(
        &self,
        account: String,
    ) -> request_model::PostLinkedAccountsAccountDisconnectRequest {
        request_model::PostLinkedAccountsAccountDisconnectRequest {
            client: &self,
            account,
        }
    }
    ///<p>Lists all owners for a given <code>Account</code></p>
    pub fn get_linked_accounts_account_owners(
        &self,
        account: String,
        ownership: String,
    ) -> request_model::GetLinkedAccountsAccountOwnersRequest {
        request_model::GetLinkedAccountsAccountOwnersRequest {
            client: &self,
            account,
            ending_before: None,
            expand: None,
            limit: None,
            ownership,
            starting_after: None,
        }
    }
    ///<p>Refreshes the data associated with a Financial Connections <code>Account</code>.</p>
    pub fn post_linked_accounts_account_refresh(
        &self,
        account: String,
    ) -> request_model::PostLinkedAccountsAccountRefreshRequest {
        request_model::PostLinkedAccountsAccountRefreshRequest {
            client: &self,
            account,
        }
    }
    ///<p>Retrieves a Mandate object.</p>
    pub fn get_mandates_mandate(
        &self,
        mandate: String,
    ) -> request_model::GetMandatesMandateRequest {
        request_model::GetMandatesMandateRequest {
            client: &self,
            expand: None,
            mandate,
        }
    }
    ///<p>Returns a list of your orders. The orders are returned sorted by creation date, with the most recently created orders appearing first.</p>
    pub fn get_orders(&self) -> request_model::GetOrdersRequest {
        request_model::GetOrdersRequest {
            client: &self,
            customer: None,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
        }
    }
    ///<p>Creates a new <code>open</code> order object.</p>
    pub fn post_orders(&self) -> request_model::PostOrdersRequest {
        request_model::PostOrdersRequest {
            client: &self,
        }
    }
    ///<p>Retrieves the details of an existing order. Supply the unique order ID from either an order creation request or the order list, and Stripe will return the corresponding order information.</p>
    pub fn get_orders_id(&self, id: String) -> request_model::GetOrdersIdRequest {
        request_model::GetOrdersIdRequest {
            client: &self,
            expand: None,
            id,
        }
    }
    ///<p>Updates the specific order by setting the values of the parameters passed. Any parameters not provided will be left unchanged.</p>
    pub fn post_orders_id(&self, id: String) -> request_model::PostOrdersIdRequest {
        request_model::PostOrdersIdRequest {
            client: &self,
            id,
        }
    }
    ///<p>Cancels the order as well as the payment intent if one is attached.</p>
    pub fn post_orders_id_cancel(
        &self,
        id: String,
    ) -> request_model::PostOrdersIdCancelRequest {
        request_model::PostOrdersIdCancelRequest {
            client: &self,
            id,
        }
    }
    ///<p>When retrieving an order, there is an includable <strong>line_items</strong> property containing the first handful of those items. There is also a URL where you can retrieve the full (paginated) list of line items.</p>
    pub fn get_orders_id_line_items(
        &self,
        id: String,
    ) -> request_model::GetOrdersIdLineItemsRequest {
        request_model::GetOrdersIdLineItemsRequest {
            client: &self,
            ending_before: None,
            expand: None,
            id,
            limit: None,
            starting_after: None,
        }
    }
    ///<p>Reopens a <code>submitted</code> order.</p>
    pub fn post_orders_id_reopen(
        &self,
        id: String,
    ) -> request_model::PostOrdersIdReopenRequest {
        request_model::PostOrdersIdReopenRequest {
            client: &self,
            id,
        }
    }
    ///<p>Submitting an Order transitions the status to <code>processing</code> and creates a PaymentIntent object so the order can be paid. If the Order has an <code>amount_total</code> of 0, no PaymentIntent object will be created. Once the order is submitted, its contents cannot be changed, unless the <a href="#reopen_order">reopen</a> method is called.</p>
    pub fn post_orders_id_submit(
        &self,
        id: String,
    ) -> request_model::PostOrdersIdSubmitRequest {
        request_model::PostOrdersIdSubmitRequest {
            client: &self,
            id,
        }
    }
    ///<p>Returns a list of PaymentIntents.</p>
    pub fn get_payment_intents(&self) -> request_model::GetPaymentIntentsRequest {
        request_model::GetPaymentIntentsRequest {
            client: &self,
            created: None,
            customer: None,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
        }
    }
    /**<p>Creates a PaymentIntent object.</p>

<p>After the PaymentIntent is created, attach a payment method and <a href="/docs/api/payment_intents/confirm">confirm</a>
to continue the payment. You can read more about the different payment flows
available via the Payment Intents API <a href="/docs/payments/payment-intents">here</a>.</p>

<p>When <code>confirm=true</code> is used during creation, it is equivalent to creating
and confirming the PaymentIntent in the same call. You may use any parameters
available in the <a href="/docs/api/payment_intents/confirm">confirm API</a> when <code>confirm=true</code>
is supplied.</p>*/
    pub fn post_payment_intents(&self) -> request_model::PostPaymentIntentsRequest {
        request_model::PostPaymentIntentsRequest {
            client: &self,
        }
    }
    /**<p>Search for PaymentIntents you’ve previously created using Stripe’s <a href="/docs/search#search-query-language">Search Query Language</a>.
Don’t use search in read-after-write flows where strict consistency is necessary. Under normal operating
conditions, data is searchable in less than a minute. Occasionally, propagation of new or updated data can be up
to an hour behind during outages. Search functionality is not available to merchants in India.</p>*/
    pub fn get_payment_intents_search(
        &self,
        query: String,
    ) -> request_model::GetPaymentIntentsSearchRequest {
        request_model::GetPaymentIntentsSearchRequest {
            client: &self,
            expand: None,
            limit: None,
            page: None,
            query,
        }
    }
    /**<p>Retrieves the details of a PaymentIntent that has previously been created. </p>

<p>Client-side retrieval using a publishable key is allowed when the <code>client_secret</code> is provided in the query string. </p>

<p>When retrieved with a publishable key, only a subset of properties will be returned. Please refer to the <a href="#payment_intent_object">payment intent</a> object reference for more details.</p>*/
    pub fn get_payment_intents_intent(
        &self,
        intent: String,
    ) -> request_model::GetPaymentIntentsIntentRequest {
        request_model::GetPaymentIntentsIntentRequest {
            client: &self,
            client_secret: None,
            expand: None,
            intent,
        }
    }
    /**<p>Updates properties on a PaymentIntent object without confirming.</p>

<p>Depending on which properties you update, you may need to confirm the
PaymentIntent again. For example, updating the <code>payment_method</code> will
always require you to confirm the PaymentIntent again. If you prefer to
update and confirm at the same time, we recommend updating properties via
the <a href="/docs/api/payment_intents/confirm">confirm API</a> instead.</p>*/
    pub fn post_payment_intents_intent(
        &self,
        intent: String,
    ) -> request_model::PostPaymentIntentsIntentRequest {
        request_model::PostPaymentIntentsIntentRequest {
            client: &self,
            intent,
        }
    }
    ///<p>Manually reconcile the remaining amount for a customer_balance PaymentIntent.</p>
    pub fn post_payment_intents_intent_apply_customer_balance(
        &self,
        intent: String,
    ) -> request_model::PostPaymentIntentsIntentApplyCustomerBalanceRequest {
        request_model::PostPaymentIntentsIntentApplyCustomerBalanceRequest {
            client: &self,
            intent,
        }
    }
    /**<p>A PaymentIntent object can be canceled when it is in one of these statuses: <code>requires_payment_method</code>, <code>requires_capture</code>, <code>requires_confirmation</code>, <code>requires_action</code>, or <code>processing</code>. </p>

<p>Once canceled, no additional charges will be made by the PaymentIntent and any operations on the PaymentIntent will fail with an error. For PaymentIntents with <code>status=’requires_capture’</code>, the remaining <code>amount_capturable</code> will automatically be refunded. </p>

<p>You cannot cancel the PaymentIntent for a Checkout Session. <a href="/docs/api/checkout/sessions/expire">Expire the Checkout Session</a> instead</p>*/
    pub fn post_payment_intents_intent_cancel(
        &self,
        intent: String,
    ) -> request_model::PostPaymentIntentsIntentCancelRequest {
        request_model::PostPaymentIntentsIntentCancelRequest {
            client: &self,
            intent,
        }
    }
    /**<p>Capture the funds of an existing uncaptured PaymentIntent when its status is <code>requires_capture</code>.</p>

<p>Uncaptured PaymentIntents will be canceled a set number of days after they are created (7 by default).</p>

<p>Learn more about <a href="/docs/payments/capture-later">separate authorization and capture</a>.</p>*/
    pub fn post_payment_intents_intent_capture(
        &self,
        intent: String,
    ) -> request_model::PostPaymentIntentsIntentCaptureRequest {
        request_model::PostPaymentIntentsIntentCaptureRequest {
            client: &self,
            intent,
        }
    }
    /**<p>Confirm that your customer intends to pay with current or provided
payment method. Upon confirmation, the PaymentIntent will attempt to initiate
a payment.</p>

<p>If the selected payment method requires additional authentication steps, the
PaymentIntent will transition to the <code>requires_action</code> status and
suggest additional actions via <code>next_action</code>. If payment fails,
the PaymentIntent will transition to the <code>requires_payment_method</code> status. If
payment succeeds, the PaymentIntent will transition to the <code>succeeded</code>
status (or <code>requires_capture</code>, if <code>capture_method</code> is set to <code>manual</code>).</p>

<p>If the <code>confirmation_method</code> is <code>automatic</code>, payment may be attempted
using our <a href="/docs/stripe-js/reference#stripe-handle-card-payment">client SDKs</a>
and the PaymentIntent’s <a href="#payment_intent_object-client_secret">client_secret</a>.
After <code>next_action</code>s are handled by the client, no additional
confirmation is required to complete the payment.</p>

<p>If the <code>confirmation_method</code> is <code>manual</code>, all payment attempts must be
initiated using a secret key.
If any actions are required for the payment, the PaymentIntent will
return to the <code>requires_confirmation</code> state
after those actions are completed. Your server needs to then
explicitly re-confirm the PaymentIntent to initiate the next payment
attempt. Read the <a href="/docs/payments/payment-intents/web-manual">expanded documentation</a>
to learn more about manual confirmation.</p>*/
    pub fn post_payment_intents_intent_confirm(
        &self,
        intent: String,
    ) -> request_model::PostPaymentIntentsIntentConfirmRequest {
        request_model::PostPaymentIntentsIntentConfirmRequest {
            client: &self,
            intent,
        }
    }
    /**<p>Perform an incremental authorization on an eligible
<a href="/docs/api/payment_intents/object">PaymentIntent</a>. To be eligible, the
PaymentIntent’s status must be <code>requires_capture</code> and
<a href="/docs/api/charges/object#charge_object-payment_method_details-card_present-incremental_authorization_supported">incremental_authorization_supported</a>
must be <code>true</code>.</p>

<p>Incremental authorizations attempt to increase the authorized amount on
your customer’s card to the new, higher <code>amount</code> provided. As with the
initial authorization, incremental authorizations may be declined. A
single PaymentIntent can call this endpoint multiple times to further
increase the authorized amount.</p>

<p>If the incremental authorization succeeds, the PaymentIntent object is
returned with the updated
<a href="/docs/api/payment_intents/object#payment_intent_object-amount">amount</a>.
If the incremental authorization fails, a
<a href="/docs/error-codes#card-declined">card_declined</a> error is returned, and no
fields on the PaymentIntent or Charge are updated. The PaymentIntent
object remains capturable for the previously authorized amount.</p>

<p>Each PaymentIntent can have a maximum of 10 incremental authorization attempts, including declines.
Once captured, a PaymentIntent can no longer be incremented.</p>

<p>Learn more about <a href="/docs/terminal/features/incremental-authorizations">incremental authorizations</a>.</p>*/
    pub fn post_payment_intents_intent_increment_authorization(
        &self,
        intent: String,
    ) -> request_model::PostPaymentIntentsIntentIncrementAuthorizationRequest {
        request_model::PostPaymentIntentsIntentIncrementAuthorizationRequest {
            client: &self,
            intent,
        }
    }
    ///<p>Verifies microdeposits on a PaymentIntent object.</p>
    pub fn post_payment_intents_intent_verify_microdeposits(
        &self,
        intent: String,
    ) -> request_model::PostPaymentIntentsIntentVerifyMicrodepositsRequest {
        request_model::PostPaymentIntentsIntentVerifyMicrodepositsRequest {
            client: &self,
            intent,
        }
    }
    ///<p>Returns a list of your payment links.</p>
    pub fn get_payment_links(&self) -> request_model::GetPaymentLinksRequest {
        request_model::GetPaymentLinksRequest {
            client: &self,
            active: None,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
        }
    }
    ///<p>Creates a payment link.</p>
    pub fn post_payment_links(&self) -> request_model::PostPaymentLinksRequest {
        request_model::PostPaymentLinksRequest {
            client: &self,
        }
    }
    ///<p>Retrieve a payment link.</p>
    pub fn get_payment_links_payment_link(
        &self,
        payment_link: String,
    ) -> request_model::GetPaymentLinksPaymentLinkRequest {
        request_model::GetPaymentLinksPaymentLinkRequest {
            client: &self,
            expand: None,
            payment_link,
        }
    }
    ///<p>Updates a payment link.</p>
    pub fn post_payment_links_payment_link(
        &self,
        payment_link: String,
    ) -> request_model::PostPaymentLinksPaymentLinkRequest {
        request_model::PostPaymentLinksPaymentLinkRequest {
            client: &self,
            payment_link,
        }
    }
    ///<p>When retrieving a payment link, there is an includable <strong>line_items</strong> property containing the first handful of those items. There is also a URL where you can retrieve the full (paginated) list of line items.</p>
    pub fn get_payment_links_payment_link_line_items(
        &self,
        payment_link: String,
    ) -> request_model::GetPaymentLinksPaymentLinkLineItemsRequest {
        request_model::GetPaymentLinksPaymentLinkLineItemsRequest {
            client: &self,
            ending_before: None,
            expand: None,
            limit: None,
            payment_link,
            starting_after: None,
        }
    }
    ///<p>Returns a list of PaymentMethods attached to the StripeAccount. For listing a customer’s payment methods, you should use <a href="/docs/api/payment_methods/customer_list">List a Customer’s PaymentMethods</a></p>
    pub fn get_payment_methods(
        &self,
        type_: String,
    ) -> request_model::GetPaymentMethodsRequest {
        request_model::GetPaymentMethodsRequest {
            client: &self,
            customer: None,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
            type_,
        }
    }
    /**<p>Creates a PaymentMethod object. Read the <a href="/docs/stripe-js/reference#stripe-create-payment-method">Stripe.js reference</a> to learn how to create PaymentMethods via Stripe.js.</p>

<p>Instead of creating a PaymentMethod directly, we recommend using the <a href="/docs/payments/accept-a-payment">PaymentIntents</a> API to accept a payment immediately or the <a href="/docs/payments/save-and-reuse">SetupIntent</a> API to collect payment method details ahead of a future payment.</p>*/
    pub fn post_payment_methods(&self) -> request_model::PostPaymentMethodsRequest {
        request_model::PostPaymentMethodsRequest {
            client: &self,
        }
    }
    ///<p>Retrieves a PaymentMethod object attached to the StripeAccount. To retrieve a payment method attached to a Customer, you should use <a href="/docs/api/payment_methods/customer">Retrieve a Customer’s PaymentMethods</a></p>
    pub fn get_payment_methods_payment_method(
        &self,
        payment_method: String,
    ) -> request_model::GetPaymentMethodsPaymentMethodRequest {
        request_model::GetPaymentMethodsPaymentMethodRequest {
            client: &self,
            expand: None,
            payment_method,
        }
    }
    ///<p>Updates a PaymentMethod object. A PaymentMethod must be attached a customer to be updated.</p>
    pub fn post_payment_methods_payment_method(
        &self,
        payment_method: String,
    ) -> request_model::PostPaymentMethodsPaymentMethodRequest {
        request_model::PostPaymentMethodsPaymentMethodRequest {
            client: &self,
            payment_method,
        }
    }
    /**<p>Attaches a PaymentMethod object to a Customer.</p>

<p>To attach a new PaymentMethod to a customer for future payments, we recommend you use a <a href="/docs/api/setup_intents">SetupIntent</a>
or a PaymentIntent with <a href="/docs/api/payment_intents/create#create_payment_intent-setup_future_usage">setup_future_usage</a>.
These approaches will perform any necessary steps to set up the PaymentMethod for future payments. Using the <code>/v1/payment_methods/:id/attach</code>
endpoint without first using a SetupIntent or PaymentIntent with <code>setup_future_usage</code> does not optimize the PaymentMethod for
future use, which makes later declines and payment friction more likely.
See <a href="/docs/payments/payment-intents#future-usage">Optimizing cards for future payments</a> for more information about setting up
future payments.</p>

<p>To use this PaymentMethod as the default for invoice or subscription payments,
set <a href="/docs/api/customers/update#update_customer-invoice_settings-default_payment_method"><code>invoice_settings.default_payment_method</code></a>,
on the Customer to the PaymentMethod’s ID.</p>*/
    pub fn post_payment_methods_payment_method_attach(
        &self,
        payment_method: String,
    ) -> request_model::PostPaymentMethodsPaymentMethodAttachRequest {
        request_model::PostPaymentMethodsPaymentMethodAttachRequest {
            client: &self,
            payment_method,
        }
    }
    ///<p>Detaches a PaymentMethod object from a Customer. After a PaymentMethod is detached, it can no longer be used for a payment or re-attached to a Customer.</p>
    pub fn post_payment_methods_payment_method_detach(
        &self,
        payment_method: String,
    ) -> request_model::PostPaymentMethodsPaymentMethodDetachRequest {
        request_model::PostPaymentMethodsPaymentMethodDetachRequest {
            client: &self,
            payment_method,
        }
    }
    ///<p>Returns a list of existing payouts sent to third-party bank accounts or that Stripe has sent you. The payouts are returned in sorted order, with the most recently created payouts appearing first.</p>
    pub fn get_payouts(&self) -> request_model::GetPayoutsRequest {
        request_model::GetPayoutsRequest {
            client: &self,
            arrival_date: None,
            created: None,
            destination: None,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
            status: None,
        }
    }
    /**<p>To send funds to your own bank account, you create a new payout object. Your <a href="#balance">Stripe balance</a> must be able to cover the payout amount, or you’ll receive an “Insufficient Funds” error.</p>

<p>If your API key is in test mode, money won’t actually be sent, though everything else will occur as if in live mode.</p>

<p>If you are creating a manual payout on a Stripe account that uses multiple payment source types, you’ll need to specify the source type balance that the payout should draw from. The <a href="#balance_object">balance object</a> details available and pending amounts by source type.</p>*/
    pub fn post_payouts(&self) -> request_model::PostPayoutsRequest {
        request_model::PostPayoutsRequest {
            client: &self,
        }
    }
    ///<p>Retrieves the details of an existing payout. Supply the unique payout ID from either a payout creation request or the payout list, and Stripe will return the corresponding payout information.</p>
    pub fn get_payouts_payout(
        &self,
        payout: String,
    ) -> request_model::GetPayoutsPayoutRequest {
        request_model::GetPayoutsPayoutRequest {
            client: &self,
            expand: None,
            payout,
        }
    }
    ///<p>Updates the specified payout by setting the values of the parameters passed. Any parameters not provided will be left unchanged. This request accepts only the metadata as arguments.</p>
    pub fn post_payouts_payout(
        &self,
        payout: String,
    ) -> request_model::PostPayoutsPayoutRequest {
        request_model::PostPayoutsPayoutRequest {
            client: &self,
            payout,
        }
    }
    ///<p>A previously created payout can be canceled if it has not yet been paid out. Funds will be refunded to your available balance. You may not cancel automatic Stripe payouts.</p>
    pub fn post_payouts_payout_cancel(
        &self,
        payout: String,
    ) -> request_model::PostPayoutsPayoutCancelRequest {
        request_model::PostPayoutsPayoutCancelRequest {
            client: &self,
            payout,
        }
    }
    /**<p>Reverses a payout by debiting the destination bank account. Only payouts for connected accounts to US bank accounts may be reversed at this time. If the payout is in the <code>pending</code> status, <code>/v1/payouts/:id/cancel</code> should be used instead.</p>

<p>By requesting a reversal via <code>/v1/payouts/:id/reverse</code>, you confirm that the authorized signatory of the selected bank account has authorized the debit on the bank account and that no other authorization is required.</p>*/
    pub fn post_payouts_payout_reverse(
        &self,
        payout: String,
    ) -> request_model::PostPayoutsPayoutReverseRequest {
        request_model::PostPayoutsPayoutReverseRequest {
            client: &self,
            payout,
        }
    }
    ///<p>Returns a list of your plans.</p>
    pub fn get_plans(&self) -> request_model::GetPlansRequest {
        request_model::GetPlansRequest {
            client: &self,
            active: None,
            created: None,
            ending_before: None,
            expand: None,
            limit: None,
            product: None,
            starting_after: None,
        }
    }
    ///<p>You can now model subscriptions more flexibly using the <a href="#prices">Prices API</a>. It replaces the Plans API and is backwards compatible to simplify your migration.</p>
    pub fn post_plans(&self) -> request_model::PostPlansRequest {
        request_model::PostPlansRequest {
            client: &self,
        }
    }
    ///<p>Retrieves the plan with the given ID.</p>
    pub fn get_plans_plan(&self, plan: String) -> request_model::GetPlansPlanRequest {
        request_model::GetPlansPlanRequest {
            client: &self,
            expand: None,
            plan,
        }
    }
    ///<p>Updates the specified plan by setting the values of the parameters passed. Any parameters not provided are left unchanged. By design, you cannot change a plan’s ID, amount, currency, or billing cycle.</p>
    pub fn post_plans_plan(&self, plan: String) -> request_model::PostPlansPlanRequest {
        request_model::PostPlansPlanRequest {
            client: &self,
            plan,
        }
    }
    ///<p>Deleting plans means new subscribers can’t be added. Existing subscribers aren’t affected.</p>
    pub fn delete_plans_plan(
        &self,
        plan: String,
    ) -> request_model::DeletePlansPlanRequest {
        request_model::DeletePlansPlanRequest {
            client: &self,
            plan,
        }
    }
    ///<p>Returns a list of your prices.</p>
    pub fn get_prices(&self) -> request_model::GetPricesRequest {
        request_model::GetPricesRequest {
            client: &self,
            active: None,
            created: None,
            currency: None,
            ending_before: None,
            expand: None,
            limit: None,
            lookup_keys: None,
            product: None,
            recurring: None,
            starting_after: None,
            type_: None,
        }
    }
    ///<p>Creates a new price for an existing product. The price can be recurring or one-time.</p>
    pub fn post_prices(&self) -> request_model::PostPricesRequest {
        request_model::PostPricesRequest {
            client: &self,
        }
    }
    /**<p>Search for prices you’ve previously created using Stripe’s <a href="/docs/search#search-query-language">Search Query Language</a>.
Don’t use search in read-after-write flows where strict consistency is necessary. Under normal operating
conditions, data is searchable in less than a minute. Occasionally, propagation of new or updated data can be up
to an hour behind during outages. Search functionality is not available to merchants in India.</p>*/
    pub fn get_prices_search(
        &self,
        query: String,
    ) -> request_model::GetPricesSearchRequest {
        request_model::GetPricesSearchRequest {
            client: &self,
            expand: None,
            limit: None,
            page: None,
            query,
        }
    }
    ///<p>Retrieves the price with the given ID.</p>
    pub fn get_prices_price(
        &self,
        price: String,
    ) -> request_model::GetPricesPriceRequest {
        request_model::GetPricesPriceRequest {
            client: &self,
            expand: None,
            price,
        }
    }
    ///<p>Updates the specified price by setting the values of the parameters passed. Any parameters not provided are left unchanged.</p>
    pub fn post_prices_price(
        &self,
        price: String,
    ) -> request_model::PostPricesPriceRequest {
        request_model::PostPricesPriceRequest {
            client: &self,
            price,
        }
    }
    ///<p>Returns a list of your products. The products are returned sorted by creation date, with the most recently created products appearing first.</p>
    pub fn get_products(&self) -> request_model::GetProductsRequest {
        request_model::GetProductsRequest {
            client: &self,
            active: None,
            created: None,
            ending_before: None,
            expand: None,
            ids: None,
            limit: None,
            shippable: None,
            starting_after: None,
            url: None,
        }
    }
    ///<p>Creates a new product object.</p>
    pub fn post_products(&self) -> request_model::PostProductsRequest {
        request_model::PostProductsRequest {
            client: &self,
        }
    }
    /**<p>Search for products you’ve previously created using Stripe’s <a href="/docs/search#search-query-language">Search Query Language</a>.
Don’t use search in read-after-write flows where strict consistency is necessary. Under normal operating
conditions, data is searchable in less than a minute. Occasionally, propagation of new or updated data can be up
to an hour behind during outages. Search functionality is not available to merchants in India.</p>*/
    pub fn get_products_search(
        &self,
        query: String,
    ) -> request_model::GetProductsSearchRequest {
        request_model::GetProductsSearchRequest {
            client: &self,
            expand: None,
            limit: None,
            page: None,
            query,
        }
    }
    ///<p>Retrieves the details of an existing product. Supply the unique product ID from either a product creation request or the product list, and Stripe will return the corresponding product information.</p>
    pub fn get_products_id(&self, id: String) -> request_model::GetProductsIdRequest {
        request_model::GetProductsIdRequest {
            client: &self,
            expand: None,
            id,
        }
    }
    ///<p>Updates the specific product by setting the values of the parameters passed. Any parameters not provided will be left unchanged.</p>
    pub fn post_products_id(&self, id: String) -> request_model::PostProductsIdRequest {
        request_model::PostProductsIdRequest {
            client: &self,
            id,
        }
    }
    ///<p>Delete a product. Deleting a product is only possible if it has no prices associated with it. Additionally, deleting a product with <code>type=good</code> is only possible if it has no SKUs associated with it.</p>
    pub fn delete_products_id(
        &self,
        id: String,
    ) -> request_model::DeleteProductsIdRequest {
        request_model::DeleteProductsIdRequest {
            client: &self,
            id,
        }
    }
    ///<p>Returns a list of your promotion codes.</p>
    pub fn get_promotion_codes(&self) -> request_model::GetPromotionCodesRequest {
        request_model::GetPromotionCodesRequest {
            client: &self,
            active: None,
            code: None,
            coupon: None,
            created: None,
            customer: None,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
        }
    }
    ///<p>A promotion code points to a coupon. You can optionally restrict the code to a specific customer, redemption limit, and expiration date.</p>
    pub fn post_promotion_codes(&self) -> request_model::PostPromotionCodesRequest {
        request_model::PostPromotionCodesRequest {
            client: &self,
        }
    }
    ///<p>Retrieves the promotion code with the given ID. In order to retrieve a promotion code by the customer-facing <code>code</code> use <a href="/docs/api/promotion_codes/list">list</a> with the desired <code>code</code>.</p>
    pub fn get_promotion_codes_promotion_code(
        &self,
        promotion_code: String,
    ) -> request_model::GetPromotionCodesPromotionCodeRequest {
        request_model::GetPromotionCodesPromotionCodeRequest {
            client: &self,
            expand: None,
            promotion_code,
        }
    }
    ///<p>Updates the specified promotion code by setting the values of the parameters passed. Most fields are, by design, not editable.</p>
    pub fn post_promotion_codes_promotion_code(
        &self,
        promotion_code: String,
    ) -> request_model::PostPromotionCodesPromotionCodeRequest {
        request_model::PostPromotionCodesPromotionCodeRequest {
            client: &self,
            promotion_code,
        }
    }
    ///<p>Returns a list of your quotes.</p>
    pub fn get_quotes(&self) -> request_model::GetQuotesRequest {
        request_model::GetQuotesRequest {
            client: &self,
            customer: None,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
            status: None,
            test_clock: None,
        }
    }
    ///<p>A quote models prices and services for a customer. Default options for <code>header</code>, <code>description</code>, <code>footer</code>, and <code>expires_at</code> can be set in the dashboard via the <a href="https://dashboard.stripe.com/settings/billing/quote">quote template</a>.</p>
    pub fn post_quotes(&self) -> request_model::PostQuotesRequest {
        request_model::PostQuotesRequest {
            client: &self,
        }
    }
    ///<p>Retrieves the quote with the given ID.</p>
    pub fn get_quotes_quote(
        &self,
        quote: String,
    ) -> request_model::GetQuotesQuoteRequest {
        request_model::GetQuotesQuoteRequest {
            client: &self,
            expand: None,
            quote,
        }
    }
    ///<p>A quote models prices and services for a customer.</p>
    pub fn post_quotes_quote(
        &self,
        quote: String,
    ) -> request_model::PostQuotesQuoteRequest {
        request_model::PostQuotesQuoteRequest {
            client: &self,
            quote,
        }
    }
    ///<p>Accepts the specified quote.</p>
    pub fn post_quotes_quote_accept(
        &self,
        quote: String,
    ) -> request_model::PostQuotesQuoteAcceptRequest {
        request_model::PostQuotesQuoteAcceptRequest {
            client: &self,
            quote,
        }
    }
    ///<p>Cancels the quote.</p>
    pub fn post_quotes_quote_cancel(
        &self,
        quote: String,
    ) -> request_model::PostQuotesQuoteCancelRequest {
        request_model::PostQuotesQuoteCancelRequest {
            client: &self,
            quote,
        }
    }
    ///<p>When retrieving a quote, there is an includable <a href="https://stripe.com/docs/api/quotes/object#quote_object-computed-upfront-line_items"><strong>computed.upfront.line_items</strong></a> property containing the first handful of those items. There is also a URL where you can retrieve the full (paginated) list of upfront line items.</p>
    pub fn get_quotes_quote_computed_upfront_line_items(
        &self,
        quote: String,
    ) -> request_model::GetQuotesQuoteComputedUpfrontLineItemsRequest {
        request_model::GetQuotesQuoteComputedUpfrontLineItemsRequest {
            client: &self,
            ending_before: None,
            expand: None,
            limit: None,
            quote,
            starting_after: None,
        }
    }
    ///<p>Finalizes the quote.</p>
    pub fn post_quotes_quote_finalize(
        &self,
        quote: String,
    ) -> request_model::PostQuotesQuoteFinalizeRequest {
        request_model::PostQuotesQuoteFinalizeRequest {
            client: &self,
            quote,
        }
    }
    ///<p>When retrieving a quote, there is an includable <strong>line_items</strong> property containing the first handful of those items. There is also a URL where you can retrieve the full (paginated) list of line items.</p>
    pub fn get_quotes_quote_line_items(
        &self,
        quote: String,
    ) -> request_model::GetQuotesQuoteLineItemsRequest {
        request_model::GetQuotesQuoteLineItemsRequest {
            client: &self,
            ending_before: None,
            expand: None,
            limit: None,
            quote,
            starting_after: None,
        }
    }
    ///<p>Returns a list of early fraud warnings.</p>
    pub fn get_radar_early_fraud_warnings(
        &self,
    ) -> request_model::GetRadarEarlyFraudWarningsRequest {
        request_model::GetRadarEarlyFraudWarningsRequest {
            client: &self,
            charge: None,
            ending_before: None,
            expand: None,
            limit: None,
            payment_intent: None,
            starting_after: None,
        }
    }
    /**<p>Retrieves the details of an early fraud warning that has previously been created. </p>

<p>Please refer to the <a href="#early_fraud_warning_object">early fraud warning</a> object reference for more details.</p>*/
    pub fn get_radar_early_fraud_warnings_early_fraud_warning(
        &self,
        early_fraud_warning: String,
    ) -> request_model::GetRadarEarlyFraudWarningsEarlyFraudWarningRequest {
        request_model::GetRadarEarlyFraudWarningsEarlyFraudWarningRequest {
            client: &self,
            early_fraud_warning,
            expand: None,
        }
    }
    ///<p>Returns a list of <code>ValueListItem</code> objects. The objects are sorted in descending order by creation date, with the most recently created object appearing first.</p>
    pub fn get_radar_value_list_items(
        &self,
        value_list: String,
    ) -> request_model::GetRadarValueListItemsRequest {
        request_model::GetRadarValueListItemsRequest {
            client: &self,
            created: None,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
            value: None,
            value_list,
        }
    }
    ///<p>Creates a new <code>ValueListItem</code> object, which is added to the specified parent value list.</p>
    pub fn post_radar_value_list_items(
        &self,
    ) -> request_model::PostRadarValueListItemsRequest {
        request_model::PostRadarValueListItemsRequest {
            client: &self,
        }
    }
    ///<p>Retrieves a <code>ValueListItem</code> object.</p>
    pub fn get_radar_value_list_items_item(
        &self,
        item: String,
    ) -> request_model::GetRadarValueListItemsItemRequest {
        request_model::GetRadarValueListItemsItemRequest {
            client: &self,
            expand: None,
            item,
        }
    }
    ///<p>Deletes a <code>ValueListItem</code> object, removing it from its parent value list.</p>
    pub fn delete_radar_value_list_items_item(
        &self,
        item: String,
    ) -> request_model::DeleteRadarValueListItemsItemRequest {
        request_model::DeleteRadarValueListItemsItemRequest {
            client: &self,
            item,
        }
    }
    ///<p>Returns a list of <code>ValueList</code> objects. The objects are sorted in descending order by creation date, with the most recently created object appearing first.</p>
    pub fn get_radar_value_lists(&self) -> request_model::GetRadarValueListsRequest {
        request_model::GetRadarValueListsRequest {
            client: &self,
            alias: None,
            contains: None,
            created: None,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
        }
    }
    ///<p>Creates a new <code>ValueList</code> object, which can then be referenced in rules.</p>
    pub fn post_radar_value_lists(&self) -> request_model::PostRadarValueListsRequest {
        request_model::PostRadarValueListsRequest {
            client: &self,
        }
    }
    ///<p>Retrieves a <code>ValueList</code> object.</p>
    pub fn get_radar_value_lists_value_list(
        &self,
        value_list: String,
    ) -> request_model::GetRadarValueListsValueListRequest {
        request_model::GetRadarValueListsValueListRequest {
            client: &self,
            expand: None,
            value_list,
        }
    }
    ///<p>Updates a <code>ValueList</code> object by setting the values of the parameters passed. Any parameters not provided will be left unchanged. Note that <code>item_type</code> is immutable.</p>
    pub fn post_radar_value_lists_value_list(
        &self,
        value_list: String,
    ) -> request_model::PostRadarValueListsValueListRequest {
        request_model::PostRadarValueListsValueListRequest {
            client: &self,
            value_list,
        }
    }
    ///<p>Deletes a <code>ValueList</code> object, also deleting any items contained within the value list. To be deleted, a value list must not be referenced in any rules.</p>
    pub fn delete_radar_value_lists_value_list(
        &self,
        value_list: String,
    ) -> request_model::DeleteRadarValueListsValueListRequest {
        request_model::DeleteRadarValueListsValueListRequest {
            client: &self,
            value_list,
        }
    }
    ///<p>Returns a list of all refunds you’ve previously created. The refunds are returned in sorted order, with the most recent refunds appearing first. For convenience, the 10 most recent refunds are always available by default on the charge object.</p>
    pub fn get_refunds(&self) -> request_model::GetRefundsRequest {
        request_model::GetRefundsRequest {
            client: &self,
            charge: None,
            created: None,
            ending_before: None,
            expand: None,
            limit: None,
            payment_intent: None,
            starting_after: None,
        }
    }
    ///<p>Create a refund.</p>
    pub fn post_refunds(&self) -> request_model::PostRefundsRequest {
        request_model::PostRefundsRequest {
            client: &self,
        }
    }
    ///<p>Retrieves the details of an existing refund.</p>
    pub fn get_refunds_refund(
        &self,
        refund: String,
    ) -> request_model::GetRefundsRefundRequest {
        request_model::GetRefundsRefundRequest {
            client: &self,
            expand: None,
            refund,
        }
    }
    /**<p>Updates the specified refund by setting the values of the parameters passed. Any parameters not provided will be left unchanged.</p>

<p>This request only accepts <code>metadata</code> as an argument.</p>*/
    pub fn post_refunds_refund(
        &self,
        refund: String,
    ) -> request_model::PostRefundsRefundRequest {
        request_model::PostRefundsRefundRequest {
            client: &self,
            refund,
        }
    }
    /**<p>Cancels a refund with a status of <code>requires_action</code>.</p>

<p>Refunds in other states cannot be canceled, and only refunds for payment methods that require customer action will enter the <code>requires_action</code> state.</p>*/
    pub fn post_refunds_refund_cancel(
        &self,
        refund: String,
    ) -> request_model::PostRefundsRefundCancelRequest {
        request_model::PostRefundsRefundCancelRequest {
            client: &self,
            refund,
        }
    }
    ///<p>Returns a list of Report Runs, with the most recent appearing first.</p>
    pub fn get_reporting_report_runs(
        &self,
    ) -> request_model::GetReportingReportRunsRequest {
        request_model::GetReportingReportRunsRequest {
            client: &self,
            created: None,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
        }
    }
    ///<p>Creates a new object and begin running the report. (Certain report types require a <a href="https://stripe.com/docs/keys#test-live-modes">live-mode API key</a>.)</p>
    pub fn post_reporting_report_runs(
        &self,
    ) -> request_model::PostReportingReportRunsRequest {
        request_model::PostReportingReportRunsRequest {
            client: &self,
        }
    }
    ///<p>Retrieves the details of an existing Report Run.</p>
    pub fn get_reporting_report_runs_report_run(
        &self,
        report_run: String,
    ) -> request_model::GetReportingReportRunsReportRunRequest {
        request_model::GetReportingReportRunsReportRunRequest {
            client: &self,
            expand: None,
            report_run,
        }
    }
    ///<p>Returns a full list of Report Types.</p>
    pub fn get_reporting_report_types(
        &self,
    ) -> request_model::GetReportingReportTypesRequest {
        request_model::GetReportingReportTypesRequest {
            client: &self,
            expand: None,
        }
    }
    ///<p>Retrieves the details of a Report Type. (Certain report types require a <a href="https://stripe.com/docs/keys#test-live-modes">live-mode API key</a>.)</p>
    pub fn get_reporting_report_types_report_type(
        &self,
        report_type: String,
    ) -> request_model::GetReportingReportTypesReportTypeRequest {
        request_model::GetReportingReportTypesReportTypeRequest {
            client: &self,
            expand: None,
            report_type,
        }
    }
    ///<p>Returns a list of <code>Review</code> objects that have <code>open</code> set to <code>true</code>. The objects are sorted in descending order by creation date, with the most recently created object appearing first.</p>
    pub fn get_reviews(&self) -> request_model::GetReviewsRequest {
        request_model::GetReviewsRequest {
            client: &self,
            created: None,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
        }
    }
    ///<p>Retrieves a <code>Review</code> object.</p>
    pub fn get_reviews_review(
        &self,
        review: String,
    ) -> request_model::GetReviewsReviewRequest {
        request_model::GetReviewsReviewRequest {
            client: &self,
            expand: None,
            review,
        }
    }
    ///<p>Approves a <code>Review</code> object, closing it and removing it from the list of reviews.</p>
    pub fn post_reviews_review_approve(
        &self,
        review: String,
    ) -> request_model::PostReviewsReviewApproveRequest {
        request_model::PostReviewsReviewApproveRequest {
            client: &self,
            review,
        }
    }
    ///<p>Returns a list of SetupAttempts associated with a provided SetupIntent.</p>
    pub fn get_setup_attempts(
        &self,
        setup_intent: String,
    ) -> request_model::GetSetupAttemptsRequest {
        request_model::GetSetupAttemptsRequest {
            client: &self,
            created: None,
            ending_before: None,
            expand: None,
            limit: None,
            setup_intent,
            starting_after: None,
        }
    }
    ///<p>Returns a list of SetupIntents.</p>
    pub fn get_setup_intents(&self) -> request_model::GetSetupIntentsRequest {
        request_model::GetSetupIntentsRequest {
            client: &self,
            attach_to_self: None,
            created: None,
            customer: None,
            ending_before: None,
            expand: None,
            limit: None,
            payment_method: None,
            starting_after: None,
        }
    }
    /**<p>Creates a SetupIntent object.</p>

<p>After the SetupIntent is created, attach a payment method and <a href="/docs/api/setup_intents/confirm">confirm</a>
to collect any required permissions to charge the payment method later.</p>*/
    pub fn post_setup_intents(&self) -> request_model::PostSetupIntentsRequest {
        request_model::PostSetupIntentsRequest {
            client: &self,
        }
    }
    /**<p>Retrieves the details of a SetupIntent that has previously been created. </p>

<p>Client-side retrieval using a publishable key is allowed when the <code>client_secret</code> is provided in the query string. </p>

<p>When retrieved with a publishable key, only a subset of properties will be returned. Please refer to the <a href="#setup_intent_object">SetupIntent</a> object reference for more details.</p>*/
    pub fn get_setup_intents_intent(
        &self,
        intent: String,
    ) -> request_model::GetSetupIntentsIntentRequest {
        request_model::GetSetupIntentsIntentRequest {
            client: &self,
            client_secret: None,
            expand: None,
            intent,
        }
    }
    ///<p>Updates a SetupIntent object.</p>
    pub fn post_setup_intents_intent(
        &self,
        intent: String,
    ) -> request_model::PostSetupIntentsIntentRequest {
        request_model::PostSetupIntentsIntentRequest {
            client: &self,
            intent,
        }
    }
    /**<p>A SetupIntent object can be canceled when it is in one of these statuses: <code>requires_payment_method</code>, <code>requires_confirmation</code>, or <code>requires_action</code>. </p>

<p>Once canceled, setup is abandoned and any operations on the SetupIntent will fail with an error.</p>*/
    pub fn post_setup_intents_intent_cancel(
        &self,
        intent: String,
    ) -> request_model::PostSetupIntentsIntentCancelRequest {
        request_model::PostSetupIntentsIntentCancelRequest {
            client: &self,
            intent,
        }
    }
    /**<p>Confirm that your customer intends to set up the current or
provided payment method. For example, you would confirm a SetupIntent
when a customer hits the “Save” button on a payment method management
page on your website.</p>

<p>If the selected payment method does not require any additional
steps from the customer, the SetupIntent will transition to the
<code>succeeded</code> status.</p>

<p>Otherwise, it will transition to the <code>requires_action</code> status and
suggest additional actions via <code>next_action</code>. If setup fails,
the SetupIntent will transition to the
<code>requires_payment_method</code> status.</p>*/
    pub fn post_setup_intents_intent_confirm(
        &self,
        intent: String,
    ) -> request_model::PostSetupIntentsIntentConfirmRequest {
        request_model::PostSetupIntentsIntentConfirmRequest {
            client: &self,
            intent,
        }
    }
    ///<p>Verifies microdeposits on a SetupIntent object.</p>
    pub fn post_setup_intents_intent_verify_microdeposits(
        &self,
        intent: String,
    ) -> request_model::PostSetupIntentsIntentVerifyMicrodepositsRequest {
        request_model::PostSetupIntentsIntentVerifyMicrodepositsRequest {
            client: &self,
            intent,
        }
    }
    ///<p>Returns a list of your shipping rates.</p>
    pub fn get_shipping_rates(&self) -> request_model::GetShippingRatesRequest {
        request_model::GetShippingRatesRequest {
            client: &self,
            active: None,
            created: None,
            currency: None,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
        }
    }
    ///<p>Creates a new shipping rate object.</p>
    pub fn post_shipping_rates(&self) -> request_model::PostShippingRatesRequest {
        request_model::PostShippingRatesRequest {
            client: &self,
        }
    }
    ///<p>Returns the shipping rate object with the given ID.</p>
    pub fn get_shipping_rates_shipping_rate_token(
        &self,
        shipping_rate_token: String,
    ) -> request_model::GetShippingRatesShippingRateTokenRequest {
        request_model::GetShippingRatesShippingRateTokenRequest {
            client: &self,
            expand: None,
            shipping_rate_token,
        }
    }
    ///<p>Updates an existing shipping rate object.</p>
    pub fn post_shipping_rates_shipping_rate_token(
        &self,
        shipping_rate_token: String,
    ) -> request_model::PostShippingRatesShippingRateTokenRequest {
        request_model::PostShippingRatesShippingRateTokenRequest {
            client: &self,
            shipping_rate_token,
        }
    }
    ///<p>Returns a list of scheduled query runs.</p>
    pub fn get_sigma_scheduled_query_runs(
        &self,
    ) -> request_model::GetSigmaScheduledQueryRunsRequest {
        request_model::GetSigmaScheduledQueryRunsRequest {
            client: &self,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
        }
    }
    ///<p>Retrieves the details of an scheduled query run.</p>
    pub fn get_sigma_scheduled_query_runs_scheduled_query_run(
        &self,
        scheduled_query_run: String,
    ) -> request_model::GetSigmaScheduledQueryRunsScheduledQueryRunRequest {
        request_model::GetSigmaScheduledQueryRunsScheduledQueryRunRequest {
            client: &self,
            expand: None,
            scheduled_query_run,
        }
    }
    ///<p>Returns a list of your SKUs. The SKUs are returned sorted by creation date, with the most recently created SKUs appearing first.</p>
    pub fn get_skus(&self) -> request_model::GetSkusRequest {
        request_model::GetSkusRequest {
            client: &self,
            active: None,
            attributes: None,
            ending_before: None,
            expand: None,
            ids: None,
            in_stock: None,
            limit: None,
            product: None,
            starting_after: None,
        }
    }
    ///<p>Creates a new SKU associated with a product.</p>
    pub fn post_skus(&self) -> request_model::PostSkusRequest {
        request_model::PostSkusRequest {
            client: &self,
        }
    }
    ///<p>Retrieves the details of an existing SKU. Supply the unique SKU identifier from either a SKU creation request or from the product, and Stripe will return the corresponding SKU information.</p>
    pub fn get_skus_id(&self, id: String) -> request_model::GetSkusIdRequest {
        request_model::GetSkusIdRequest {
            client: &self,
            expand: None,
            id,
        }
    }
    /**<p>Updates the specific SKU by setting the values of the parameters passed. Any parameters not provided will be left unchanged.</p>

<p>Note that a SKU’s <code>attributes</code> are not editable. Instead, you would need to deactivate the existing SKU and create a new one with the new attribute values.</p>*/
    pub fn post_skus_id(&self, id: String) -> request_model::PostSkusIdRequest {
        request_model::PostSkusIdRequest {
            client: &self,
            id,
        }
    }
    ///<p>Delete a SKU. Deleting a SKU is only possible until it has been used in an order.</p>
    pub fn delete_skus_id(&self, id: String) -> request_model::DeleteSkusIdRequest {
        request_model::DeleteSkusIdRequest {
            client: &self,
            id,
        }
    }
    ///<p>Creates a new source object.</p>
    pub fn post_sources(&self) -> request_model::PostSourcesRequest {
        request_model::PostSourcesRequest {
            client: &self,
        }
    }
    ///<p>Retrieves an existing source object. Supply the unique source ID from a source creation request and Stripe will return the corresponding up-to-date source object information.</p>
    pub fn get_sources_source(
        &self,
        source: String,
    ) -> request_model::GetSourcesSourceRequest {
        request_model::GetSourcesSourceRequest {
            client: &self,
            client_secret: None,
            expand: None,
            source,
        }
    }
    /**<p>Updates the specified source by setting the values of the parameters passed. Any parameters not provided will be left unchanged.</p>

<p>This request accepts the <code>metadata</code> and <code>owner</code> as arguments. It is also possible to update type specific information for selected payment methods. Please refer to our <a href="/docs/sources">payment method guides</a> for more detail.</p>*/
    pub fn post_sources_source(
        &self,
        source: String,
    ) -> request_model::PostSourcesSourceRequest {
        request_model::PostSourcesSourceRequest {
            client: &self,
            source,
        }
    }
    ///<p>Retrieves a new Source MandateNotification.</p>
    pub fn get_sources_source_mandate_notifications_mandate_notification(
        &self,
        mandate_notification: String,
        source: String,
    ) -> request_model::GetSourcesSourceMandateNotificationsMandateNotificationRequest {
        request_model::GetSourcesSourceMandateNotificationsMandateNotificationRequest {
            client: &self,
            expand: None,
            mandate_notification,
            source,
        }
    }
    ///<p>List source transactions for a given source.</p>
    pub fn get_sources_source_source_transactions(
        &self,
        source: String,
    ) -> request_model::GetSourcesSourceSourceTransactionsRequest {
        request_model::GetSourcesSourceSourceTransactionsRequest {
            client: &self,
            ending_before: None,
            expand: None,
            limit: None,
            source,
            starting_after: None,
        }
    }
    ///<p>Retrieve an existing source transaction object. Supply the unique source ID from a source creation request and the source transaction ID and Stripe will return the corresponding up-to-date source object information.</p>
    pub fn get_sources_source_source_transactions_source_transaction(
        &self,
        source: String,
        source_transaction: String,
    ) -> request_model::GetSourcesSourceSourceTransactionsSourceTransactionRequest {
        request_model::GetSourcesSourceSourceTransactionsSourceTransactionRequest {
            client: &self,
            expand: None,
            source,
            source_transaction,
        }
    }
    ///<p>Verify a given source.</p>
    pub fn post_sources_source_verify(
        &self,
        source: String,
    ) -> request_model::PostSourcesSourceVerifyRequest {
        request_model::PostSourcesSourceVerifyRequest {
            client: &self,
            source,
        }
    }
    ///<p>Returns a list of your subscription items for a given subscription.</p>
    pub fn get_subscription_items(
        &self,
        subscription: String,
    ) -> request_model::GetSubscriptionItemsRequest {
        request_model::GetSubscriptionItemsRequest {
            client: &self,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
            subscription,
        }
    }
    ///<p>Adds a new item to an existing subscription. No existing items will be changed or replaced.</p>
    pub fn post_subscription_items(
        &self,
    ) -> request_model::PostSubscriptionItemsRequest {
        request_model::PostSubscriptionItemsRequest {
            client: &self,
        }
    }
    ///<p>Retrieves the subscription item with the given ID.</p>
    pub fn get_subscription_items_item(
        &self,
        item: String,
    ) -> request_model::GetSubscriptionItemsItemRequest {
        request_model::GetSubscriptionItemsItemRequest {
            client: &self,
            expand: None,
            item,
        }
    }
    ///<p>Updates the plan or quantity of an item on a current subscription.</p>
    pub fn post_subscription_items_item(
        &self,
        item: String,
    ) -> request_model::PostSubscriptionItemsItemRequest {
        request_model::PostSubscriptionItemsItemRequest {
            client: &self,
            item,
        }
    }
    ///<p>Deletes an item from the subscription. Removing a subscription item from a subscription will not cancel the subscription.</p>
    pub fn delete_subscription_items_item(
        &self,
        item: String,
    ) -> request_model::DeleteSubscriptionItemsItemRequest {
        request_model::DeleteSubscriptionItemsItemRequest {
            client: &self,
            item,
        }
    }
    /**<p>For the specified subscription item, returns a list of summary objects. Each object in the list provides usage information that’s been summarized from multiple usage records and over a subscription billing period (e.g., 15 usage records in the month of September).</p>

<p>The list is sorted in reverse-chronological order (newest first). The first list item represents the most current usage period that hasn’t ended yet. Since new usage records can still be added, the returned summary information for the subscription item’s ID should be seen as unstable until the subscription billing period ends.</p>*/
    pub fn get_subscription_items_subscription_item_usage_record_summaries(
        &self,
        subscription_item: String,
    ) -> request_model::GetSubscriptionItemsSubscriptionItemUsageRecordSummariesRequest {
        request_model::GetSubscriptionItemsSubscriptionItemUsageRecordSummariesRequest {
            client: &self,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
            subscription_item,
        }
    }
    /**<p>Creates a usage record for a specified subscription item and date, and fills it with a quantity.</p>

<p>Usage records provide <code>quantity</code> information that Stripe uses to track how much a customer is using your service. With usage information and the pricing model set up by the <a href="https://stripe.com/docs/billing/subscriptions/metered-billing">metered billing</a> plan, Stripe helps you send accurate invoices to your customers.</p>

<p>The default calculation for usage is to add up all the <code>quantity</code> values of the usage records within a billing period. You can change this default behavior with the billing plan’s <code>aggregate_usage</code> <a href="/docs/api/plans/create#create_plan-aggregate_usage">parameter</a>. When there is more than one usage record with the same timestamp, Stripe adds the <code>quantity</code> values together. In most cases, this is the desired resolution, however, you can change this behavior with the <code>action</code> parameter.</p>

<p>The default pricing model for metered billing is <a href="/docs/api/plans/object#plan_object-billing_scheme">per-unit pricing</a>. For finer granularity, you can configure metered billing to have a <a href="https://stripe.com/docs/billing/subscriptions/tiers">tiered pricing</a> model.</p>*/
    pub fn post_subscription_items_subscription_item_usage_records(
        &self,
        subscription_item: String,
    ) -> request_model::PostSubscriptionItemsSubscriptionItemUsageRecordsRequest {
        request_model::PostSubscriptionItemsSubscriptionItemUsageRecordsRequest {
            client: &self,
            subscription_item,
        }
    }
    ///<p>Retrieves the list of your subscription schedules.</p>
    pub fn get_subscription_schedules(
        &self,
    ) -> request_model::GetSubscriptionSchedulesRequest {
        request_model::GetSubscriptionSchedulesRequest {
            client: &self,
            canceled_at: None,
            completed_at: None,
            created: None,
            customer: None,
            ending_before: None,
            expand: None,
            limit: None,
            released_at: None,
            scheduled: None,
            starting_after: None,
        }
    }
    ///<p>Creates a new subscription schedule object. Each customer can have up to 500 active or scheduled subscriptions.</p>
    pub fn post_subscription_schedules(
        &self,
    ) -> request_model::PostSubscriptionSchedulesRequest {
        request_model::PostSubscriptionSchedulesRequest {
            client: &self,
        }
    }
    ///<p>Retrieves the details of an existing subscription schedule. You only need to supply the unique subscription schedule identifier that was returned upon subscription schedule creation.</p>
    pub fn get_subscription_schedules_schedule(
        &self,
        schedule: String,
    ) -> request_model::GetSubscriptionSchedulesScheduleRequest {
        request_model::GetSubscriptionSchedulesScheduleRequest {
            client: &self,
            expand: None,
            schedule,
        }
    }
    ///<p>Updates an existing subscription schedule.</p>
    pub fn post_subscription_schedules_schedule(
        &self,
        schedule: String,
    ) -> request_model::PostSubscriptionSchedulesScheduleRequest {
        request_model::PostSubscriptionSchedulesScheduleRequest {
            client: &self,
            schedule,
        }
    }
    ///<p>Cancels a subscription schedule and its associated subscription immediately (if the subscription schedule has an active subscription). A subscription schedule can only be canceled if its status is <code>not_started</code> or <code>active</code>.</p>
    pub fn post_subscription_schedules_schedule_cancel(
        &self,
        schedule: String,
    ) -> request_model::PostSubscriptionSchedulesScheduleCancelRequest {
        request_model::PostSubscriptionSchedulesScheduleCancelRequest {
            client: &self,
            schedule,
        }
    }
    ///<p>Releases the subscription schedule immediately, which will stop scheduling of its phases, but leave any existing subscription in place. A schedule can only be released if its status is <code>not_started</code> or <code>active</code>. If the subscription schedule is currently associated with a subscription, releasing it will remove its <code>subscription</code> property and set the subscription’s ID to the <code>released_subscription</code> property.</p>
    pub fn post_subscription_schedules_schedule_release(
        &self,
        schedule: String,
    ) -> request_model::PostSubscriptionSchedulesScheduleReleaseRequest {
        request_model::PostSubscriptionSchedulesScheduleReleaseRequest {
            client: &self,
            schedule,
        }
    }
    ///<p>By default, returns a list of subscriptions that have not been canceled. In order to list canceled subscriptions, specify <code>status=canceled</code>.</p>
    pub fn get_subscriptions(&self) -> request_model::GetSubscriptionsRequest {
        request_model::GetSubscriptionsRequest {
            client: &self,
            collection_method: None,
            created: None,
            current_period_end: None,
            current_period_start: None,
            customer: None,
            ending_before: None,
            expand: None,
            limit: None,
            price: None,
            starting_after: None,
            status: None,
            test_clock: None,
        }
    }
    /**<p>Creates a new subscription on an existing customer. Each customer can have up to 500 active or scheduled subscriptions.</p>

<p>When you create a subscription with <code>collection_method=charge_automatically</code>, the first invoice is finalized as part of the request.
The <code>payment_behavior</code> parameter determines the exact behavior of the initial payment.</p>

<p>To start subscriptions where the first invoice always begins in a <code>draft</code> status, use <a href="/docs/billing/subscriptions/subscription-schedules#managing">subscription schedules</a> instead.
Schedules provide the flexibility to model more complex billing configurations that change over time.</p>*/
    pub fn post_subscriptions(&self) -> request_model::PostSubscriptionsRequest {
        request_model::PostSubscriptionsRequest {
            client: &self,
        }
    }
    /**<p>Search for subscriptions you’ve previously created using Stripe’s <a href="/docs/search#search-query-language">Search Query Language</a>.
Don’t use search in read-after-write flows where strict consistency is necessary. Under normal operating
conditions, data is searchable in less than a minute. Occasionally, propagation of new or updated data can be up
to an hour behind during outages. Search functionality is not available to merchants in India.</p>*/
    pub fn get_subscriptions_search(
        &self,
        query: String,
    ) -> request_model::GetSubscriptionsSearchRequest {
        request_model::GetSubscriptionsSearchRequest {
            client: &self,
            expand: None,
            limit: None,
            page: None,
            query,
        }
    }
    ///<p>Retrieves the subscription with the given ID.</p>
    pub fn get_subscriptions_subscription_exposed_id(
        &self,
        subscription_exposed_id: String,
    ) -> request_model::GetSubscriptionsSubscriptionExposedIdRequest {
        request_model::GetSubscriptionsSubscriptionExposedIdRequest {
            client: &self,
            expand: None,
            subscription_exposed_id,
        }
    }
    ///<p>Updates an existing subscription on a customer to match the specified parameters. When changing plans or quantities, we will optionally prorate the price we charge next month to make up for any price changes. To preview how the proration will be calculated, use the <a href="#upcoming_invoice">upcoming invoice</a> endpoint.</p>
    pub fn post_subscriptions_subscription_exposed_id(
        &self,
        subscription_exposed_id: String,
    ) -> request_model::PostSubscriptionsSubscriptionExposedIdRequest {
        request_model::PostSubscriptionsSubscriptionExposedIdRequest {
            client: &self,
            subscription_exposed_id,
        }
    }
    /**<p>Cancels a customer’s subscription immediately. The customer will not be charged again for the subscription.</p>

<p>Note, however, that any pending invoice items that you’ve created will still be charged for at the end of the period, unless manually <a href="#delete_invoiceitem">deleted</a>. If you’ve set the subscription to cancel at the end of the period, any pending prorations will also be left in place and collected at the end of the period. But if the subscription is set to cancel immediately, pending prorations will be removed.</p>

<p>By default, upon subscription cancellation, Stripe will stop automatic collection of all finalized invoices for the customer. This is intended to prevent unexpected payment attempts after the customer has canceled a subscription. However, you can resume automatic collection of the invoices manually after subscription cancellation to have us proceed. Or, you could check for unpaid invoices before allowing the customer to cancel the subscription at all.</p>*/
    pub fn delete_subscriptions_subscription_exposed_id(
        &self,
        subscription_exposed_id: String,
    ) -> request_model::DeleteSubscriptionsSubscriptionExposedIdRequest {
        request_model::DeleteSubscriptionsSubscriptionExposedIdRequest {
            client: &self,
            subscription_exposed_id,
        }
    }
    ///<p>Removes the currently applied discount on a subscription.</p>
    pub fn delete_subscriptions_subscription_exposed_id_discount(
        &self,
        subscription_exposed_id: String,
    ) -> request_model::DeleteSubscriptionsSubscriptionExposedIdDiscountRequest {
        request_model::DeleteSubscriptionsSubscriptionExposedIdDiscountRequest {
            client: &self,
            subscription_exposed_id,
        }
    }
    ///<p>A list of <a href="https://stripe.com/docs/tax/tax-categories">all tax codes available</a> to add to Products in order to allow specific tax calculations.</p>
    pub fn get_tax_codes(&self) -> request_model::GetTaxCodesRequest {
        request_model::GetTaxCodesRequest {
            client: &self,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
        }
    }
    ///<p>Retrieves the details of an existing tax code. Supply the unique tax code ID and Stripe will return the corresponding tax code information.</p>
    pub fn get_tax_codes_id(&self, id: String) -> request_model::GetTaxCodesIdRequest {
        request_model::GetTaxCodesIdRequest {
            client: &self,
            expand: None,
            id,
        }
    }
    ///<p>Returns a list of your tax rates. Tax rates are returned sorted by creation date, with the most recently created tax rates appearing first.</p>
    pub fn get_tax_rates(&self) -> request_model::GetTaxRatesRequest {
        request_model::GetTaxRatesRequest {
            client: &self,
            active: None,
            created: None,
            ending_before: None,
            expand: None,
            inclusive: None,
            limit: None,
            starting_after: None,
        }
    }
    ///<p>Creates a new tax rate.</p>
    pub fn post_tax_rates(&self) -> request_model::PostTaxRatesRequest {
        request_model::PostTaxRatesRequest {
            client: &self,
        }
    }
    ///<p>Retrieves a tax rate with the given ID</p>
    pub fn get_tax_rates_tax_rate(
        &self,
        tax_rate: String,
    ) -> request_model::GetTaxRatesTaxRateRequest {
        request_model::GetTaxRatesTaxRateRequest {
            client: &self,
            expand: None,
            tax_rate,
        }
    }
    ///<p>Updates an existing tax rate.</p>
    pub fn post_tax_rates_tax_rate(
        &self,
        tax_rate: String,
    ) -> request_model::PostTaxRatesTaxRateRequest {
        request_model::PostTaxRatesTaxRateRequest {
            client: &self,
            tax_rate,
        }
    }
    ///<p>Returns a list of <code>Configuration</code> objects.</p>
    pub fn get_terminal_configurations(
        &self,
    ) -> request_model::GetTerminalConfigurationsRequest {
        request_model::GetTerminalConfigurationsRequest {
            client: &self,
            ending_before: None,
            expand: None,
            is_account_default: None,
            limit: None,
            starting_after: None,
        }
    }
    ///<p>Creates a new <code>Configuration</code> object.</p>
    pub fn post_terminal_configurations(
        &self,
    ) -> request_model::PostTerminalConfigurationsRequest {
        request_model::PostTerminalConfigurationsRequest {
            client: &self,
        }
    }
    ///<p>Retrieves a <code>Configuration</code> object.</p>
    pub fn get_terminal_configurations_configuration(
        &self,
        configuration: String,
    ) -> request_model::GetTerminalConfigurationsConfigurationRequest {
        request_model::GetTerminalConfigurationsConfigurationRequest {
            client: &self,
            configuration,
            expand: None,
        }
    }
    ///<p>Updates a new <code>Configuration</code> object.</p>
    pub fn post_terminal_configurations_configuration(
        &self,
        configuration: String,
    ) -> request_model::PostTerminalConfigurationsConfigurationRequest {
        request_model::PostTerminalConfigurationsConfigurationRequest {
            client: &self,
            configuration,
        }
    }
    ///<p>Deletes a <code>Configuration</code> object.</p>
    pub fn delete_terminal_configurations_configuration(
        &self,
        configuration: String,
    ) -> request_model::DeleteTerminalConfigurationsConfigurationRequest {
        request_model::DeleteTerminalConfigurationsConfigurationRequest {
            client: &self,
            configuration,
        }
    }
    ///<p>To connect to a reader the Stripe Terminal SDK needs to retrieve a short-lived connection token from Stripe, proxied through your server. On your backend, add an endpoint that creates and returns a connection token.</p>
    pub fn post_terminal_connection_tokens(
        &self,
    ) -> request_model::PostTerminalConnectionTokensRequest {
        request_model::PostTerminalConnectionTokensRequest {
            client: &self,
        }
    }
    ///<p>Returns a list of <code>Location</code> objects.</p>
    pub fn get_terminal_locations(&self) -> request_model::GetTerminalLocationsRequest {
        request_model::GetTerminalLocationsRequest {
            client: &self,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
        }
    }
    /**<p>Creates a new <code>Location</code> object.
For further details, including which address fields are required in each country, see the <a href="/docs/terminal/fleet/locations">Manage locations</a> guide.</p>*/
    pub fn post_terminal_locations(
        &self,
    ) -> request_model::PostTerminalLocationsRequest {
        request_model::PostTerminalLocationsRequest {
            client: &self,
        }
    }
    ///<p>Retrieves a <code>Location</code> object.</p>
    pub fn get_terminal_locations_location(
        &self,
        location: String,
    ) -> request_model::GetTerminalLocationsLocationRequest {
        request_model::GetTerminalLocationsLocationRequest {
            client: &self,
            expand: None,
            location,
        }
    }
    ///<p>Updates a <code>Location</code> object by setting the values of the parameters passed. Any parameters not provided will be left unchanged.</p>
    pub fn post_terminal_locations_location(
        &self,
        location: String,
    ) -> request_model::PostTerminalLocationsLocationRequest {
        request_model::PostTerminalLocationsLocationRequest {
            client: &self,
            location,
        }
    }
    ///<p>Deletes a <code>Location</code> object.</p>
    pub fn delete_terminal_locations_location(
        &self,
        location: String,
    ) -> request_model::DeleteTerminalLocationsLocationRequest {
        request_model::DeleteTerminalLocationsLocationRequest {
            client: &self,
            location,
        }
    }
    ///<p>Returns a list of <code>Reader</code> objects.</p>
    pub fn get_terminal_readers(&self) -> request_model::GetTerminalReadersRequest {
        request_model::GetTerminalReadersRequest {
            client: &self,
            device_type: None,
            ending_before: None,
            expand: None,
            limit: None,
            location: None,
            starting_after: None,
            status: None,
        }
    }
    ///<p>Creates a new <code>Reader</code> object.</p>
    pub fn post_terminal_readers(&self) -> request_model::PostTerminalReadersRequest {
        request_model::PostTerminalReadersRequest {
            client: &self,
        }
    }
    ///<p>Retrieves a <code>Reader</code> object.</p>
    pub fn get_terminal_readers_reader(
        &self,
        reader: String,
    ) -> request_model::GetTerminalReadersReaderRequest {
        request_model::GetTerminalReadersReaderRequest {
            client: &self,
            expand: None,
            reader,
        }
    }
    ///<p>Updates a <code>Reader</code> object by setting the values of the parameters passed. Any parameters not provided will be left unchanged.</p>
    pub fn post_terminal_readers_reader(
        &self,
        reader: String,
    ) -> request_model::PostTerminalReadersReaderRequest {
        request_model::PostTerminalReadersReaderRequest {
            client: &self,
            reader,
        }
    }
    ///<p>Deletes a <code>Reader</code> object.</p>
    pub fn delete_terminal_readers_reader(
        &self,
        reader: String,
    ) -> request_model::DeleteTerminalReadersReaderRequest {
        request_model::DeleteTerminalReadersReaderRequest {
            client: &self,
            reader,
        }
    }
    ///<p>Cancels the current reader action.</p>
    pub fn post_terminal_readers_reader_cancel_action(
        &self,
        reader: String,
    ) -> request_model::PostTerminalReadersReaderCancelActionRequest {
        request_model::PostTerminalReadersReaderCancelActionRequest {
            client: &self,
            reader,
        }
    }
    ///<p>Initiates a payment flow on a Reader.</p>
    pub fn post_terminal_readers_reader_process_payment_intent(
        &self,
        reader: String,
    ) -> request_model::PostTerminalReadersReaderProcessPaymentIntentRequest {
        request_model::PostTerminalReadersReaderProcessPaymentIntentRequest {
            client: &self,
            reader,
        }
    }
    ///<p>Initiates a setup intent flow on a Reader.</p>
    pub fn post_terminal_readers_reader_process_setup_intent(
        &self,
        reader: String,
    ) -> request_model::PostTerminalReadersReaderProcessSetupIntentRequest {
        request_model::PostTerminalReadersReaderProcessSetupIntentRequest {
            client: &self,
            reader,
        }
    }
    ///<p>Sets reader display to show cart details.</p>
    pub fn post_terminal_readers_reader_set_reader_display(
        &self,
        reader: String,
    ) -> request_model::PostTerminalReadersReaderSetReaderDisplayRequest {
        request_model::PostTerminalReadersReaderSetReaderDisplayRequest {
            client: &self,
            reader,
        }
    }
    ///<p>Create an incoming testmode bank transfer</p>
    pub fn post_test_helpers_customers_customer_fund_cash_balance(
        &self,
        customer: String,
    ) -> request_model::PostTestHelpersCustomersCustomerFundCashBalanceRequest {
        request_model::PostTestHelpersCustomersCustomerFundCashBalanceRequest {
            client: &self,
            customer,
        }
    }
    ///<p>Updates the shipping status of the specified Issuing <code>Card</code> object to <code>delivered</code>.</p>
    pub fn post_test_helpers_issuing_cards_card_shipping_deliver(
        &self,
        card: String,
    ) -> request_model::PostTestHelpersIssuingCardsCardShippingDeliverRequest {
        request_model::PostTestHelpersIssuingCardsCardShippingDeliverRequest {
            client: &self,
            card,
        }
    }
    ///<p>Updates the shipping status of the specified Issuing <code>Card</code> object to <code>failure</code>.</p>
    pub fn post_test_helpers_issuing_cards_card_shipping_fail(
        &self,
        card: String,
    ) -> request_model::PostTestHelpersIssuingCardsCardShippingFailRequest {
        request_model::PostTestHelpersIssuingCardsCardShippingFailRequest {
            client: &self,
            card,
        }
    }
    ///<p>Updates the shipping status of the specified Issuing <code>Card</code> object to <code>returned</code>.</p>
    pub fn post_test_helpers_issuing_cards_card_shipping_return(
        &self,
        card: String,
    ) -> request_model::PostTestHelpersIssuingCardsCardShippingReturnRequest {
        request_model::PostTestHelpersIssuingCardsCardShippingReturnRequest {
            client: &self,
            card,
        }
    }
    ///<p>Updates the shipping status of the specified Issuing <code>Card</code> object to <code>shipped</code>.</p>
    pub fn post_test_helpers_issuing_cards_card_shipping_ship(
        &self,
        card: String,
    ) -> request_model::PostTestHelpersIssuingCardsCardShippingShipRequest {
        request_model::PostTestHelpersIssuingCardsCardShippingShipRequest {
            client: &self,
            card,
        }
    }
    ///<p>Expire a refund with a status of <code>requires_action</code>.</p>
    pub fn post_test_helpers_refunds_refund_expire(
        &self,
        refund: String,
    ) -> request_model::PostTestHelpersRefundsRefundExpireRequest {
        request_model::PostTestHelpersRefundsRefundExpireRequest {
            client: &self,
            refund,
        }
    }
    ///<p>Presents a payment method on a simulated reader. Can be used to simulate accepting a payment, saving a card or refunding a transaction.</p>
    pub fn post_test_helpers_terminal_readers_reader_present_payment_method(
        &self,
        reader: String,
    ) -> request_model::PostTestHelpersTerminalReadersReaderPresentPaymentMethodRequest {
        request_model::PostTestHelpersTerminalReadersReaderPresentPaymentMethodRequest {
            client: &self,
            reader,
        }
    }
    ///<p>Returns a list of your test clocks.</p>
    pub fn get_test_helpers_test_clocks(
        &self,
    ) -> request_model::GetTestHelpersTestClocksRequest {
        request_model::GetTestHelpersTestClocksRequest {
            client: &self,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
        }
    }
    ///<p>Creates a new test clock that can be attached to new customers and quotes.</p>
    pub fn post_test_helpers_test_clocks(
        &self,
    ) -> request_model::PostTestHelpersTestClocksRequest {
        request_model::PostTestHelpersTestClocksRequest {
            client: &self,
        }
    }
    ///<p>Retrieves a test clock.</p>
    pub fn get_test_helpers_test_clocks_test_clock(
        &self,
        test_clock: String,
    ) -> request_model::GetTestHelpersTestClocksTestClockRequest {
        request_model::GetTestHelpersTestClocksTestClockRequest {
            client: &self,
            expand: None,
            test_clock,
        }
    }
    ///<p>Deletes a test clock.</p>
    pub fn delete_test_helpers_test_clocks_test_clock(
        &self,
        test_clock: String,
    ) -> request_model::DeleteTestHelpersTestClocksTestClockRequest {
        request_model::DeleteTestHelpersTestClocksTestClockRequest {
            client: &self,
            test_clock,
        }
    }
    ///<p>Starts advancing a test clock to a specified time in the future. Advancement is done when status changes to <code>Ready</code>.</p>
    pub fn post_test_helpers_test_clocks_test_clock_advance(
        &self,
        test_clock: String,
    ) -> request_model::PostTestHelpersTestClocksTestClockAdvanceRequest {
        request_model::PostTestHelpersTestClocksTestClockAdvanceRequest {
            client: &self,
            test_clock,
        }
    }
    ///<p>Transitions a test mode created InboundTransfer to the <code>failed</code> status. The InboundTransfer must already be in the <code>processing</code> state.</p>
    pub fn post_test_helpers_treasury_inbound_transfers_id_fail(
        &self,
        id: String,
    ) -> request_model::PostTestHelpersTreasuryInboundTransfersIdFailRequest {
        request_model::PostTestHelpersTreasuryInboundTransfersIdFailRequest {
            client: &self,
            id,
        }
    }
    ///<p>Marks the test mode InboundTransfer object as returned and links the InboundTransfer to a ReceivedDebit. The InboundTransfer must already be in the <code>succeeded</code> state.</p>
    pub fn post_test_helpers_treasury_inbound_transfers_id_return(
        &self,
        id: String,
    ) -> request_model::PostTestHelpersTreasuryInboundTransfersIdReturnRequest {
        request_model::PostTestHelpersTreasuryInboundTransfersIdReturnRequest {
            client: &self,
            id,
        }
    }
    ///<p>Transitions a test mode created InboundTransfer to the <code>succeeded</code> status. The InboundTransfer must already be in the <code>processing</code> state.</p>
    pub fn post_test_helpers_treasury_inbound_transfers_id_succeed(
        &self,
        id: String,
    ) -> request_model::PostTestHelpersTreasuryInboundTransfersIdSucceedRequest {
        request_model::PostTestHelpersTreasuryInboundTransfersIdSucceedRequest {
            client: &self,
            id,
        }
    }
    ///<p>Transitions a test mode created OutboundPayment to the <code>failed</code> status. The OutboundPayment must already be in the <code>processing</code> state.</p>
    pub fn post_test_helpers_treasury_outbound_payments_id_fail(
        &self,
        id: String,
    ) -> request_model::PostTestHelpersTreasuryOutboundPaymentsIdFailRequest {
        request_model::PostTestHelpersTreasuryOutboundPaymentsIdFailRequest {
            client: &self,
            id,
        }
    }
    ///<p>Transitions a test mode created OutboundPayment to the <code>posted</code> status. The OutboundPayment must already be in the <code>processing</code> state.</p>
    pub fn post_test_helpers_treasury_outbound_payments_id_post(
        &self,
        id: String,
    ) -> request_model::PostTestHelpersTreasuryOutboundPaymentsIdPostRequest {
        request_model::PostTestHelpersTreasuryOutboundPaymentsIdPostRequest {
            client: &self,
            id,
        }
    }
    ///<p>Transitions a test mode created OutboundPayment to the <code>returned</code> status. The OutboundPayment must already be in the <code>processing</code> state.</p>
    pub fn post_test_helpers_treasury_outbound_payments_id_return(
        &self,
        id: String,
    ) -> request_model::PostTestHelpersTreasuryOutboundPaymentsIdReturnRequest {
        request_model::PostTestHelpersTreasuryOutboundPaymentsIdReturnRequest {
            client: &self,
            id,
        }
    }
    ///<p>Transitions a test mode created OutboundTransfer to the <code>failed</code> status. The OutboundTransfer must already be in the <code>processing</code> state.</p>
    pub fn post_test_helpers_treasury_outbound_transfers_outbound_transfer_fail(
        &self,
        outbound_transfer: String,
    ) -> request_model::PostTestHelpersTreasuryOutboundTransfersOutboundTransferFailRequest {
        request_model::PostTestHelpersTreasuryOutboundTransfersOutboundTransferFailRequest {
            client: &self,
            outbound_transfer,
        }
    }
    ///<p>Transitions a test mode created OutboundTransfer to the <code>posted</code> status. The OutboundTransfer must already be in the <code>processing</code> state.</p>
    pub fn post_test_helpers_treasury_outbound_transfers_outbound_transfer_post(
        &self,
        outbound_transfer: String,
    ) -> request_model::PostTestHelpersTreasuryOutboundTransfersOutboundTransferPostRequest {
        request_model::PostTestHelpersTreasuryOutboundTransfersOutboundTransferPostRequest {
            client: &self,
            outbound_transfer,
        }
    }
    ///<p>Transitions a test mode created OutboundTransfer to the <code>returned</code> status. The OutboundTransfer must already be in the <code>processing</code> state.</p>
    pub fn post_test_helpers_treasury_outbound_transfers_outbound_transfer_return(
        &self,
        outbound_transfer: String,
    ) -> request_model::PostTestHelpersTreasuryOutboundTransfersOutboundTransferReturnRequest {
        request_model::PostTestHelpersTreasuryOutboundTransfersOutboundTransferReturnRequest {
            client: &self,
            outbound_transfer,
        }
    }
    ///<p>Use this endpoint to simulate a test mode ReceivedCredit initiated by a third party. In live mode, you can’t directly create ReceivedCredits initiated by third parties.</p>
    pub fn post_test_helpers_treasury_received_credits(
        &self,
    ) -> request_model::PostTestHelpersTreasuryReceivedCreditsRequest {
        request_model::PostTestHelpersTreasuryReceivedCreditsRequest {
            client: &self,
        }
    }
    ///<p>Use this endpoint to simulate a test mode ReceivedDebit initiated by a third party. In live mode, you can’t directly create ReceivedDebits initiated by third parties.</p>
    pub fn post_test_helpers_treasury_received_debits(
        &self,
    ) -> request_model::PostTestHelpersTreasuryReceivedDebitsRequest {
        request_model::PostTestHelpersTreasuryReceivedDebitsRequest {
            client: &self,
        }
    }
    /**<p>Creates a single-use token that represents a bank account’s details.
This token can be used with any API method in place of a bank account dictionary. This token can be used only once, by attaching it to a <a href="#accounts">Custom account</a>.</p>*/
    pub fn post_tokens(&self) -> request_model::PostTokensRequest {
        request_model::PostTokensRequest {
            client: &self,
        }
    }
    ///<p>Retrieves the token with the given ID.</p>
    pub fn get_tokens_token(
        &self,
        token: String,
    ) -> request_model::GetTokensTokenRequest {
        request_model::GetTokensTokenRequest {
            client: &self,
            expand: None,
            token,
        }
    }
    ///<p>Returns a list of top-ups.</p>
    pub fn get_topups(&self) -> request_model::GetTopupsRequest {
        request_model::GetTopupsRequest {
            client: &self,
            amount: None,
            created: None,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
            status: None,
        }
    }
    ///<p>Top up the balance of an account</p>
    pub fn post_topups(&self) -> request_model::PostTopupsRequest {
        request_model::PostTopupsRequest {
            client: &self,
        }
    }
    ///<p>Retrieves the details of a top-up that has previously been created. Supply the unique top-up ID that was returned from your previous request, and Stripe will return the corresponding top-up information.</p>
    pub fn get_topups_topup(
        &self,
        topup: String,
    ) -> request_model::GetTopupsTopupRequest {
        request_model::GetTopupsTopupRequest {
            client: &self,
            expand: None,
            topup,
        }
    }
    ///<p>Updates the metadata of a top-up. Other top-up details are not editable by design.</p>
    pub fn post_topups_topup(
        &self,
        topup: String,
    ) -> request_model::PostTopupsTopupRequest {
        request_model::PostTopupsTopupRequest {
            client: &self,
            topup,
        }
    }
    ///<p>Cancels a top-up. Only pending top-ups can be canceled.</p>
    pub fn post_topups_topup_cancel(
        &self,
        topup: String,
    ) -> request_model::PostTopupsTopupCancelRequest {
        request_model::PostTopupsTopupCancelRequest {
            client: &self,
            topup,
        }
    }
    ///<p>Returns a list of existing transfers sent to connected accounts. The transfers are returned in sorted order, with the most recently created transfers appearing first.</p>
    pub fn get_transfers(&self) -> request_model::GetTransfersRequest {
        request_model::GetTransfersRequest {
            client: &self,
            created: None,
            destination: None,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
            transfer_group: None,
        }
    }
    ///<p>To send funds from your Stripe account to a connected account, you create a new transfer object. Your <a href="#balance">Stripe balance</a> must be able to cover the transfer amount, or you’ll receive an “Insufficient Funds” error.</p>
    pub fn post_transfers(&self) -> request_model::PostTransfersRequest {
        request_model::PostTransfersRequest {
            client: &self,
        }
    }
    ///<p>You can see a list of the reversals belonging to a specific transfer. Note that the 10 most recent reversals are always available by default on the transfer object. If you need more than those 10, you can use this API method and the <code>limit</code> and <code>starting_after</code> parameters to page through additional reversals.</p>
    pub fn get_transfers_id_reversals(
        &self,
        id: String,
    ) -> request_model::GetTransfersIdReversalsRequest {
        request_model::GetTransfersIdReversalsRequest {
            client: &self,
            ending_before: None,
            expand: None,
            id,
            limit: None,
            starting_after: None,
        }
    }
    /**<p>When you create a new reversal, you must specify a transfer to create it on.</p>

<p>When reversing transfers, you can optionally reverse part of the transfer. You can do so as many times as you wish until the entire transfer has been reversed.</p>

<p>Once entirely reversed, a transfer can’t be reversed again. This method will return an error when called on an already-reversed transfer, or when trying to reverse more money than is left on a transfer.</p>*/
    pub fn post_transfers_id_reversals(
        &self,
        id: String,
    ) -> request_model::PostTransfersIdReversalsRequest {
        request_model::PostTransfersIdReversalsRequest {
            client: &self,
            id,
        }
    }
    ///<p>Retrieves the details of an existing transfer. Supply the unique transfer ID from either a transfer creation request or the transfer list, and Stripe will return the corresponding transfer information.</p>
    pub fn get_transfers_transfer(
        &self,
        transfer: String,
    ) -> request_model::GetTransfersTransferRequest {
        request_model::GetTransfersTransferRequest {
            client: &self,
            expand: None,
            transfer,
        }
    }
    /**<p>Updates the specified transfer by setting the values of the parameters passed. Any parameters not provided will be left unchanged.</p>

<p>This request accepts only metadata as an argument.</p>*/
    pub fn post_transfers_transfer(
        &self,
        transfer: String,
    ) -> request_model::PostTransfersTransferRequest {
        request_model::PostTransfersTransferRequest {
            client: &self,
            transfer,
        }
    }
    ///<p>By default, you can see the 10 most recent reversals stored directly on the transfer object, but you can also retrieve details about a specific reversal stored on the transfer.</p>
    pub fn get_transfers_transfer_reversals_id(
        &self,
        id: String,
        transfer: String,
    ) -> request_model::GetTransfersTransferReversalsIdRequest {
        request_model::GetTransfersTransferReversalsIdRequest {
            client: &self,
            expand: None,
            id,
            transfer,
        }
    }
    /**<p>Updates the specified reversal by setting the values of the parameters passed. Any parameters not provided will be left unchanged.</p>

<p>This request only accepts metadata and description as arguments.</p>*/
    pub fn post_transfers_transfer_reversals_id(
        &self,
        id: String,
        transfer: String,
    ) -> request_model::PostTransfersTransferReversalsIdRequest {
        request_model::PostTransfersTransferReversalsIdRequest {
            client: &self,
            id,
            transfer,
        }
    }
    ///<p>Returns a list of CreditReversals.</p>
    pub fn get_treasury_credit_reversals(
        &self,
        financial_account: String,
    ) -> request_model::GetTreasuryCreditReversalsRequest {
        request_model::GetTreasuryCreditReversalsRequest {
            client: &self,
            ending_before: None,
            expand: None,
            financial_account,
            limit: None,
            received_credit: None,
            starting_after: None,
            status: None,
        }
    }
    ///<p>Reverses a ReceivedCredit and creates a CreditReversal object.</p>
    pub fn post_treasury_credit_reversals(
        &self,
    ) -> request_model::PostTreasuryCreditReversalsRequest {
        request_model::PostTreasuryCreditReversalsRequest {
            client: &self,
        }
    }
    ///<p>Retrieves the details of an existing CreditReversal by passing the unique CreditReversal ID from either the CreditReversal creation request or CreditReversal list</p>
    pub fn get_treasury_credit_reversals_credit_reversal(
        &self,
        credit_reversal: String,
    ) -> request_model::GetTreasuryCreditReversalsCreditReversalRequest {
        request_model::GetTreasuryCreditReversalsCreditReversalRequest {
            client: &self,
            credit_reversal,
            expand: None,
        }
    }
    ///<p>Returns a list of DebitReversals.</p>
    pub fn get_treasury_debit_reversals(
        &self,
        financial_account: String,
    ) -> request_model::GetTreasuryDebitReversalsRequest {
        request_model::GetTreasuryDebitReversalsRequest {
            client: &self,
            ending_before: None,
            expand: None,
            financial_account,
            limit: None,
            received_debit: None,
            resolution: None,
            starting_after: None,
            status: None,
        }
    }
    ///<p>Reverses a ReceivedDebit and creates a DebitReversal object.</p>
    pub fn post_treasury_debit_reversals(
        &self,
    ) -> request_model::PostTreasuryDebitReversalsRequest {
        request_model::PostTreasuryDebitReversalsRequest {
            client: &self,
        }
    }
    ///<p>Retrieves a DebitReversal object.</p>
    pub fn get_treasury_debit_reversals_debit_reversal(
        &self,
        debit_reversal: String,
    ) -> request_model::GetTreasuryDebitReversalsDebitReversalRequest {
        request_model::GetTreasuryDebitReversalsDebitReversalRequest {
            client: &self,
            debit_reversal,
            expand: None,
        }
    }
    ///<p>Returns a list of FinancialAccounts.</p>
    pub fn get_treasury_financial_accounts(
        &self,
    ) -> request_model::GetTreasuryFinancialAccountsRequest {
        request_model::GetTreasuryFinancialAccountsRequest {
            client: &self,
            created: None,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
        }
    }
    ///<p>Creates a new FinancialAccount. For now, each connected account can only have one FinancialAccount.</p>
    pub fn post_treasury_financial_accounts(
        &self,
    ) -> request_model::PostTreasuryFinancialAccountsRequest {
        request_model::PostTreasuryFinancialAccountsRequest {
            client: &self,
        }
    }
    ///<p>Retrieves the details of a FinancialAccount.</p>
    pub fn get_treasury_financial_accounts_financial_account(
        &self,
        financial_account: String,
    ) -> request_model::GetTreasuryFinancialAccountsFinancialAccountRequest {
        request_model::GetTreasuryFinancialAccountsFinancialAccountRequest {
            client: &self,
            expand: None,
            financial_account,
        }
    }
    ///<p>Updates the details of a FinancialAccount.</p>
    pub fn post_treasury_financial_accounts_financial_account(
        &self,
        financial_account: String,
    ) -> request_model::PostTreasuryFinancialAccountsFinancialAccountRequest {
        request_model::PostTreasuryFinancialAccountsFinancialAccountRequest {
            client: &self,
            financial_account,
        }
    }
    ///<p>Retrieves Features information associated with the FinancialAccount.</p>
    pub fn get_treasury_financial_accounts_financial_account_features(
        &self,
        financial_account: String,
    ) -> request_model::GetTreasuryFinancialAccountsFinancialAccountFeaturesRequest {
        request_model::GetTreasuryFinancialAccountsFinancialAccountFeaturesRequest {
            client: &self,
            expand: None,
            financial_account,
        }
    }
    ///<p>Updates the Features associated with a FinancialAccount.</p>
    pub fn post_treasury_financial_accounts_financial_account_features(
        &self,
        financial_account: String,
    ) -> request_model::PostTreasuryFinancialAccountsFinancialAccountFeaturesRequest {
        request_model::PostTreasuryFinancialAccountsFinancialAccountFeaturesRequest {
            client: &self,
            financial_account,
        }
    }
    ///<p>Returns a list of InboundTransfers sent from the specified FinancialAccount.</p>
    pub fn get_treasury_inbound_transfers(
        &self,
        financial_account: String,
    ) -> request_model::GetTreasuryInboundTransfersRequest {
        request_model::GetTreasuryInboundTransfersRequest {
            client: &self,
            ending_before: None,
            expand: None,
            financial_account,
            limit: None,
            starting_after: None,
            status: None,
        }
    }
    ///<p>Creates an InboundTransfer.</p>
    pub fn post_treasury_inbound_transfers(
        &self,
    ) -> request_model::PostTreasuryInboundTransfersRequest {
        request_model::PostTreasuryInboundTransfersRequest {
            client: &self,
        }
    }
    ///<p>Retrieves the details of an existing InboundTransfer.</p>
    pub fn get_treasury_inbound_transfers_id(
        &self,
        id: String,
    ) -> request_model::GetTreasuryInboundTransfersIdRequest {
        request_model::GetTreasuryInboundTransfersIdRequest {
            client: &self,
            expand: None,
            id,
        }
    }
    ///<p>Cancels an InboundTransfer.</p>
    pub fn post_treasury_inbound_transfers_inbound_transfer_cancel(
        &self,
        inbound_transfer: String,
    ) -> request_model::PostTreasuryInboundTransfersInboundTransferCancelRequest {
        request_model::PostTreasuryInboundTransfersInboundTransferCancelRequest {
            client: &self,
            inbound_transfer,
        }
    }
    ///<p>Returns a list of OutboundPayments sent from the specified FinancialAccount.</p>
    pub fn get_treasury_outbound_payments(
        &self,
        financial_account: String,
    ) -> request_model::GetTreasuryOutboundPaymentsRequest {
        request_model::GetTreasuryOutboundPaymentsRequest {
            client: &self,
            customer: None,
            ending_before: None,
            expand: None,
            financial_account,
            limit: None,
            starting_after: None,
            status: None,
        }
    }
    ///<p>Creates an OutboundPayment.</p>
    pub fn post_treasury_outbound_payments(
        &self,
    ) -> request_model::PostTreasuryOutboundPaymentsRequest {
        request_model::PostTreasuryOutboundPaymentsRequest {
            client: &self,
        }
    }
    ///<p>Retrieves the details of an existing OutboundPayment by passing the unique OutboundPayment ID from either the OutboundPayment creation request or OutboundPayment list.</p>
    pub fn get_treasury_outbound_payments_id(
        &self,
        id: String,
    ) -> request_model::GetTreasuryOutboundPaymentsIdRequest {
        request_model::GetTreasuryOutboundPaymentsIdRequest {
            client: &self,
            expand: None,
            id,
        }
    }
    ///<p>Cancel an OutboundPayment.</p>
    pub fn post_treasury_outbound_payments_id_cancel(
        &self,
        id: String,
    ) -> request_model::PostTreasuryOutboundPaymentsIdCancelRequest {
        request_model::PostTreasuryOutboundPaymentsIdCancelRequest {
            client: &self,
            id,
        }
    }
    ///<p>Returns a list of OutboundTransfers sent from the specified FinancialAccount.</p>
    pub fn get_treasury_outbound_transfers(
        &self,
        financial_account: String,
    ) -> request_model::GetTreasuryOutboundTransfersRequest {
        request_model::GetTreasuryOutboundTransfersRequest {
            client: &self,
            ending_before: None,
            expand: None,
            financial_account,
            limit: None,
            starting_after: None,
            status: None,
        }
    }
    ///<p>Creates an OutboundTransfer.</p>
    pub fn post_treasury_outbound_transfers(
        &self,
    ) -> request_model::PostTreasuryOutboundTransfersRequest {
        request_model::PostTreasuryOutboundTransfersRequest {
            client: &self,
        }
    }
    ///<p>Retrieves the details of an existing OutboundTransfer by passing the unique OutboundTransfer ID from either the OutboundTransfer creation request or OutboundTransfer list.</p>
    pub fn get_treasury_outbound_transfers_outbound_transfer(
        &self,
        outbound_transfer: String,
    ) -> request_model::GetTreasuryOutboundTransfersOutboundTransferRequest {
        request_model::GetTreasuryOutboundTransfersOutboundTransferRequest {
            client: &self,
            expand: None,
            outbound_transfer,
        }
    }
    ///<p>An OutboundTransfer can be canceled if the funds have not yet been paid out.</p>
    pub fn post_treasury_outbound_transfers_outbound_transfer_cancel(
        &self,
        outbound_transfer: String,
    ) -> request_model::PostTreasuryOutboundTransfersOutboundTransferCancelRequest {
        request_model::PostTreasuryOutboundTransfersOutboundTransferCancelRequest {
            client: &self,
            outbound_transfer,
        }
    }
    ///<p>Returns a list of ReceivedCredits.</p>
    pub fn get_treasury_received_credits(
        &self,
        financial_account: String,
    ) -> request_model::GetTreasuryReceivedCreditsRequest {
        request_model::GetTreasuryReceivedCreditsRequest {
            client: &self,
            ending_before: None,
            expand: None,
            financial_account,
            limit: None,
            linked_flows: None,
            starting_after: None,
            status: None,
        }
    }
    ///<p>Retrieves the details of an existing ReceivedCredit by passing the unique ReceivedCredit ID from the ReceivedCredit list.</p>
    pub fn get_treasury_received_credits_id(
        &self,
        id: String,
    ) -> request_model::GetTreasuryReceivedCreditsIdRequest {
        request_model::GetTreasuryReceivedCreditsIdRequest {
            client: &self,
            expand: None,
            id,
        }
    }
    ///<p>Returns a list of ReceivedDebits.</p>
    pub fn get_treasury_received_debits(
        &self,
        financial_account: String,
    ) -> request_model::GetTreasuryReceivedDebitsRequest {
        request_model::GetTreasuryReceivedDebitsRequest {
            client: &self,
            ending_before: None,
            expand: None,
            financial_account,
            limit: None,
            starting_after: None,
            status: None,
        }
    }
    ///<p>Retrieves the details of an existing ReceivedDebit by passing the unique ReceivedDebit ID from the ReceivedDebit list</p>
    pub fn get_treasury_received_debits_id(
        &self,
        id: String,
    ) -> request_model::GetTreasuryReceivedDebitsIdRequest {
        request_model::GetTreasuryReceivedDebitsIdRequest {
            client: &self,
            expand: None,
            id,
        }
    }
    ///<p>Retrieves a list of TransactionEntry objects.</p>
    pub fn get_treasury_transaction_entries(
        &self,
        financial_account: String,
    ) -> request_model::GetTreasuryTransactionEntriesRequest {
        request_model::GetTreasuryTransactionEntriesRequest {
            client: &self,
            created: None,
            effective_at: None,
            ending_before: None,
            expand: None,
            financial_account,
            limit: None,
            order_by: None,
            starting_after: None,
            transaction: None,
        }
    }
    ///<p>Retrieves a TransactionEntry object.</p>
    pub fn get_treasury_transaction_entries_id(
        &self,
        id: String,
    ) -> request_model::GetTreasuryTransactionEntriesIdRequest {
        request_model::GetTreasuryTransactionEntriesIdRequest {
            client: &self,
            expand: None,
            id,
        }
    }
    ///<p>Retrieves a list of Transaction objects.</p>
    pub fn get_treasury_transactions(
        &self,
        financial_account: String,
    ) -> request_model::GetTreasuryTransactionsRequest {
        request_model::GetTreasuryTransactionsRequest {
            client: &self,
            created: None,
            ending_before: None,
            expand: None,
            financial_account,
            limit: None,
            order_by: None,
            starting_after: None,
            status: None,
            status_transitions: None,
        }
    }
    ///<p>Retrieves the details of an existing Transaction.</p>
    pub fn get_treasury_transactions_id(
        &self,
        id: String,
    ) -> request_model::GetTreasuryTransactionsIdRequest {
        request_model::GetTreasuryTransactionsIdRequest {
            client: &self,
            expand: None,
            id,
        }
    }
    ///<p>Returns a list of your webhook endpoints.</p>
    pub fn get_webhook_endpoints(&self) -> request_model::GetWebhookEndpointsRequest {
        request_model::GetWebhookEndpointsRequest {
            client: &self,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
        }
    }
    ///<p>A webhook endpoint must have a <code>url</code> and a list of <code>enabled_events</code>. You may optionally specify the Boolean <code>connect</code> parameter. If set to true, then a Connect webhook endpoint that notifies the specified <code>url</code> about events from all connected accounts is created; otherwise an account webhook endpoint that notifies the specified <code>url</code> only about events from your account is created. You can also create webhook endpoints in the <a href="https://dashboard.stripe.com/account/webhooks">webhooks settings</a> section of the Dashboard.</p>
    pub fn post_webhook_endpoints(&self) -> request_model::PostWebhookEndpointsRequest {
        request_model::PostWebhookEndpointsRequest {
            client: &self,
        }
    }
    ///<p>Retrieves the webhook endpoint with the given ID.</p>
    pub fn get_webhook_endpoints_webhook_endpoint(
        &self,
        webhook_endpoint: String,
    ) -> request_model::GetWebhookEndpointsWebhookEndpointRequest {
        request_model::GetWebhookEndpointsWebhookEndpointRequest {
            client: &self,
            expand: None,
            webhook_endpoint,
        }
    }
    ///<p>Updates the webhook endpoint. You may edit the <code>url</code>, the list of <code>enabled_events</code>, and the status of your endpoint.</p>
    pub fn post_webhook_endpoints_webhook_endpoint(
        &self,
        webhook_endpoint: String,
    ) -> request_model::PostWebhookEndpointsWebhookEndpointRequest {
        request_model::PostWebhookEndpointsWebhookEndpointRequest {
            client: &self,
            webhook_endpoint,
        }
    }
    ///<p>You can also delete webhook endpoints via the <a href="https://dashboard.stripe.com/account/webhooks">webhook endpoint management</a> page of the Stripe dashboard.</p>
    pub fn delete_webhook_endpoints_webhook_endpoint(
        &self,
        webhook_endpoint: String,
    ) -> request_model::DeleteWebhookEndpointsWebhookEndpointRequest {
        request_model::DeleteWebhookEndpointsWebhookEndpointRequest {
            client: &self,
            webhook_endpoint,
        }
    }
}
pub enum StripeAuthentication {
    BasicAuth { basic_auth: String },
    BearerAuth { bearer_auth: String },
}
impl StripeAuthentication {
    pub fn from_env() -> Self {
        Self::BasicAuth {
            basic_auth: std::env::var("STRIPE_BASIC_AUTH")
                .expect("Environment variable STRIPE_BASIC_AUTH is not set."),
        }
    }
}
