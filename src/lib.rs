//! [`StripeClient`](struct.StripeClient.html) is the main entry point for this library.
//!
//! Library created with [`libninja`](https://www.libninja.com).
#![allow(non_camel_case_types)]
#![allow(unused)]
pub mod model;
pub mod request;
use crate::model::*;
pub struct StripeClient {
    pub client: httpclient::Client,
    authentication: StripeAuthentication,
}
impl StripeClient {
    pub fn from_env() -> Self {
        Self {
            client: httpclient::Client::new().base_url("https://api.stripe.com/"),
            authentication: StripeAuthentication::from_env(),
        }
    }
}
impl StripeClient {
    pub fn new(url: &str, authentication: StripeAuthentication) -> Self {
        let client = httpclient::Client::new().base_url(url);
        Self { client, authentication }
    }
    pub fn with_authentication(mut self, authentication: StripeAuthentication) -> Self {
        self.authentication = authentication;
        self
    }
    pub(crate) fn authenticate<'a>(
        &self,
        mut r: httpclient::RequestBuilder<'a>,
    ) -> httpclient::RequestBuilder<'a> {
        match &self.authentication {
            StripeAuthentication::BasicAuth { basic_auth } => {
                r = r.basic_auth(basic_auth);
            }
            StripeAuthentication::BearerAuth { bearer_auth } => {
                r = r.bearer_auth(bearer_auth);
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
    pub fn get_account(&self) -> request::GetAccountRequest {
        request::GetAccountRequest {
            http_client: &self,
            expand: None,
        }
    }
    ///<p>Creates an AccountLink object that includes a single-use Stripe URL that the platform can redirect their user to in order to take them through the Connect Onboarding flow.</p>
    pub fn post_account_links(&self) -> request::PostAccountLinksRequest {
        request::PostAccountLinksRequest {
            http_client: &self,
        }
    }
    ///<p>Creates a AccountSession object that includes a single-use token that the platform can use on their front-end to grant client-side API access.</p>
    pub fn post_account_sessions(&self) -> request::PostAccountSessionsRequest {
        request::PostAccountSessionsRequest {
            http_client: &self,
        }
    }
    ///<p>Returns a list of accounts connected to your platform via <a href="/docs/connect">Connect</a>. If you’re not a platform, the list is empty.</p>
    pub fn get_accounts(&self) -> request::GetAccountsRequest {
        request::GetAccountsRequest {
            http_client: &self,
            created: None,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
        }
    }
    /**<p>With <a href="/docs/connect">Connect</a>, you can create Stripe accounts for your users.
To do this, you’ll first need to <a href="https://dashboard.stripe.com/account/applications/settings">register your platform</a>.</p>

<p>If you’ve already collected information for your connected accounts, you <a href="/docs/connect/best-practices#onboarding">can prefill that information</a> when
creating the account. Connect Onboarding won’t ask for the prefilled information during account onboarding.
You can prefill any information on the account.</p>*/
    pub fn post_accounts(&self) -> request::PostAccountsRequest {
        request::PostAccountsRequest {
            http_client: &self,
        }
    }
    ///<p>Retrieves the details of an account.</p>
    pub fn get_accounts_account(
        &self,
        account: &str,
    ) -> request::GetAccountsAccountRequest {
        request::GetAccountsAccountRequest {
            http_client: &self,
            account: account.to_owned(),
            expand: None,
        }
    }
    /**<p>Updates a <a href="/docs/connect/accounts">connected account</a> by setting the values of the parameters passed. Any parameters not provided are
left unchanged.</p>

<p>For Custom accounts, you can update any information on the account. For other accounts, you can update all information until that
account has started to go through Connect Onboarding. Once you create an <a href="/docs/api/account_links">Account Link</a>
for a Standard or Express account, some parameters can no longer be changed. These are marked as <strong>Custom Only</strong> or <strong>Custom and Express</strong>
below.</p>

<p>To update your own account, use the <a href="https://dashboard.stripe.com/settings/account">Dashboard</a>. Refer to our
<a href="/docs/connect/updating-accounts">Connect</a> documentation to learn more about updating accounts.</p>*/
    pub fn post_accounts_account(
        &self,
        account: &str,
    ) -> request::PostAccountsAccountRequest {
        request::PostAccountsAccountRequest {
            http_client: &self,
            account: account.to_owned(),
        }
    }
    /**<p>With <a href="/docs/connect">Connect</a>, you can delete accounts you manage.</p>

<p>Accounts created using test-mode keys can be deleted at any time. Standard accounts created using live-mode keys cannot be deleted. Custom or Express accounts created using live-mode keys can only be deleted once all balances are zero.</p>

<p>If you want to delete your own account, use the <a href="https://dashboard.stripe.com/settings/account">account information tab in your account settings</a> instead.</p>*/
    pub fn delete_accounts_account(
        &self,
        account: &str,
    ) -> request::DeleteAccountsAccountRequest {
        request::DeleteAccountsAccountRequest {
            http_client: &self,
            account: account.to_owned(),
        }
    }
    ///<p>Create an external account for a given account.</p>
    pub fn post_accounts_account_bank_accounts(
        &self,
        account: &str,
    ) -> request::PostAccountsAccountBankAccountsRequest {
        request::PostAccountsAccountBankAccountsRequest {
            http_client: &self,
            account: account.to_owned(),
        }
    }
    ///<p>Retrieve a specified external account for a given account.</p>
    pub fn get_accounts_account_bank_accounts_id(
        &self,
        account: &str,
        id: &str,
    ) -> request::GetAccountsAccountBankAccountsIdRequest {
        request::GetAccountsAccountBankAccountsIdRequest {
            http_client: &self,
            account: account.to_owned(),
            expand: None,
            id: id.to_owned(),
        }
    }
    /**<p>Updates the metadata, account holder name, account holder type of a bank account belonging to a <a href="/docs/connect/custom-accounts">Custom account</a>, and optionally sets it as the default for its currency. Other bank account details are not editable by design.</p>

<p>You can re-enable a disabled bank account by performing an update call without providing any arguments or changes.</p>*/
    pub fn post_accounts_account_bank_accounts_id(
        &self,
        account: &str,
        id: &str,
    ) -> request::PostAccountsAccountBankAccountsIdRequest {
        request::PostAccountsAccountBankAccountsIdRequest {
            http_client: &self,
            account: account.to_owned(),
            id: id.to_owned(),
        }
    }
    ///<p>Delete a specified external account for a given account.</p>
    pub fn delete_accounts_account_bank_accounts_id(
        &self,
        account: &str,
        id: &str,
    ) -> request::DeleteAccountsAccountBankAccountsIdRequest {
        request::DeleteAccountsAccountBankAccountsIdRequest {
            http_client: &self,
            account: account.to_owned(),
            id: id.to_owned(),
        }
    }
    ///<p>Returns a list of capabilities associated with the account. The capabilities are returned sorted by creation date, with the most recent capability appearing first.</p>
    pub fn get_accounts_account_capabilities(
        &self,
        account: &str,
    ) -> request::GetAccountsAccountCapabilitiesRequest {
        request::GetAccountsAccountCapabilitiesRequest {
            http_client: &self,
            account: account.to_owned(),
            expand: None,
        }
    }
    ///<p>Retrieves information about the specified Account Capability.</p>
    pub fn get_accounts_account_capabilities_capability(
        &self,
        account: &str,
        capability: &str,
    ) -> request::GetAccountsAccountCapabilitiesCapabilityRequest {
        request::GetAccountsAccountCapabilitiesCapabilityRequest {
            http_client: &self,
            account: account.to_owned(),
            capability: capability.to_owned(),
            expand: None,
        }
    }
    ///<p>Updates an existing Account Capability. Request or remove a capability by updating its <code>requested</code> parameter.</p>
    pub fn post_accounts_account_capabilities_capability(
        &self,
        account: &str,
        capability: &str,
    ) -> request::PostAccountsAccountCapabilitiesCapabilityRequest {
        request::PostAccountsAccountCapabilitiesCapabilityRequest {
            http_client: &self,
            account: account.to_owned(),
            capability: capability.to_owned(),
        }
    }
    ///<p>List external accounts for an account.</p>
    pub fn get_accounts_account_external_accounts(
        &self,
        account: &str,
    ) -> request::GetAccountsAccountExternalAccountsRequest {
        request::GetAccountsAccountExternalAccountsRequest {
            http_client: &self,
            account: account.to_owned(),
            ending_before: None,
            expand: None,
            limit: None,
            object: None,
            starting_after: None,
        }
    }
    ///<p>Create an external account for a given account.</p>
    pub fn post_accounts_account_external_accounts(
        &self,
        account: &str,
    ) -> request::PostAccountsAccountExternalAccountsRequest {
        request::PostAccountsAccountExternalAccountsRequest {
            http_client: &self,
            account: account.to_owned(),
        }
    }
    ///<p>Retrieve a specified external account for a given account.</p>
    pub fn get_accounts_account_external_accounts_id(
        &self,
        account: &str,
        id: &str,
    ) -> request::GetAccountsAccountExternalAccountsIdRequest {
        request::GetAccountsAccountExternalAccountsIdRequest {
            http_client: &self,
            account: account.to_owned(),
            expand: None,
            id: id.to_owned(),
        }
    }
    /**<p>Updates the metadata, account holder name, account holder type of a bank account belonging to a <a href="/docs/connect/custom-accounts">Custom account</a>, and optionally sets it as the default for its currency. Other bank account details are not editable by design.</p>

<p>You can re-enable a disabled bank account by performing an update call without providing any arguments or changes.</p>*/
    pub fn post_accounts_account_external_accounts_id(
        &self,
        account: &str,
        id: &str,
    ) -> request::PostAccountsAccountExternalAccountsIdRequest {
        request::PostAccountsAccountExternalAccountsIdRequest {
            http_client: &self,
            account: account.to_owned(),
            id: id.to_owned(),
        }
    }
    ///<p>Delete a specified external account for a given account.</p>
    pub fn delete_accounts_account_external_accounts_id(
        &self,
        account: &str,
        id: &str,
    ) -> request::DeleteAccountsAccountExternalAccountsIdRequest {
        request::DeleteAccountsAccountExternalAccountsIdRequest {
            http_client: &self,
            account: account.to_owned(),
            id: id.to_owned(),
        }
    }
    /**<p>Creates a single-use login link for an Express account to access their Stripe dashboard.</p>

<p><strong>You may only create login links for <a href="/docs/connect/express-accounts">Express accounts</a> connected to your platform</strong>.</p>*/
    pub fn post_accounts_account_login_links(
        &self,
        account: &str,
    ) -> request::PostAccountsAccountLoginLinksRequest {
        request::PostAccountsAccountLoginLinksRequest {
            http_client: &self,
            account: account.to_owned(),
        }
    }
    ///<p>Returns a list of people associated with the account’s legal entity. The people are returned sorted by creation date, with the most recent people appearing first.</p>
    pub fn get_accounts_account_people(
        &self,
        account: &str,
    ) -> request::GetAccountsAccountPeopleRequest {
        request::GetAccountsAccountPeopleRequest {
            http_client: &self,
            account: account.to_owned(),
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
        account: &str,
    ) -> request::PostAccountsAccountPeopleRequest {
        request::PostAccountsAccountPeopleRequest {
            http_client: &self,
            account: account.to_owned(),
        }
    }
    ///<p>Retrieves an existing person.</p>
    pub fn get_accounts_account_people_person(
        &self,
        account: &str,
        person: &str,
    ) -> request::GetAccountsAccountPeoplePersonRequest {
        request::GetAccountsAccountPeoplePersonRequest {
            http_client: &self,
            account: account.to_owned(),
            expand: None,
            person: person.to_owned(),
        }
    }
    ///<p>Updates an existing person.</p>
    pub fn post_accounts_account_people_person(
        &self,
        account: &str,
        person: &str,
    ) -> request::PostAccountsAccountPeoplePersonRequest {
        request::PostAccountsAccountPeoplePersonRequest {
            http_client: &self,
            account: account.to_owned(),
            person: person.to_owned(),
        }
    }
    ///<p>Deletes an existing person’s relationship to the account’s legal entity. Any person with a relationship for an account can be deleted through the API, except if the person is the <code>account_opener</code>. If your integration is using the <code>executive</code> parameter, you cannot delete the only verified <code>executive</code> on file.</p>
    pub fn delete_accounts_account_people_person(
        &self,
        account: &str,
        person: &str,
    ) -> request::DeleteAccountsAccountPeoplePersonRequest {
        request::DeleteAccountsAccountPeoplePersonRequest {
            http_client: &self,
            account: account.to_owned(),
            person: person.to_owned(),
        }
    }
    ///<p>Returns a list of people associated with the account’s legal entity. The people are returned sorted by creation date, with the most recent people appearing first.</p>
    pub fn get_accounts_account_persons(
        &self,
        account: &str,
    ) -> request::GetAccountsAccountPersonsRequest {
        request::GetAccountsAccountPersonsRequest {
            http_client: &self,
            account: account.to_owned(),
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
        account: &str,
    ) -> request::PostAccountsAccountPersonsRequest {
        request::PostAccountsAccountPersonsRequest {
            http_client: &self,
            account: account.to_owned(),
        }
    }
    ///<p>Retrieves an existing person.</p>
    pub fn get_accounts_account_persons_person(
        &self,
        account: &str,
        person: &str,
    ) -> request::GetAccountsAccountPersonsPersonRequest {
        request::GetAccountsAccountPersonsPersonRequest {
            http_client: &self,
            account: account.to_owned(),
            expand: None,
            person: person.to_owned(),
        }
    }
    ///<p>Updates an existing person.</p>
    pub fn post_accounts_account_persons_person(
        &self,
        account: &str,
        person: &str,
    ) -> request::PostAccountsAccountPersonsPersonRequest {
        request::PostAccountsAccountPersonsPersonRequest {
            http_client: &self,
            account: account.to_owned(),
            person: person.to_owned(),
        }
    }
    ///<p>Deletes an existing person’s relationship to the account’s legal entity. Any person with a relationship for an account can be deleted through the API, except if the person is the <code>account_opener</code>. If your integration is using the <code>executive</code> parameter, you cannot delete the only verified <code>executive</code> on file.</p>
    pub fn delete_accounts_account_persons_person(
        &self,
        account: &str,
        person: &str,
    ) -> request::DeleteAccountsAccountPersonsPersonRequest {
        request::DeleteAccountsAccountPersonsPersonRequest {
            http_client: &self,
            account: account.to_owned(),
            person: person.to_owned(),
        }
    }
    /**<p>With <a href="/docs/connect">Connect</a>, you may flag accounts as suspicious.</p>

<p>Test-mode Custom and Express accounts can be rejected at any time. Accounts created using live-mode keys may only be rejected once all balances are zero.</p>*/
    pub fn post_accounts_account_reject(
        &self,
        account: &str,
    ) -> request::PostAccountsAccountRejectRequest {
        request::PostAccountsAccountRejectRequest {
            http_client: &self,
            account: account.to_owned(),
        }
    }
    ///<p>List apple pay domains.</p>
    pub fn get_apple_pay_domains(&self) -> request::GetApplePayDomainsRequest {
        request::GetApplePayDomainsRequest {
            http_client: &self,
            domain_name: None,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
        }
    }
    ///<p>Create an apple pay domain.</p>
    pub fn post_apple_pay_domains(&self) -> request::PostApplePayDomainsRequest {
        request::PostApplePayDomainsRequest {
            http_client: &self,
        }
    }
    ///<p>Retrieve an apple pay domain.</p>
    pub fn get_apple_pay_domains_domain(
        &self,
        domain: &str,
    ) -> request::GetApplePayDomainsDomainRequest {
        request::GetApplePayDomainsDomainRequest {
            http_client: &self,
            domain: domain.to_owned(),
            expand: None,
        }
    }
    ///<p>Delete an apple pay domain.</p>
    pub fn delete_apple_pay_domains_domain(
        &self,
        domain: &str,
    ) -> request::DeleteApplePayDomainsDomainRequest {
        request::DeleteApplePayDomainsDomainRequest {
            http_client: &self,
            domain: domain.to_owned(),
        }
    }
    ///<p>Returns a list of application fees you’ve previously collected. The application fees are returned in sorted order, with the most recent fees appearing first.</p>
    pub fn get_application_fees(&self) -> request::GetApplicationFeesRequest {
        request::GetApplicationFeesRequest {
            http_client: &self,
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
        fee: &str,
        id: &str,
    ) -> request::GetApplicationFeesFeeRefundsIdRequest {
        request::GetApplicationFeesFeeRefundsIdRequest {
            http_client: &self,
            expand: None,
            fee: fee.to_owned(),
            id: id.to_owned(),
        }
    }
    /**<p>Updates the specified application fee refund by setting the values of the parameters passed. Any parameters not provided will be left unchanged.</p>

<p>This request only accepts metadata as an argument.</p>*/
    pub fn post_application_fees_fee_refunds_id(
        &self,
        fee: &str,
        id: &str,
    ) -> request::PostApplicationFeesFeeRefundsIdRequest {
        request::PostApplicationFeesFeeRefundsIdRequest {
            http_client: &self,
            fee: fee.to_owned(),
            id: id.to_owned(),
        }
    }
    ///<p>Retrieves the details of an application fee that your account has collected. The same information is returned when refunding the application fee.</p>
    pub fn get_application_fees_id(
        &self,
        id: &str,
    ) -> request::GetApplicationFeesIdRequest {
        request::GetApplicationFeesIdRequest {
            http_client: &self,
            expand: None,
            id: id.to_owned(),
        }
    }
    pub fn post_application_fees_id_refund(
        &self,
        id: &str,
    ) -> request::PostApplicationFeesIdRefundRequest {
        request::PostApplicationFeesIdRefundRequest {
            http_client: &self,
            id: id.to_owned(),
        }
    }
    ///<p>You can see a list of the refunds belonging to a specific application fee. Note that the 10 most recent refunds are always available by default on the application fee object. If you need more than those 10, you can use this API method and the <code>limit</code> and <code>starting_after</code> parameters to page through additional refunds.</p>
    pub fn get_application_fees_id_refunds(
        &self,
        id: &str,
    ) -> request::GetApplicationFeesIdRefundsRequest {
        request::GetApplicationFeesIdRefundsRequest {
            http_client: &self,
            ending_before: None,
            expand: None,
            id: id.to_owned(),
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
        id: &str,
    ) -> request::PostApplicationFeesIdRefundsRequest {
        request::PostApplicationFeesIdRefundsRequest {
            http_client: &self,
            id: id.to_owned(),
        }
    }
    ///<p>List all secrets stored on the given scope.</p>
    pub fn get_apps_secrets(&self, scope: ScopeParam) -> request::GetAppsSecretsRequest {
        request::GetAppsSecretsRequest {
            http_client: &self,
            ending_before: None,
            expand: None,
            limit: None,
            scope,
            starting_after: None,
        }
    }
    ///<p>Create or replace a secret in the secret store.</p>
    pub fn post_apps_secrets(&self) -> request::PostAppsSecretsRequest {
        request::PostAppsSecretsRequest {
            http_client: &self,
        }
    }
    ///<p>Deletes a secret from the secret store by name and scope.</p>
    pub fn post_apps_secrets_delete(&self) -> request::PostAppsSecretsDeleteRequest {
        request::PostAppsSecretsDeleteRequest {
            http_client: &self,
        }
    }
    ///<p>Finds a secret in the secret store by name and scope.</p>
    pub fn get_apps_secrets_find(
        &self,
        name: &str,
        scope: ScopeParam,
    ) -> request::GetAppsSecretsFindRequest {
        request::GetAppsSecretsFindRequest {
            http_client: &self,
            expand: None,
            name: name.to_owned(),
            scope,
        }
    }
    /**<p>Retrieves the current account balance, based on the authentication that was used to make the request.
 For a sample request, see <a href="/docs/connect/account-balances#accounting-for-negative-balances">Accounting for negative balances</a>.</p>*/
    pub fn get_balance(&self) -> request::GetBalanceRequest {
        request::GetBalanceRequest {
            http_client: &self,
            expand: None,
        }
    }
    /**<p>Returns a list of transactions that have contributed to the Stripe account balance (e.g., charges, transfers, and so forth). The transactions are returned in sorted order, with the most recent transactions appearing first.</p>

<p>Note that this endpoint was previously called “Balance history” and used the path <code>/v1/balance/history</code>.</p>*/
    pub fn get_balance_history(&self) -> request::GetBalanceHistoryRequest {
        request::GetBalanceHistoryRequest {
            http_client: &self,
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
        id: &str,
    ) -> request::GetBalanceHistoryIdRequest {
        request::GetBalanceHistoryIdRequest {
            http_client: &self,
            expand: None,
            id: id.to_owned(),
        }
    }
    /**<p>Returns a list of transactions that have contributed to the Stripe account balance (e.g., charges, transfers, and so forth). The transactions are returned in sorted order, with the most recent transactions appearing first.</p>

<p>Note that this endpoint was previously called “Balance history” and used the path <code>/v1/balance/history</code>.</p>*/
    pub fn get_balance_transactions(&self) -> request::GetBalanceTransactionsRequest {
        request::GetBalanceTransactionsRequest {
            http_client: &self,
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
        id: &str,
    ) -> request::GetBalanceTransactionsIdRequest {
        request::GetBalanceTransactionsIdRequest {
            http_client: &self,
            expand: None,
            id: id.to_owned(),
        }
    }
    ///<p>Returns a list of configurations that describe the functionality of the customer portal.</p>
    pub fn get_billing_portal_configurations(
        &self,
    ) -> request::GetBillingPortalConfigurationsRequest {
        request::GetBillingPortalConfigurationsRequest {
            http_client: &self,
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
    ) -> request::PostBillingPortalConfigurationsRequest {
        request::PostBillingPortalConfigurationsRequest {
            http_client: &self,
        }
    }
    ///<p>Retrieves a configuration that describes the functionality of the customer portal.</p>
    pub fn get_billing_portal_configurations_configuration(
        &self,
        configuration: &str,
    ) -> request::GetBillingPortalConfigurationsConfigurationRequest {
        request::GetBillingPortalConfigurationsConfigurationRequest {
            http_client: &self,
            configuration: configuration.to_owned(),
            expand: None,
        }
    }
    ///<p>Updates a configuration that describes the functionality of the customer portal.</p>
    pub fn post_billing_portal_configurations_configuration(
        &self,
        configuration: &str,
    ) -> request::PostBillingPortalConfigurationsConfigurationRequest {
        request::PostBillingPortalConfigurationsConfigurationRequest {
            http_client: &self,
            configuration: configuration.to_owned(),
        }
    }
    ///<p>Creates a session of the customer portal.</p>
    pub fn post_billing_portal_sessions(
        &self,
    ) -> request::PostBillingPortalSessionsRequest {
        request::PostBillingPortalSessionsRequest {
            http_client: &self,
        }
    }
    ///<p>Returns a list of charges you’ve previously created. The charges are returned in sorted order, with the most recent charges appearing first.</p>
    pub fn get_charges(&self) -> request::GetChargesRequest {
        request::GetChargesRequest {
            http_client: &self,
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
    /**<p>Use the <a href="/docs/api/payment_intents">Payment Intents API</a> to initiate a new payment instead
of using this method. Confirmation of the PaymentIntent creates the <code>Charge</code>
object used to request payment, so this method is limited to legacy integrations.</p>*/
    pub fn post_charges(&self) -> request::PostChargesRequest {
        request::PostChargesRequest {
            http_client: &self,
        }
    }
    /**<p>Search for charges you’ve previously created using Stripe’s <a href="/docs/search#search-query-language">Search Query Language</a>.
Don’t use search in read-after-write flows where strict consistency is necessary. Under normal operating
conditions, data is searchable in less than a minute. Occasionally, propagation of new or updated data can be up
to an hour behind during outages. Search functionality is not available to merchants in India.</p>*/
    pub fn get_charges_search(&self, query: &str) -> request::GetChargesSearchRequest {
        request::GetChargesSearchRequest {
            http_client: &self,
            expand: None,
            limit: None,
            page: None,
            query: query.to_owned(),
        }
    }
    ///<p>Retrieves the details of a charge that has previously been created. Supply the unique charge ID that was returned from your previous request, and Stripe will return the corresponding charge information. The same information is returned when creating or refunding the charge.</p>
    pub fn get_charges_charge(&self, charge: &str) -> request::GetChargesChargeRequest {
        request::GetChargesChargeRequest {
            http_client: &self,
            charge: charge.to_owned(),
            expand: None,
        }
    }
    ///<p>Updates the specified charge by setting the values of the parameters passed. Any parameters not provided will be left unchanged.</p>
    pub fn post_charges_charge(
        &self,
        charge: &str,
    ) -> request::PostChargesChargeRequest {
        request::PostChargesChargeRequest {
            http_client: &self,
            charge: charge.to_owned(),
        }
    }
    /**<p>Capture the payment of an existing, uncaptured charge that was created with the <code>capture</code> option set to false.</p>

<p>Uncaptured payments expire a set number of days after they are created (<a href="/docs/charges/placing-a-hold">7 by default</a>), after which they are marked as refunded and capture attempts will fail.</p>

<p>Don’t use this method to capture a PaymentIntent-initiated charge. Use <a href="/docs/api/payment_intents/capture">Capture a PaymentIntent</a>.</p>*/
    pub fn post_charges_charge_capture(
        &self,
        charge: &str,
    ) -> request::PostChargesChargeCaptureRequest {
        request::PostChargesChargeCaptureRequest {
            http_client: &self,
            charge: charge.to_owned(),
        }
    }
    ///<p>Retrieve a dispute for a specified charge.</p>
    pub fn get_charges_charge_dispute(
        &self,
        charge: &str,
    ) -> request::GetChargesChargeDisputeRequest {
        request::GetChargesChargeDisputeRequest {
            http_client: &self,
            charge: charge.to_owned(),
            expand: None,
        }
    }
    pub fn post_charges_charge_dispute(
        &self,
        charge: &str,
    ) -> request::PostChargesChargeDisputeRequest {
        request::PostChargesChargeDisputeRequest {
            http_client: &self,
            charge: charge.to_owned(),
        }
    }
    pub fn post_charges_charge_dispute_close(
        &self,
        charge: &str,
    ) -> request::PostChargesChargeDisputeCloseRequest {
        request::PostChargesChargeDisputeCloseRequest {
            http_client: &self,
            charge: charge.to_owned(),
        }
    }
    /**<p>When you create a new refund, you must specify either a Charge or a PaymentIntent object.</p>

<p>This action refunds a previously created charge that’s not refunded yet.
Funds are refunded to the credit or debit card that’s originally charged.</p>

<p>You can optionally refund only part of a charge.
You can repeat this until the entire charge is refunded.</p>

<p>After you entirely refund a charge, you can’t refund it again.
This method raises an error when it’s called on an already-refunded charge,
or when you attempt to refund more money than is left on a charge.</p>*/
    pub fn post_charges_charge_refund(
        &self,
        charge: &str,
    ) -> request::PostChargesChargeRefundRequest {
        request::PostChargesChargeRefundRequest {
            http_client: &self,
            charge: charge.to_owned(),
        }
    }
    ///<p>You can see a list of the refunds belonging to a specific charge. Note that the 10 most recent refunds are always available by default on the charge object. If you need more than those 10, you can use this API method and the <code>limit</code> and <code>starting_after</code> parameters to page through additional refunds.</p>
    pub fn get_charges_charge_refunds(
        &self,
        charge: &str,
    ) -> request::GetChargesChargeRefundsRequest {
        request::GetChargesChargeRefundsRequest {
            http_client: &self,
            charge: charge.to_owned(),
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
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
    pub fn post_charges_charge_refunds(
        &self,
        charge: &str,
    ) -> request::PostChargesChargeRefundsRequest {
        request::PostChargesChargeRefundsRequest {
            http_client: &self,
            charge: charge.to_owned(),
        }
    }
    ///<p>Retrieves the details of an existing refund.</p>
    pub fn get_charges_charge_refunds_refund(
        &self,
        charge: &str,
        refund: &str,
    ) -> request::GetChargesChargeRefundsRefundRequest {
        request::GetChargesChargeRefundsRefundRequest {
            http_client: &self,
            charge: charge.to_owned(),
            expand: None,
            refund: refund.to_owned(),
        }
    }
    ///<p>Update a specified refund.</p>
    pub fn post_charges_charge_refunds_refund(
        &self,
        charge: &str,
        refund: &str,
    ) -> request::PostChargesChargeRefundsRefundRequest {
        request::PostChargesChargeRefundsRefundRequest {
            http_client: &self,
            charge: charge.to_owned(),
            refund: refund.to_owned(),
        }
    }
    ///<p>Returns a list of Checkout Sessions.</p>
    pub fn get_checkout_sessions(&self) -> request::GetCheckoutSessionsRequest {
        request::GetCheckoutSessionsRequest {
            http_client: &self,
            customer: None,
            customer_details: None,
            ending_before: None,
            expand: None,
            limit: None,
            payment_intent: None,
            payment_link: None,
            starting_after: None,
            status: None,
            subscription: None,
        }
    }
    ///<p>Creates a Session object.</p>
    pub fn post_checkout_sessions(&self) -> request::PostCheckoutSessionsRequest {
        request::PostCheckoutSessionsRequest {
            http_client: &self,
        }
    }
    ///<p>Retrieves a Session object.</p>
    pub fn get_checkout_sessions_session(
        &self,
        session: &str,
    ) -> request::GetCheckoutSessionsSessionRequest {
        request::GetCheckoutSessionsSessionRequest {
            http_client: &self,
            expand: None,
            session: session.to_owned(),
        }
    }
    /**<p>A Session can be expired when it is in one of these statuses: <code>open</code> </p>

<p>After it expires, a customer can’t complete a Session and customers loading the Session see a message saying the Session is expired.</p>*/
    pub fn post_checkout_sessions_session_expire(
        &self,
        session: &str,
    ) -> request::PostCheckoutSessionsSessionExpireRequest {
        request::PostCheckoutSessionsSessionExpireRequest {
            http_client: &self,
            session: session.to_owned(),
        }
    }
    ///<p>When retrieving a Checkout Session, there is an includable <strong>line_items</strong> property containing the first handful of those items. There is also a URL where you can retrieve the full (paginated) list of line items.</p>
    pub fn get_checkout_sessions_session_line_items(
        &self,
        session: &str,
    ) -> request::GetCheckoutSessionsSessionLineItemsRequest {
        request::GetCheckoutSessionsSessionLineItemsRequest {
            http_client: &self,
            ending_before: None,
            expand: None,
            limit: None,
            session: session.to_owned(),
            starting_after: None,
        }
    }
    /**<p>Lists all Climate order objects. The orders are returned sorted by creation date, with the
most recently created orders appearing first.</p>*/
    pub fn get_climate_orders(&self) -> request::GetClimateOrdersRequest {
        request::GetClimateOrdersRequest {
            http_client: &self,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
        }
    }
    /**<p>Creates a Climate order object for a given Climate product. The order will be processed immediately
after creation and payment will be deducted your Stripe balance.</p>*/
    pub fn post_climate_orders(&self) -> request::PostClimateOrdersRequest {
        request::PostClimateOrdersRequest {
            http_client: &self,
        }
    }
    ///<p>Retrieves the details of a Climate order object with the given ID.</p>
    pub fn get_climate_orders_order(
        &self,
        order: &str,
    ) -> request::GetClimateOrdersOrderRequest {
        request::GetClimateOrdersOrderRequest {
            http_client: &self,
            expand: None,
            order: order.to_owned(),
        }
    }
    ///<p>Updates the specified order by setting the values of the parameters passed.</p>
    pub fn post_climate_orders_order(
        &self,
        order: &str,
    ) -> request::PostClimateOrdersOrderRequest {
        request::PostClimateOrdersOrderRequest {
            http_client: &self,
            order: order.to_owned(),
        }
    }
    /**<p>Cancels a Climate order. You can cancel an order within 30 days of creation. Stripe refunds the
reservation <code>amount_subtotal</code>, but not the <code>amount_fees</code> for user-triggered cancellations. Frontier
might cancel reservations if suppliers fail to deliver. If Frontier cancels the reservation, Stripe
provides 90 days advance notice and refunds the <code>amount_total</code>.</p>*/
    pub fn post_climate_orders_order_cancel(
        &self,
        order: &str,
    ) -> request::PostClimateOrdersOrderCancelRequest {
        request::PostClimateOrdersOrderCancelRequest {
            http_client: &self,
            order: order.to_owned(),
        }
    }
    ///<p>Lists all available Climate product objects.</p>
    pub fn get_climate_products(&self) -> request::GetClimateProductsRequest {
        request::GetClimateProductsRequest {
            http_client: &self,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
        }
    }
    ///<p>Retrieves the details of a Climate product with the given ID.</p>
    pub fn get_climate_products_product(
        &self,
        product: &str,
    ) -> request::GetClimateProductsProductRequest {
        request::GetClimateProductsProductRequest {
            http_client: &self,
            expand: None,
            product: product.to_owned(),
        }
    }
    /**<p>Lists all Climate order objects. The orders are returned sorted by creation date, with the
most recently created orders appearing first.</p>*/
    pub fn get_climate_reservations(&self) -> request::GetClimateReservationsRequest {
        request::GetClimateReservationsRequest {
            http_client: &self,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
        }
    }
    /**<p>Creates a Climate order object for a given Climate product. The order will be processed immediately
after creation and payment will be deducted your Stripe balance.</p>*/
    pub fn post_climate_reservations(&self) -> request::PostClimateReservationsRequest {
        request::PostClimateReservationsRequest {
            http_client: &self,
        }
    }
    ///<p>Retrieves the details of a Climate order object with the given ID.</p>
    pub fn get_climate_reservations_order(
        &self,
        order: &str,
    ) -> request::GetClimateReservationsOrderRequest {
        request::GetClimateReservationsOrderRequest {
            http_client: &self,
            expand: None,
            order: order.to_owned(),
        }
    }
    ///<p>Updates the specified order by setting the values of the parameters passed.</p>
    pub fn post_climate_reservations_order(
        &self,
        order: &str,
    ) -> request::PostClimateReservationsOrderRequest {
        request::PostClimateReservationsOrderRequest {
            http_client: &self,
            order: order.to_owned(),
        }
    }
    /**<p>Cancels a Climate order. You can cancel an order within 30 days of creation. Stripe refunds the
reservation <code>amount_subtotal</code>, but not the <code>amount_fees</code> for user-triggered cancellations. Frontier
might cancel reservations if suppliers fail to deliver. If Frontier cancels the reservation, Stripe
provides 90 days advance notice and refunds the <code>amount_total</code>.</p>*/
    pub fn post_climate_reservations_order_cancel(
        &self,
        order: &str,
    ) -> request::PostClimateReservationsOrderCancelRequest {
        request::PostClimateReservationsOrderCancelRequest {
            http_client: &self,
            order: order.to_owned(),
        }
    }
    /**<p>Confirms a Climate order. When you confirm your order, we immediately deduct the funds from your
Stripe balance.</p>*/
    pub fn post_climate_reservations_order_confirm(
        &self,
        order: &str,
    ) -> request::PostClimateReservationsOrderConfirmRequest {
        request::PostClimateReservationsOrderConfirmRequest {
            http_client: &self,
            order: order.to_owned(),
        }
    }
    ///<p>Lists all available Climate supplier objects.</p>
    pub fn get_climate_suppliers(&self) -> request::GetClimateSuppliersRequest {
        request::GetClimateSuppliersRequest {
            http_client: &self,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
        }
    }
    ///<p>Retrieves a Climate supplier object.</p>
    pub fn get_climate_suppliers_supplier(
        &self,
        supplier: &str,
    ) -> request::GetClimateSuppliersSupplierRequest {
        request::GetClimateSuppliersSupplierRequest {
            http_client: &self,
            expand: None,
            supplier: supplier.to_owned(),
        }
    }
    ///<p>Lists all Country Spec objects available in the API.</p>
    pub fn get_country_specs(&self) -> request::GetCountrySpecsRequest {
        request::GetCountrySpecsRequest {
            http_client: &self,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
        }
    }
    ///<p>Returns a Country Spec for a given Country code.</p>
    pub fn get_country_specs_country(
        &self,
        country: &str,
    ) -> request::GetCountrySpecsCountryRequest {
        request::GetCountrySpecsCountryRequest {
            http_client: &self,
            country: country.to_owned(),
            expand: None,
        }
    }
    ///<p>Returns a list of your coupons.</p>
    pub fn get_coupons(&self) -> request::GetCouponsRequest {
        request::GetCouponsRequest {
            http_client: &self,
            created: None,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
        }
    }
    /**<p>You can create coupons easily via the <a href="https://dashboard.stripe.com/coupons">coupon management</a> page of the Stripe dashboard. Coupon creation is also accessible via the API if you need to create coupons on the fly.</p>

<p>A coupon has either a <code>percent_off</code> or an <code>amount_off</code> and <code>currency</code>. If you set an <code>amount_off</code>, that amount will be subtracted from any invoice’s subtotal. For example, an invoice with a subtotal of <currency>100</currency> will have a final total of <currency>0</currency> if a coupon with an <code>amount_off</code> of <amount>200</amount> is applied to it and an invoice with a subtotal of <currency>300</currency> will have a final total of <currency>100</currency> if a coupon with an <code>amount_off</code> of <amount>200</amount> is applied to it.</p>*/
    pub fn post_coupons(&self) -> request::PostCouponsRequest {
        request::PostCouponsRequest {
            http_client: &self,
        }
    }
    ///<p>Retrieves the coupon with the given ID.</p>
    pub fn get_coupons_coupon(&self, coupon: &str) -> request::GetCouponsCouponRequest {
        request::GetCouponsCouponRequest {
            http_client: &self,
            coupon: coupon.to_owned(),
            expand: None,
        }
    }
    ///<p>Updates the metadata of a coupon. Other coupon details (currency, duration, amount_off) are, by design, not editable.</p>
    pub fn post_coupons_coupon(
        &self,
        coupon: &str,
    ) -> request::PostCouponsCouponRequest {
        request::PostCouponsCouponRequest {
            http_client: &self,
            coupon: coupon.to_owned(),
        }
    }
    ///<p>You can delete coupons via the <a href="https://dashboard.stripe.com/coupons">coupon management</a> page of the Stripe dashboard. However, deleting a coupon does not affect any customers who have already applied the coupon; it means that new customers can’t redeem the coupon. You can also delete coupons via the API.</p>
    pub fn delete_coupons_coupon(
        &self,
        coupon: &str,
    ) -> request::DeleteCouponsCouponRequest {
        request::DeleteCouponsCouponRequest {
            http_client: &self,
            coupon: coupon.to_owned(),
        }
    }
    ///<p>Returns a list of credit notes.</p>
    pub fn get_credit_notes(&self) -> request::GetCreditNotesRequest {
        request::GetCreditNotesRequest {
            http_client: &self,
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
    pub fn post_credit_notes(&self) -> request::PostCreditNotesRequest {
        request::PostCreditNotesRequest {
            http_client: &self,
        }
    }
    ///<p>Get a preview of a credit note without creating it.</p>
    pub fn get_credit_notes_preview(
        &self,
        invoice: &str,
    ) -> request::GetCreditNotesPreviewRequest {
        request::GetCreditNotesPreviewRequest {
            http_client: &self,
            amount: None,
            credit_amount: None,
            effective_at: None,
            expand: None,
            invoice: invoice.to_owned(),
            lines: None,
            memo: None,
            metadata: None,
            out_of_band_amount: None,
            reason: None,
            refund: None,
            refund_amount: None,
            shipping_cost: None,
        }
    }
    ///<p>When retrieving a credit note preview, you’ll get a <strong>lines</strong> property containing the first handful of those items. This URL you can retrieve the full (paginated) list of line items.</p>
    pub fn get_credit_notes_preview_lines(
        &self,
        invoice: &str,
    ) -> request::GetCreditNotesPreviewLinesRequest {
        request::GetCreditNotesPreviewLinesRequest {
            http_client: &self,
            amount: None,
            credit_amount: None,
            effective_at: None,
            ending_before: None,
            expand: None,
            invoice: invoice.to_owned(),
            limit: None,
            lines: None,
            memo: None,
            metadata: None,
            out_of_band_amount: None,
            reason: None,
            refund: None,
            refund_amount: None,
            shipping_cost: None,
            starting_after: None,
        }
    }
    ///<p>When retrieving a credit note, you’ll get a <strong>lines</strong> property containing the the first handful of those items. There is also a URL where you can retrieve the full (paginated) list of line items.</p>
    pub fn get_credit_notes_credit_note_lines(
        &self,
        credit_note: &str,
    ) -> request::GetCreditNotesCreditNoteLinesRequest {
        request::GetCreditNotesCreditNoteLinesRequest {
            http_client: &self,
            credit_note: credit_note.to_owned(),
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
        }
    }
    ///<p>Retrieves the credit note object with the given identifier.</p>
    pub fn get_credit_notes_id(&self, id: &str) -> request::GetCreditNotesIdRequest {
        request::GetCreditNotesIdRequest {
            http_client: &self,
            expand: None,
            id: id.to_owned(),
        }
    }
    ///<p>Updates an existing credit note.</p>
    pub fn post_credit_notes_id(&self, id: &str) -> request::PostCreditNotesIdRequest {
        request::PostCreditNotesIdRequest {
            http_client: &self,
            id: id.to_owned(),
        }
    }
    ///<p>Marks a credit note as void. Learn more about <a href="/docs/billing/invoices/credit-notes#voiding">voiding credit notes</a>.</p>
    pub fn post_credit_notes_id_void(
        &self,
        id: &str,
    ) -> request::PostCreditNotesIdVoidRequest {
        request::PostCreditNotesIdVoidRequest {
            http_client: &self,
            id: id.to_owned(),
        }
    }
    ///<p>Returns a list of your customers. The customers are returned sorted by creation date, with the most recent customers appearing first.</p>
    pub fn get_customers(&self) -> request::GetCustomersRequest {
        request::GetCustomersRequest {
            http_client: &self,
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
    pub fn post_customers(&self) -> request::PostCustomersRequest {
        request::PostCustomersRequest {
            http_client: &self,
        }
    }
    /**<p>Search for customers you’ve previously created using Stripe’s <a href="/docs/search#search-query-language">Search Query Language</a>.
Don’t use search in read-after-write flows where strict consistency is necessary. Under normal operating
conditions, data is searchable in less than a minute. Occasionally, propagation of new or updated data can be up
to an hour behind during outages. Search functionality is not available to merchants in India.</p>*/
    pub fn get_customers_search(
        &self,
        query: &str,
    ) -> request::GetCustomersSearchRequest {
        request::GetCustomersSearchRequest {
            http_client: &self,
            expand: None,
            limit: None,
            page: None,
            query: query.to_owned(),
        }
    }
    ///<p>Retrieves a Customer object.</p>
    pub fn get_customers_customer(
        &self,
        customer: &str,
    ) -> request::GetCustomersCustomerRequest {
        request::GetCustomersCustomerRequest {
            http_client: &self,
            customer: customer.to_owned(),
            expand: None,
        }
    }
    /**<p>Updates the specified customer by setting the values of the parameters passed. Any parameters not provided will be left unchanged. For example, if you pass the <strong>source</strong> parameter, that becomes the customer’s active source (e.g., a card) to be used for all charges in the future. When you update a customer to a new valid card source by passing the <strong>source</strong> parameter: for each of the customer’s current subscriptions, if the subscription bills automatically and is in the <code>past_due</code> state, then the latest open invoice for the subscription with automatic collection enabled will be retried. This retry will not count as an automatic retry, and will not affect the next regularly scheduled payment for the invoice. Changing the <strong>default_source</strong> for a customer will not trigger this behavior.</p>

<p>This request accepts mostly the same arguments as the customer creation call.</p>*/
    pub fn post_customers_customer(
        &self,
        customer: &str,
    ) -> request::PostCustomersCustomerRequest {
        request::PostCustomersCustomerRequest {
            http_client: &self,
            customer: customer.to_owned(),
        }
    }
    ///<p>Permanently deletes a customer. It cannot be undone. Also immediately cancels any active subscriptions on the customer.</p>
    pub fn delete_customers_customer(
        &self,
        customer: &str,
    ) -> request::DeleteCustomersCustomerRequest {
        request::DeleteCustomersCustomerRequest {
            http_client: &self,
            customer: customer.to_owned(),
        }
    }
    ///<p>Returns a list of transactions that updated the customer’s <a href="/docs/billing/customer/balance">balances</a>.</p>
    pub fn get_customers_customer_balance_transactions(
        &self,
        customer: &str,
    ) -> request::GetCustomersCustomerBalanceTransactionsRequest {
        request::GetCustomersCustomerBalanceTransactionsRequest {
            http_client: &self,
            customer: customer.to_owned(),
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
        }
    }
    ///<p>Creates an immutable transaction that updates the customer’s credit <a href="/docs/billing/customer/balance">balance</a>.</p>
    pub fn post_customers_customer_balance_transactions(
        &self,
        customer: &str,
    ) -> request::PostCustomersCustomerBalanceTransactionsRequest {
        request::PostCustomersCustomerBalanceTransactionsRequest {
            http_client: &self,
            customer: customer.to_owned(),
        }
    }
    ///<p>Retrieves a specific customer balance transaction that updated the customer’s <a href="/docs/billing/customer/balance">balances</a>.</p>
    pub fn get_customers_customer_balance_transactions_transaction(
        &self,
        customer: &str,
        transaction: &str,
    ) -> request::GetCustomersCustomerBalanceTransactionsTransactionRequest {
        request::GetCustomersCustomerBalanceTransactionsTransactionRequest {
            http_client: &self,
            customer: customer.to_owned(),
            expand: None,
            transaction: transaction.to_owned(),
        }
    }
    ///<p>Most credit balance transaction fields are immutable, but you may update its <code>description</code> and <code>metadata</code>.</p>
    pub fn post_customers_customer_balance_transactions_transaction(
        &self,
        customer: &str,
        transaction: &str,
    ) -> request::PostCustomersCustomerBalanceTransactionsTransactionRequest {
        request::PostCustomersCustomerBalanceTransactionsTransactionRequest {
            http_client: &self,
            customer: customer.to_owned(),
            transaction: transaction.to_owned(),
        }
    }
    ///<p>You can see a list of the bank accounts belonging to a Customer. Note that the 10 most recent sources are always available by default on the Customer. If you need more than those 10, you can use this API method and the <code>limit</code> and <code>starting_after</code> parameters to page through additional bank accounts.</p>
    pub fn get_customers_customer_bank_accounts(
        &self,
        customer: &str,
    ) -> request::GetCustomersCustomerBankAccountsRequest {
        request::GetCustomersCustomerBankAccountsRequest {
            http_client: &self,
            customer: customer.to_owned(),
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
        customer: &str,
    ) -> request::PostCustomersCustomerBankAccountsRequest {
        request::PostCustomersCustomerBankAccountsRequest {
            http_client: &self,
            customer: customer.to_owned(),
        }
    }
    ///<p>By default, you can see the 10 most recent sources stored on a Customer directly on the object, but you can also retrieve details about a specific bank account stored on the Stripe account.</p>
    pub fn get_customers_customer_bank_accounts_id(
        &self,
        customer: &str,
        id: &str,
    ) -> request::GetCustomersCustomerBankAccountsIdRequest {
        request::GetCustomersCustomerBankAccountsIdRequest {
            http_client: &self,
            customer: customer.to_owned(),
            expand: None,
            id: id.to_owned(),
        }
    }
    ///<p>Update a specified source for a given customer.</p>
    pub fn post_customers_customer_bank_accounts_id(
        &self,
        customer: &str,
        id: &str,
    ) -> request::PostCustomersCustomerBankAccountsIdRequest {
        request::PostCustomersCustomerBankAccountsIdRequest {
            http_client: &self,
            customer: customer.to_owned(),
            id: id.to_owned(),
        }
    }
    ///<p>Delete a specified source for a given customer.</p>
    pub fn delete_customers_customer_bank_accounts_id(
        &self,
        customer: &str,
        id: &str,
    ) -> request::DeleteCustomersCustomerBankAccountsIdRequest {
        request::DeleteCustomersCustomerBankAccountsIdRequest {
            http_client: &self,
            customer: customer.to_owned(),
            id: id.to_owned(),
        }
    }
    ///<p>Verify a specified bank account for a given customer.</p>
    pub fn post_customers_customer_bank_accounts_id_verify(
        &self,
        customer: &str,
        id: &str,
    ) -> request::PostCustomersCustomerBankAccountsIdVerifyRequest {
        request::PostCustomersCustomerBankAccountsIdVerifyRequest {
            http_client: &self,
            customer: customer.to_owned(),
            id: id.to_owned(),
        }
    }
    /**<p>You can see a list of the cards belonging to a customer.
Note that the 10 most recent sources are always available on the <code>Customer</code> object.
If you need more than those 10, you can use this API method and the <code>limit</code> and <code>starting_after</code> parameters to page through additional cards.</p>*/
    pub fn get_customers_customer_cards(
        &self,
        customer: &str,
    ) -> request::GetCustomersCustomerCardsRequest {
        request::GetCustomersCustomerCardsRequest {
            http_client: &self,
            customer: customer.to_owned(),
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
        customer: &str,
    ) -> request::PostCustomersCustomerCardsRequest {
        request::PostCustomersCustomerCardsRequest {
            http_client: &self,
            customer: customer.to_owned(),
        }
    }
    ///<p>You can always see the 10 most recent cards directly on a customer; this method lets you retrieve details about a specific card stored on the customer.</p>
    pub fn get_customers_customer_cards_id(
        &self,
        customer: &str,
        id: &str,
    ) -> request::GetCustomersCustomerCardsIdRequest {
        request::GetCustomersCustomerCardsIdRequest {
            http_client: &self,
            customer: customer.to_owned(),
            expand: None,
            id: id.to_owned(),
        }
    }
    ///<p>Update a specified source for a given customer.</p>
    pub fn post_customers_customer_cards_id(
        &self,
        customer: &str,
        id: &str,
    ) -> request::PostCustomersCustomerCardsIdRequest {
        request::PostCustomersCustomerCardsIdRequest {
            http_client: &self,
            customer: customer.to_owned(),
            id: id.to_owned(),
        }
    }
    ///<p>Delete a specified source for a given customer.</p>
    pub fn delete_customers_customer_cards_id(
        &self,
        customer: &str,
        id: &str,
    ) -> request::DeleteCustomersCustomerCardsIdRequest {
        request::DeleteCustomersCustomerCardsIdRequest {
            http_client: &self,
            customer: customer.to_owned(),
            id: id.to_owned(),
        }
    }
    ///<p>Retrieves a customer’s cash balance.</p>
    pub fn get_customers_customer_cash_balance(
        &self,
        customer: &str,
    ) -> request::GetCustomersCustomerCashBalanceRequest {
        request::GetCustomersCustomerCashBalanceRequest {
            http_client: &self,
            customer: customer.to_owned(),
            expand: None,
        }
    }
    ///<p>Changes the settings on a customer’s cash balance.</p>
    pub fn post_customers_customer_cash_balance(
        &self,
        customer: &str,
    ) -> request::PostCustomersCustomerCashBalanceRequest {
        request::PostCustomersCustomerCashBalanceRequest {
            http_client: &self,
            customer: customer.to_owned(),
        }
    }
    ///<p>Returns a list of transactions that modified the customer’s <a href="/docs/payments/customer-balance">cash balance</a>.</p>
    pub fn get_customers_customer_cash_balance_transactions(
        &self,
        customer: &str,
    ) -> request::GetCustomersCustomerCashBalanceTransactionsRequest {
        request::GetCustomersCustomerCashBalanceTransactionsRequest {
            http_client: &self,
            customer: customer.to_owned(),
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
        }
    }
    ///<p>Retrieves a specific cash balance transaction, which updated the customer’s <a href="/docs/payments/customer-balance">cash balance</a>.</p>
    pub fn get_customers_customer_cash_balance_transactions_transaction(
        &self,
        customer: &str,
        transaction: &str,
    ) -> request::GetCustomersCustomerCashBalanceTransactionsTransactionRequest {
        request::GetCustomersCustomerCashBalanceTransactionsTransactionRequest {
            http_client: &self,
            customer: customer.to_owned(),
            expand: None,
            transaction: transaction.to_owned(),
        }
    }
    pub fn get_customers_customer_discount(
        &self,
        customer: &str,
    ) -> request::GetCustomersCustomerDiscountRequest {
        request::GetCustomersCustomerDiscountRequest {
            http_client: &self,
            customer: customer.to_owned(),
            expand: None,
        }
    }
    ///<p>Removes the currently applied discount on a customer.</p>
    pub fn delete_customers_customer_discount(
        &self,
        customer: &str,
    ) -> request::DeleteCustomersCustomerDiscountRequest {
        request::DeleteCustomersCustomerDiscountRequest {
            http_client: &self,
            customer: customer.to_owned(),
        }
    }
    /**<p>Retrieve funding instructions for a customer cash balance. If funding instructions do not yet exist for the customer, new
funding instructions will be created. If funding instructions have already been created for a given customer, the same
funding instructions will be retrieved. In other words, we will return the same funding instructions each time.</p>*/
    pub fn post_customers_customer_funding_instructions(
        &self,
        customer: &str,
    ) -> request::PostCustomersCustomerFundingInstructionsRequest {
        request::PostCustomersCustomerFundingInstructionsRequest {
            http_client: &self,
            customer: customer.to_owned(),
        }
    }
    ///<p>Returns a list of PaymentMethods for a given Customer</p>
    pub fn get_customers_customer_payment_methods(
        &self,
        customer: &str,
    ) -> request::GetCustomersCustomerPaymentMethodsRequest {
        request::GetCustomersCustomerPaymentMethodsRequest {
            http_client: &self,
            customer: customer.to_owned(),
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
            type_: None,
        }
    }
    ///<p>Retrieves a PaymentMethod object for a given Customer.</p>
    pub fn get_customers_customer_payment_methods_payment_method(
        &self,
        customer: &str,
        payment_method: &str,
    ) -> request::GetCustomersCustomerPaymentMethodsPaymentMethodRequest {
        request::GetCustomersCustomerPaymentMethodsPaymentMethodRequest {
            http_client: &self,
            customer: customer.to_owned(),
            expand: None,
            payment_method: payment_method.to_owned(),
        }
    }
    ///<p>List sources for a specified customer.</p>
    pub fn get_customers_customer_sources(
        &self,
        customer: &str,
    ) -> request::GetCustomersCustomerSourcesRequest {
        request::GetCustomersCustomerSourcesRequest {
            http_client: &self,
            customer: customer.to_owned(),
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
        customer: &str,
    ) -> request::PostCustomersCustomerSourcesRequest {
        request::PostCustomersCustomerSourcesRequest {
            http_client: &self,
            customer: customer.to_owned(),
        }
    }
    ///<p>Retrieve a specified source for a given customer.</p>
    pub fn get_customers_customer_sources_id(
        &self,
        customer: &str,
        id: &str,
    ) -> request::GetCustomersCustomerSourcesIdRequest {
        request::GetCustomersCustomerSourcesIdRequest {
            http_client: &self,
            customer: customer.to_owned(),
            expand: None,
            id: id.to_owned(),
        }
    }
    ///<p>Update a specified source for a given customer.</p>
    pub fn post_customers_customer_sources_id(
        &self,
        customer: &str,
        id: &str,
    ) -> request::PostCustomersCustomerSourcesIdRequest {
        request::PostCustomersCustomerSourcesIdRequest {
            http_client: &self,
            customer: customer.to_owned(),
            id: id.to_owned(),
        }
    }
    ///<p>Delete a specified source for a given customer.</p>
    pub fn delete_customers_customer_sources_id(
        &self,
        customer: &str,
        id: &str,
    ) -> request::DeleteCustomersCustomerSourcesIdRequest {
        request::DeleteCustomersCustomerSourcesIdRequest {
            http_client: &self,
            customer: customer.to_owned(),
            id: id.to_owned(),
        }
    }
    ///<p>Verify a specified bank account for a given customer.</p>
    pub fn post_customers_customer_sources_id_verify(
        &self,
        customer: &str,
        id: &str,
    ) -> request::PostCustomersCustomerSourcesIdVerifyRequest {
        request::PostCustomersCustomerSourcesIdVerifyRequest {
            http_client: &self,
            customer: customer.to_owned(),
            id: id.to_owned(),
        }
    }
    ///<p>You can see a list of the customer’s active subscriptions. Note that the 10 most recent active subscriptions are always available by default on the customer object. If you need more than those 10, you can use the limit and starting_after parameters to page through additional subscriptions.</p>
    pub fn get_customers_customer_subscriptions(
        &self,
        customer: &str,
    ) -> request::GetCustomersCustomerSubscriptionsRequest {
        request::GetCustomersCustomerSubscriptionsRequest {
            http_client: &self,
            customer: customer.to_owned(),
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
        }
    }
    ///<p>Creates a new subscription on an existing customer.</p>
    pub fn post_customers_customer_subscriptions(
        &self,
        customer: &str,
    ) -> request::PostCustomersCustomerSubscriptionsRequest {
        request::PostCustomersCustomerSubscriptionsRequest {
            http_client: &self,
            customer: customer.to_owned(),
        }
    }
    ///<p>Retrieves the subscription with the given ID.</p>
    pub fn get_customers_customer_subscriptions_subscription_exposed_id(
        &self,
        customer: &str,
        subscription_exposed_id: &str,
    ) -> request::GetCustomersCustomerSubscriptionsSubscriptionExposedIdRequest {
        request::GetCustomersCustomerSubscriptionsSubscriptionExposedIdRequest {
            http_client: &self,
            customer: customer.to_owned(),
            expand: None,
            subscription_exposed_id: subscription_exposed_id.to_owned(),
        }
    }
    ///<p>Updates an existing subscription on a customer to match the specified parameters. When changing plans or quantities, we will optionally prorate the price we charge next month to make up for any price changes. To preview how the proration will be calculated, use the <a href="#upcoming_invoice">upcoming invoice</a> endpoint.</p>
    pub fn post_customers_customer_subscriptions_subscription_exposed_id(
        &self,
        customer: &str,
        subscription_exposed_id: &str,
    ) -> request::PostCustomersCustomerSubscriptionsSubscriptionExposedIdRequest {
        request::PostCustomersCustomerSubscriptionsSubscriptionExposedIdRequest {
            http_client: &self,
            customer: customer.to_owned(),
            subscription_exposed_id: subscription_exposed_id.to_owned(),
        }
    }
    /**<p>Cancels a customer’s subscription. If you set the <code>at_period_end</code> parameter to <code>true</code>, the subscription will remain active until the end of the period, at which point it will be canceled and not renewed. Otherwise, with the default <code>false</code> value, the subscription is terminated immediately. In either case, the customer will not be charged again for the subscription.</p>

<p>Note, however, that any pending invoice items that you’ve created will still be charged for at the end of the period, unless manually <a href="#delete_invoiceitem">deleted</a>. If you’ve set the subscription to cancel at the end of the period, any pending prorations will also be left in place and collected at the end of the period. But if the subscription is set to cancel immediately, pending prorations will be removed.</p>

<p>By default, upon subscription cancellation, Stripe will stop automatic collection of all finalized invoices for the customer. This is intended to prevent unexpected payment attempts after the customer has canceled a subscription. However, you can resume automatic collection of the invoices manually after subscription cancellation to have us proceed. Or, you could check for unpaid invoices before allowing the customer to cancel the subscription at all.</p>*/
    pub fn delete_customers_customer_subscriptions_subscription_exposed_id(
        &self,
        customer: &str,
        subscription_exposed_id: &str,
    ) -> request::DeleteCustomersCustomerSubscriptionsSubscriptionExposedIdRequest {
        request::DeleteCustomersCustomerSubscriptionsSubscriptionExposedIdRequest {
            http_client: &self,
            customer: customer.to_owned(),
            subscription_exposed_id: subscription_exposed_id.to_owned(),
        }
    }
    pub fn get_customers_customer_subscriptions_subscription_exposed_id_discount(
        &self,
        customer: &str,
        subscription_exposed_id: &str,
    ) -> request::GetCustomersCustomerSubscriptionsSubscriptionExposedIdDiscountRequest {
        request::GetCustomersCustomerSubscriptionsSubscriptionExposedIdDiscountRequest {
            http_client: &self,
            customer: customer.to_owned(),
            expand: None,
            subscription_exposed_id: subscription_exposed_id.to_owned(),
        }
    }
    ///<p>Removes the currently applied discount on a customer.</p>
    pub fn delete_customers_customer_subscriptions_subscription_exposed_id_discount(
        &self,
        customer: &str,
        subscription_exposed_id: &str,
    ) -> request::DeleteCustomersCustomerSubscriptionsSubscriptionExposedIdDiscountRequest {
        request::DeleteCustomersCustomerSubscriptionsSubscriptionExposedIdDiscountRequest {
            http_client: &self,
            customer: customer.to_owned(),
            subscription_exposed_id: subscription_exposed_id.to_owned(),
        }
    }
    ///<p>Returns a list of tax IDs for a customer.</p>
    pub fn get_customers_customer_tax_ids(
        &self,
        customer: &str,
    ) -> request::GetCustomersCustomerTaxIdsRequest {
        request::GetCustomersCustomerTaxIdsRequest {
            http_client: &self,
            customer: customer.to_owned(),
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
        }
    }
    ///<p>Creates a new <code>tax_id</code> object for a customer.</p>
    pub fn post_customers_customer_tax_ids(
        &self,
        customer: &str,
    ) -> request::PostCustomersCustomerTaxIdsRequest {
        request::PostCustomersCustomerTaxIdsRequest {
            http_client: &self,
            customer: customer.to_owned(),
        }
    }
    ///<p>Retrieves the <code>tax_id</code> object with the given identifier.</p>
    pub fn get_customers_customer_tax_ids_id(
        &self,
        customer: &str,
        id: &str,
    ) -> request::GetCustomersCustomerTaxIdsIdRequest {
        request::GetCustomersCustomerTaxIdsIdRequest {
            http_client: &self,
            customer: customer.to_owned(),
            expand: None,
            id: id.to_owned(),
        }
    }
    ///<p>Deletes an existing <code>tax_id</code> object.</p>
    pub fn delete_customers_customer_tax_ids_id(
        &self,
        customer: &str,
        id: &str,
    ) -> request::DeleteCustomersCustomerTaxIdsIdRequest {
        request::DeleteCustomersCustomerTaxIdsIdRequest {
            http_client: &self,
            customer: customer.to_owned(),
            id: id.to_owned(),
        }
    }
    ///<p>Returns a list of your disputes.</p>
    pub fn get_disputes(&self) -> request::GetDisputesRequest {
        request::GetDisputesRequest {
            http_client: &self,
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
        dispute: &str,
    ) -> request::GetDisputesDisputeRequest {
        request::GetDisputesDisputeRequest {
            http_client: &self,
            dispute: dispute.to_owned(),
            expand: None,
        }
    }
    /**<p>When you get a dispute, contacting your customer is always the best first step. If that doesn’t work, you can submit evidence to help us resolve the dispute in your favor. You can do this in your <a href="https://dashboard.stripe.com/disputes">dashboard</a>, but if you prefer, you can use the API to submit evidence programmatically.</p>

<p>Depending on your dispute type, different evidence fields will give you a better chance of winning your dispute. To figure out which evidence fields to provide, see our <a href="/docs/disputes/categories">guide to dispute types</a>.</p>*/
    pub fn post_disputes_dispute(
        &self,
        dispute: &str,
    ) -> request::PostDisputesDisputeRequest {
        request::PostDisputesDisputeRequest {
            http_client: &self,
            dispute: dispute.to_owned(),
        }
    }
    /**<p>Closing the dispute for a charge indicates that you do not have any evidence to submit and are essentially dismissing the dispute, acknowledging it as lost.</p>

<p>The status of the dispute will change from <code>needs_response</code> to <code>lost</code>. <em>Closing a dispute is irreversible</em>.</p>*/
    pub fn post_disputes_dispute_close(
        &self,
        dispute: &str,
    ) -> request::PostDisputesDisputeCloseRequest {
        request::PostDisputesDisputeCloseRequest {
            http_client: &self,
            dispute: dispute.to_owned(),
        }
    }
    ///<p>Creates a short-lived API key for a given resource.</p>
    pub fn post_ephemeral_keys(&self) -> request::PostEphemeralKeysRequest {
        request::PostEphemeralKeysRequest {
            http_client: &self,
        }
    }
    ///<p>Invalidates a short-lived API key for a given resource.</p>
    pub fn delete_ephemeral_keys_key(
        &self,
        key: &str,
    ) -> request::DeleteEphemeralKeysKeyRequest {
        request::DeleteEphemeralKeysKeyRequest {
            http_client: &self,
            key: key.to_owned(),
        }
    }
    ///<p>List events, going back up to 30 days. Each event data is rendered according to Stripe API version at its creation time, specified in <a href="/docs/api/events/object">event object</a> <code>api_version</code> attribute (not according to your current Stripe API version or <code>Stripe-Version</code> header).</p>
    pub fn get_events(&self) -> request::GetEventsRequest {
        request::GetEventsRequest {
            http_client: &self,
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
    pub fn get_events_id(&self, id: &str) -> request::GetEventsIdRequest {
        request::GetEventsIdRequest {
            http_client: &self,
            expand: None,
            id: id.to_owned(),
        }
    }
    ///<p>Returns a list of objects that contain the rates at which foreign currencies are converted to one another. Only shows the currencies for which Stripe supports.</p>
    pub fn get_exchange_rates(&self) -> request::GetExchangeRatesRequest {
        request::GetExchangeRatesRequest {
            http_client: &self,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
        }
    }
    ///<p>Retrieves the exchange rates from the given currency to every supported currency.</p>
    pub fn get_exchange_rates_rate_id(
        &self,
        rate_id: &str,
    ) -> request::GetExchangeRatesRateIdRequest {
        request::GetExchangeRatesRateIdRequest {
            http_client: &self,
            expand: None,
            rate_id: rate_id.to_owned(),
        }
    }
    ///<p>Returns a list of file links.</p>
    pub fn get_file_links(&self) -> request::GetFileLinksRequest {
        request::GetFileLinksRequest {
            http_client: &self,
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
    pub fn post_file_links(&self) -> request::PostFileLinksRequest {
        request::PostFileLinksRequest {
            http_client: &self,
        }
    }
    ///<p>Retrieves the file link with the given ID.</p>
    pub fn get_file_links_link(&self, link: &str) -> request::GetFileLinksLinkRequest {
        request::GetFileLinksLinkRequest {
            http_client: &self,
            expand: None,
            link: link.to_owned(),
        }
    }
    ///<p>Updates an existing file link object. Expired links can no longer be updated.</p>
    pub fn post_file_links_link(&self, link: &str) -> request::PostFileLinksLinkRequest {
        request::PostFileLinksLinkRequest {
            http_client: &self,
            link: link.to_owned(),
        }
    }
    ///<p>Returns a list of the files that your account has access to. Stripe sorts and returns the files by their creation dates, placing the most recently created files at the top.</p>
    pub fn get_files(&self) -> request::GetFilesRequest {
        request::GetFilesRequest {
            http_client: &self,
            created: None,
            ending_before: None,
            expand: None,
            limit: None,
            purpose: None,
            starting_after: None,
        }
    }
    /**<p>To upload a file to Stripe, you need to send a request of type <code>multipart/form-data</code>. Include the file you want to upload in the request, and the parameters for creating a file.</p>

<p>All of Stripe’s officially supported Client libraries support sending <code>multipart/form-data</code>.</p>*/
    pub fn post_files(&self) -> request::PostFilesRequest {
        request::PostFilesRequest {
            http_client: &self,
        }
    }
    ///<p>Retrieves the details of an existing file object. After you supply a unique file ID, Stripe returns the corresponding file object. Learn how to <a href="/docs/file-upload#download-file-contents">access file contents</a>.</p>
    pub fn get_files_file(&self, file: &str) -> request::GetFilesFileRequest {
        request::GetFilesFileRequest {
            http_client: &self,
            expand: None,
            file: file.to_owned(),
        }
    }
    ///<p>Returns a list of Financial Connections <code>Account</code> objects.</p>
    pub fn get_financial_connections_accounts(
        &self,
    ) -> request::GetFinancialConnectionsAccountsRequest {
        request::GetFinancialConnectionsAccountsRequest {
            http_client: &self,
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
        account: &str,
    ) -> request::GetFinancialConnectionsAccountsAccountRequest {
        request::GetFinancialConnectionsAccountsAccountRequest {
            http_client: &self,
            account: account.to_owned(),
            expand: None,
        }
    }
    ///<p>Disables your access to a Financial Connections <code>Account</code>. You will no longer be able to access data associated with the account (e.g. balances, transactions).</p>
    pub fn post_financial_connections_accounts_account_disconnect(
        &self,
        account: &str,
    ) -> request::PostFinancialConnectionsAccountsAccountDisconnectRequest {
        request::PostFinancialConnectionsAccountsAccountDisconnectRequest {
            http_client: &self,
            account: account.to_owned(),
        }
    }
    ///<p>Lists all owners for a given <code>Account</code></p>
    pub fn get_financial_connections_accounts_account_owners(
        &self,
        account: &str,
        ownership: &str,
    ) -> request::GetFinancialConnectionsAccountsAccountOwnersRequest {
        request::GetFinancialConnectionsAccountsAccountOwnersRequest {
            http_client: &self,
            account: account.to_owned(),
            ending_before: None,
            expand: None,
            limit: None,
            ownership: ownership.to_owned(),
            starting_after: None,
        }
    }
    ///<p>Refreshes the data associated with a Financial Connections <code>Account</code>.</p>
    pub fn post_financial_connections_accounts_account_refresh(
        &self,
        account: &str,
    ) -> request::PostFinancialConnectionsAccountsAccountRefreshRequest {
        request::PostFinancialConnectionsAccountsAccountRefreshRequest {
            http_client: &self,
            account: account.to_owned(),
        }
    }
    ///<p>To launch the Financial Connections authorization flow, create a <code>Session</code>. The session’s <code>client_secret</code> can be used to launch the flow using Stripe.js.</p>
    pub fn post_financial_connections_sessions(
        &self,
    ) -> request::PostFinancialConnectionsSessionsRequest {
        request::PostFinancialConnectionsSessionsRequest {
            http_client: &self,
        }
    }
    ///<p>Retrieves the details of a Financial Connections <code>Session</code></p>
    pub fn get_financial_connections_sessions_session(
        &self,
        session: &str,
    ) -> request::GetFinancialConnectionsSessionsSessionRequest {
        request::GetFinancialConnectionsSessionsSessionRequest {
            http_client: &self,
            expand: None,
            session: session.to_owned(),
        }
    }
    ///<p>List all verification reports.</p>
    pub fn get_identity_verification_reports(
        &self,
    ) -> request::GetIdentityVerificationReportsRequest {
        request::GetIdentityVerificationReportsRequest {
            http_client: &self,
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
        report: &str,
    ) -> request::GetIdentityVerificationReportsReportRequest {
        request::GetIdentityVerificationReportsReportRequest {
            http_client: &self,
            expand: None,
            report: report.to_owned(),
        }
    }
    ///<p>Returns a list of VerificationSessions</p>
    pub fn get_identity_verification_sessions(
        &self,
    ) -> request::GetIdentityVerificationSessionsRequest {
        request::GetIdentityVerificationSessionsRequest {
            http_client: &self,
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

<p>Related guide: <a href="/docs/identity/verify-identity-documents">Verify your users’ identity documents</a></p>*/
    pub fn post_identity_verification_sessions(
        &self,
    ) -> request::PostIdentityVerificationSessionsRequest {
        request::PostIdentityVerificationSessionsRequest {
            http_client: &self,
        }
    }
    /**<p>Retrieves the details of a VerificationSession that was previously created.</p>

<p>When the session status is <code>requires_input</code>, you can use this method to retrieve a valid
<code>client_secret</code> or <code>url</code> to allow re-submission.</p>*/
    pub fn get_identity_verification_sessions_session(
        &self,
        session: &str,
    ) -> request::GetIdentityVerificationSessionsSessionRequest {
        request::GetIdentityVerificationSessionsSessionRequest {
            http_client: &self,
            expand: None,
            session: session.to_owned(),
        }
    }
    /**<p>Updates a VerificationSession object.</p>

<p>When the session status is <code>requires_input</code>, you can use this method to update the
verification check and options.</p>*/
    pub fn post_identity_verification_sessions_session(
        &self,
        session: &str,
    ) -> request::PostIdentityVerificationSessionsSessionRequest {
        request::PostIdentityVerificationSessionsSessionRequest {
            http_client: &self,
            session: session.to_owned(),
        }
    }
    /**<p>A VerificationSession object can be canceled when it is in <code>requires_input</code> <a href="/docs/identity/how-sessions-work">status</a>.</p>

<p>Once canceled, future submission attempts are disabled. This cannot be undone. <a href="/docs/identity/verification-sessions#cancel">Learn more</a>.</p>*/
    pub fn post_identity_verification_sessions_session_cancel(
        &self,
        session: &str,
    ) -> request::PostIdentityVerificationSessionsSessionCancelRequest {
        request::PostIdentityVerificationSessionsSessionCancelRequest {
            http_client: &self,
            session: session.to_owned(),
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
        session: &str,
    ) -> request::PostIdentityVerificationSessionsSessionRedactRequest {
        request::PostIdentityVerificationSessionsSessionRedactRequest {
            http_client: &self,
            session: session.to_owned(),
        }
    }
    ///<p>Returns a list of your invoice items. Invoice items are returned sorted by creation date, with the most recently created invoice items appearing first.</p>
    pub fn get_invoiceitems(&self) -> request::GetInvoiceitemsRequest {
        request::GetInvoiceitemsRequest {
            http_client: &self,
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
    pub fn post_invoiceitems(&self) -> request::PostInvoiceitemsRequest {
        request::PostInvoiceitemsRequest {
            http_client: &self,
        }
    }
    ///<p>Retrieves the invoice item with the given ID.</p>
    pub fn get_invoiceitems_invoiceitem(
        &self,
        invoiceitem: &str,
    ) -> request::GetInvoiceitemsInvoiceitemRequest {
        request::GetInvoiceitemsInvoiceitemRequest {
            http_client: &self,
            expand: None,
            invoiceitem: invoiceitem.to_owned(),
        }
    }
    ///<p>Updates the amount or description of an invoice item on an upcoming invoice. Updating an invoice item is only possible before the invoice it’s attached to is closed.</p>
    pub fn post_invoiceitems_invoiceitem(
        &self,
        invoiceitem: &str,
    ) -> request::PostInvoiceitemsInvoiceitemRequest {
        request::PostInvoiceitemsInvoiceitemRequest {
            http_client: &self,
            invoiceitem: invoiceitem.to_owned(),
        }
    }
    ///<p>Deletes an invoice item, removing it from an invoice. Deleting invoice items is only possible when they’re not attached to invoices, or if it’s attached to a draft invoice.</p>
    pub fn delete_invoiceitems_invoiceitem(
        &self,
        invoiceitem: &str,
    ) -> request::DeleteInvoiceitemsInvoiceitemRequest {
        request::DeleteInvoiceitemsInvoiceitemRequest {
            http_client: &self,
            invoiceitem: invoiceitem.to_owned(),
        }
    }
    ///<p>You can list all invoices, or list the invoices for a specific customer. The invoices are returned sorted by creation date, with the most recently created invoices appearing first.</p>
    pub fn get_invoices(&self) -> request::GetInvoicesRequest {
        request::GetInvoicesRequest {
            http_client: &self,
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
    ///<p>This endpoint creates a draft invoice for a given customer. The invoice remains a draft until you <a href="#finalize_invoice">finalize</a> the invoice, which allows you to <a href="#pay_invoice">pay</a> or <a href="#send_invoice">send</a> the invoice to your customers.</p>
    pub fn post_invoices(&self) -> request::PostInvoicesRequest {
        request::PostInvoicesRequest {
            http_client: &self,
        }
    }
    /**<p>Search for invoices you’ve previously created using Stripe’s <a href="/docs/search#search-query-language">Search Query Language</a>.
Don’t use search in read-after-write flows where strict consistency is necessary. Under normal operating
conditions, data is searchable in less than a minute. Occasionally, propagation of new or updated data can be up
to an hour behind during outages. Search functionality is not available to merchants in India.</p>*/
    pub fn get_invoices_search(&self, query: &str) -> request::GetInvoicesSearchRequest {
        request::GetInvoicesSearchRequest {
            http_client: &self,
            expand: None,
            limit: None,
            page: None,
            query: query.to_owned(),
        }
    }
    /**<p>At any time, you can preview the upcoming invoice for a customer. This will show you all the charges that are pending, including subscription renewal charges, invoice item charges, etc. It will also show you any discounts that are applicable to the invoice.</p>

<p>Note that when you are viewing an upcoming invoice, you are simply viewing a preview – the invoice has not yet been created. As such, the upcoming invoice will not show up in invoice listing calls, and you cannot use the API to pay or edit the invoice. If you want to change the amount that your customer will be billed, you can add, remove, or update pending invoice items, or update the customer’s discount.</p>

<p>You can preview the effects of updating a subscription, including a preview of what proration will take place. To ensure that the actual proration is calculated exactly the same as the previewed proration, you should pass a <code>proration_date</code> parameter when doing the actual subscription update. The value passed in should be the same as the <code>subscription_proration_date</code> returned on the upcoming invoice resource. The recommended way to get only the prorations being previewed is to consider only proration line items where <code>period[start]</code> is equal to the <code>subscription_proration_date</code> on the upcoming invoice resource.</p>*/
    pub fn get_invoices_upcoming(&self) -> request::GetInvoicesUpcomingRequest {
        request::GetInvoicesUpcomingRequest {
            http_client: &self,
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
            subscription_resume_at: None,
            subscription_start_date: None,
            subscription_trial_end: None,
            subscription_trial_from_plan: None,
        }
    }
    ///<p>When retrieving an upcoming invoice, you’ll get a <strong>lines</strong> property containing the total count of line items and the first handful of those items. There is also a URL where you can retrieve the full (paginated) list of line items.</p>
    pub fn get_invoices_upcoming_lines(
        &self,
    ) -> request::GetInvoicesUpcomingLinesRequest {
        request::GetInvoicesUpcomingLinesRequest {
            http_client: &self,
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
            subscription_resume_at: None,
            subscription_start_date: None,
            subscription_trial_end: None,
            subscription_trial_from_plan: None,
        }
    }
    ///<p>Retrieves the invoice with the given ID.</p>
    pub fn get_invoices_invoice(
        &self,
        invoice: &str,
    ) -> request::GetInvoicesInvoiceRequest {
        request::GetInvoicesInvoiceRequest {
            http_client: &self,
            expand: None,
            invoice: invoice.to_owned(),
        }
    }
    /**<p>Draft invoices are fully editable. Once an invoice is <a href="/docs/billing/invoices/workflow#finalized">finalized</a>,
monetary values, as well as <code>collection_method</code>, become uneditable.</p>

<p>If you would like to stop the Stripe Billing engine from automatically finalizing, reattempting payments on,
sending reminders for, or <a href="/docs/billing/invoices/reconciliation">automatically reconciling</a> invoices, pass
<code>auto_advance=false</code>.</p>*/
    pub fn post_invoices_invoice(
        &self,
        invoice: &str,
    ) -> request::PostInvoicesInvoiceRequest {
        request::PostInvoicesInvoiceRequest {
            http_client: &self,
            invoice: invoice.to_owned(),
        }
    }
    ///<p>Permanently deletes a one-off invoice draft. This cannot be undone. Attempts to delete invoices that are no longer in a draft state will fail; once an invoice has been finalized or if an invoice is for a subscription, it must be <a href="#void_invoice">voided</a>.</p>
    pub fn delete_invoices_invoice(
        &self,
        invoice: &str,
    ) -> request::DeleteInvoicesInvoiceRequest {
        request::DeleteInvoicesInvoiceRequest {
            http_client: &self,
            invoice: invoice.to_owned(),
        }
    }
    ///<p>Stripe automatically finalizes drafts before sending and attempting payment on invoices. However, if you’d like to finalize a draft invoice manually, you can do so using this method.</p>
    pub fn post_invoices_invoice_finalize(
        &self,
        invoice: &str,
    ) -> request::PostInvoicesInvoiceFinalizeRequest {
        request::PostInvoicesInvoiceFinalizeRequest {
            http_client: &self,
            invoice: invoice.to_owned(),
        }
    }
    ///<p>When retrieving an invoice, you’ll get a <strong>lines</strong> property containing the total count of line items and the first handful of those items. There is also a URL where you can retrieve the full (paginated) list of line items.</p>
    pub fn get_invoices_invoice_lines(
        &self,
        invoice: &str,
    ) -> request::GetInvoicesInvoiceLinesRequest {
        request::GetInvoicesInvoiceLinesRequest {
            http_client: &self,
            ending_before: None,
            expand: None,
            invoice: invoice.to_owned(),
            limit: None,
            starting_after: None,
        }
    }
    /**<p>Updates an invoice’s line item. Some fields, such as <code>tax_amounts</code>, only live on the invoice line item,
so they can only be updated through this endpoint. Other fields, such as <code>amount</code>, live on both the invoice
item and the invoice line item, so updates on this endpoint will propagate to the invoice item as well.
Updating an invoice’s line item is only possible before the invoice is finalized.</p>*/
    pub fn post_invoices_invoice_lines_line_item_id(
        &self,
        invoice: &str,
        line_item_id: &str,
    ) -> request::PostInvoicesInvoiceLinesLineItemIdRequest {
        request::PostInvoicesInvoiceLinesLineItemIdRequest {
            http_client: &self,
            invoice: invoice.to_owned(),
            line_item_id: line_item_id.to_owned(),
        }
    }
    ///<p>Marking an invoice as uncollectible is useful for keeping track of bad debts that can be written off for accounting purposes.</p>
    pub fn post_invoices_invoice_mark_uncollectible(
        &self,
        invoice: &str,
    ) -> request::PostInvoicesInvoiceMarkUncollectibleRequest {
        request::PostInvoicesInvoiceMarkUncollectibleRequest {
            http_client: &self,
            invoice: invoice.to_owned(),
        }
    }
    ///<p>Stripe automatically creates and then attempts to collect payment on invoices for customers on subscriptions according to your <a href="https://dashboard.stripe.com/account/billing/automatic">subscriptions settings</a>. However, if you’d like to attempt payment on an invoice out of the normal collection schedule or for some other reason, you can do so.</p>
    pub fn post_invoices_invoice_pay(
        &self,
        invoice: &str,
    ) -> request::PostInvoicesInvoicePayRequest {
        request::PostInvoicesInvoicePayRequest {
            http_client: &self,
            invoice: invoice.to_owned(),
        }
    }
    /**<p>Stripe will automatically send invoices to customers according to your <a href="https://dashboard.stripe.com/account/billing/automatic">subscriptions settings</a>. However, if you’d like to manually send an invoice to your customer out of the normal schedule, you can do so. When sending invoices that have already been paid, there will be no reference to the payment in the email.</p>

<p>Requests made in test-mode result in no emails being sent, despite sending an <code>invoice.sent</code> event.</p>*/
    pub fn post_invoices_invoice_send(
        &self,
        invoice: &str,
    ) -> request::PostInvoicesInvoiceSendRequest {
        request::PostInvoicesInvoiceSendRequest {
            http_client: &self,
            invoice: invoice.to_owned(),
        }
    }
    ///<p>Mark a finalized invoice as void. This cannot be undone. Voiding an invoice is similar to <a href="#delete_invoice">deletion</a>, however it only applies to finalized invoices and maintains a papertrail where the invoice can still be found.</p>
    pub fn post_invoices_invoice_void(
        &self,
        invoice: &str,
    ) -> request::PostInvoicesInvoiceVoidRequest {
        request::PostInvoicesInvoiceVoidRequest {
            http_client: &self,
            invoice: invoice.to_owned(),
        }
    }
    ///<p>Returns a list of Issuing <code>Authorization</code> objects. The objects are sorted in descending order by creation date, with the most recently created object appearing first.</p>
    pub fn get_issuing_authorizations(
        &self,
    ) -> request::GetIssuingAuthorizationsRequest {
        request::GetIssuingAuthorizationsRequest {
            http_client: &self,
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
        authorization: &str,
    ) -> request::GetIssuingAuthorizationsAuthorizationRequest {
        request::GetIssuingAuthorizationsAuthorizationRequest {
            http_client: &self,
            authorization: authorization.to_owned(),
            expand: None,
        }
    }
    ///<p>Updates the specified Issuing <code>Authorization</code> object by setting the values of the parameters passed. Any parameters not provided will be left unchanged.</p>
    pub fn post_issuing_authorizations_authorization(
        &self,
        authorization: &str,
    ) -> request::PostIssuingAuthorizationsAuthorizationRequest {
        request::PostIssuingAuthorizationsAuthorizationRequest {
            http_client: &self,
            authorization: authorization.to_owned(),
        }
    }
    /**<p>[Deprecated] Approves a pending Issuing <code>Authorization</code> object. This request should be made within the timeout window of the <a href="/docs/issuing/controls/real-time-authorizations">real-time authorization</a> flow.
This method is deprecated. Instead, <a href="/docs/issuing/controls/real-time-authorizations#authorization-handling">respond directly to the webhook request to approve an authorization</a>.</p>*/
    pub fn post_issuing_authorizations_authorization_approve(
        &self,
        authorization: &str,
    ) -> request::PostIssuingAuthorizationsAuthorizationApproveRequest {
        request::PostIssuingAuthorizationsAuthorizationApproveRequest {
            http_client: &self,
            authorization: authorization.to_owned(),
        }
    }
    /**<p>[Deprecated] Declines a pending Issuing <code>Authorization</code> object. This request should be made within the timeout window of the <a href="/docs/issuing/controls/real-time-authorizations">real time authorization</a> flow.
This method is deprecated. Instead, <a href="/docs/issuing/controls/real-time-authorizations#authorization-handling">respond directly to the webhook request to decline an authorization</a>.</p>*/
    pub fn post_issuing_authorizations_authorization_decline(
        &self,
        authorization: &str,
    ) -> request::PostIssuingAuthorizationsAuthorizationDeclineRequest {
        request::PostIssuingAuthorizationsAuthorizationDeclineRequest {
            http_client: &self,
            authorization: authorization.to_owned(),
        }
    }
    ///<p>Returns a list of Issuing <code>Cardholder</code> objects. The objects are sorted in descending order by creation date, with the most recently created object appearing first.</p>
    pub fn get_issuing_cardholders(&self) -> request::GetIssuingCardholdersRequest {
        request::GetIssuingCardholdersRequest {
            http_client: &self,
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
    pub fn post_issuing_cardholders(&self) -> request::PostIssuingCardholdersRequest {
        request::PostIssuingCardholdersRequest {
            http_client: &self,
        }
    }
    ///<p>Retrieves an Issuing <code>Cardholder</code> object.</p>
    pub fn get_issuing_cardholders_cardholder(
        &self,
        cardholder: &str,
    ) -> request::GetIssuingCardholdersCardholderRequest {
        request::GetIssuingCardholdersCardholderRequest {
            http_client: &self,
            cardholder: cardholder.to_owned(),
            expand: None,
        }
    }
    ///<p>Updates the specified Issuing <code>Cardholder</code> object by setting the values of the parameters passed. Any parameters not provided will be left unchanged.</p>
    pub fn post_issuing_cardholders_cardholder(
        &self,
        cardholder: &str,
    ) -> request::PostIssuingCardholdersCardholderRequest {
        request::PostIssuingCardholdersCardholderRequest {
            http_client: &self,
            cardholder: cardholder.to_owned(),
        }
    }
    ///<p>Returns a list of Issuing <code>Card</code> objects. The objects are sorted in descending order by creation date, with the most recently created object appearing first.</p>
    pub fn get_issuing_cards(&self) -> request::GetIssuingCardsRequest {
        request::GetIssuingCardsRequest {
            http_client: &self,
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
    pub fn post_issuing_cards(&self) -> request::PostIssuingCardsRequest {
        request::PostIssuingCardsRequest {
            http_client: &self,
        }
    }
    ///<p>Retrieves an Issuing <code>Card</code> object.</p>
    pub fn get_issuing_cards_card(
        &self,
        card: &str,
    ) -> request::GetIssuingCardsCardRequest {
        request::GetIssuingCardsCardRequest {
            http_client: &self,
            card: card.to_owned(),
            expand: None,
        }
    }
    ///<p>Updates the specified Issuing <code>Card</code> object by setting the values of the parameters passed. Any parameters not provided will be left unchanged.</p>
    pub fn post_issuing_cards_card(
        &self,
        card: &str,
    ) -> request::PostIssuingCardsCardRequest {
        request::PostIssuingCardsCardRequest {
            http_client: &self,
            card: card.to_owned(),
        }
    }
    ///<p>Returns a list of Issuing <code>Dispute</code> objects. The objects are sorted in descending order by creation date, with the most recently created object appearing first.</p>
    pub fn get_issuing_disputes(&self) -> request::GetIssuingDisputesRequest {
        request::GetIssuingDisputesRequest {
            http_client: &self,
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
    pub fn post_issuing_disputes(&self) -> request::PostIssuingDisputesRequest {
        request::PostIssuingDisputesRequest {
            http_client: &self,
        }
    }
    ///<p>Retrieves an Issuing <code>Dispute</code> object.</p>
    pub fn get_issuing_disputes_dispute(
        &self,
        dispute: &str,
    ) -> request::GetIssuingDisputesDisputeRequest {
        request::GetIssuingDisputesDisputeRequest {
            http_client: &self,
            dispute: dispute.to_owned(),
            expand: None,
        }
    }
    ///<p>Updates the specified Issuing <code>Dispute</code> object by setting the values of the parameters passed. Any parameters not provided will be left unchanged. Properties on the <code>evidence</code> object can be unset by passing in an empty string.</p>
    pub fn post_issuing_disputes_dispute(
        &self,
        dispute: &str,
    ) -> request::PostIssuingDisputesDisputeRequest {
        request::PostIssuingDisputesDisputeRequest {
            http_client: &self,
            dispute: dispute.to_owned(),
        }
    }
    ///<p>Submits an Issuing <code>Dispute</code> to the card network. Stripe validates that all evidence fields required for the dispute’s reason are present. For more details, see <a href="/docs/issuing/purchases/disputes#dispute-reasons-and-evidence">Dispute reasons and evidence</a>.</p>
    pub fn post_issuing_disputes_dispute_submit(
        &self,
        dispute: &str,
    ) -> request::PostIssuingDisputesDisputeSubmitRequest {
        request::PostIssuingDisputesDisputeSubmitRequest {
            http_client: &self,
            dispute: dispute.to_owned(),
        }
    }
    ///<p>Returns a list of Issuing <code>Settlement</code> objects. The objects are sorted in descending order by creation date, with the most recently created object appearing first.</p>
    pub fn get_issuing_settlements(&self) -> request::GetIssuingSettlementsRequest {
        request::GetIssuingSettlementsRequest {
            http_client: &self,
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
        settlement: &str,
    ) -> request::GetIssuingSettlementsSettlementRequest {
        request::GetIssuingSettlementsSettlementRequest {
            http_client: &self,
            expand: None,
            settlement: settlement.to_owned(),
        }
    }
    ///<p>Updates the specified Issuing <code>Settlement</code> object by setting the values of the parameters passed. Any parameters not provided will be left unchanged.</p>
    pub fn post_issuing_settlements_settlement(
        &self,
        settlement: &str,
    ) -> request::PostIssuingSettlementsSettlementRequest {
        request::PostIssuingSettlementsSettlementRequest {
            http_client: &self,
            settlement: settlement.to_owned(),
        }
    }
    ///<p>Lists all Issuing <code>Token</code> objects for a given card.</p>
    pub fn get_issuing_tokens(&self, card: &str) -> request::GetIssuingTokensRequest {
        request::GetIssuingTokensRequest {
            http_client: &self,
            card: card.to_owned(),
            created: None,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
            status: None,
        }
    }
    ///<p>Retrieves an Issuing <code>Token</code> object.</p>
    pub fn get_issuing_tokens_token(
        &self,
        token: &str,
    ) -> request::GetIssuingTokensTokenRequest {
        request::GetIssuingTokensTokenRequest {
            http_client: &self,
            expand: None,
            token: token.to_owned(),
        }
    }
    ///<p>Attempts to update the specified Issuing <code>Token</code> object to the status specified.</p>
    pub fn post_issuing_tokens_token(
        &self,
        token: &str,
    ) -> request::PostIssuingTokensTokenRequest {
        request::PostIssuingTokensTokenRequest {
            http_client: &self,
            token: token.to_owned(),
        }
    }
    ///<p>Returns a list of Issuing <code>Transaction</code> objects. The objects are sorted in descending order by creation date, with the most recently created object appearing first.</p>
    pub fn get_issuing_transactions(&self) -> request::GetIssuingTransactionsRequest {
        request::GetIssuingTransactionsRequest {
            http_client: &self,
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
        transaction: &str,
    ) -> request::GetIssuingTransactionsTransactionRequest {
        request::GetIssuingTransactionsTransactionRequest {
            http_client: &self,
            expand: None,
            transaction: transaction.to_owned(),
        }
    }
    ///<p>Updates the specified Issuing <code>Transaction</code> object by setting the values of the parameters passed. Any parameters not provided will be left unchanged.</p>
    pub fn post_issuing_transactions_transaction(
        &self,
        transaction: &str,
    ) -> request::PostIssuingTransactionsTransactionRequest {
        request::PostIssuingTransactionsTransactionRequest {
            http_client: &self,
            transaction: transaction.to_owned(),
        }
    }
    ///<p>To launch the Financial Connections authorization flow, create a <code>Session</code>. The session’s <code>client_secret</code> can be used to launch the flow using Stripe.js.</p>
    pub fn post_link_account_sessions(&self) -> request::PostLinkAccountSessionsRequest {
        request::PostLinkAccountSessionsRequest {
            http_client: &self,
        }
    }
    ///<p>Retrieves the details of a Financial Connections <code>Session</code></p>
    pub fn get_link_account_sessions_session(
        &self,
        session: &str,
    ) -> request::GetLinkAccountSessionsSessionRequest {
        request::GetLinkAccountSessionsSessionRequest {
            http_client: &self,
            expand: None,
            session: session.to_owned(),
        }
    }
    ///<p>Returns a list of Financial Connections <code>Account</code> objects.</p>
    pub fn get_linked_accounts(&self) -> request::GetLinkedAccountsRequest {
        request::GetLinkedAccountsRequest {
            http_client: &self,
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
        account: &str,
    ) -> request::GetLinkedAccountsAccountRequest {
        request::GetLinkedAccountsAccountRequest {
            http_client: &self,
            account: account.to_owned(),
            expand: None,
        }
    }
    ///<p>Disables your access to a Financial Connections <code>Account</code>. You will no longer be able to access data associated with the account (e.g. balances, transactions).</p>
    pub fn post_linked_accounts_account_disconnect(
        &self,
        account: &str,
    ) -> request::PostLinkedAccountsAccountDisconnectRequest {
        request::PostLinkedAccountsAccountDisconnectRequest {
            http_client: &self,
            account: account.to_owned(),
        }
    }
    ///<p>Lists all owners for a given <code>Account</code></p>
    pub fn get_linked_accounts_account_owners(
        &self,
        account: &str,
        ownership: &str,
    ) -> request::GetLinkedAccountsAccountOwnersRequest {
        request::GetLinkedAccountsAccountOwnersRequest {
            http_client: &self,
            account: account.to_owned(),
            ending_before: None,
            expand: None,
            limit: None,
            ownership: ownership.to_owned(),
            starting_after: None,
        }
    }
    ///<p>Refreshes the data associated with a Financial Connections <code>Account</code>.</p>
    pub fn post_linked_accounts_account_refresh(
        &self,
        account: &str,
    ) -> request::PostLinkedAccountsAccountRefreshRequest {
        request::PostLinkedAccountsAccountRefreshRequest {
            http_client: &self,
            account: account.to_owned(),
        }
    }
    ///<p>Retrieves a Mandate object.</p>
    pub fn get_mandates_mandate(
        &self,
        mandate: &str,
    ) -> request::GetMandatesMandateRequest {
        request::GetMandatesMandateRequest {
            http_client: &self,
            expand: None,
            mandate: mandate.to_owned(),
        }
    }
    ///<p>Returns a list of PaymentIntents.</p>
    pub fn get_payment_intents(&self) -> request::GetPaymentIntentsRequest {
        request::GetPaymentIntentsRequest {
            http_client: &self,
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
to continue the payment. Learn more about <a href="/docs/payments/payment-intents">the available payment flows
with the Payment Intents API</a>.</p>

<p>When you use <code>confirm=true</code> during creation, it’s equivalent to creating
and confirming the PaymentIntent in the same call. You can use any parameters
available in the <a href="/docs/api/payment_intents/confirm">confirm API</a> when you supply
<code>confirm=true</code>.</p>*/
    pub fn post_payment_intents(&self) -> request::PostPaymentIntentsRequest {
        request::PostPaymentIntentsRequest {
            http_client: &self,
        }
    }
    /**<p>Search for PaymentIntents you’ve previously created using Stripe’s <a href="/docs/search#search-query-language">Search Query Language</a>.
Don’t use search in read-after-write flows where strict consistency is necessary. Under normal operating
conditions, data is searchable in less than a minute. Occasionally, propagation of new or updated data can be up
to an hour behind during outages. Search functionality is not available to merchants in India.</p>*/
    pub fn get_payment_intents_search(
        &self,
        query: &str,
    ) -> request::GetPaymentIntentsSearchRequest {
        request::GetPaymentIntentsSearchRequest {
            http_client: &self,
            expand: None,
            limit: None,
            page: None,
            query: query.to_owned(),
        }
    }
    /**<p>Retrieves the details of a PaymentIntent that has previously been created. </p>

<p>You can retrieve a PaymentIntent client-side using a publishable key when the <code>client_secret</code> is in the query string. </p>

<p>If you retrieve a PaymentIntent with a publishable key, it only returns a subset of properties. Refer to the <a href="#payment_intent_object">payment intent</a> object reference for more details.</p>*/
    pub fn get_payment_intents_intent(
        &self,
        intent: &str,
    ) -> request::GetPaymentIntentsIntentRequest {
        request::GetPaymentIntentsIntentRequest {
            http_client: &self,
            client_secret: None,
            expand: None,
            intent: intent.to_owned(),
        }
    }
    /**<p>Updates properties on a PaymentIntent object without confirming.</p>

<p>Depending on which properties you update, you might need to confirm the
PaymentIntent again. For example, updating the <code>payment_method</code>
always requires you to confirm the PaymentIntent again. If you prefer to
update and confirm at the same time, we recommend updating properties through
the <a href="/docs/api/payment_intents/confirm">confirm API</a> instead.</p>*/
    pub fn post_payment_intents_intent(
        &self,
        intent: &str,
    ) -> request::PostPaymentIntentsIntentRequest {
        request::PostPaymentIntentsIntentRequest {
            http_client: &self,
            intent: intent.to_owned(),
        }
    }
    ///<p>Manually reconcile the remaining amount for a <code>customer_balance</code> PaymentIntent.</p>
    pub fn post_payment_intents_intent_apply_customer_balance(
        &self,
        intent: &str,
    ) -> request::PostPaymentIntentsIntentApplyCustomerBalanceRequest {
        request::PostPaymentIntentsIntentApplyCustomerBalanceRequest {
            http_client: &self,
            intent: intent.to_owned(),
        }
    }
    /**<p>You can cancel a PaymentIntent object when it’s in one of these statuses: <code>requires_payment_method</code>, <code>requires_capture</code>, <code>requires_confirmation</code>, <code>requires_action</code> or, <a href="/docs/payments/intents">in rare cases</a>, <code>processing</code>. </p>

<p>After it’s canceled, no additional charges are made by the PaymentIntent and any operations on the PaymentIntent fail with an error. For PaymentIntents with a <code>status</code> of <code>requires_capture</code>, the remaining <code>amount_capturable</code> is automatically refunded. </p>

<p>You can’t cancel the PaymentIntent for a Checkout Session. <a href="/docs/api/checkout/sessions/expire">Expire the Checkout Session</a> instead.</p>*/
    pub fn post_payment_intents_intent_cancel(
        &self,
        intent: &str,
    ) -> request::PostPaymentIntentsIntentCancelRequest {
        request::PostPaymentIntentsIntentCancelRequest {
            http_client: &self,
            intent: intent.to_owned(),
        }
    }
    /**<p>Capture the funds of an existing uncaptured PaymentIntent when its status is <code>requires_capture</code>.</p>

<p>Uncaptured PaymentIntents are cancelled a set number of days (7 by default) after their creation.</p>

<p>Learn more about <a href="/docs/payments/capture-later">separate authorization and capture</a>.</p>*/
    pub fn post_payment_intents_intent_capture(
        &self,
        intent: &str,
    ) -> request::PostPaymentIntentsIntentCaptureRequest {
        request::PostPaymentIntentsIntentCaptureRequest {
            http_client: &self,
            intent: intent.to_owned(),
        }
    }
    /**<p>Confirm that your customer intends to pay with current or provided
payment method. Upon confirmation, the PaymentIntent will attempt to initiate
a payment.
If the selected payment method requires additional authentication steps, the
PaymentIntent will transition to the <code>requires_action</code> status and
suggest additional actions via <code>next_action</code>. If payment fails,
the PaymentIntent transitions to the <code>requires_payment_method</code> status or the
<code>canceled</code> status if the confirmation limit is reached. If
payment succeeds, the PaymentIntent will transition to the <code>succeeded</code>
status (or <code>requires_capture</code>, if <code>capture_method</code> is set to <code>manual</code>).
If the <code>confirmation_method</code> is <code>automatic</code>, payment may be attempted
using our <a href="/docs/stripe-js/reference#stripe-handle-card-payment">client SDKs</a>
and the PaymentIntent’s <a href="#payment_intent_object-client_secret">client_secret</a>.
After <code>next_action</code>s are handled by the client, no additional
confirmation is required to complete the payment.
If the <code>confirmation_method</code> is <code>manual</code>, all payment attempts must be
initiated using a secret key.
If any actions are required for the payment, the PaymentIntent will
return to the <code>requires_confirmation</code> state
after those actions are completed. Your server needs to then
explicitly re-confirm the PaymentIntent to initiate the next payment
attempt. Read the <a href="/docs/payments/payment-intents/web-manual">expanded documentation</a>
to learn more about manual confirmation.</p>*/
    pub fn post_payment_intents_intent_confirm(
        &self,
        intent: &str,
    ) -> request::PostPaymentIntentsIntentConfirmRequest {
        request::PostPaymentIntentsIntentConfirmRequest {
            http_client: &self,
            intent: intent.to_owned(),
        }
    }
    /**<p>Perform an incremental authorization on an eligible
<a href="/docs/api/payment_intents/object">PaymentIntent</a>. To be eligible, the
PaymentIntent’s status must be <code>requires_capture</code> and
<a href="/docs/api/charges/object#charge_object-payment_method_details-card_present-incremental_authorization_supported">incremental_authorization_supported</a>
must be <code>true</code>.</p>

<p>Incremental authorizations attempt to increase the authorized amount on
your customer’s card to the new, higher <code>amount</code> provided. Similar to the
initial authorization, incremental authorizations can be declined. A
single PaymentIntent can call this endpoint multiple times to further
increase the authorized amount.</p>

<p>If the incremental authorization succeeds, the PaymentIntent object
returns with the updated
<a href="/docs/api/payment_intents/object#payment_intent_object-amount">amount</a>.
If the incremental authorization fails, a
<a href="/docs/error-codes#card-declined">card_declined</a> error returns, and no other
fields on the PaymentIntent or Charge update. The PaymentIntent
object remains capturable for the previously authorized amount.</p>

<p>Each PaymentIntent can have a maximum of 10 incremental authorization attempts, including declines.
After it’s captured, a PaymentIntent can no longer be incremented.</p>

<p>Learn more about <a href="/docs/terminal/features/incremental-authorizations">incremental authorizations</a>.</p>*/
    pub fn post_payment_intents_intent_increment_authorization(
        &self,
        intent: &str,
    ) -> request::PostPaymentIntentsIntentIncrementAuthorizationRequest {
        request::PostPaymentIntentsIntentIncrementAuthorizationRequest {
            http_client: &self,
            intent: intent.to_owned(),
        }
    }
    ///<p>Verifies microdeposits on a PaymentIntent object.</p>
    pub fn post_payment_intents_intent_verify_microdeposits(
        &self,
        intent: &str,
    ) -> request::PostPaymentIntentsIntentVerifyMicrodepositsRequest {
        request::PostPaymentIntentsIntentVerifyMicrodepositsRequest {
            http_client: &self,
            intent: intent.to_owned(),
        }
    }
    ///<p>Returns a list of your payment links.</p>
    pub fn get_payment_links(&self) -> request::GetPaymentLinksRequest {
        request::GetPaymentLinksRequest {
            http_client: &self,
            active: None,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
        }
    }
    ///<p>Creates a payment link.</p>
    pub fn post_payment_links(&self) -> request::PostPaymentLinksRequest {
        request::PostPaymentLinksRequest {
            http_client: &self,
        }
    }
    ///<p>Retrieve a payment link.</p>
    pub fn get_payment_links_payment_link(
        &self,
        payment_link: &str,
    ) -> request::GetPaymentLinksPaymentLinkRequest {
        request::GetPaymentLinksPaymentLinkRequest {
            http_client: &self,
            expand: None,
            payment_link: payment_link.to_owned(),
        }
    }
    ///<p>Updates a payment link.</p>
    pub fn post_payment_links_payment_link(
        &self,
        payment_link: &str,
    ) -> request::PostPaymentLinksPaymentLinkRequest {
        request::PostPaymentLinksPaymentLinkRequest {
            http_client: &self,
            payment_link: payment_link.to_owned(),
        }
    }
    ///<p>When retrieving a payment link, there is an includable <strong>line_items</strong> property containing the first handful of those items. There is also a URL where you can retrieve the full (paginated) list of line items.</p>
    pub fn get_payment_links_payment_link_line_items(
        &self,
        payment_link: &str,
    ) -> request::GetPaymentLinksPaymentLinkLineItemsRequest {
        request::GetPaymentLinksPaymentLinkLineItemsRequest {
            http_client: &self,
            ending_before: None,
            expand: None,
            limit: None,
            payment_link: payment_link.to_owned(),
            starting_after: None,
        }
    }
    ///<p>List payment method configurations</p>
    pub fn get_payment_method_configurations(
        &self,
    ) -> request::GetPaymentMethodConfigurationsRequest {
        request::GetPaymentMethodConfigurationsRequest {
            http_client: &self,
            application: None,
            expand: None,
        }
    }
    ///<p>Creates a payment method configuration</p>
    pub fn post_payment_method_configurations(
        &self,
    ) -> request::PostPaymentMethodConfigurationsRequest {
        request::PostPaymentMethodConfigurationsRequest {
            http_client: &self,
        }
    }
    ///<p>Retrieve payment method configuration</p>
    pub fn get_payment_method_configurations_configuration(
        &self,
        configuration: &str,
    ) -> request::GetPaymentMethodConfigurationsConfigurationRequest {
        request::GetPaymentMethodConfigurationsConfigurationRequest {
            http_client: &self,
            configuration: configuration.to_owned(),
            expand: None,
        }
    }
    ///<p>Update payment method configuration</p>
    pub fn post_payment_method_configurations_configuration(
        &self,
        configuration: &str,
    ) -> request::PostPaymentMethodConfigurationsConfigurationRequest {
        request::PostPaymentMethodConfigurationsConfigurationRequest {
            http_client: &self,
            configuration: configuration.to_owned(),
        }
    }
    ///<p>Lists the details of existing payment method domains.</p>
    pub fn get_payment_method_domains(&self) -> request::GetPaymentMethodDomainsRequest {
        request::GetPaymentMethodDomainsRequest {
            http_client: &self,
            domain_name: None,
            enabled: None,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
        }
    }
    ///<p>Creates a payment method domain.</p>
    pub fn post_payment_method_domains(
        &self,
    ) -> request::PostPaymentMethodDomainsRequest {
        request::PostPaymentMethodDomainsRequest {
            http_client: &self,
        }
    }
    ///<p>Retrieves the details of an existing payment method domain.</p>
    pub fn get_payment_method_domains_payment_method_domain(
        &self,
        payment_method_domain: &str,
    ) -> request::GetPaymentMethodDomainsPaymentMethodDomainRequest {
        request::GetPaymentMethodDomainsPaymentMethodDomainRequest {
            http_client: &self,
            expand: None,
            payment_method_domain: payment_method_domain.to_owned(),
        }
    }
    ///<p>Updates an existing payment method domain.</p>
    pub fn post_payment_method_domains_payment_method_domain(
        &self,
        payment_method_domain: &str,
    ) -> request::PostPaymentMethodDomainsPaymentMethodDomainRequest {
        request::PostPaymentMethodDomainsPaymentMethodDomainRequest {
            http_client: &self,
            payment_method_domain: payment_method_domain.to_owned(),
        }
    }
    /**<p>Some payment methods such as Apple Pay require additional steps to verify a domain. If the requirements weren’t satisfied when the domain was created, the payment method will be inactive on the domain.
The payment method doesn’t appear in Elements for this domain until it is active.</p>

<p>To activate a payment method on an existing payment method domain, complete the required validation steps specific to the payment method, and then validate the payment method domain with this endpoint.</p>

<p>Related guides: <a href="/docs/payments/payment-methods/pmd-registration">Payment method domains</a>.</p>*/
    pub fn post_payment_method_domains_payment_method_domain_validate(
        &self,
        payment_method_domain: &str,
    ) -> request::PostPaymentMethodDomainsPaymentMethodDomainValidateRequest {
        request::PostPaymentMethodDomainsPaymentMethodDomainValidateRequest {
            http_client: &self,
            payment_method_domain: payment_method_domain.to_owned(),
        }
    }
    ///<p>Returns a list of PaymentMethods for Treasury flows. If you want to list the PaymentMethods attached to a Customer for payments, you should use the <a href="/docs/api/payment_methods/customer_list">List a Customer’s PaymentMethods</a> API instead.</p>
    pub fn get_payment_methods(&self) -> request::GetPaymentMethodsRequest {
        request::GetPaymentMethodsRequest {
            http_client: &self,
            customer: None,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
            type_: None,
        }
    }
    /**<p>Creates a PaymentMethod object. Read the <a href="/docs/stripe-js/reference#stripe-create-payment-method">Stripe.js reference</a> to learn how to create PaymentMethods via Stripe.js.</p>

<p>Instead of creating a PaymentMethod directly, we recommend using the <a href="/docs/payments/accept-a-payment">PaymentIntents</a> API to accept a payment immediately or the <a href="/docs/payments/save-and-reuse">SetupIntent</a> API to collect payment method details ahead of a future payment.</p>*/
    pub fn post_payment_methods(&self) -> request::PostPaymentMethodsRequest {
        request::PostPaymentMethodsRequest {
            http_client: &self,
        }
    }
    ///<p>Retrieves a PaymentMethod object attached to the StripeAccount. To retrieve a payment method attached to a Customer, you should use <a href="/docs/api/payment_methods/customer">Retrieve a Customer’s PaymentMethods</a></p>
    pub fn get_payment_methods_payment_method(
        &self,
        payment_method: &str,
    ) -> request::GetPaymentMethodsPaymentMethodRequest {
        request::GetPaymentMethodsPaymentMethodRequest {
            http_client: &self,
            expand: None,
            payment_method: payment_method.to_owned(),
        }
    }
    ///<p>Updates a PaymentMethod object. A PaymentMethod must be attached a customer to be updated.</p>
    pub fn post_payment_methods_payment_method(
        &self,
        payment_method: &str,
    ) -> request::PostPaymentMethodsPaymentMethodRequest {
        request::PostPaymentMethodsPaymentMethodRequest {
            http_client: &self,
            payment_method: payment_method.to_owned(),
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
        payment_method: &str,
    ) -> request::PostPaymentMethodsPaymentMethodAttachRequest {
        request::PostPaymentMethodsPaymentMethodAttachRequest {
            http_client: &self,
            payment_method: payment_method.to_owned(),
        }
    }
    ///<p>Detaches a PaymentMethod object from a Customer. After a PaymentMethod is detached, it can no longer be used for a payment or re-attached to a Customer.</p>
    pub fn post_payment_methods_payment_method_detach(
        &self,
        payment_method: &str,
    ) -> request::PostPaymentMethodsPaymentMethodDetachRequest {
        request::PostPaymentMethodsPaymentMethodDetachRequest {
            http_client: &self,
            payment_method: payment_method.to_owned(),
        }
    }
    ///<p>Returns a list of existing payouts sent to third-party bank accounts or payouts that Stripe sent to you. The payouts return in sorted order, with the most recently created payouts appearing first.</p>
    pub fn get_payouts(&self) -> request::GetPayoutsRequest {
        request::GetPayoutsRequest {
            http_client: &self,
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
    /**<p>To send funds to your own bank account, create a new payout object. Your <a href="#balance">Stripe balance</a> must cover the payout amount. If it doesn’t, you receive an “Insufficient Funds” error.</p>

<p>If your API key is in test mode, money won’t actually be sent, though every other action occurs as if you’re in live mode.</p>

<p>If you create a manual payout on a Stripe account that uses multiple payment source types, you need to specify the source type balance that the payout draws from. The <a href="#balance_object">balance object</a> details available and pending amounts by source type.</p>*/
    pub fn post_payouts(&self) -> request::PostPayoutsRequest {
        request::PostPayoutsRequest {
            http_client: &self,
        }
    }
    ///<p>Retrieves the details of an existing payout. Supply the unique payout ID from either a payout creation request or the payout list. Stripe returns the corresponding payout information.</p>
    pub fn get_payouts_payout(&self, payout: &str) -> request::GetPayoutsPayoutRequest {
        request::GetPayoutsPayoutRequest {
            http_client: &self,
            expand: None,
            payout: payout.to_owned(),
        }
    }
    ///<p>Updates the specified payout by setting the values of the parameters you pass. We don’t change parameters that you don’t provide. This request only accepts the metadata as arguments.</p>
    pub fn post_payouts_payout(
        &self,
        payout: &str,
    ) -> request::PostPayoutsPayoutRequest {
        request::PostPayoutsPayoutRequest {
            http_client: &self,
            payout: payout.to_owned(),
        }
    }
    ///<p>You can cancel a previously created payout if it hasn’t been paid out yet. Stripe refunds the funds to your available balance. You can’t cancel automatic Stripe payouts.</p>
    pub fn post_payouts_payout_cancel(
        &self,
        payout: &str,
    ) -> request::PostPayoutsPayoutCancelRequest {
        request::PostPayoutsPayoutCancelRequest {
            http_client: &self,
            payout: payout.to_owned(),
        }
    }
    /**<p>Reverses a payout by debiting the destination bank account. At this time, you can only reverse payouts for connected accounts to US bank accounts. If the payout is in the <code>pending</code> status, use <code>/v1/payouts/:id/cancel</code> instead.</p>

<p>By requesting a reversal through <code>/v1/payouts/:id/reverse</code>, you confirm that the authorized signatory of the selected bank account authorizes the debit on the bank account and that no other authorization is required.</p>*/
    pub fn post_payouts_payout_reverse(
        &self,
        payout: &str,
    ) -> request::PostPayoutsPayoutReverseRequest {
        request::PostPayoutsPayoutReverseRequest {
            http_client: &self,
            payout: payout.to_owned(),
        }
    }
    ///<p>Returns a list of your plans.</p>
    pub fn get_plans(&self) -> request::GetPlansRequest {
        request::GetPlansRequest {
            http_client: &self,
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
    pub fn post_plans(&self) -> request::PostPlansRequest {
        request::PostPlansRequest {
            http_client: &self,
        }
    }
    ///<p>Retrieves the plan with the given ID.</p>
    pub fn get_plans_plan(&self, plan: &str) -> request::GetPlansPlanRequest {
        request::GetPlansPlanRequest {
            http_client: &self,
            expand: None,
            plan: plan.to_owned(),
        }
    }
    ///<p>Updates the specified plan by setting the values of the parameters passed. Any parameters not provided are left unchanged. By design, you cannot change a plan’s ID, amount, currency, or billing cycle.</p>
    pub fn post_plans_plan(&self, plan: &str) -> request::PostPlansPlanRequest {
        request::PostPlansPlanRequest {
            http_client: &self,
            plan: plan.to_owned(),
        }
    }
    ///<p>Deleting plans means new subscribers can’t be added. Existing subscribers aren’t affected.</p>
    pub fn delete_plans_plan(&self, plan: &str) -> request::DeletePlansPlanRequest {
        request::DeletePlansPlanRequest {
            http_client: &self,
            plan: plan.to_owned(),
        }
    }
    ///<p>Returns a list of your active prices, excluding <a href="/docs/products-prices/pricing-models#inline-pricing">inline prices</a>. For the list of inactive prices, set <code>active</code> to false.</p>
    pub fn get_prices(&self) -> request::GetPricesRequest {
        request::GetPricesRequest {
            http_client: &self,
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
    pub fn post_prices(&self) -> request::PostPricesRequest {
        request::PostPricesRequest {
            http_client: &self,
        }
    }
    /**<p>Search for prices you’ve previously created using Stripe’s <a href="/docs/search#search-query-language">Search Query Language</a>.
Don’t use search in read-after-write flows where strict consistency is necessary. Under normal operating
conditions, data is searchable in less than a minute. Occasionally, propagation of new or updated data can be up
to an hour behind during outages. Search functionality is not available to merchants in India.</p>*/
    pub fn get_prices_search(&self, query: &str) -> request::GetPricesSearchRequest {
        request::GetPricesSearchRequest {
            http_client: &self,
            expand: None,
            limit: None,
            page: None,
            query: query.to_owned(),
        }
    }
    ///<p>Retrieves the price with the given ID.</p>
    pub fn get_prices_price(&self, price: &str) -> request::GetPricesPriceRequest {
        request::GetPricesPriceRequest {
            http_client: &self,
            expand: None,
            price: price.to_owned(),
        }
    }
    ///<p>Updates the specified price by setting the values of the parameters passed. Any parameters not provided are left unchanged.</p>
    pub fn post_prices_price(&self, price: &str) -> request::PostPricesPriceRequest {
        request::PostPricesPriceRequest {
            http_client: &self,
            price: price.to_owned(),
        }
    }
    ///<p>Returns a list of your products. The products are returned sorted by creation date, with the most recently created products appearing first.</p>
    pub fn get_products(&self) -> request::GetProductsRequest {
        request::GetProductsRequest {
            http_client: &self,
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
    pub fn post_products(&self) -> request::PostProductsRequest {
        request::PostProductsRequest {
            http_client: &self,
        }
    }
    /**<p>Search for products you’ve previously created using Stripe’s <a href="/docs/search#search-query-language">Search Query Language</a>.
Don’t use search in read-after-write flows where strict consistency is necessary. Under normal operating
conditions, data is searchable in less than a minute. Occasionally, propagation of new or updated data can be up
to an hour behind during outages. Search functionality is not available to merchants in India.</p>*/
    pub fn get_products_search(&self, query: &str) -> request::GetProductsSearchRequest {
        request::GetProductsSearchRequest {
            http_client: &self,
            expand: None,
            limit: None,
            page: None,
            query: query.to_owned(),
        }
    }
    ///<p>Retrieves the details of an existing product. Supply the unique product ID from either a product creation request or the product list, and Stripe will return the corresponding product information.</p>
    pub fn get_products_id(&self, id: &str) -> request::GetProductsIdRequest {
        request::GetProductsIdRequest {
            http_client: &self,
            expand: None,
            id: id.to_owned(),
        }
    }
    ///<p>Updates the specific product by setting the values of the parameters passed. Any parameters not provided will be left unchanged.</p>
    pub fn post_products_id(&self, id: &str) -> request::PostProductsIdRequest {
        request::PostProductsIdRequest {
            http_client: &self,
            id: id.to_owned(),
        }
    }
    ///<p>Delete a product. Deleting a product is only possible if it has no prices associated with it. Additionally, deleting a product with <code>type=good</code> is only possible if it has no SKUs associated with it.</p>
    pub fn delete_products_id(&self, id: &str) -> request::DeleteProductsIdRequest {
        request::DeleteProductsIdRequest {
            http_client: &self,
            id: id.to_owned(),
        }
    }
    ///<p>Returns a list of your promotion codes.</p>
    pub fn get_promotion_codes(&self) -> request::GetPromotionCodesRequest {
        request::GetPromotionCodesRequest {
            http_client: &self,
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
    pub fn post_promotion_codes(&self) -> request::PostPromotionCodesRequest {
        request::PostPromotionCodesRequest {
            http_client: &self,
        }
    }
    ///<p>Retrieves the promotion code with the given ID. In order to retrieve a promotion code by the customer-facing <code>code</code> use <a href="/docs/api/promotion_codes/list">list</a> with the desired <code>code</code>.</p>
    pub fn get_promotion_codes_promotion_code(
        &self,
        promotion_code: &str,
    ) -> request::GetPromotionCodesPromotionCodeRequest {
        request::GetPromotionCodesPromotionCodeRequest {
            http_client: &self,
            expand: None,
            promotion_code: promotion_code.to_owned(),
        }
    }
    ///<p>Updates the specified promotion code by setting the values of the parameters passed. Most fields are, by design, not editable.</p>
    pub fn post_promotion_codes_promotion_code(
        &self,
        promotion_code: &str,
    ) -> request::PostPromotionCodesPromotionCodeRequest {
        request::PostPromotionCodesPromotionCodeRequest {
            http_client: &self,
            promotion_code: promotion_code.to_owned(),
        }
    }
    ///<p>Returns a list of your quotes.</p>
    pub fn get_quotes(&self) -> request::GetQuotesRequest {
        request::GetQuotesRequest {
            http_client: &self,
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
    pub fn post_quotes(&self) -> request::PostQuotesRequest {
        request::PostQuotesRequest {
            http_client: &self,
        }
    }
    ///<p>Retrieves the quote with the given ID.</p>
    pub fn get_quotes_quote(&self, quote: &str) -> request::GetQuotesQuoteRequest {
        request::GetQuotesQuoteRequest {
            http_client: &self,
            expand: None,
            quote: quote.to_owned(),
        }
    }
    ///<p>A quote models prices and services for a customer.</p>
    pub fn post_quotes_quote(&self, quote: &str) -> request::PostQuotesQuoteRequest {
        request::PostQuotesQuoteRequest {
            http_client: &self,
            quote: quote.to_owned(),
        }
    }
    ///<p>Accepts the specified quote.</p>
    pub fn post_quotes_quote_accept(
        &self,
        quote: &str,
    ) -> request::PostQuotesQuoteAcceptRequest {
        request::PostQuotesQuoteAcceptRequest {
            http_client: &self,
            quote: quote.to_owned(),
        }
    }
    ///<p>Cancels the quote.</p>
    pub fn post_quotes_quote_cancel(
        &self,
        quote: &str,
    ) -> request::PostQuotesQuoteCancelRequest {
        request::PostQuotesQuoteCancelRequest {
            http_client: &self,
            quote: quote.to_owned(),
        }
    }
    ///<p>When retrieving a quote, there is an includable <a href="https://stripe.com/docs/api/quotes/object#quote_object-computed-upfront-line_items"><strong>computed.upfront.line_items</strong></a> property containing the first handful of those items. There is also a URL where you can retrieve the full (paginated) list of upfront line items.</p>
    pub fn get_quotes_quote_computed_upfront_line_items(
        &self,
        quote: &str,
    ) -> request::GetQuotesQuoteComputedUpfrontLineItemsRequest {
        request::GetQuotesQuoteComputedUpfrontLineItemsRequest {
            http_client: &self,
            ending_before: None,
            expand: None,
            limit: None,
            quote: quote.to_owned(),
            starting_after: None,
        }
    }
    ///<p>Finalizes the quote.</p>
    pub fn post_quotes_quote_finalize(
        &self,
        quote: &str,
    ) -> request::PostQuotesQuoteFinalizeRequest {
        request::PostQuotesQuoteFinalizeRequest {
            http_client: &self,
            quote: quote.to_owned(),
        }
    }
    ///<p>When retrieving a quote, there is an includable <strong>line_items</strong> property containing the first handful of those items. There is also a URL where you can retrieve the full (paginated) list of line items.</p>
    pub fn get_quotes_quote_line_items(
        &self,
        quote: &str,
    ) -> request::GetQuotesQuoteLineItemsRequest {
        request::GetQuotesQuoteLineItemsRequest {
            http_client: &self,
            ending_before: None,
            expand: None,
            limit: None,
            quote: quote.to_owned(),
            starting_after: None,
        }
    }
    ///<p>Download the PDF for a finalized quote</p>
    pub fn get_quotes_quote_pdf(
        &self,
        quote: &str,
    ) -> request::GetQuotesQuotePdfRequest {
        request::GetQuotesQuotePdfRequest {
            http_client: &self,
            expand: None,
            quote: quote.to_owned(),
        }
    }
    ///<p>Returns a list of early fraud warnings.</p>
    pub fn get_radar_early_fraud_warnings(
        &self,
    ) -> request::GetRadarEarlyFraudWarningsRequest {
        request::GetRadarEarlyFraudWarningsRequest {
            http_client: &self,
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
        early_fraud_warning: &str,
    ) -> request::GetRadarEarlyFraudWarningsEarlyFraudWarningRequest {
        request::GetRadarEarlyFraudWarningsEarlyFraudWarningRequest {
            http_client: &self,
            early_fraud_warning: early_fraud_warning.to_owned(),
            expand: None,
        }
    }
    ///<p>Returns a list of <code>ValueListItem</code> objects. The objects are sorted in descending order by creation date, with the most recently created object appearing first.</p>
    pub fn get_radar_value_list_items(
        &self,
        value_list: &str,
    ) -> request::GetRadarValueListItemsRequest {
        request::GetRadarValueListItemsRequest {
            http_client: &self,
            created: None,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
            value: None,
            value_list: value_list.to_owned(),
        }
    }
    ///<p>Creates a new <code>ValueListItem</code> object, which is added to the specified parent value list.</p>
    pub fn post_radar_value_list_items(
        &self,
    ) -> request::PostRadarValueListItemsRequest {
        request::PostRadarValueListItemsRequest {
            http_client: &self,
        }
    }
    ///<p>Retrieves a <code>ValueListItem</code> object.</p>
    pub fn get_radar_value_list_items_item(
        &self,
        item: &str,
    ) -> request::GetRadarValueListItemsItemRequest {
        request::GetRadarValueListItemsItemRequest {
            http_client: &self,
            expand: None,
            item: item.to_owned(),
        }
    }
    ///<p>Deletes a <code>ValueListItem</code> object, removing it from its parent value list.</p>
    pub fn delete_radar_value_list_items_item(
        &self,
        item: &str,
    ) -> request::DeleteRadarValueListItemsItemRequest {
        request::DeleteRadarValueListItemsItemRequest {
            http_client: &self,
            item: item.to_owned(),
        }
    }
    ///<p>Returns a list of <code>ValueList</code> objects. The objects are sorted in descending order by creation date, with the most recently created object appearing first.</p>
    pub fn get_radar_value_lists(&self) -> request::GetRadarValueListsRequest {
        request::GetRadarValueListsRequest {
            http_client: &self,
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
    pub fn post_radar_value_lists(&self) -> request::PostRadarValueListsRequest {
        request::PostRadarValueListsRequest {
            http_client: &self,
        }
    }
    ///<p>Retrieves a <code>ValueList</code> object.</p>
    pub fn get_radar_value_lists_value_list(
        &self,
        value_list: &str,
    ) -> request::GetRadarValueListsValueListRequest {
        request::GetRadarValueListsValueListRequest {
            http_client: &self,
            expand: None,
            value_list: value_list.to_owned(),
        }
    }
    ///<p>Updates a <code>ValueList</code> object by setting the values of the parameters passed. Any parameters not provided will be left unchanged. Note that <code>item_type</code> is immutable.</p>
    pub fn post_radar_value_lists_value_list(
        &self,
        value_list: &str,
    ) -> request::PostRadarValueListsValueListRequest {
        request::PostRadarValueListsValueListRequest {
            http_client: &self,
            value_list: value_list.to_owned(),
        }
    }
    ///<p>Deletes a <code>ValueList</code> object, also deleting any items contained within the value list. To be deleted, a value list must not be referenced in any rules.</p>
    pub fn delete_radar_value_lists_value_list(
        &self,
        value_list: &str,
    ) -> request::DeleteRadarValueListsValueListRequest {
        request::DeleteRadarValueListsValueListRequest {
            http_client: &self,
            value_list: value_list.to_owned(),
        }
    }
    ///<p>Returns a list of all refunds you created. We return the refunds in sorted order, with the most recent refunds appearing first The 10 most recent refunds are always available by default on the Charge object.</p>
    pub fn get_refunds(&self) -> request::GetRefundsRequest {
        request::GetRefundsRequest {
            http_client: &self,
            charge: None,
            created: None,
            ending_before: None,
            expand: None,
            limit: None,
            payment_intent: None,
            starting_after: None,
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
    pub fn post_refunds(&self) -> request::PostRefundsRequest {
        request::PostRefundsRequest {
            http_client: &self,
        }
    }
    ///<p>Retrieves the details of an existing refund.</p>
    pub fn get_refunds_refund(&self, refund: &str) -> request::GetRefundsRefundRequest {
        request::GetRefundsRefundRequest {
            http_client: &self,
            expand: None,
            refund: refund.to_owned(),
        }
    }
    /**<p>Updates the refund that you specify by setting the values of the passed parameters. Any parameters that you don’t provide remain unchanged.</p>

<p>This request only accepts <code>metadata</code> as an argument.</p>*/
    pub fn post_refunds_refund(
        &self,
        refund: &str,
    ) -> request::PostRefundsRefundRequest {
        request::PostRefundsRefundRequest {
            http_client: &self,
            refund: refund.to_owned(),
        }
    }
    /**<p>Cancels a refund with a status of <code>requires_action</code>.</p>

<p>You can’t cancel refunds in other states. Only refunds for payment methods that require customer action can enter the <code>requires_action</code> state.</p>*/
    pub fn post_refunds_refund_cancel(
        &self,
        refund: &str,
    ) -> request::PostRefundsRefundCancelRequest {
        request::PostRefundsRefundCancelRequest {
            http_client: &self,
            refund: refund.to_owned(),
        }
    }
    ///<p>Returns a list of Report Runs, with the most recent appearing first.</p>
    pub fn get_reporting_report_runs(&self) -> request::GetReportingReportRunsRequest {
        request::GetReportingReportRunsRequest {
            http_client: &self,
            created: None,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
        }
    }
    ///<p>Creates a new object and begin running the report. (Certain report types require a <a href="https://stripe.com/docs/keys#test-live-modes">live-mode API key</a>.)</p>
    pub fn post_reporting_report_runs(&self) -> request::PostReportingReportRunsRequest {
        request::PostReportingReportRunsRequest {
            http_client: &self,
        }
    }
    ///<p>Retrieves the details of an existing Report Run.</p>
    pub fn get_reporting_report_runs_report_run(
        &self,
        report_run: &str,
    ) -> request::GetReportingReportRunsReportRunRequest {
        request::GetReportingReportRunsReportRunRequest {
            http_client: &self,
            expand: None,
            report_run: report_run.to_owned(),
        }
    }
    ///<p>Returns a full list of Report Types.</p>
    pub fn get_reporting_report_types(&self) -> request::GetReportingReportTypesRequest {
        request::GetReportingReportTypesRequest {
            http_client: &self,
            expand: None,
        }
    }
    ///<p>Retrieves the details of a Report Type. (Certain report types require a <a href="https://stripe.com/docs/keys#test-live-modes">live-mode API key</a>.)</p>
    pub fn get_reporting_report_types_report_type(
        &self,
        report_type: &str,
    ) -> request::GetReportingReportTypesReportTypeRequest {
        request::GetReportingReportTypesReportTypeRequest {
            http_client: &self,
            expand: None,
            report_type: report_type.to_owned(),
        }
    }
    ///<p>Returns a list of <code>Review</code> objects that have <code>open</code> set to <code>true</code>. The objects are sorted in descending order by creation date, with the most recently created object appearing first.</p>
    pub fn get_reviews(&self) -> request::GetReviewsRequest {
        request::GetReviewsRequest {
            http_client: &self,
            created: None,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
        }
    }
    ///<p>Retrieves a <code>Review</code> object.</p>
    pub fn get_reviews_review(&self, review: &str) -> request::GetReviewsReviewRequest {
        request::GetReviewsReviewRequest {
            http_client: &self,
            expand: None,
            review: review.to_owned(),
        }
    }
    ///<p>Approves a <code>Review</code> object, closing it and removing it from the list of reviews.</p>
    pub fn post_reviews_review_approve(
        &self,
        review: &str,
    ) -> request::PostReviewsReviewApproveRequest {
        request::PostReviewsReviewApproveRequest {
            http_client: &self,
            review: review.to_owned(),
        }
    }
    ///<p>Returns a list of SetupAttempts that associate with a provided SetupIntent.</p>
    pub fn get_setup_attempts(
        &self,
        setup_intent: &str,
    ) -> request::GetSetupAttemptsRequest {
        request::GetSetupAttemptsRequest {
            http_client: &self,
            created: None,
            ending_before: None,
            expand: None,
            limit: None,
            setup_intent: setup_intent.to_owned(),
            starting_after: None,
        }
    }
    ///<p>Returns a list of SetupIntents.</p>
    pub fn get_setup_intents(&self) -> request::GetSetupIntentsRequest {
        request::GetSetupIntentsRequest {
            http_client: &self,
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

<p>After you create the SetupIntent, attach a payment method and <a href="/docs/api/setup_intents/confirm">confirm</a>
it to collect any required permissions to charge the payment method later.</p>*/
    pub fn post_setup_intents(&self) -> request::PostSetupIntentsRequest {
        request::PostSetupIntentsRequest {
            http_client: &self,
        }
    }
    /**<p>Retrieves the details of a SetupIntent that has previously been created. </p>

<p>Client-side retrieval using a publishable key is allowed when the <code>client_secret</code> is provided in the query string. </p>

<p>When retrieved with a publishable key, only a subset of properties will be returned. Please refer to the <a href="#setup_intent_object">SetupIntent</a> object reference for more details.</p>*/
    pub fn get_setup_intents_intent(
        &self,
        intent: &str,
    ) -> request::GetSetupIntentsIntentRequest {
        request::GetSetupIntentsIntentRequest {
            http_client: &self,
            client_secret: None,
            expand: None,
            intent: intent.to_owned(),
        }
    }
    ///<p>Updates a SetupIntent object.</p>
    pub fn post_setup_intents_intent(
        &self,
        intent: &str,
    ) -> request::PostSetupIntentsIntentRequest {
        request::PostSetupIntentsIntentRequest {
            http_client: &self,
            intent: intent.to_owned(),
        }
    }
    /**<p>You can cancel a SetupIntent object when it’s in one of these statuses: <code>requires_payment_method</code>, <code>requires_confirmation</code>, or <code>requires_action</code>. </p>

<p>After you cancel it, setup is abandoned and any operations on the SetupIntent fail with an error.</p>*/
    pub fn post_setup_intents_intent_cancel(
        &self,
        intent: &str,
    ) -> request::PostSetupIntentsIntentCancelRequest {
        request::PostSetupIntentsIntentCancelRequest {
            http_client: &self,
            intent: intent.to_owned(),
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
<code>requires_payment_method</code> status or the <code>canceled</code> status if the
confirmation limit is reached.</p>*/
    pub fn post_setup_intents_intent_confirm(
        &self,
        intent: &str,
    ) -> request::PostSetupIntentsIntentConfirmRequest {
        request::PostSetupIntentsIntentConfirmRequest {
            http_client: &self,
            intent: intent.to_owned(),
        }
    }
    ///<p>Verifies microdeposits on a SetupIntent object.</p>
    pub fn post_setup_intents_intent_verify_microdeposits(
        &self,
        intent: &str,
    ) -> request::PostSetupIntentsIntentVerifyMicrodepositsRequest {
        request::PostSetupIntentsIntentVerifyMicrodepositsRequest {
            http_client: &self,
            intent: intent.to_owned(),
        }
    }
    ///<p>Returns a list of your shipping rates.</p>
    pub fn get_shipping_rates(&self) -> request::GetShippingRatesRequest {
        request::GetShippingRatesRequest {
            http_client: &self,
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
    pub fn post_shipping_rates(&self) -> request::PostShippingRatesRequest {
        request::PostShippingRatesRequest {
            http_client: &self,
        }
    }
    ///<p>Returns the shipping rate object with the given ID.</p>
    pub fn get_shipping_rates_shipping_rate_token(
        &self,
        shipping_rate_token: &str,
    ) -> request::GetShippingRatesShippingRateTokenRequest {
        request::GetShippingRatesShippingRateTokenRequest {
            http_client: &self,
            expand: None,
            shipping_rate_token: shipping_rate_token.to_owned(),
        }
    }
    ///<p>Updates an existing shipping rate object.</p>
    pub fn post_shipping_rates_shipping_rate_token(
        &self,
        shipping_rate_token: &str,
    ) -> request::PostShippingRatesShippingRateTokenRequest {
        request::PostShippingRatesShippingRateTokenRequest {
            http_client: &self,
            shipping_rate_token: shipping_rate_token.to_owned(),
        }
    }
    ///<p>Returns a list of scheduled query runs.</p>
    pub fn get_sigma_scheduled_query_runs(
        &self,
    ) -> request::GetSigmaScheduledQueryRunsRequest {
        request::GetSigmaScheduledQueryRunsRequest {
            http_client: &self,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
        }
    }
    ///<p>Retrieves the details of an scheduled query run.</p>
    pub fn get_sigma_scheduled_query_runs_scheduled_query_run(
        &self,
        scheduled_query_run: &str,
    ) -> request::GetSigmaScheduledQueryRunsScheduledQueryRunRequest {
        request::GetSigmaScheduledQueryRunsScheduledQueryRunRequest {
            http_client: &self,
            expand: None,
            scheduled_query_run: scheduled_query_run.to_owned(),
        }
    }
    ///<p>Creates a new source object.</p>
    pub fn post_sources(&self) -> request::PostSourcesRequest {
        request::PostSourcesRequest {
            http_client: &self,
        }
    }
    ///<p>Retrieves an existing source object. Supply the unique source ID from a source creation request and Stripe will return the corresponding up-to-date source object information.</p>
    pub fn get_sources_source(&self, source: &str) -> request::GetSourcesSourceRequest {
        request::GetSourcesSourceRequest {
            http_client: &self,
            client_secret: None,
            expand: None,
            source: source.to_owned(),
        }
    }
    /**<p>Updates the specified source by setting the values of the parameters passed. Any parameters not provided will be left unchanged.</p>

<p>This request accepts the <code>metadata</code> and <code>owner</code> as arguments. It is also possible to update type specific information for selected payment methods. Please refer to our <a href="/docs/sources">payment method guides</a> for more detail.</p>*/
    pub fn post_sources_source(
        &self,
        source: &str,
    ) -> request::PostSourcesSourceRequest {
        request::PostSourcesSourceRequest {
            http_client: &self,
            source: source.to_owned(),
        }
    }
    ///<p>Retrieves a new Source MandateNotification.</p>
    pub fn get_sources_source_mandate_notifications_mandate_notification(
        &self,
        mandate_notification: &str,
        source: &str,
    ) -> request::GetSourcesSourceMandateNotificationsMandateNotificationRequest {
        request::GetSourcesSourceMandateNotificationsMandateNotificationRequest {
            http_client: &self,
            expand: None,
            mandate_notification: mandate_notification.to_owned(),
            source: source.to_owned(),
        }
    }
    ///<p>List source transactions for a given source.</p>
    pub fn get_sources_source_source_transactions(
        &self,
        source: &str,
    ) -> request::GetSourcesSourceSourceTransactionsRequest {
        request::GetSourcesSourceSourceTransactionsRequest {
            http_client: &self,
            ending_before: None,
            expand: None,
            limit: None,
            source: source.to_owned(),
            starting_after: None,
        }
    }
    ///<p>Retrieve an existing source transaction object. Supply the unique source ID from a source creation request and the source transaction ID and Stripe will return the corresponding up-to-date source object information.</p>
    pub fn get_sources_source_source_transactions_source_transaction(
        &self,
        source: &str,
        source_transaction: &str,
    ) -> request::GetSourcesSourceSourceTransactionsSourceTransactionRequest {
        request::GetSourcesSourceSourceTransactionsSourceTransactionRequest {
            http_client: &self,
            expand: None,
            source: source.to_owned(),
            source_transaction: source_transaction.to_owned(),
        }
    }
    ///<p>Verify a given source.</p>
    pub fn post_sources_source_verify(
        &self,
        source: &str,
    ) -> request::PostSourcesSourceVerifyRequest {
        request::PostSourcesSourceVerifyRequest {
            http_client: &self,
            source: source.to_owned(),
        }
    }
    ///<p>Returns a list of your subscription items for a given subscription.</p>
    pub fn get_subscription_items(
        &self,
        subscription: &str,
    ) -> request::GetSubscriptionItemsRequest {
        request::GetSubscriptionItemsRequest {
            http_client: &self,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
            subscription: subscription.to_owned(),
        }
    }
    ///<p>Adds a new item to an existing subscription. No existing items will be changed or replaced.</p>
    pub fn post_subscription_items(&self) -> request::PostSubscriptionItemsRequest {
        request::PostSubscriptionItemsRequest {
            http_client: &self,
        }
    }
    ///<p>Retrieves the subscription item with the given ID.</p>
    pub fn get_subscription_items_item(
        &self,
        item: &str,
    ) -> request::GetSubscriptionItemsItemRequest {
        request::GetSubscriptionItemsItemRequest {
            http_client: &self,
            expand: None,
            item: item.to_owned(),
        }
    }
    ///<p>Updates the plan or quantity of an item on a current subscription.</p>
    pub fn post_subscription_items_item(
        &self,
        item: &str,
    ) -> request::PostSubscriptionItemsItemRequest {
        request::PostSubscriptionItemsItemRequest {
            http_client: &self,
            item: item.to_owned(),
        }
    }
    ///<p>Deletes an item from the subscription. Removing a subscription item from a subscription will not cancel the subscription.</p>
    pub fn delete_subscription_items_item(
        &self,
        item: &str,
    ) -> request::DeleteSubscriptionItemsItemRequest {
        request::DeleteSubscriptionItemsItemRequest {
            http_client: &self,
            item: item.to_owned(),
        }
    }
    /**<p>For the specified subscription item, returns a list of summary objects. Each object in the list provides usage information that’s been summarized from multiple usage records and over a subscription billing period (e.g., 15 usage records in the month of September).</p>

<p>The list is sorted in reverse-chronological order (newest first). The first list item represents the most current usage period that hasn’t ended yet. Since new usage records can still be added, the returned summary information for the subscription item’s ID should be seen as unstable until the subscription billing period ends.</p>*/
    pub fn get_subscription_items_subscription_item_usage_record_summaries(
        &self,
        subscription_item: &str,
    ) -> request::GetSubscriptionItemsSubscriptionItemUsageRecordSummariesRequest {
        request::GetSubscriptionItemsSubscriptionItemUsageRecordSummariesRequest {
            http_client: &self,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
            subscription_item: subscription_item.to_owned(),
        }
    }
    /**<p>Creates a usage record for a specified subscription item and date, and fills it with a quantity.</p>

<p>Usage records provide <code>quantity</code> information that Stripe uses to track how much a customer is using your service. With usage information and the pricing model set up by the <a href="https://stripe.com/docs/billing/subscriptions/metered-billing">metered billing</a> plan, Stripe helps you send accurate invoices to your customers.</p>

<p>The default calculation for usage is to add up all the <code>quantity</code> values of the usage records within a billing period. You can change this default behavior with the billing plan’s <code>aggregate_usage</code> <a href="/docs/api/plans/create#create_plan-aggregate_usage">parameter</a>. When there is more than one usage record with the same timestamp, Stripe adds the <code>quantity</code> values together. In most cases, this is the desired resolution, however, you can change this behavior with the <code>action</code> parameter.</p>

<p>The default pricing model for metered billing is <a href="/docs/api/plans/object#plan_object-billing_scheme">per-unit pricing</a>. For finer granularity, you can configure metered billing to have a <a href="https://stripe.com/docs/billing/subscriptions/tiers">tiered pricing</a> model.</p>*/
    pub fn post_subscription_items_subscription_item_usage_records(
        &self,
        subscription_item: &str,
    ) -> request::PostSubscriptionItemsSubscriptionItemUsageRecordsRequest {
        request::PostSubscriptionItemsSubscriptionItemUsageRecordsRequest {
            http_client: &self,
            subscription_item: subscription_item.to_owned(),
        }
    }
    ///<p>Retrieves the list of your subscription schedules.</p>
    pub fn get_subscription_schedules(
        &self,
    ) -> request::GetSubscriptionSchedulesRequest {
        request::GetSubscriptionSchedulesRequest {
            http_client: &self,
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
    ) -> request::PostSubscriptionSchedulesRequest {
        request::PostSubscriptionSchedulesRequest {
            http_client: &self,
        }
    }
    ///<p>Retrieves the details of an existing subscription schedule. You only need to supply the unique subscription schedule identifier that was returned upon subscription schedule creation.</p>
    pub fn get_subscription_schedules_schedule(
        &self,
        schedule: &str,
    ) -> request::GetSubscriptionSchedulesScheduleRequest {
        request::GetSubscriptionSchedulesScheduleRequest {
            http_client: &self,
            expand: None,
            schedule: schedule.to_owned(),
        }
    }
    ///<p>Updates an existing subscription schedule.</p>
    pub fn post_subscription_schedules_schedule(
        &self,
        schedule: &str,
    ) -> request::PostSubscriptionSchedulesScheduleRequest {
        request::PostSubscriptionSchedulesScheduleRequest {
            http_client: &self,
            schedule: schedule.to_owned(),
        }
    }
    ///<p>Cancels a subscription schedule and its associated subscription immediately (if the subscription schedule has an active subscription). A subscription schedule can only be canceled if its status is <code>not_started</code> or <code>active</code>.</p>
    pub fn post_subscription_schedules_schedule_cancel(
        &self,
        schedule: &str,
    ) -> request::PostSubscriptionSchedulesScheduleCancelRequest {
        request::PostSubscriptionSchedulesScheduleCancelRequest {
            http_client: &self,
            schedule: schedule.to_owned(),
        }
    }
    ///<p>Releases the subscription schedule immediately, which will stop scheduling of its phases, but leave any existing subscription in place. A schedule can only be released if its status is <code>not_started</code> or <code>active</code>. If the subscription schedule is currently associated with a subscription, releasing it will remove its <code>subscription</code> property and set the subscription’s ID to the <code>released_subscription</code> property.</p>
    pub fn post_subscription_schedules_schedule_release(
        &self,
        schedule: &str,
    ) -> request::PostSubscriptionSchedulesScheduleReleaseRequest {
        request::PostSubscriptionSchedulesScheduleReleaseRequest {
            http_client: &self,
            schedule: schedule.to_owned(),
        }
    }
    ///<p>By default, returns a list of subscriptions that have not been canceled. In order to list canceled subscriptions, specify <code>status=canceled</code>.</p>
    pub fn get_subscriptions(&self) -> request::GetSubscriptionsRequest {
        request::GetSubscriptionsRequest {
            http_client: &self,
            automatic_tax: None,
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
    pub fn post_subscriptions(&self) -> request::PostSubscriptionsRequest {
        request::PostSubscriptionsRequest {
            http_client: &self,
        }
    }
    /**<p>Search for subscriptions you’ve previously created using Stripe’s <a href="/docs/search#search-query-language">Search Query Language</a>.
Don’t use search in read-after-write flows where strict consistency is necessary. Under normal operating
conditions, data is searchable in less than a minute. Occasionally, propagation of new or updated data can be up
to an hour behind during outages. Search functionality is not available to merchants in India.</p>*/
    pub fn get_subscriptions_search(
        &self,
        query: &str,
    ) -> request::GetSubscriptionsSearchRequest {
        request::GetSubscriptionsSearchRequest {
            http_client: &self,
            expand: None,
            limit: None,
            page: None,
            query: query.to_owned(),
        }
    }
    ///<p>Retrieves the subscription with the given ID.</p>
    pub fn get_subscriptions_subscription_exposed_id(
        &self,
        subscription_exposed_id: &str,
    ) -> request::GetSubscriptionsSubscriptionExposedIdRequest {
        request::GetSubscriptionsSubscriptionExposedIdRequest {
            http_client: &self,
            expand: None,
            subscription_exposed_id: subscription_exposed_id.to_owned(),
        }
    }
    /**<p>Updates an existing subscription to match the specified parameters.
When changing prices or quantities, we optionally prorate the price we charge next month to make up for any price changes.
To preview how the proration is calculated, use the <a href="/docs/api/invoices/upcoming">upcoming invoice</a> endpoint.</p>

<p>By default, we prorate subscription changes. For example, if a customer signs up on May 1 for a <currency>100</currency> price, they’ll be billed <currency>100</currency> immediately. If on May 15 they switch to a <currency>200</currency> price, then on June 1 they’ll be billed <currency>250</currency> (<currency>200</currency> for a renewal of her subscription, plus a <currency>50</currency> prorating adjustment for half of the previous month’s <currency>100</currency> difference). Similarly, a downgrade generates a credit that is applied to the next invoice. We also prorate when you make quantity changes.</p>

<p>Switching prices does not normally change the billing date or generate an immediate charge unless:</p>

<ul>
<li>The billing interval is changed (for example, from monthly to yearly).</li>
<li>The subscription moves from free to paid, or paid to free.</li>
<li>A trial starts or ends.</li>
</ul>

<p>In these cases, we apply a credit for the unused time on the previous price, immediately charge the customer using the new price, and reset the billing date.</p>

<p>If you want to charge for an upgrade immediately, pass <code>proration_behavior</code> as <code>always_invoice</code> to create prorations, automatically invoice the customer for those proration adjustments, and attempt to collect payment. If you pass <code>create_prorations</code>, the prorations are created but not automatically invoiced. If you want to bill the customer for the prorations before the subscription’s renewal date, you need to manually <a href="/docs/api/invoices/create">invoice the customer</a>.</p>

<p>If you don’t want to prorate, set the <code>proration_behavior</code> option to <code>none</code>. With this option, the customer is billed <currency>100</currency> on May 1 and <currency>200</currency> on June 1. Similarly, if you set <code>proration_behavior</code> to <code>none</code> when switching between different billing intervals (for example, from monthly to yearly), we don’t generate any credits for the old subscription’s unused time. We still reset the billing date and bill immediately for the new subscription.</p>

<p>Updating the quantity on a subscription many times in an hour may result in <a href="/docs/rate-limits">rate limiting</a>. If you need to bill for a frequently changing quantity, consider integrating <a href="/docs/billing/subscriptions/usage-based">usage-based billing</a> instead.</p>*/
    pub fn post_subscriptions_subscription_exposed_id(
        &self,
        subscription_exposed_id: &str,
    ) -> request::PostSubscriptionsSubscriptionExposedIdRequest {
        request::PostSubscriptionsSubscriptionExposedIdRequest {
            http_client: &self,
            subscription_exposed_id: subscription_exposed_id.to_owned(),
        }
    }
    /**<p>Cancels a customer’s subscription immediately. The customer will not be charged again for the subscription.</p>

<p>Note, however, that any pending invoice items that you’ve created will still be charged for at the end of the period, unless manually <a href="#delete_invoiceitem">deleted</a>. If you’ve set the subscription to cancel at the end of the period, any pending prorations will also be left in place and collected at the end of the period. But if the subscription is set to cancel immediately, pending prorations will be removed.</p>

<p>By default, upon subscription cancellation, Stripe will stop automatic collection of all finalized invoices for the customer. This is intended to prevent unexpected payment attempts after the customer has canceled a subscription. However, you can resume automatic collection of the invoices manually after subscription cancellation to have us proceed. Or, you could check for unpaid invoices before allowing the customer to cancel the subscription at all.</p>*/
    pub fn delete_subscriptions_subscription_exposed_id(
        &self,
        subscription_exposed_id: &str,
    ) -> request::DeleteSubscriptionsSubscriptionExposedIdRequest {
        request::DeleteSubscriptionsSubscriptionExposedIdRequest {
            http_client: &self,
            subscription_exposed_id: subscription_exposed_id.to_owned(),
        }
    }
    ///<p>Removes the currently applied discount on a subscription.</p>
    pub fn delete_subscriptions_subscription_exposed_id_discount(
        &self,
        subscription_exposed_id: &str,
    ) -> request::DeleteSubscriptionsSubscriptionExposedIdDiscountRequest {
        request::DeleteSubscriptionsSubscriptionExposedIdDiscountRequest {
            http_client: &self,
            subscription_exposed_id: subscription_exposed_id.to_owned(),
        }
    }
    ///<p>Initiates resumption of a paused subscription, optionally resetting the billing cycle anchor and creating prorations. If a resumption invoice is generated, it must be paid or marked uncollectible before the subscription will be unpaused. If payment succeeds the subscription will become <code>active</code>, and if payment fails the subscription will be <code>past_due</code>. The resumption invoice will void automatically if not paid by the expiration date.</p>
    pub fn post_subscriptions_subscription_resume(
        &self,
        subscription: &str,
    ) -> request::PostSubscriptionsSubscriptionResumeRequest {
        request::PostSubscriptionsSubscriptionResumeRequest {
            http_client: &self,
            subscription: subscription.to_owned(),
        }
    }
    ///<p>Calculates tax based on input and returns a Tax <code>Calculation</code> object.</p>
    pub fn post_tax_calculations(&self) -> request::PostTaxCalculationsRequest {
        request::PostTaxCalculationsRequest {
            http_client: &self,
        }
    }
    ///<p>Retrieves the line items of a persisted tax calculation as a collection.</p>
    pub fn get_tax_calculations_calculation_line_items(
        &self,
        calculation: &str,
    ) -> request::GetTaxCalculationsCalculationLineItemsRequest {
        request::GetTaxCalculationsCalculationLineItemsRequest {
            http_client: &self,
            calculation: calculation.to_owned(),
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
        }
    }
    ///<p>Returns a list of Tax <code>Registration</code> objects.</p>
    pub fn get_tax_registrations(&self) -> request::GetTaxRegistrationsRequest {
        request::GetTaxRegistrationsRequest {
            http_client: &self,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
            status: None,
        }
    }
    ///<p>Creates a new Tax <code>Registration</code> object.</p>
    pub fn post_tax_registrations(&self) -> request::PostTaxRegistrationsRequest {
        request::PostTaxRegistrationsRequest {
            http_client: &self,
        }
    }
    /**<p>Updates an existing Tax <code>Registration</code> object.</p>

<p>A registration cannot be deleted after it has been created. If you wish to end a registration you may do so by setting <code>expires_at</code>.</p>*/
    pub fn post_tax_registrations_id(
        &self,
        id: &str,
    ) -> request::PostTaxRegistrationsIdRequest {
        request::PostTaxRegistrationsIdRequest {
            http_client: &self,
            id: id.to_owned(),
        }
    }
    ///<p>Retrieves Tax <code>Settings</code> for a merchant.</p>
    pub fn get_tax_settings(&self) -> request::GetTaxSettingsRequest {
        request::GetTaxSettingsRequest {
            http_client: &self,
            expand: None,
        }
    }
    ///<p>Updates Tax <code>Settings</code> parameters used in tax calculations. All parameters are editable but none can be removed once set.</p>
    pub fn post_tax_settings(&self) -> request::PostTaxSettingsRequest {
        request::PostTaxSettingsRequest {
            http_client: &self,
        }
    }
    ///<p>Creates a Tax <code>Transaction</code> from a calculation.</p>
    pub fn post_tax_transactions_create_from_calculation(
        &self,
    ) -> request::PostTaxTransactionsCreateFromCalculationRequest {
        request::PostTaxTransactionsCreateFromCalculationRequest {
            http_client: &self,
        }
    }
    ///<p>Partially or fully reverses a previously created <code>Transaction</code>.</p>
    pub fn post_tax_transactions_create_reversal(
        &self,
    ) -> request::PostTaxTransactionsCreateReversalRequest {
        request::PostTaxTransactionsCreateReversalRequest {
            http_client: &self,
        }
    }
    ///<p>Retrieves a Tax <code>Transaction</code> object.</p>
    pub fn get_tax_transactions_transaction(
        &self,
        transaction: &str,
    ) -> request::GetTaxTransactionsTransactionRequest {
        request::GetTaxTransactionsTransactionRequest {
            http_client: &self,
            expand: None,
            transaction: transaction.to_owned(),
        }
    }
    ///<p>Retrieves the line items of a committed standalone transaction as a collection.</p>
    pub fn get_tax_transactions_transaction_line_items(
        &self,
        transaction: &str,
    ) -> request::GetTaxTransactionsTransactionLineItemsRequest {
        request::GetTaxTransactionsTransactionLineItemsRequest {
            http_client: &self,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
            transaction: transaction.to_owned(),
        }
    }
    ///<p>A list of <a href="https://stripe.com/docs/tax/tax-categories">all tax codes available</a> to add to Products in order to allow specific tax calculations.</p>
    pub fn get_tax_codes(&self) -> request::GetTaxCodesRequest {
        request::GetTaxCodesRequest {
            http_client: &self,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
        }
    }
    ///<p>Retrieves the details of an existing tax code. Supply the unique tax code ID and Stripe will return the corresponding tax code information.</p>
    pub fn get_tax_codes_id(&self, id: &str) -> request::GetTaxCodesIdRequest {
        request::GetTaxCodesIdRequest {
            http_client: &self,
            expand: None,
            id: id.to_owned(),
        }
    }
    ///<p>Returns a list of your tax rates. Tax rates are returned sorted by creation date, with the most recently created tax rates appearing first.</p>
    pub fn get_tax_rates(&self) -> request::GetTaxRatesRequest {
        request::GetTaxRatesRequest {
            http_client: &self,
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
    pub fn post_tax_rates(&self) -> request::PostTaxRatesRequest {
        request::PostTaxRatesRequest {
            http_client: &self,
        }
    }
    ///<p>Retrieves a tax rate with the given ID</p>
    pub fn get_tax_rates_tax_rate(
        &self,
        tax_rate: &str,
    ) -> request::GetTaxRatesTaxRateRequest {
        request::GetTaxRatesTaxRateRequest {
            http_client: &self,
            expand: None,
            tax_rate: tax_rate.to_owned(),
        }
    }
    ///<p>Updates an existing tax rate.</p>
    pub fn post_tax_rates_tax_rate(
        &self,
        tax_rate: &str,
    ) -> request::PostTaxRatesTaxRateRequest {
        request::PostTaxRatesTaxRateRequest {
            http_client: &self,
            tax_rate: tax_rate.to_owned(),
        }
    }
    ///<p>Returns a list of <code>Configuration</code> objects.</p>
    pub fn get_terminal_configurations(
        &self,
    ) -> request::GetTerminalConfigurationsRequest {
        request::GetTerminalConfigurationsRequest {
            http_client: &self,
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
    ) -> request::PostTerminalConfigurationsRequest {
        request::PostTerminalConfigurationsRequest {
            http_client: &self,
        }
    }
    ///<p>Retrieves a <code>Configuration</code> object.</p>
    pub fn get_terminal_configurations_configuration(
        &self,
        configuration: &str,
    ) -> request::GetTerminalConfigurationsConfigurationRequest {
        request::GetTerminalConfigurationsConfigurationRequest {
            http_client: &self,
            configuration: configuration.to_owned(),
            expand: None,
        }
    }
    ///<p>Updates a new <code>Configuration</code> object.</p>
    pub fn post_terminal_configurations_configuration(
        &self,
        configuration: &str,
    ) -> request::PostTerminalConfigurationsConfigurationRequest {
        request::PostTerminalConfigurationsConfigurationRequest {
            http_client: &self,
            configuration: configuration.to_owned(),
        }
    }
    ///<p>Deletes a <code>Configuration</code> object.</p>
    pub fn delete_terminal_configurations_configuration(
        &self,
        configuration: &str,
    ) -> request::DeleteTerminalConfigurationsConfigurationRequest {
        request::DeleteTerminalConfigurationsConfigurationRequest {
            http_client: &self,
            configuration: configuration.to_owned(),
        }
    }
    ///<p>To connect to a reader the Stripe Terminal SDK needs to retrieve a short-lived connection token from Stripe, proxied through your server. On your backend, add an endpoint that creates and returns a connection token.</p>
    pub fn post_terminal_connection_tokens(
        &self,
    ) -> request::PostTerminalConnectionTokensRequest {
        request::PostTerminalConnectionTokensRequest {
            http_client: &self,
        }
    }
    ///<p>Returns a list of <code>Location</code> objects.</p>
    pub fn get_terminal_locations(&self) -> request::GetTerminalLocationsRequest {
        request::GetTerminalLocationsRequest {
            http_client: &self,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
        }
    }
    /**<p>Creates a new <code>Location</code> object.
For further details, including which address fields are required in each country, see the <a href="/docs/terminal/fleet/locations">Manage locations</a> guide.</p>*/
    pub fn post_terminal_locations(&self) -> request::PostTerminalLocationsRequest {
        request::PostTerminalLocationsRequest {
            http_client: &self,
        }
    }
    ///<p>Retrieves a <code>Location</code> object.</p>
    pub fn get_terminal_locations_location(
        &self,
        location: &str,
    ) -> request::GetTerminalLocationsLocationRequest {
        request::GetTerminalLocationsLocationRequest {
            http_client: &self,
            expand: None,
            location: location.to_owned(),
        }
    }
    ///<p>Updates a <code>Location</code> object by setting the values of the parameters passed. Any parameters not provided will be left unchanged.</p>
    pub fn post_terminal_locations_location(
        &self,
        location: &str,
    ) -> request::PostTerminalLocationsLocationRequest {
        request::PostTerminalLocationsLocationRequest {
            http_client: &self,
            location: location.to_owned(),
        }
    }
    ///<p>Deletes a <code>Location</code> object.</p>
    pub fn delete_terminal_locations_location(
        &self,
        location: &str,
    ) -> request::DeleteTerminalLocationsLocationRequest {
        request::DeleteTerminalLocationsLocationRequest {
            http_client: &self,
            location: location.to_owned(),
        }
    }
    ///<p>Returns a list of <code>Reader</code> objects.</p>
    pub fn get_terminal_readers(&self) -> request::GetTerminalReadersRequest {
        request::GetTerminalReadersRequest {
            http_client: &self,
            device_type: None,
            ending_before: None,
            expand: None,
            limit: None,
            location: None,
            serial_number: None,
            starting_after: None,
            status: None,
        }
    }
    ///<p>Creates a new <code>Reader</code> object.</p>
    pub fn post_terminal_readers(&self) -> request::PostTerminalReadersRequest {
        request::PostTerminalReadersRequest {
            http_client: &self,
        }
    }
    ///<p>Retrieves a <code>Reader</code> object.</p>
    pub fn get_terminal_readers_reader(
        &self,
        reader: &str,
    ) -> request::GetTerminalReadersReaderRequest {
        request::GetTerminalReadersReaderRequest {
            http_client: &self,
            expand: None,
            reader: reader.to_owned(),
        }
    }
    ///<p>Updates a <code>Reader</code> object by setting the values of the parameters passed. Any parameters not provided will be left unchanged.</p>
    pub fn post_terminal_readers_reader(
        &self,
        reader: &str,
    ) -> request::PostTerminalReadersReaderRequest {
        request::PostTerminalReadersReaderRequest {
            http_client: &self,
            reader: reader.to_owned(),
        }
    }
    ///<p>Deletes a <code>Reader</code> object.</p>
    pub fn delete_terminal_readers_reader(
        &self,
        reader: &str,
    ) -> request::DeleteTerminalReadersReaderRequest {
        request::DeleteTerminalReadersReaderRequest {
            http_client: &self,
            reader: reader.to_owned(),
        }
    }
    ///<p>Cancels the current reader action.</p>
    pub fn post_terminal_readers_reader_cancel_action(
        &self,
        reader: &str,
    ) -> request::PostTerminalReadersReaderCancelActionRequest {
        request::PostTerminalReadersReaderCancelActionRequest {
            http_client: &self,
            reader: reader.to_owned(),
        }
    }
    ///<p>Initiates a payment flow on a Reader.</p>
    pub fn post_terminal_readers_reader_process_payment_intent(
        &self,
        reader: &str,
    ) -> request::PostTerminalReadersReaderProcessPaymentIntentRequest {
        request::PostTerminalReadersReaderProcessPaymentIntentRequest {
            http_client: &self,
            reader: reader.to_owned(),
        }
    }
    ///<p>Initiates a setup intent flow on a Reader.</p>
    pub fn post_terminal_readers_reader_process_setup_intent(
        &self,
        reader: &str,
    ) -> request::PostTerminalReadersReaderProcessSetupIntentRequest {
        request::PostTerminalReadersReaderProcessSetupIntentRequest {
            http_client: &self,
            reader: reader.to_owned(),
        }
    }
    ///<p>Initiates a refund on a Reader</p>
    pub fn post_terminal_readers_reader_refund_payment(
        &self,
        reader: &str,
    ) -> request::PostTerminalReadersReaderRefundPaymentRequest {
        request::PostTerminalReadersReaderRefundPaymentRequest {
            http_client: &self,
            reader: reader.to_owned(),
        }
    }
    ///<p>Sets reader display to show cart details.</p>
    pub fn post_terminal_readers_reader_set_reader_display(
        &self,
        reader: &str,
    ) -> request::PostTerminalReadersReaderSetReaderDisplayRequest {
        request::PostTerminalReadersReaderSetReaderDisplayRequest {
            http_client: &self,
            reader: reader.to_owned(),
        }
    }
    ///<p>Create an incoming testmode bank transfer</p>
    pub fn post_test_helpers_customers_customer_fund_cash_balance(
        &self,
        customer: &str,
    ) -> request::PostTestHelpersCustomersCustomerFundCashBalanceRequest {
        request::PostTestHelpersCustomersCustomerFundCashBalanceRequest {
            http_client: &self,
            customer: customer.to_owned(),
        }
    }
    ///<p>Create a test-mode authorization.</p>
    pub fn post_test_helpers_issuing_authorizations(
        &self,
    ) -> request::PostTestHelpersIssuingAuthorizationsRequest {
        request::PostTestHelpersIssuingAuthorizationsRequest {
            http_client: &self,
        }
    }
    ///<p>Capture a test-mode authorization.</p>
    pub fn post_test_helpers_issuing_authorizations_authorization_capture(
        &self,
        authorization: &str,
    ) -> request::PostTestHelpersIssuingAuthorizationsAuthorizationCaptureRequest {
        request::PostTestHelpersIssuingAuthorizationsAuthorizationCaptureRequest {
            http_client: &self,
            authorization: authorization.to_owned(),
        }
    }
    ///<p>Expire a test-mode Authorization.</p>
    pub fn post_test_helpers_issuing_authorizations_authorization_expire(
        &self,
        authorization: &str,
    ) -> request::PostTestHelpersIssuingAuthorizationsAuthorizationExpireRequest {
        request::PostTestHelpersIssuingAuthorizationsAuthorizationExpireRequest {
            http_client: &self,
            authorization: authorization.to_owned(),
        }
    }
    ///<p>Increment a test-mode Authorization.</p>
    pub fn post_test_helpers_issuing_authorizations_authorization_increment(
        &self,
        authorization: &str,
    ) -> request::PostTestHelpersIssuingAuthorizationsAuthorizationIncrementRequest {
        request::PostTestHelpersIssuingAuthorizationsAuthorizationIncrementRequest {
            http_client: &self,
            authorization: authorization.to_owned(),
        }
    }
    ///<p>Reverse a test-mode Authorization.</p>
    pub fn post_test_helpers_issuing_authorizations_authorization_reverse(
        &self,
        authorization: &str,
    ) -> request::PostTestHelpersIssuingAuthorizationsAuthorizationReverseRequest {
        request::PostTestHelpersIssuingAuthorizationsAuthorizationReverseRequest {
            http_client: &self,
            authorization: authorization.to_owned(),
        }
    }
    ///<p>Updates the shipping status of the specified Issuing <code>Card</code> object to <code>delivered</code>.</p>
    pub fn post_test_helpers_issuing_cards_card_shipping_deliver(
        &self,
        card: &str,
    ) -> request::PostTestHelpersIssuingCardsCardShippingDeliverRequest {
        request::PostTestHelpersIssuingCardsCardShippingDeliverRequest {
            http_client: &self,
            card: card.to_owned(),
        }
    }
    ///<p>Updates the shipping status of the specified Issuing <code>Card</code> object to <code>failure</code>.</p>
    pub fn post_test_helpers_issuing_cards_card_shipping_fail(
        &self,
        card: &str,
    ) -> request::PostTestHelpersIssuingCardsCardShippingFailRequest {
        request::PostTestHelpersIssuingCardsCardShippingFailRequest {
            http_client: &self,
            card: card.to_owned(),
        }
    }
    ///<p>Updates the shipping status of the specified Issuing <code>Card</code> object to <code>returned</code>.</p>
    pub fn post_test_helpers_issuing_cards_card_shipping_return(
        &self,
        card: &str,
    ) -> request::PostTestHelpersIssuingCardsCardShippingReturnRequest {
        request::PostTestHelpersIssuingCardsCardShippingReturnRequest {
            http_client: &self,
            card: card.to_owned(),
        }
    }
    ///<p>Updates the shipping status of the specified Issuing <code>Card</code> object to <code>shipped</code>.</p>
    pub fn post_test_helpers_issuing_cards_card_shipping_ship(
        &self,
        card: &str,
    ) -> request::PostTestHelpersIssuingCardsCardShippingShipRequest {
        request::PostTestHelpersIssuingCardsCardShippingShipRequest {
            http_client: &self,
            card: card.to_owned(),
        }
    }
    ///<p>Allows the user to capture an arbitrary amount, also known as a forced capture.</p>
    pub fn post_test_helpers_issuing_transactions_create_force_capture(
        &self,
    ) -> request::PostTestHelpersIssuingTransactionsCreateForceCaptureRequest {
        request::PostTestHelpersIssuingTransactionsCreateForceCaptureRequest {
            http_client: &self,
        }
    }
    ///<p>Allows the user to refund an arbitrary amount, also known as a unlinked refund.</p>
    pub fn post_test_helpers_issuing_transactions_create_unlinked_refund(
        &self,
    ) -> request::PostTestHelpersIssuingTransactionsCreateUnlinkedRefundRequest {
        request::PostTestHelpersIssuingTransactionsCreateUnlinkedRefundRequest {
            http_client: &self,
        }
    }
    ///<p>Refund a test-mode Transaction.</p>
    pub fn post_test_helpers_issuing_transactions_transaction_refund(
        &self,
        transaction: &str,
    ) -> request::PostTestHelpersIssuingTransactionsTransactionRefundRequest {
        request::PostTestHelpersIssuingTransactionsTransactionRefundRequest {
            http_client: &self,
            transaction: transaction.to_owned(),
        }
    }
    ///<p>Expire a refund with a status of <code>requires_action</code>.</p>
    pub fn post_test_helpers_refunds_refund_expire(
        &self,
        refund: &str,
    ) -> request::PostTestHelpersRefundsRefundExpireRequest {
        request::PostTestHelpersRefundsRefundExpireRequest {
            http_client: &self,
            refund: refund.to_owned(),
        }
    }
    ///<p>Presents a payment method on a simulated reader. Can be used to simulate accepting a payment, saving a card or refunding a transaction.</p>
    pub fn post_test_helpers_terminal_readers_reader_present_payment_method(
        &self,
        reader: &str,
    ) -> request::PostTestHelpersTerminalReadersReaderPresentPaymentMethodRequest {
        request::PostTestHelpersTerminalReadersReaderPresentPaymentMethodRequest {
            http_client: &self,
            reader: reader.to_owned(),
        }
    }
    ///<p>Returns a list of your test clocks.</p>
    pub fn get_test_helpers_test_clocks(
        &self,
    ) -> request::GetTestHelpersTestClocksRequest {
        request::GetTestHelpersTestClocksRequest {
            http_client: &self,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
        }
    }
    ///<p>Creates a new test clock that can be attached to new customers and quotes.</p>
    pub fn post_test_helpers_test_clocks(
        &self,
    ) -> request::PostTestHelpersTestClocksRequest {
        request::PostTestHelpersTestClocksRequest {
            http_client: &self,
        }
    }
    ///<p>Retrieves a test clock.</p>
    pub fn get_test_helpers_test_clocks_test_clock(
        &self,
        test_clock: &str,
    ) -> request::GetTestHelpersTestClocksTestClockRequest {
        request::GetTestHelpersTestClocksTestClockRequest {
            http_client: &self,
            expand: None,
            test_clock: test_clock.to_owned(),
        }
    }
    ///<p>Deletes a test clock.</p>
    pub fn delete_test_helpers_test_clocks_test_clock(
        &self,
        test_clock: &str,
    ) -> request::DeleteTestHelpersTestClocksTestClockRequest {
        request::DeleteTestHelpersTestClocksTestClockRequest {
            http_client: &self,
            test_clock: test_clock.to_owned(),
        }
    }
    ///<p>Starts advancing a test clock to a specified time in the future. Advancement is done when status changes to <code>Ready</code>.</p>
    pub fn post_test_helpers_test_clocks_test_clock_advance(
        &self,
        test_clock: &str,
    ) -> request::PostTestHelpersTestClocksTestClockAdvanceRequest {
        request::PostTestHelpersTestClocksTestClockAdvanceRequest {
            http_client: &self,
            test_clock: test_clock.to_owned(),
        }
    }
    ///<p>Transitions a test mode created InboundTransfer to the <code>failed</code> status. The InboundTransfer must already be in the <code>processing</code> state.</p>
    pub fn post_test_helpers_treasury_inbound_transfers_id_fail(
        &self,
        id: &str,
    ) -> request::PostTestHelpersTreasuryInboundTransfersIdFailRequest {
        request::PostTestHelpersTreasuryInboundTransfersIdFailRequest {
            http_client: &self,
            id: id.to_owned(),
        }
    }
    ///<p>Marks the test mode InboundTransfer object as returned and links the InboundTransfer to a ReceivedDebit. The InboundTransfer must already be in the <code>succeeded</code> state.</p>
    pub fn post_test_helpers_treasury_inbound_transfers_id_return(
        &self,
        id: &str,
    ) -> request::PostTestHelpersTreasuryInboundTransfersIdReturnRequest {
        request::PostTestHelpersTreasuryInboundTransfersIdReturnRequest {
            http_client: &self,
            id: id.to_owned(),
        }
    }
    ///<p>Transitions a test mode created InboundTransfer to the <code>succeeded</code> status. The InboundTransfer must already be in the <code>processing</code> state.</p>
    pub fn post_test_helpers_treasury_inbound_transfers_id_succeed(
        &self,
        id: &str,
    ) -> request::PostTestHelpersTreasuryInboundTransfersIdSucceedRequest {
        request::PostTestHelpersTreasuryInboundTransfersIdSucceedRequest {
            http_client: &self,
            id: id.to_owned(),
        }
    }
    ///<p>Transitions a test mode created OutboundPayment to the <code>failed</code> status. The OutboundPayment must already be in the <code>processing</code> state.</p>
    pub fn post_test_helpers_treasury_outbound_payments_id_fail(
        &self,
        id: &str,
    ) -> request::PostTestHelpersTreasuryOutboundPaymentsIdFailRequest {
        request::PostTestHelpersTreasuryOutboundPaymentsIdFailRequest {
            http_client: &self,
            id: id.to_owned(),
        }
    }
    ///<p>Transitions a test mode created OutboundPayment to the <code>posted</code> status. The OutboundPayment must already be in the <code>processing</code> state.</p>
    pub fn post_test_helpers_treasury_outbound_payments_id_post(
        &self,
        id: &str,
    ) -> request::PostTestHelpersTreasuryOutboundPaymentsIdPostRequest {
        request::PostTestHelpersTreasuryOutboundPaymentsIdPostRequest {
            http_client: &self,
            id: id.to_owned(),
        }
    }
    ///<p>Transitions a test mode created OutboundPayment to the <code>returned</code> status. The OutboundPayment must already be in the <code>processing</code> state.</p>
    pub fn post_test_helpers_treasury_outbound_payments_id_return(
        &self,
        id: &str,
    ) -> request::PostTestHelpersTreasuryOutboundPaymentsIdReturnRequest {
        request::PostTestHelpersTreasuryOutboundPaymentsIdReturnRequest {
            http_client: &self,
            id: id.to_owned(),
        }
    }
    ///<p>Transitions a test mode created OutboundTransfer to the <code>failed</code> status. The OutboundTransfer must already be in the <code>processing</code> state.</p>
    pub fn post_test_helpers_treasury_outbound_transfers_outbound_transfer_fail(
        &self,
        outbound_transfer: &str,
    ) -> request::PostTestHelpersTreasuryOutboundTransfersOutboundTransferFailRequest {
        request::PostTestHelpersTreasuryOutboundTransfersOutboundTransferFailRequest {
            http_client: &self,
            outbound_transfer: outbound_transfer.to_owned(),
        }
    }
    ///<p>Transitions a test mode created OutboundTransfer to the <code>posted</code> status. The OutboundTransfer must already be in the <code>processing</code> state.</p>
    pub fn post_test_helpers_treasury_outbound_transfers_outbound_transfer_post(
        &self,
        outbound_transfer: &str,
    ) -> request::PostTestHelpersTreasuryOutboundTransfersOutboundTransferPostRequest {
        request::PostTestHelpersTreasuryOutboundTransfersOutboundTransferPostRequest {
            http_client: &self,
            outbound_transfer: outbound_transfer.to_owned(),
        }
    }
    ///<p>Transitions a test mode created OutboundTransfer to the <code>returned</code> status. The OutboundTransfer must already be in the <code>processing</code> state.</p>
    pub fn post_test_helpers_treasury_outbound_transfers_outbound_transfer_return(
        &self,
        outbound_transfer: &str,
    ) -> request::PostTestHelpersTreasuryOutboundTransfersOutboundTransferReturnRequest {
        request::PostTestHelpersTreasuryOutboundTransfersOutboundTransferReturnRequest {
            http_client: &self,
            outbound_transfer: outbound_transfer.to_owned(),
        }
    }
    ///<p>Use this endpoint to simulate a test mode ReceivedCredit initiated by a third party. In live mode, you can’t directly create ReceivedCredits initiated by third parties.</p>
    pub fn post_test_helpers_treasury_received_credits(
        &self,
    ) -> request::PostTestHelpersTreasuryReceivedCreditsRequest {
        request::PostTestHelpersTreasuryReceivedCreditsRequest {
            http_client: &self,
        }
    }
    ///<p>Use this endpoint to simulate a test mode ReceivedDebit initiated by a third party. In live mode, you can’t directly create ReceivedDebits initiated by third parties.</p>
    pub fn post_test_helpers_treasury_received_debits(
        &self,
    ) -> request::PostTestHelpersTreasuryReceivedDebitsRequest {
        request::PostTestHelpersTreasuryReceivedDebitsRequest {
            http_client: &self,
        }
    }
    /**<p>Creates a single-use token that represents a bank account’s details.
You can use this token with any API method in place of a bank account dictionary. You can only use this token once. To do so, attach it to a <a href="#accounts">Custom account</a>.</p>*/
    pub fn post_tokens(&self) -> request::PostTokensRequest {
        request::PostTokensRequest {
            http_client: &self,
        }
    }
    ///<p>Retrieves the token with the given ID.</p>
    pub fn get_tokens_token(&self, token: &str) -> request::GetTokensTokenRequest {
        request::GetTokensTokenRequest {
            http_client: &self,
            expand: None,
            token: token.to_owned(),
        }
    }
    ///<p>Returns a list of top-ups.</p>
    pub fn get_topups(&self) -> request::GetTopupsRequest {
        request::GetTopupsRequest {
            http_client: &self,
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
    pub fn post_topups(&self) -> request::PostTopupsRequest {
        request::PostTopupsRequest {
            http_client: &self,
        }
    }
    ///<p>Retrieves the details of a top-up that has previously been created. Supply the unique top-up ID that was returned from your previous request, and Stripe will return the corresponding top-up information.</p>
    pub fn get_topups_topup(&self, topup: &str) -> request::GetTopupsTopupRequest {
        request::GetTopupsTopupRequest {
            http_client: &self,
            expand: None,
            topup: topup.to_owned(),
        }
    }
    ///<p>Updates the metadata of a top-up. Other top-up details are not editable by design.</p>
    pub fn post_topups_topup(&self, topup: &str) -> request::PostTopupsTopupRequest {
        request::PostTopupsTopupRequest {
            http_client: &self,
            topup: topup.to_owned(),
        }
    }
    ///<p>Cancels a top-up. Only pending top-ups can be canceled.</p>
    pub fn post_topups_topup_cancel(
        &self,
        topup: &str,
    ) -> request::PostTopupsTopupCancelRequest {
        request::PostTopupsTopupCancelRequest {
            http_client: &self,
            topup: topup.to_owned(),
        }
    }
    ///<p>Returns a list of existing transfers sent to connected accounts. The transfers are returned in sorted order, with the most recently created transfers appearing first.</p>
    pub fn get_transfers(&self) -> request::GetTransfersRequest {
        request::GetTransfersRequest {
            http_client: &self,
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
    pub fn post_transfers(&self) -> request::PostTransfersRequest {
        request::PostTransfersRequest {
            http_client: &self,
        }
    }
    ///<p>You can see a list of the reversals belonging to a specific transfer. Note that the 10 most recent reversals are always available by default on the transfer object. If you need more than those 10, you can use this API method and the <code>limit</code> and <code>starting_after</code> parameters to page through additional reversals.</p>
    pub fn get_transfers_id_reversals(
        &self,
        id: &str,
    ) -> request::GetTransfersIdReversalsRequest {
        request::GetTransfersIdReversalsRequest {
            http_client: &self,
            ending_before: None,
            expand: None,
            id: id.to_owned(),
            limit: None,
            starting_after: None,
        }
    }
    /**<p>When you create a new reversal, you must specify a transfer to create it on.</p>

<p>When reversing transfers, you can optionally reverse part of the transfer. You can do so as many times as you wish until the entire transfer has been reversed.</p>

<p>Once entirely reversed, a transfer can’t be reversed again. This method will return an error when called on an already-reversed transfer, or when trying to reverse more money than is left on a transfer.</p>*/
    pub fn post_transfers_id_reversals(
        &self,
        id: &str,
    ) -> request::PostTransfersIdReversalsRequest {
        request::PostTransfersIdReversalsRequest {
            http_client: &self,
            id: id.to_owned(),
        }
    }
    ///<p>Retrieves the details of an existing transfer. Supply the unique transfer ID from either a transfer creation request or the transfer list, and Stripe will return the corresponding transfer information.</p>
    pub fn get_transfers_transfer(
        &self,
        transfer: &str,
    ) -> request::GetTransfersTransferRequest {
        request::GetTransfersTransferRequest {
            http_client: &self,
            expand: None,
            transfer: transfer.to_owned(),
        }
    }
    /**<p>Updates the specified transfer by setting the values of the parameters passed. Any parameters not provided will be left unchanged.</p>

<p>This request accepts only metadata as an argument.</p>*/
    pub fn post_transfers_transfer(
        &self,
        transfer: &str,
    ) -> request::PostTransfersTransferRequest {
        request::PostTransfersTransferRequest {
            http_client: &self,
            transfer: transfer.to_owned(),
        }
    }
    ///<p>By default, you can see the 10 most recent reversals stored directly on the transfer object, but you can also retrieve details about a specific reversal stored on the transfer.</p>
    pub fn get_transfers_transfer_reversals_id(
        &self,
        id: &str,
        transfer: &str,
    ) -> request::GetTransfersTransferReversalsIdRequest {
        request::GetTransfersTransferReversalsIdRequest {
            http_client: &self,
            expand: None,
            id: id.to_owned(),
            transfer: transfer.to_owned(),
        }
    }
    /**<p>Updates the specified reversal by setting the values of the parameters passed. Any parameters not provided will be left unchanged.</p>

<p>This request only accepts metadata and description as arguments.</p>*/
    pub fn post_transfers_transfer_reversals_id(
        &self,
        id: &str,
        transfer: &str,
    ) -> request::PostTransfersTransferReversalsIdRequest {
        request::PostTransfersTransferReversalsIdRequest {
            http_client: &self,
            id: id.to_owned(),
            transfer: transfer.to_owned(),
        }
    }
    ///<p>Returns a list of CreditReversals.</p>
    pub fn get_treasury_credit_reversals(
        &self,
        financial_account: &str,
    ) -> request::GetTreasuryCreditReversalsRequest {
        request::GetTreasuryCreditReversalsRequest {
            http_client: &self,
            ending_before: None,
            expand: None,
            financial_account: financial_account.to_owned(),
            limit: None,
            received_credit: None,
            starting_after: None,
            status: None,
        }
    }
    ///<p>Reverses a ReceivedCredit and creates a CreditReversal object.</p>
    pub fn post_treasury_credit_reversals(
        &self,
    ) -> request::PostTreasuryCreditReversalsRequest {
        request::PostTreasuryCreditReversalsRequest {
            http_client: &self,
        }
    }
    ///<p>Retrieves the details of an existing CreditReversal by passing the unique CreditReversal ID from either the CreditReversal creation request or CreditReversal list</p>
    pub fn get_treasury_credit_reversals_credit_reversal(
        &self,
        credit_reversal: &str,
    ) -> request::GetTreasuryCreditReversalsCreditReversalRequest {
        request::GetTreasuryCreditReversalsCreditReversalRequest {
            http_client: &self,
            credit_reversal: credit_reversal.to_owned(),
            expand: None,
        }
    }
    ///<p>Returns a list of DebitReversals.</p>
    pub fn get_treasury_debit_reversals(
        &self,
        financial_account: &str,
    ) -> request::GetTreasuryDebitReversalsRequest {
        request::GetTreasuryDebitReversalsRequest {
            http_client: &self,
            ending_before: None,
            expand: None,
            financial_account: financial_account.to_owned(),
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
    ) -> request::PostTreasuryDebitReversalsRequest {
        request::PostTreasuryDebitReversalsRequest {
            http_client: &self,
        }
    }
    ///<p>Retrieves a DebitReversal object.</p>
    pub fn get_treasury_debit_reversals_debit_reversal(
        &self,
        debit_reversal: &str,
    ) -> request::GetTreasuryDebitReversalsDebitReversalRequest {
        request::GetTreasuryDebitReversalsDebitReversalRequest {
            http_client: &self,
            debit_reversal: debit_reversal.to_owned(),
            expand: None,
        }
    }
    ///<p>Returns a list of FinancialAccounts.</p>
    pub fn get_treasury_financial_accounts(
        &self,
    ) -> request::GetTreasuryFinancialAccountsRequest {
        request::GetTreasuryFinancialAccountsRequest {
            http_client: &self,
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
    ) -> request::PostTreasuryFinancialAccountsRequest {
        request::PostTreasuryFinancialAccountsRequest {
            http_client: &self,
        }
    }
    ///<p>Retrieves the details of a FinancialAccount.</p>
    pub fn get_treasury_financial_accounts_financial_account(
        &self,
        financial_account: &str,
    ) -> request::GetTreasuryFinancialAccountsFinancialAccountRequest {
        request::GetTreasuryFinancialAccountsFinancialAccountRequest {
            http_client: &self,
            expand: None,
            financial_account: financial_account.to_owned(),
        }
    }
    ///<p>Updates the details of a FinancialAccount.</p>
    pub fn post_treasury_financial_accounts_financial_account(
        &self,
        financial_account: &str,
    ) -> request::PostTreasuryFinancialAccountsFinancialAccountRequest {
        request::PostTreasuryFinancialAccountsFinancialAccountRequest {
            http_client: &self,
            financial_account: financial_account.to_owned(),
        }
    }
    ///<p>Retrieves Features information associated with the FinancialAccount.</p>
    pub fn get_treasury_financial_accounts_financial_account_features(
        &self,
        financial_account: &str,
    ) -> request::GetTreasuryFinancialAccountsFinancialAccountFeaturesRequest {
        request::GetTreasuryFinancialAccountsFinancialAccountFeaturesRequest {
            http_client: &self,
            expand: None,
            financial_account: financial_account.to_owned(),
        }
    }
    ///<p>Updates the Features associated with a FinancialAccount.</p>
    pub fn post_treasury_financial_accounts_financial_account_features(
        &self,
        financial_account: &str,
    ) -> request::PostTreasuryFinancialAccountsFinancialAccountFeaturesRequest {
        request::PostTreasuryFinancialAccountsFinancialAccountFeaturesRequest {
            http_client: &self,
            financial_account: financial_account.to_owned(),
        }
    }
    ///<p>Returns a list of InboundTransfers sent from the specified FinancialAccount.</p>
    pub fn get_treasury_inbound_transfers(
        &self,
        financial_account: &str,
    ) -> request::GetTreasuryInboundTransfersRequest {
        request::GetTreasuryInboundTransfersRequest {
            http_client: &self,
            ending_before: None,
            expand: None,
            financial_account: financial_account.to_owned(),
            limit: None,
            starting_after: None,
            status: None,
        }
    }
    ///<p>Creates an InboundTransfer.</p>
    pub fn post_treasury_inbound_transfers(
        &self,
    ) -> request::PostTreasuryInboundTransfersRequest {
        request::PostTreasuryInboundTransfersRequest {
            http_client: &self,
        }
    }
    ///<p>Retrieves the details of an existing InboundTransfer.</p>
    pub fn get_treasury_inbound_transfers_id(
        &self,
        id: &str,
    ) -> request::GetTreasuryInboundTransfersIdRequest {
        request::GetTreasuryInboundTransfersIdRequest {
            http_client: &self,
            expand: None,
            id: id.to_owned(),
        }
    }
    ///<p>Cancels an InboundTransfer.</p>
    pub fn post_treasury_inbound_transfers_inbound_transfer_cancel(
        &self,
        inbound_transfer: &str,
    ) -> request::PostTreasuryInboundTransfersInboundTransferCancelRequest {
        request::PostTreasuryInboundTransfersInboundTransferCancelRequest {
            http_client: &self,
            inbound_transfer: inbound_transfer.to_owned(),
        }
    }
    ///<p>Returns a list of OutboundPayments sent from the specified FinancialAccount.</p>
    pub fn get_treasury_outbound_payments(
        &self,
        financial_account: &str,
    ) -> request::GetTreasuryOutboundPaymentsRequest {
        request::GetTreasuryOutboundPaymentsRequest {
            http_client: &self,
            customer: None,
            ending_before: None,
            expand: None,
            financial_account: financial_account.to_owned(),
            limit: None,
            starting_after: None,
            status: None,
        }
    }
    ///<p>Creates an OutboundPayment.</p>
    pub fn post_treasury_outbound_payments(
        &self,
    ) -> request::PostTreasuryOutboundPaymentsRequest {
        request::PostTreasuryOutboundPaymentsRequest {
            http_client: &self,
        }
    }
    ///<p>Retrieves the details of an existing OutboundPayment by passing the unique OutboundPayment ID from either the OutboundPayment creation request or OutboundPayment list.</p>
    pub fn get_treasury_outbound_payments_id(
        &self,
        id: &str,
    ) -> request::GetTreasuryOutboundPaymentsIdRequest {
        request::GetTreasuryOutboundPaymentsIdRequest {
            http_client: &self,
            expand: None,
            id: id.to_owned(),
        }
    }
    ///<p>Cancel an OutboundPayment.</p>
    pub fn post_treasury_outbound_payments_id_cancel(
        &self,
        id: &str,
    ) -> request::PostTreasuryOutboundPaymentsIdCancelRequest {
        request::PostTreasuryOutboundPaymentsIdCancelRequest {
            http_client: &self,
            id: id.to_owned(),
        }
    }
    ///<p>Returns a list of OutboundTransfers sent from the specified FinancialAccount.</p>
    pub fn get_treasury_outbound_transfers(
        &self,
        financial_account: &str,
    ) -> request::GetTreasuryOutboundTransfersRequest {
        request::GetTreasuryOutboundTransfersRequest {
            http_client: &self,
            ending_before: None,
            expand: None,
            financial_account: financial_account.to_owned(),
            limit: None,
            starting_after: None,
            status: None,
        }
    }
    ///<p>Creates an OutboundTransfer.</p>
    pub fn post_treasury_outbound_transfers(
        &self,
    ) -> request::PostTreasuryOutboundTransfersRequest {
        request::PostTreasuryOutboundTransfersRequest {
            http_client: &self,
        }
    }
    ///<p>Retrieves the details of an existing OutboundTransfer by passing the unique OutboundTransfer ID from either the OutboundTransfer creation request or OutboundTransfer list.</p>
    pub fn get_treasury_outbound_transfers_outbound_transfer(
        &self,
        outbound_transfer: &str,
    ) -> request::GetTreasuryOutboundTransfersOutboundTransferRequest {
        request::GetTreasuryOutboundTransfersOutboundTransferRequest {
            http_client: &self,
            expand: None,
            outbound_transfer: outbound_transfer.to_owned(),
        }
    }
    ///<p>An OutboundTransfer can be canceled if the funds have not yet been paid out.</p>
    pub fn post_treasury_outbound_transfers_outbound_transfer_cancel(
        &self,
        outbound_transfer: &str,
    ) -> request::PostTreasuryOutboundTransfersOutboundTransferCancelRequest {
        request::PostTreasuryOutboundTransfersOutboundTransferCancelRequest {
            http_client: &self,
            outbound_transfer: outbound_transfer.to_owned(),
        }
    }
    ///<p>Returns a list of ReceivedCredits.</p>
    pub fn get_treasury_received_credits(
        &self,
        financial_account: &str,
    ) -> request::GetTreasuryReceivedCreditsRequest {
        request::GetTreasuryReceivedCreditsRequest {
            http_client: &self,
            ending_before: None,
            expand: None,
            financial_account: financial_account.to_owned(),
            limit: None,
            linked_flows: None,
            starting_after: None,
            status: None,
        }
    }
    ///<p>Retrieves the details of an existing ReceivedCredit by passing the unique ReceivedCredit ID from the ReceivedCredit list.</p>
    pub fn get_treasury_received_credits_id(
        &self,
        id: &str,
    ) -> request::GetTreasuryReceivedCreditsIdRequest {
        request::GetTreasuryReceivedCreditsIdRequest {
            http_client: &self,
            expand: None,
            id: id.to_owned(),
        }
    }
    ///<p>Returns a list of ReceivedDebits.</p>
    pub fn get_treasury_received_debits(
        &self,
        financial_account: &str,
    ) -> request::GetTreasuryReceivedDebitsRequest {
        request::GetTreasuryReceivedDebitsRequest {
            http_client: &self,
            ending_before: None,
            expand: None,
            financial_account: financial_account.to_owned(),
            limit: None,
            starting_after: None,
            status: None,
        }
    }
    ///<p>Retrieves the details of an existing ReceivedDebit by passing the unique ReceivedDebit ID from the ReceivedDebit list</p>
    pub fn get_treasury_received_debits_id(
        &self,
        id: &str,
    ) -> request::GetTreasuryReceivedDebitsIdRequest {
        request::GetTreasuryReceivedDebitsIdRequest {
            http_client: &self,
            expand: None,
            id: id.to_owned(),
        }
    }
    ///<p>Retrieves a list of TransactionEntry objects.</p>
    pub fn get_treasury_transaction_entries(
        &self,
        financial_account: &str,
    ) -> request::GetTreasuryTransactionEntriesRequest {
        request::GetTreasuryTransactionEntriesRequest {
            http_client: &self,
            created: None,
            effective_at: None,
            ending_before: None,
            expand: None,
            financial_account: financial_account.to_owned(),
            limit: None,
            order_by: None,
            starting_after: None,
            transaction: None,
        }
    }
    ///<p>Retrieves a TransactionEntry object.</p>
    pub fn get_treasury_transaction_entries_id(
        &self,
        id: &str,
    ) -> request::GetTreasuryTransactionEntriesIdRequest {
        request::GetTreasuryTransactionEntriesIdRequest {
            http_client: &self,
            expand: None,
            id: id.to_owned(),
        }
    }
    ///<p>Retrieves a list of Transaction objects.</p>
    pub fn get_treasury_transactions(
        &self,
        financial_account: &str,
    ) -> request::GetTreasuryTransactionsRequest {
        request::GetTreasuryTransactionsRequest {
            http_client: &self,
            created: None,
            ending_before: None,
            expand: None,
            financial_account: financial_account.to_owned(),
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
        id: &str,
    ) -> request::GetTreasuryTransactionsIdRequest {
        request::GetTreasuryTransactionsIdRequest {
            http_client: &self,
            expand: None,
            id: id.to_owned(),
        }
    }
    ///<p>Returns a list of your webhook endpoints.</p>
    pub fn get_webhook_endpoints(&self) -> request::GetWebhookEndpointsRequest {
        request::GetWebhookEndpointsRequest {
            http_client: &self,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
        }
    }
    ///<p>A webhook endpoint must have a <code>url</code> and a list of <code>enabled_events</code>. You may optionally specify the Boolean <code>connect</code> parameter. If set to true, then a Connect webhook endpoint that notifies the specified <code>url</code> about events from all connected accounts is created; otherwise an account webhook endpoint that notifies the specified <code>url</code> only about events from your account is created. You can also create webhook endpoints in the <a href="https://dashboard.stripe.com/account/webhooks">webhooks settings</a> section of the Dashboard.</p>
    pub fn post_webhook_endpoints(&self) -> request::PostWebhookEndpointsRequest {
        request::PostWebhookEndpointsRequest {
            http_client: &self,
        }
    }
    ///<p>Retrieves the webhook endpoint with the given ID.</p>
    pub fn get_webhook_endpoints_webhook_endpoint(
        &self,
        webhook_endpoint: &str,
    ) -> request::GetWebhookEndpointsWebhookEndpointRequest {
        request::GetWebhookEndpointsWebhookEndpointRequest {
            http_client: &self,
            expand: None,
            webhook_endpoint: webhook_endpoint.to_owned(),
        }
    }
    ///<p>Updates the webhook endpoint. You may edit the <code>url</code>, the list of <code>enabled_events</code>, and the status of your endpoint.</p>
    pub fn post_webhook_endpoints_webhook_endpoint(
        &self,
        webhook_endpoint: &str,
    ) -> request::PostWebhookEndpointsWebhookEndpointRequest {
        request::PostWebhookEndpointsWebhookEndpointRequest {
            http_client: &self,
            webhook_endpoint: webhook_endpoint.to_owned(),
        }
    }
    ///<p>You can also delete webhook endpoints via the <a href="https://dashboard.stripe.com/account/webhooks">webhook endpoint management</a> page of the Stripe dashboard.</p>
    pub fn delete_webhook_endpoints_webhook_endpoint(
        &self,
        webhook_endpoint: &str,
    ) -> request::DeleteWebhookEndpointsWebhookEndpointRequest {
        request::DeleteWebhookEndpointsWebhookEndpointRequest {
            http_client: &self,
            webhook_endpoint: webhook_endpoint.to_owned(),
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
                .expect("Environment variable BASIC_AUTH is not set."),
        }
    }
}