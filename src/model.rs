use serde::{Serialize, Deserialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    #[serde(rename = "business_profile")]
    ///Business information about the account.
    pub business_profile: Option<serde_json::Value>,
    #[serde(rename = "business_type")]
    ///The business type.
    pub business_type: Option<String>,
    #[serde(rename = "capabilities")]
    ///
    pub capabilities: Option<AccountCapabilities>,
    #[serde(rename = "charges_enabled")]
    ///Whether the account can create live charges.
    pub charges_enabled: Option<bool>,
    #[serde(rename = "company")]
    ///
    pub company: Option<LegalEntityCompany>,
    #[serde(rename = "controller")]
    ///
    pub controller: Option<AccountUnificationAccountController>,
    #[serde(rename = "country")]
    ///The account's country.
    pub country: Option<String>,
    #[serde(rename = "created")]
    ///Time at which the account was connected. Measured in seconds since the Unix epoch.
    pub created: Option<i64>,
    #[serde(rename = "default_currency")]
    ///Three-letter ISO currency code representing the default currency for the account. This must be a currency that [Stripe supports in the account's country](https://stripe.com/docs/payouts).
    pub default_currency: Option<String>,
    #[serde(rename = "details_submitted")]
    ///Whether account details have been submitted. Standard accounts cannot receive payouts before this is true.
    pub details_submitted: Option<bool>,
    #[serde(rename = "email")]
    ///An email address associated with the account. You can treat this as metadata: it is not used for authentication or messaging account holders.
    pub email: Option<String>,
    #[serde(rename = "external_accounts")]
    ///External accounts (bank accounts and debit cards) currently attached to this account
    pub external_accounts: Option<serde_json::Value>,
    #[serde(rename = "future_requirements")]
    ///
    pub future_requirements: Option<AccountFutureRequirements>,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "individual")]
    /**This is an object representing a person associated with a Stripe account.

A platform cannot access a Standard or Express account's persons after the account starts onboarding, such as after generating an account link for the account.
See the [Standard onboarding](https://stripe.com/docs/connect/standard-accounts) or [Express onboarding documentation](https://stripe.com/docs/connect/express-accounts) for information about platform pre-filling and account onboarding steps.

Related guide: [Handling Identity Verification with the API](https://stripe.com/docs/connect/identity-verification-api#person-information).*/
    pub individual: Option<Person>,
    #[serde(rename = "metadata")]
    ///Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<serde_json::Value>,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "payouts_enabled")]
    ///Whether Stripe can send payouts to this account.
    pub payouts_enabled: Option<bool>,
    #[serde(rename = "requirements")]
    ///
    pub requirements: Option<AccountRequirements>,
    #[serde(rename = "settings")]
    ///Options for customizing how the account functions within Stripe.
    pub settings: Option<serde_json::Value>,
    #[serde(rename = "tos_acceptance")]
    ///
    pub tos_acceptance: Option<AccountTosAcceptance>,
    #[serde(rename = "type")]
    ///The Stripe account type. Can be `standard`, `express`, or `custom`.
    pub type_: Option<String>,
}
impl std::fmt::Display for Account {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountBacsDebitPaymentsSettings {
    #[serde(rename = "display_name")]
    ///The Bacs Direct Debit Display Name for this account. For payments made with Bacs Direct Debit, this will appear on the mandate, and as the statement descriptor.
    pub display_name: Option<String>,
}
impl std::fmt::Display for AccountBacsDebitPaymentsSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountBrandingSettings {
    #[serde(rename = "icon")]
    ///(ID of a [file upload](https://stripe.com/docs/guides/file-upload)) An icon for the account. Must be square and at least 128px x 128px.
    pub icon: Option<serde_json::Value>,
    #[serde(rename = "logo")]
    ///(ID of a [file upload](https://stripe.com/docs/guides/file-upload)) A logo for the account that will be used in Checkout instead of the icon and without the account's name next to it if provided. Must be at least 128px x 128px.
    pub logo: Option<serde_json::Value>,
    #[serde(rename = "primary_color")]
    ///A CSS hex color value representing the primary branding color for this account
    pub primary_color: Option<String>,
    #[serde(rename = "secondary_color")]
    ///A CSS hex color value representing the secondary branding color for this account
    pub secondary_color: Option<String>,
}
impl std::fmt::Display for AccountBrandingSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountBusinessProfile {
    #[serde(rename = "mcc")]
    ///[The merchant category code for the account](https://stripe.com/docs/connect/setting-mcc). MCCs are used to classify businesses based on the goods or services they provide.
    pub mcc: Option<String>,
    #[serde(rename = "name")]
    ///The customer-facing business name.
    pub name: Option<String>,
    #[serde(rename = "product_description")]
    ///Internal-only description of the product sold or service provided by the business. It's used by Stripe for risk and underwriting purposes.
    pub product_description: Option<String>,
    #[serde(rename = "support_address")]
    ///A publicly available mailing address for sending support issues to.
    pub support_address: Option<serde_json::Value>,
    #[serde(rename = "support_email")]
    ///A publicly available email address for sending support issues to.
    pub support_email: Option<String>,
    #[serde(rename = "support_phone")]
    ///A publicly available phone number to call with support issues.
    pub support_phone: Option<String>,
    #[serde(rename = "support_url")]
    ///A publicly available website for handling support issues.
    pub support_url: Option<String>,
    #[serde(rename = "url")]
    ///The business's publicly available website.
    pub url: Option<String>,
}
impl std::fmt::Display for AccountBusinessProfile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountCapabilities {
    #[serde(rename = "acss_debit_payments")]
    ///The status of the Canadian pre-authorized debits payments capability of the account, or whether the account can directly process Canadian pre-authorized debits charges.
    pub acss_debit_payments: Option<String>,
    #[serde(rename = "affirm_payments")]
    ///The status of the Affirm capability of the account, or whether the account can directly process Affirm charges.
    pub affirm_payments: Option<String>,
    #[serde(rename = "afterpay_clearpay_payments")]
    ///The status of the Afterpay Clearpay capability of the account, or whether the account can directly process Afterpay Clearpay charges.
    pub afterpay_clearpay_payments: Option<String>,
    #[serde(rename = "au_becs_debit_payments")]
    ///The status of the BECS Direct Debit (AU) payments capability of the account, or whether the account can directly process BECS Direct Debit (AU) charges.
    pub au_becs_debit_payments: Option<String>,
    #[serde(rename = "bacs_debit_payments")]
    ///The status of the Bacs Direct Debits payments capability of the account, or whether the account can directly process Bacs Direct Debits charges.
    pub bacs_debit_payments: Option<String>,
    #[serde(rename = "bancontact_payments")]
    ///The status of the Bancontact payments capability of the account, or whether the account can directly process Bancontact charges.
    pub bancontact_payments: Option<String>,
    #[serde(rename = "bank_transfer_payments")]
    ///The status of the customer_balance payments capability of the account, or whether the account can directly process customer_balance charges.
    pub bank_transfer_payments: Option<String>,
    #[serde(rename = "blik_payments")]
    ///The status of the blik payments capability of the account, or whether the account can directly process blik charges.
    pub blik_payments: Option<String>,
    #[serde(rename = "boleto_payments")]
    ///The status of the boleto payments capability of the account, or whether the account can directly process boleto charges.
    pub boleto_payments: Option<String>,
    #[serde(rename = "card_issuing")]
    ///The status of the card issuing capability of the account, or whether you can use Issuing to distribute funds on cards
    pub card_issuing: Option<String>,
    #[serde(rename = "card_payments")]
    ///The status of the card payments capability of the account, or whether the account can directly process credit and debit card charges.
    pub card_payments: Option<String>,
    #[serde(rename = "cartes_bancaires_payments")]
    ///The status of the Cartes Bancaires payments capability of the account, or whether the account can directly process Cartes Bancaires card charges in EUR currency.
    pub cartes_bancaires_payments: Option<String>,
    #[serde(rename = "eps_payments")]
    ///The status of the EPS payments capability of the account, or whether the account can directly process EPS charges.
    pub eps_payments: Option<String>,
    #[serde(rename = "fpx_payments")]
    ///The status of the FPX payments capability of the account, or whether the account can directly process FPX charges.
    pub fpx_payments: Option<String>,
    #[serde(rename = "giropay_payments")]
    ///The status of the giropay payments capability of the account, or whether the account can directly process giropay charges.
    pub giropay_payments: Option<String>,
    #[serde(rename = "grabpay_payments")]
    ///The status of the GrabPay payments capability of the account, or whether the account can directly process GrabPay charges.
    pub grabpay_payments: Option<String>,
    #[serde(rename = "ideal_payments")]
    ///The status of the iDEAL payments capability of the account, or whether the account can directly process iDEAL charges.
    pub ideal_payments: Option<String>,
    #[serde(rename = "jcb_payments")]
    ///The status of the JCB payments capability of the account, or whether the account (Japan only) can directly process JCB credit card charges in JPY currency.
    pub jcb_payments: Option<String>,
    #[serde(rename = "klarna_payments")]
    ///The status of the Klarna payments capability of the account, or whether the account can directly process Klarna charges.
    pub klarna_payments: Option<String>,
    #[serde(rename = "konbini_payments")]
    ///The status of the konbini payments capability of the account, or whether the account can directly process konbini charges.
    pub konbini_payments: Option<String>,
    #[serde(rename = "legacy_payments")]
    ///The status of the legacy payments capability of the account.
    pub legacy_payments: Option<String>,
    #[serde(rename = "link_payments")]
    ///The status of the link_payments capability of the account, or whether the account can directly process Link charges.
    pub link_payments: Option<String>,
    #[serde(rename = "oxxo_payments")]
    ///The status of the OXXO payments capability of the account, or whether the account can directly process OXXO charges.
    pub oxxo_payments: Option<String>,
    #[serde(rename = "p24_payments")]
    ///The status of the P24 payments capability of the account, or whether the account can directly process P24 charges.
    pub p24_payments: Option<String>,
    #[serde(rename = "paynow_payments")]
    ///The status of the paynow payments capability of the account, or whether the account can directly process paynow charges.
    pub paynow_payments: Option<String>,
    #[serde(rename = "promptpay_payments")]
    ///The status of the promptpay payments capability of the account, or whether the account can directly process promptpay charges.
    pub promptpay_payments: Option<String>,
    #[serde(rename = "sepa_debit_payments")]
    ///The status of the SEPA Direct Debits payments capability of the account, or whether the account can directly process SEPA Direct Debits charges.
    pub sepa_debit_payments: Option<String>,
    #[serde(rename = "sofort_payments")]
    ///The status of the Sofort payments capability of the account, or whether the account can directly process Sofort charges.
    pub sofort_payments: Option<String>,
    #[serde(rename = "tax_reporting_us_1099_k")]
    ///The status of the tax reporting 1099-K (US) capability of the account.
    pub tax_reporting_us1099_k: Option<String>,
    #[serde(rename = "tax_reporting_us_1099_misc")]
    ///The status of the tax reporting 1099-MISC (US) capability of the account.
    pub tax_reporting_us1099_misc: Option<String>,
    #[serde(rename = "transfers")]
    ///The status of the transfers capability of the account, or whether your platform can transfer funds to the account.
    pub transfers: Option<String>,
    #[serde(rename = "treasury")]
    ///The status of the banking capability, or whether the account can have bank accounts.
    pub treasury: Option<String>,
    #[serde(rename = "us_bank_account_ach_payments")]
    ///The status of the US bank account ACH payments capability of the account, or whether the account can directly process US bank account charges.
    pub us_bank_account_ach_payments: Option<String>,
}
impl std::fmt::Display for AccountCapabilities {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountCapabilityFutureRequirements {
    #[serde(rename = "alternatives")]
    ///Fields that are due and can be satisfied by providing the corresponding alternative fields instead.
    pub alternatives: Option<Vec<AccountRequirementsAlternative>>,
    #[serde(rename = "current_deadline")]
    ///Date on which `future_requirements` merges with the main `requirements` hash and `future_requirements` becomes empty. After the transition, `currently_due` requirements may immediately become `past_due`, but the account may also be given a grace period depending on the capability's enablement state prior to transitioning.
    pub current_deadline: Option<i64>,
    #[serde(rename = "currently_due")]
    ///Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    pub currently_due: Vec<String>,
    #[serde(rename = "disabled_reason")]
    ///This is typed as a string for consistency with `requirements.disabled_reason`, but it safe to assume `future_requirements.disabled_reason` is empty because fields in `future_requirements` will never disable the account.
    pub disabled_reason: Option<String>,
    #[serde(rename = "errors")]
    ///Fields that are `currently_due` and need to be collected again because validation or verification failed.
    pub errors: Vec<AccountRequirementsError>,
    #[serde(rename = "eventually_due")]
    ///Fields that need to be collected assuming all volume thresholds are reached. As they become required, they appear in `currently_due` as well.
    pub eventually_due: Vec<String>,
    #[serde(rename = "past_due")]
    ///Fields that weren't collected by `requirements.current_deadline`. These fields need to be collected to enable the capability on the account. New fields will never appear here; `future_requirements.past_due` will always be a subset of `requirements.past_due`.
    pub past_due: Vec<String>,
    #[serde(rename = "pending_verification")]
    ///Fields that may become required depending on the results of verification or review. Will be an empty array unless an asynchronous verification is pending. If verification fails, these fields move to `eventually_due` or `currently_due`.
    pub pending_verification: Vec<String>,
}
impl std::fmt::Display for AccountCapabilityFutureRequirements {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountCapabilityRequirements {
    #[serde(rename = "alternatives")]
    ///Fields that are due and can be satisfied by providing the corresponding alternative fields instead.
    pub alternatives: Option<Vec<AccountRequirementsAlternative>>,
    #[serde(rename = "current_deadline")]
    ///Date by which the fields in `currently_due` must be collected to keep the capability enabled for the account. These fields may disable the capability sooner if the next threshold is reached before they are collected.
    pub current_deadline: Option<i64>,
    #[serde(rename = "currently_due")]
    ///Fields that need to be collected to keep the capability enabled. If not collected by `current_deadline`, these fields appear in `past_due` as well, and the capability is disabled.
    pub currently_due: Vec<String>,
    #[serde(rename = "disabled_reason")]
    /**If the capability is disabled, this string describes why. Can be `requirements.past_due`, `requirements.pending_verification`, `listed`, `platform_paused`, `rejected.fraud`, `rejected.listed`, `rejected.terms_of_service`, `rejected.other`, `under_review`, or `other`.

`rejected.unsupported_business` means that the account's business is not supported by the capability. For example, payment methods may restrict the businesses they support in their terms of service:

- [Afterpay Clearpay's terms of service](/afterpay-clearpay/legal#restricted-businesses)

If you believe that the rejection is in error, please contact support at https://support.stripe.com/contact/ for assistance.*/
    pub disabled_reason: Option<String>,
    #[serde(rename = "errors")]
    ///Fields that are `currently_due` and need to be collected again because validation or verification failed.
    pub errors: Vec<AccountRequirementsError>,
    #[serde(rename = "eventually_due")]
    ///Fields that need to be collected assuming all volume thresholds are reached. As they become required, they appear in `currently_due` as well, and `current_deadline` becomes set.
    pub eventually_due: Vec<String>,
    #[serde(rename = "past_due")]
    ///Fields that weren't collected by `current_deadline`. These fields need to be collected to enable the capability on the account.
    pub past_due: Vec<String>,
    #[serde(rename = "pending_verification")]
    ///Fields that may become required depending on the results of verification or review. Will be an empty array unless an asynchronous verification is pending. If verification fails, these fields move to `eventually_due`, `currently_due`, or `past_due`.
    pub pending_verification: Vec<String>,
}
impl std::fmt::Display for AccountCapabilityRequirements {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountCardIssuingSettings {
    #[serde(rename = "tos_acceptance")]
    ///
    pub tos_acceptance: Option<CardIssuingAccountTermsOfService>,
}
impl std::fmt::Display for AccountCardIssuingSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountCardPaymentsSettings {
    #[serde(rename = "decline_on")]
    ///
    pub decline_on: Option<AccountDeclineChargeOn>,
    #[serde(rename = "statement_descriptor_prefix")]
    ///The default text that appears on credit card statements when a charge is made. This field prefixes any dynamic `statement_descriptor` specified on the charge. `statement_descriptor_prefix` is useful for maximizing descriptor space for the dynamic portion.
    pub statement_descriptor_prefix: Option<String>,
    #[serde(rename = "statement_descriptor_prefix_kana")]
    ///The Kana variation of the default text that appears on credit card statements when a charge is made (Japan only). This field prefixes any dynamic `statement_descriptor_suffix_kana` specified on the charge. `statement_descriptor_prefix_kana` is useful for maximizing descriptor space for the dynamic portion.
    pub statement_descriptor_prefix_kana: Option<String>,
    #[serde(rename = "statement_descriptor_prefix_kanji")]
    ///The Kanji variation of the default text that appears on credit card statements when a charge is made (Japan only). This field prefixes any dynamic `statement_descriptor_suffix_kanji` specified on the charge. `statement_descriptor_prefix_kanji` is useful for maximizing descriptor space for the dynamic portion.
    pub statement_descriptor_prefix_kanji: Option<String>,
}
impl std::fmt::Display for AccountCardPaymentsSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountDashboardSettings {
    #[serde(rename = "display_name")]
    ///The display name for this account. This is used on the Stripe Dashboard to differentiate between accounts.
    pub display_name: Option<String>,
    #[serde(rename = "timezone")]
    ///The timezone used in the Stripe Dashboard for this account. A list of possible time zone values is maintained at the [IANA Time Zone Database](http://www.iana.org/time-zones).
    pub timezone: Option<String>,
}
impl std::fmt::Display for AccountDashboardSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountDeclineChargeOn {
    #[serde(rename = "avs_failure")]
    ///Whether Stripe automatically declines charges with an incorrect ZIP or postal code. This setting only applies when a ZIP or postal code is provided and they fail bank verification.
    pub avs_failure: bool,
    #[serde(rename = "cvc_failure")]
    ///Whether Stripe automatically declines charges with an incorrect CVC. This setting only applies when a CVC is provided and it fails bank verification.
    pub cvc_failure: bool,
}
impl std::fmt::Display for AccountDeclineChargeOn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountFutureRequirements {
    #[serde(rename = "alternatives")]
    ///Fields that are due and can be satisfied by providing the corresponding alternative fields instead.
    pub alternatives: Option<Vec<AccountRequirementsAlternative>>,
    #[serde(rename = "current_deadline")]
    ///Date on which `future_requirements` merges with the main `requirements` hash and `future_requirements` becomes empty. After the transition, `currently_due` requirements may immediately become `past_due`, but the account may also be given a grace period depending on its enablement state prior to transitioning.
    pub current_deadline: Option<i64>,
    #[serde(rename = "currently_due")]
    ///Fields that need to be collected to keep the account enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    pub currently_due: Option<Vec<String>>,
    #[serde(rename = "disabled_reason")]
    ///This is typed as a string for consistency with `requirements.disabled_reason`, but it safe to assume `future_requirements.disabled_reason` is empty because fields in `future_requirements` will never disable the account.
    pub disabled_reason: Option<String>,
    #[serde(rename = "errors")]
    ///Fields that are `currently_due` and need to be collected again because validation or verification failed.
    pub errors: Option<Vec<AccountRequirementsError>>,
    #[serde(rename = "eventually_due")]
    ///Fields that need to be collected assuming all volume thresholds are reached. As they become required, they appear in `currently_due` as well.
    pub eventually_due: Option<Vec<String>>,
    #[serde(rename = "past_due")]
    ///Fields that weren't collected by `requirements.current_deadline`. These fields need to be collected to enable the capability on the account. New fields will never appear here; `future_requirements.past_due` will always be a subset of `requirements.past_due`.
    pub past_due: Option<Vec<String>>,
    #[serde(rename = "pending_verification")]
    ///Fields that may become required depending on the results of verification or review. Will be an empty array unless an asynchronous verification is pending. If verification fails, these fields move to `eventually_due` or `currently_due`.
    pub pending_verification: Option<Vec<String>>,
}
impl std::fmt::Display for AccountFutureRequirements {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountLink {
    #[serde(rename = "created")]
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    #[serde(rename = "expires_at")]
    ///The timestamp at which this account link will expire.
    pub expires_at: i64,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "url")]
    ///The URL for the account link.
    pub url: String,
}
impl std::fmt::Display for AccountLink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountPaymentsSettings {
    #[serde(rename = "statement_descriptor")]
    ///The default text that appears on credit card statements when a charge is made. This field prefixes any dynamic `statement_descriptor` specified on the charge.
    pub statement_descriptor: Option<String>,
    #[serde(rename = "statement_descriptor_kana")]
    ///The Kana variation of the default text that appears on credit card statements when a charge is made (Japan only)
    pub statement_descriptor_kana: Option<String>,
    #[serde(rename = "statement_descriptor_kanji")]
    ///The Kanji variation of the default text that appears on credit card statements when a charge is made (Japan only)
    pub statement_descriptor_kanji: Option<String>,
    #[serde(rename = "statement_descriptor_prefix_kana")]
    ///The Kana variation of the default text that appears on credit card statements when a charge is made (Japan only). This field prefixes any dynamic `statement_descriptor_suffix_kana` specified on the charge. `statement_descriptor_prefix_kana` is useful for maximizing descriptor space for the dynamic portion.
    pub statement_descriptor_prefix_kana: Option<String>,
    #[serde(rename = "statement_descriptor_prefix_kanji")]
    ///The Kanji variation of the default text that appears on credit card statements when a charge is made (Japan only). This field prefixes any dynamic `statement_descriptor_suffix_kanji` specified on the charge. `statement_descriptor_prefix_kanji` is useful for maximizing descriptor space for the dynamic portion.
    pub statement_descriptor_prefix_kanji: Option<String>,
}
impl std::fmt::Display for AccountPaymentsSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountPayoutSettings {
    #[serde(rename = "debit_negative_balances")]
    ///A Boolean indicating if Stripe should try to reclaim negative balances from an attached bank account. See our [Understanding Connect Account Balances](https://stripe.com/docs/connect/account-balances) documentation for details. Default value is `false` for Custom accounts, otherwise `true`.
    pub debit_negative_balances: bool,
    #[serde(rename = "schedule")]
    ///
    pub schedule: TransferSchedule,
    #[serde(rename = "statement_descriptor")]
    ///The text that appears on the bank account statement for payouts. If not set, this defaults to the platform's bank descriptor as set in the Dashboard.
    pub statement_descriptor: Option<String>,
}
impl std::fmt::Display for AccountPayoutSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountRequirements {
    #[serde(rename = "alternatives")]
    ///Fields that are due and can be satisfied by providing the corresponding alternative fields instead.
    pub alternatives: Option<Vec<AccountRequirementsAlternative>>,
    #[serde(rename = "current_deadline")]
    ///Date by which the fields in `currently_due` must be collected to keep the account enabled. These fields may disable the account sooner if the next threshold is reached before they are collected.
    pub current_deadline: Option<i64>,
    #[serde(rename = "currently_due")]
    ///Fields that need to be collected to keep the account enabled. If not collected by `current_deadline`, these fields appear in `past_due` as well, and the account is disabled.
    pub currently_due: Option<Vec<String>>,
    #[serde(rename = "disabled_reason")]
    ///If the account is disabled, this string describes why. Can be `requirements.past_due`, `requirements.pending_verification`, `listed`, `platform_paused`, `rejected.fraud`, `rejected.listed`, `rejected.terms_of_service`, `rejected.other`, `under_review`, or `other`.
    pub disabled_reason: Option<String>,
    #[serde(rename = "errors")]
    ///Fields that are `currently_due` and need to be collected again because validation or verification failed.
    pub errors: Option<Vec<AccountRequirementsError>>,
    #[serde(rename = "eventually_due")]
    ///Fields that need to be collected assuming all volume thresholds are reached. As they become required, they appear in `currently_due` as well, and `current_deadline` becomes set.
    pub eventually_due: Option<Vec<String>>,
    #[serde(rename = "past_due")]
    ///Fields that weren't collected by `current_deadline`. These fields need to be collected to enable the account.
    pub past_due: Option<Vec<String>>,
    #[serde(rename = "pending_verification")]
    ///Fields that may become required depending on the results of verification or review. Will be an empty array unless an asynchronous verification is pending. If verification fails, these fields move to `eventually_due`, `currently_due`, or `past_due`.
    pub pending_verification: Option<Vec<String>>,
}
impl std::fmt::Display for AccountRequirements {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountRequirementsAlternative {
    #[serde(rename = "alternative_fields_due")]
    ///Fields that can be provided to satisfy all fields in `original_fields_due`.
    pub alternative_fields_due: Vec<String>,
    #[serde(rename = "original_fields_due")]
    ///Fields that are due and can be satisfied by providing all fields in `alternative_fields_due`.
    pub original_fields_due: Vec<String>,
}
impl std::fmt::Display for AccountRequirementsAlternative {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountRequirementsError {
    #[serde(rename = "code")]
    ///The code for the type of error.
    pub code: String,
    #[serde(rename = "reason")]
    ///An informative message that indicates the error type and provides additional details about the error.
    pub reason: String,
    #[serde(rename = "requirement")]
    ///The specific user onboarding requirement field (in the requirements hash) that needs to be resolved.
    pub requirement: String,
}
impl std::fmt::Display for AccountRequirementsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountSepaDebitPaymentsSettings {
    #[serde(rename = "creditor_id")]
    ///SEPA creditor identifier that identifies the company making the payment.
    pub creditor_id: Option<String>,
}
impl std::fmt::Display for AccountSepaDebitPaymentsSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountSettings {
    #[serde(rename = "bacs_debit_payments")]
    ///
    pub bacs_debit_payments: Option<AccountBacsDebitPaymentsSettings>,
    #[serde(rename = "branding")]
    ///
    pub branding: AccountBrandingSettings,
    #[serde(rename = "card_issuing")]
    ///
    pub card_issuing: Option<AccountCardIssuingSettings>,
    #[serde(rename = "card_payments")]
    ///
    pub card_payments: AccountCardPaymentsSettings,
    #[serde(rename = "dashboard")]
    ///
    pub dashboard: AccountDashboardSettings,
    #[serde(rename = "payments")]
    ///
    pub payments: AccountPaymentsSettings,
    #[serde(rename = "payouts")]
    ///
    pub payouts: Option<AccountPayoutSettings>,
    #[serde(rename = "sepa_debit_payments")]
    ///
    pub sepa_debit_payments: Option<AccountSepaDebitPaymentsSettings>,
    #[serde(rename = "treasury")]
    ///
    pub treasury: Option<AccountTreasurySettings>,
}
impl std::fmt::Display for AccountSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountTermsOfService {
    #[serde(rename = "date")]
    ///The Unix timestamp marking when the account representative accepted the service agreement.
    pub date: Option<i64>,
    #[serde(rename = "ip")]
    ///The IP address from which the account representative accepted the service agreement.
    pub ip: Option<String>,
    #[serde(rename = "user_agent")]
    ///The user agent of the browser from which the account representative accepted the service agreement.
    pub user_agent: Option<String>,
}
impl std::fmt::Display for AccountTermsOfService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountTosAcceptance {
    #[serde(rename = "date")]
    ///The Unix timestamp marking when the account representative accepted their service agreement
    pub date: Option<i64>,
    #[serde(rename = "ip")]
    ///The IP address from which the account representative accepted their service agreement
    pub ip: Option<String>,
    #[serde(rename = "service_agreement")]
    ///The user's service agreement type
    pub service_agreement: Option<String>,
    #[serde(rename = "user_agent")]
    ///The user agent of the browser from which the account representative accepted their service agreement
    pub user_agent: Option<String>,
}
impl std::fmt::Display for AccountTosAcceptance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountTreasurySettings {
    #[serde(rename = "tos_acceptance")]
    ///
    pub tos_acceptance: Option<AccountTermsOfService>,
}
impl std::fmt::Display for AccountTreasurySettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountUnificationAccountController {
    #[serde(rename = "is_controller")]
    ///`true` if the Connect application retrieving the resource controls the account and can therefore exercise [platform controls](https://stripe.com/docs/connect/platform-controls-for-standard-accounts). Otherwise, this field is null.
    pub is_controller: Option<bool>,
    #[serde(rename = "type")]
    ///The controller type. Can be `application`, if a Connect application controls the account, or `account`, if the account controls itself.
    pub type_: String,
}
impl std::fmt::Display for AccountUnificationAccountController {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Address {
    #[serde(rename = "city")]
    ///City, district, suburb, town, or village.
    pub city: Option<String>,
    #[serde(rename = "country")]
    ///Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    pub country: Option<String>,
    #[serde(rename = "line1")]
    ///Address line 1 (e.g., street, PO Box, or company name).
    pub line1: Option<String>,
    #[serde(rename = "line2")]
    ///Address line 2 (e.g., apartment, suite, unit, or building).
    pub line2: Option<String>,
    #[serde(rename = "postal_code")]
    ///ZIP or postal code.
    pub postal_code: Option<String>,
    #[serde(rename = "state")]
    ///State, county, province, or region.
    pub state: Option<String>,
}
impl std::fmt::Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiErrors {
    #[serde(rename = "charge")]
    ///For card errors, the ID of the failed charge.
    pub charge: Option<String>,
    #[serde(rename = "code")]
    ///For some errors that could be handled programmatically, a short string indicating the [error code](https://stripe.com/docs/error-codes) reported.
    pub code: Option<String>,
    #[serde(rename = "decline_code")]
    ///For card errors resulting from a card issuer decline, a short string indicating the [card issuer's reason for the decline](https://stripe.com/docs/declines#issuer-declines) if they provide one.
    pub decline_code: Option<String>,
    #[serde(rename = "doc_url")]
    ///A URL to more information about the [error code](https://stripe.com/docs/error-codes) reported.
    pub doc_url: Option<String>,
    #[serde(rename = "message")]
    ///A human-readable message providing more details about the error. For card errors, these messages can be shown to your users.
    pub message: Option<String>,
    #[serde(rename = "param")]
    ///If the error is parameter-specific, the parameter related to the error. For example, you can use this to display a message near the correct form field.
    pub param: Option<String>,
    #[serde(rename = "payment_intent")]
    /**A PaymentIntent guides you through the process of collecting a payment from your customer.
We recommend that you create exactly one PaymentIntent for each order or
customer session in your system. You can reference the PaymentIntent later to
see the history of payment attempts for a particular session.

A PaymentIntent transitions through
[multiple statuses](https://stripe.com/docs/payments/intents#intent-statuses)
throughout its lifetime as it interfaces with Stripe.js to perform
authentication flows and ultimately creates at most one successful charge.

Related guide: [Payment Intents API](https://stripe.com/docs/payments/payment-intents).*/
    pub payment_intent: Option<PaymentIntent>,
    #[serde(rename = "payment_method")]
    /**PaymentMethod objects represent your customer's payment instruments.
You can use them with [PaymentIntents](https://stripe.com/docs/payments/payment-intents) to collect payments or save them to
Customer objects to store instrument details for future payments.

Related guides: [Payment Methods](https://stripe.com/docs/payments/payment-methods) and [More Payment Scenarios](https://stripe.com/docs/payments/more-payment-scenarios).*/
    pub payment_method: Option<PaymentMethod>,
    #[serde(rename = "payment_method_type")]
    ///If the error is specific to the type of payment method, the payment method type that had a problem. This field is only populated for invoice-related errors.
    pub payment_method_type: Option<String>,
    #[serde(rename = "setup_intent")]
    /**A SetupIntent guides you through the process of setting up and saving a customer's payment credentials for future payments.
For example, you could use a SetupIntent to set up and save your customer's card without immediately collecting a payment.
Later, you can use [PaymentIntents](https://stripe.com/docs/api#payment_intents) to drive the payment flow.

Create a SetupIntent as soon as you're ready to collect your customer's payment credentials.
Do not maintain long-lived, unconfirmed SetupIntents as they may no longer be valid.
The SetupIntent then transitions through multiple [statuses](https://stripe.com/docs/payments/intents#intent-statuses) as it guides
you through the setup process.

Successful SetupIntents result in payment credentials that are optimized for future payments.
For example, cardholders in [certain regions](/guides/strong-customer-authentication) may need to be run through
[Strong Customer Authentication](https://stripe.com/docs/strong-customer-authentication) at the time of payment method collection
in order to streamline later [off-session payments](https://stripe.com/docs/payments/setup-intents).
If the SetupIntent is used with a [Customer](https://stripe.com/docs/api#setup_intent_object-customer), upon success,
it will automatically attach the resulting payment method to that Customer.
We recommend using SetupIntents or [setup_future_usage](https://stripe.com/docs/api#payment_intent_object-setup_future_usage) on
PaymentIntents to save payment methods in order to prevent saving invalid or unoptimized payment methods.

By using SetupIntents, you ensure that your customers experience the minimum set of required friction,
even as regulations change over time.

Related guide: [Setup Intents API](https://stripe.com/docs/payments/setup-intents).*/
    pub setup_intent: Option<SetupIntent>,
    #[serde(rename = "source")]
    ///The source object for errors returned on a request involving a source.
    pub source: Option<serde_json::Value>,
    #[serde(rename = "type")]
    ///The type of error returned. One of `api_error`, `card_error`, `idempotency_error`, or `invalid_request_error`
    pub type_: String,
}
impl std::fmt::Display for ApiErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ApplePayDomain {
    #[serde(rename = "created")]
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    #[serde(rename = "domain_name")]
    pub domain_name: String,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "livemode")]
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
}
impl std::fmt::Display for ApplePayDomain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Application {
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "name")]
    ///The name of the application.
    pub name: Option<String>,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
}
impl std::fmt::Display for Application {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationFee {
    #[serde(rename = "account")]
    ///ID of the Stripe account this fee was taken from.
    pub account: serde_json::Value,
    #[serde(rename = "amount")]
    ///Amount earned, in %s.
    pub amount: i64,
    #[serde(rename = "amount_refunded")]
    ///Amount in %s refunded (can be less than the amount attribute on the fee if a partial refund was issued)
    pub amount_refunded: i64,
    #[serde(rename = "application")]
    ///ID of the Connect application that earned the fee.
    pub application: serde_json::Value,
    #[serde(rename = "balance_transaction")]
    ///Balance transaction that describes the impact of this collected application fee on your account balance (not including refunds).
    pub balance_transaction: Option<serde_json::Value>,
    #[serde(rename = "charge")]
    ///ID of the charge that the application fee was taken from.
    pub charge: serde_json::Value,
    #[serde(rename = "created")]
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    #[serde(rename = "currency")]
    ///Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: String,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "livemode")]
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "originating_transaction")]
    ///ID of the corresponding charge on the platform account, if this fee was the result of a charge using the `destination` parameter.
    pub originating_transaction: Option<serde_json::Value>,
    #[serde(rename = "refunded")]
    ///Whether the fee has been fully refunded. If the fee is only partially refunded, this attribute will still be false.
    pub refunded: bool,
    #[serde(rename = "refunds")]
    ///A list of refunds that have been applied to the fee.
    pub refunds: serde_json::Value,
}
impl std::fmt::Display for ApplicationFee {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AppsSecret {
    #[serde(rename = "created")]
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    #[serde(rename = "deleted")]
    ///If true, indicates that this secret has been deleted
    pub deleted: Option<bool>,
    #[serde(rename = "expires_at")]
    ///The Unix timestamp for the expiry time of the secret, after which the secret deletes.
    pub expires_at: Option<i64>,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "livemode")]
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    #[serde(rename = "name")]
    ///A name for the secret that's unique within the scope.
    pub name: String,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "payload")]
    ///The plaintext secret value to be stored.
    pub payload: Option<String>,
    #[serde(rename = "scope")]
    ///
    pub scope: SecretServiceResourceScope,
}
impl std::fmt::Display for AppsSecret {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AutomaticTax {
    #[serde(rename = "enabled")]
    ///Whether Stripe automatically computes tax on this invoice. Note that incompatible invoice items (invoice items with manually specified [tax rates](https://stripe.com/docs/api/tax_rates), negative amounts, or `tax_behavior=unspecified`) cannot be added to automatic tax invoices.
    pub enabled: bool,
    #[serde(rename = "status")]
    ///The status of the most recent automated tax calculation for this invoice.
    pub status: Option<String>,
}
impl std::fmt::Display for AutomaticTax {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Balance {
    #[serde(rename = "available")]
    ///Funds that are available to be transferred or paid out, whether automatically by Stripe or explicitly via the [Transfers API](https://stripe.com/docs/api#transfers) or [Payouts API](https://stripe.com/docs/api#payouts). The available balance for each currency and payment type can be found in the `source_types` property.
    pub available: Vec<BalanceAmount>,
    #[serde(rename = "connect_reserved")]
    ///Funds held due to negative balances on connected Custom accounts. The connect reserve balance for each currency and payment type can be found in the `source_types` property.
    pub connect_reserved: Option<Vec<BalanceAmount>>,
    #[serde(rename = "instant_available")]
    ///Funds that can be paid out using Instant Payouts.
    pub instant_available: Option<Vec<BalanceAmount>>,
    #[serde(rename = "issuing")]
    ///
    pub issuing: Option<BalanceDetail>,
    #[serde(rename = "livemode")]
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "pending")]
    ///Funds that are not yet available in the balance, due to the 7-day rolling pay cycle. The pending balance for each currency, and for each payment type, can be found in the `source_types` property.
    pub pending: Vec<BalanceAmount>,
}
impl std::fmt::Display for Balance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BalanceAmount {
    #[serde(rename = "amount")]
    ///Balance amount.
    pub amount: i64,
    #[serde(rename = "currency")]
    ///Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: String,
    #[serde(rename = "source_types")]
    ///
    pub source_types: Option<BalanceAmountBySourceType>,
}
impl std::fmt::Display for BalanceAmount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BalanceAmountBySourceType {
    #[serde(rename = "bank_account")]
    ///Amount for bank account.
    pub bank_account: Option<i64>,
    #[serde(rename = "card")]
    ///Amount for card.
    pub card: Option<i64>,
    #[serde(rename = "fpx")]
    ///Amount for FPX.
    pub fpx: Option<i64>,
}
impl std::fmt::Display for BalanceAmountBySourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BalanceDetail {
    #[serde(rename = "available")]
    ///Funds that are available for use.
    pub available: Vec<BalanceAmount>,
}
impl std::fmt::Display for BalanceDetail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BalanceTransaction {
    #[serde(rename = "amount")]
    ///Gross amount of the transaction, in %s.
    pub amount: i64,
    #[serde(rename = "available_on")]
    ///The date the transaction's net funds will become available in the Stripe balance.
    pub available_on: i64,
    #[serde(rename = "created")]
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    #[serde(rename = "currency")]
    ///Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: String,
    #[serde(rename = "description")]
    ///An arbitrary string attached to the object. Often useful for displaying to users.
    pub description: Option<String>,
    #[serde(rename = "exchange_rate")]
    ///The exchange rate used, if applicable, for this transaction. Specifically, if money was converted from currency A to currency B, then the `amount` in currency A, times `exchange_rate`, would be the `amount` in currency B. For example, suppose you charged a customer 10.00 EUR. Then the PaymentIntent's `amount` would be `1000` and `currency` would be `eur`. Suppose this was converted into 12.34 USD in your Stripe account. Then the BalanceTransaction's `amount` would be `1234`, `currency` would be `usd`, and `exchange_rate` would be `1.234`.
    pub exchange_rate: Option<f64>,
    #[serde(rename = "fee")]
    ///Fees (in %s) paid for this transaction.
    pub fee: i64,
    #[serde(rename = "fee_details")]
    ///Detailed breakdown of fees (in %s) paid for this transaction.
    pub fee_details: Vec<Fee>,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "net")]
    ///Net amount of the transaction, in %s.
    pub net: i64,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "reporting_category")]
    ///[Learn more](https://stripe.com/docs/reports/reporting-categories) about how reporting categories can help you understand balance transactions from an accounting perspective.
    pub reporting_category: String,
    #[serde(rename = "source")]
    ///The Stripe object to which this transaction is related.
    pub source: Option<serde_json::Value>,
    #[serde(rename = "status")]
    ///If the transaction's net funds are available in the Stripe balance yet. Either `available` or `pending`.
    pub status: String,
    #[serde(rename = "type")]
    ///Transaction type: `adjustment`, `advance`, `advance_funding`, `anticipation_repayment`, `application_fee`, `application_fee_refund`, `charge`, `connect_collection_transfer`, `contribution`, `issuing_authorization_hold`, `issuing_authorization_release`, `issuing_dispute`, `issuing_transaction`, `payment`, `payment_failure_refund`, `payment_refund`, `payout`, `payout_cancel`, `payout_failure`, `refund`, `refund_failure`, `reserve_transaction`, `reserved_funds`, `stripe_fee`, `stripe_fx_fee`, `tax_fee`, `topup`, `topup_reversal`, `transfer`, `transfer_cancel`, `transfer_failure`, or `transfer_refund`. [Learn more](https://stripe.com/docs/reports/balance-transaction-types) about balance transaction types and what they represent. If you are looking to classify transactions for accounting purposes, you might want to consider `reporting_category` instead.
    pub type_: String,
}
impl std::fmt::Display for BalanceTransaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BankAccount {
    #[serde(rename = "account")]
    ///The ID of the account that the bank account is associated with.
    pub account: Option<serde_json::Value>,
    #[serde(rename = "account_holder_name")]
    ///The name of the person or business that owns the bank account.
    pub account_holder_name: Option<String>,
    #[serde(rename = "account_holder_type")]
    ///The type of entity that holds the account. This can be either `individual` or `company`.
    pub account_holder_type: Option<String>,
    #[serde(rename = "account_type")]
    ///The bank account type. This can only be `checking` or `savings` in most countries. In Japan, this can only be `futsu` or `toza`.
    pub account_type: Option<String>,
    #[serde(rename = "available_payout_methods")]
    ///A set of available payout methods for this bank account. Only values from this set should be passed as the `method` when creating a payout.
    pub available_payout_methods: Option<Vec<String>>,
    #[serde(rename = "bank_name")]
    ///Name of the bank associated with the routing number (e.g., `WELLS FARGO`).
    pub bank_name: Option<String>,
    #[serde(rename = "country")]
    ///Two-letter ISO code representing the country the bank account is located in.
    pub country: String,
    #[serde(rename = "currency")]
    ///Three-letter [ISO code for the currency](https://stripe.com/docs/payouts) paid out to the bank account.
    pub currency: String,
    #[serde(rename = "customer")]
    ///The ID of the customer that the bank account is associated with.
    pub customer: Option<serde_json::Value>,
    #[serde(rename = "default_for_currency")]
    ///Whether this bank account is the default external account for its currency.
    pub default_for_currency: Option<bool>,
    #[serde(rename = "fingerprint")]
    ///Uniquely identifies this particular bank account. You can use this attribute to check whether two bank accounts are the same.
    pub fingerprint: Option<String>,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "last4")]
    ///The last four digits of the bank account number.
    pub last4: String,
    #[serde(rename = "metadata")]
    ///Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<serde_json::Value>,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "routing_number")]
    ///The routing transit number for the bank account.
    pub routing_number: Option<String>,
    #[serde(rename = "status")]
    /**For bank accounts, possible values are `new`, `validated`, `verified`, `verification_failed`, or `errored`. A bank account that hasn't had any activity or validation performed is `new`. If Stripe can determine that the bank account exists, its status will be `validated`. Note that there often isn’t enough information to know (e.g., for smaller credit unions), and the validation is not always run. If customer bank account verification has succeeded, the bank account status will be `verified`. If the verification failed for any reason, such as microdeposit failure, the status will be `verification_failed`. If a transfer sent to this bank account fails, we'll set the status to `errored` and will not continue to send transfers until the bank details are updated.

For external accounts, possible values are `new` and `errored`. Validations aren't run against external accounts because they're only used for payouts. This means the other statuses don't apply. If a transfer fails, the status is set to `errored` and transfers are stopped until account details are updated.*/
    pub status: String,
}
impl std::fmt::Display for BankAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BankConnectionsResourceAccountholder {
    #[serde(rename = "account")]
    ///The ID of the Stripe account this account belongs to. Should only be present if `account_holder.type` is `account`.
    pub account: Option<serde_json::Value>,
    #[serde(rename = "customer")]
    ///ID of the Stripe customer this account belongs to. Present if and only if `account_holder.type` is `customer`.
    pub customer: Option<serde_json::Value>,
    #[serde(rename = "type")]
    ///Type of account holder that this account belongs to.
    pub type_: String,
}
impl std::fmt::Display for BankConnectionsResourceAccountholder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BankConnectionsResourceBalance {
    #[serde(rename = "as_of")]
    ///The time that the external institution calculated this balance. Measured in seconds since the Unix epoch.
    pub as_of: i64,
    #[serde(rename = "cash")]
    ///
    pub cash: Option<BankConnectionsResourceBalanceApiResourceCashBalance>,
    #[serde(rename = "credit")]
    ///
    pub credit: Option<BankConnectionsResourceBalanceApiResourceCreditBalance>,
    #[serde(rename = "current")]
    /**The balances owed to (or by) the account holder.

Each key is a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.

Each value is a integer amount. A positive amount indicates money owed to the account holder. A negative amount indicates money owed by the account holder.*/
    pub current: serde_json::Value,
    #[serde(rename = "type")]
    ///The `type` of the balance. An additional hash is included on the balance with a name matching this value.
    pub type_: String,
}
impl std::fmt::Display for BankConnectionsResourceBalance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BankConnectionsResourceBalanceApiResourceCashBalance {
    #[serde(rename = "available")]
    /**The funds available to the account holder. Typically this is the current balance less any holds.

Each key is a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.

Each value is a integer amount. A positive amount indicates money owed to the account holder. A negative amount indicates money owed by the account holder.*/
    pub available: Option<serde_json::Value>,
}
impl std::fmt::Display for BankConnectionsResourceBalanceApiResourceCashBalance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BankConnectionsResourceBalanceApiResourceCreditBalance {
    #[serde(rename = "used")]
    /**The credit that has been used by the account holder.

Each key is a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.

Each value is a integer amount. A positive amount indicates money owed to the account holder. A negative amount indicates money owed by the account holder.*/
    pub used: Option<serde_json::Value>,
}
impl std::fmt::Display for BankConnectionsResourceBalanceApiResourceCreditBalance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BankConnectionsResourceBalanceRefresh {
    #[serde(rename = "last_attempted_at")]
    ///The time at which the last refresh attempt was initiated. Measured in seconds since the Unix epoch.
    pub last_attempted_at: i64,
    #[serde(rename = "status")]
    ///The status of the last refresh attempt.
    pub status: String,
}
impl std::fmt::Display for BankConnectionsResourceBalanceRefresh {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BankConnectionsResourceLinkAccountSessionFilters {
    #[serde(rename = "countries")]
    ///List of countries from which to filter accounts.
    pub countries: Option<Vec<String>>,
}
impl std::fmt::Display for BankConnectionsResourceLinkAccountSessionFilters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BankConnectionsResourceOwnershipRefresh {
    #[serde(rename = "last_attempted_at")]
    ///The time at which the last refresh attempt was initiated. Measured in seconds since the Unix epoch.
    pub last_attempted_at: i64,
    #[serde(rename = "status")]
    ///The status of the last refresh attempt.
    pub status: String,
}
impl std::fmt::Display for BankConnectionsResourceOwnershipRefresh {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BillingDetails {
    #[serde(rename = "address")]
    ///Billing address.
    pub address: Option<serde_json::Value>,
    #[serde(rename = "email")]
    ///Email address.
    pub email: Option<String>,
    #[serde(rename = "name")]
    ///Full name.
    pub name: Option<String>,
    #[serde(rename = "phone")]
    ///Billing phone number (including extension).
    pub phone: Option<String>,
}
impl std::fmt::Display for BillingDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BillingPortalConfiguration {
    #[serde(rename = "active")]
    ///Whether the configuration is active and can be used to create portal sessions.
    pub active: bool,
    #[serde(rename = "application")]
    ///ID of the Connect Application that created the configuration.
    pub application: Option<serde_json::Value>,
    #[serde(rename = "business_profile")]
    ///
    pub business_profile: PortalBusinessProfile,
    #[serde(rename = "created")]
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    #[serde(rename = "default_return_url")]
    ///The default URL to redirect customers to when they click on the portal's link to return to your website. This can be [overriden](https://stripe.com/docs/api/customer_portal/sessions/create#create_portal_session-return_url) when creating the session.
    pub default_return_url: Option<String>,
    #[serde(rename = "features")]
    ///
    pub features: PortalFeatures,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "is_default")]
    ///Whether the configuration is the default. If `true`, this configuration can be managed in the Dashboard and portal sessions will use this configuration unless it is overriden when creating the session.
    pub is_default: bool,
    #[serde(rename = "livemode")]
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    #[serde(rename = "metadata")]
    ///Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<serde_json::Value>,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "updated")]
    ///Time at which the object was last updated. Measured in seconds since the Unix epoch.
    pub updated: i64,
}
impl std::fmt::Display for BillingPortalConfiguration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BillingPortalSession {
    #[serde(rename = "configuration")]
    ///The configuration used by this session, describing the features available.
    pub configuration: serde_json::Value,
    #[serde(rename = "created")]
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    #[serde(rename = "customer")]
    ///The ID of the customer for this session.
    pub customer: String,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "livemode")]
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    #[serde(rename = "locale")]
    ///The IETF language tag of the locale Customer Portal is displayed in. If blank or auto, the customer’s `preferred_locales` or browser’s locale is used.
    pub locale: Option<String>,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "on_behalf_of")]
    ///The account for which the session was created on behalf of. When specified, only subscriptions and invoices with this `on_behalf_of` account appear in the portal. For more information, see the [docs](https://stripe.com/docs/connect/charges-transfers#on-behalf-of). Use the [Accounts API](https://stripe.com/docs/api/accounts/object#account_object-settings-branding) to modify the `on_behalf_of` account's branding settings, which the portal displays.
    pub on_behalf_of: Option<String>,
    #[serde(rename = "return_url")]
    ///The URL to redirect customers to when they click on the portal's link to return to your website.
    pub return_url: Option<String>,
    #[serde(rename = "url")]
    ///The short-lived URL of the session that gives customers access to the customer portal.
    pub url: String,
}
impl std::fmt::Display for BillingPortalSession {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Capability {
    #[serde(rename = "account")]
    ///The account for which the capability enables functionality.
    pub account: serde_json::Value,
    #[serde(rename = "future_requirements")]
    ///
    pub future_requirements: Option<AccountCapabilityFutureRequirements>,
    #[serde(rename = "id")]
    ///The identifier for the capability.
    pub id: String,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "requested")]
    ///Whether the capability has been requested.
    pub requested: bool,
    #[serde(rename = "requested_at")]
    ///Time at which the capability was requested. Measured in seconds since the Unix epoch.
    pub requested_at: Option<i64>,
    #[serde(rename = "requirements")]
    ///
    pub requirements: Option<AccountCapabilityRequirements>,
    #[serde(rename = "status")]
    ///The status of the capability. Can be `active`, `inactive`, `pending`, or `unrequested`.
    pub status: String,
}
impl std::fmt::Display for Capability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Card {
    #[serde(rename = "account")]
    ///The account this card belongs to. This attribute will not be in the card object if the card belongs to a customer or recipient instead.
    pub account: Option<serde_json::Value>,
    #[serde(rename = "address_city")]
    ///City/District/Suburb/Town/Village.
    pub address_city: Option<String>,
    #[serde(rename = "address_country")]
    ///Billing address country, if provided when creating card.
    pub address_country: Option<String>,
    #[serde(rename = "address_line1")]
    ///Address line 1 (Street address/PO Box/Company name).
    pub address_line1: Option<String>,
    #[serde(rename = "address_line1_check")]
    ///If `address_line1` was provided, results of the check: `pass`, `fail`, `unavailable`, or `unchecked`.
    pub address_line1_check: Option<String>,
    #[serde(rename = "address_line2")]
    ///Address line 2 (Apartment/Suite/Unit/Building).
    pub address_line2: Option<String>,
    #[serde(rename = "address_state")]
    ///State/County/Province/Region.
    pub address_state: Option<String>,
    #[serde(rename = "address_zip")]
    ///ZIP or postal code.
    pub address_zip: Option<String>,
    #[serde(rename = "address_zip_check")]
    ///If `address_zip` was provided, results of the check: `pass`, `fail`, `unavailable`, or `unchecked`.
    pub address_zip_check: Option<String>,
    #[serde(rename = "available_payout_methods")]
    ///A set of available payout methods for this card. Only values from this set should be passed as the `method` when creating a payout.
    pub available_payout_methods: Option<Vec<String>>,
    #[serde(rename = "brand")]
    ///Card brand. Can be `American Express`, `Diners Club`, `Discover`, `JCB`, `MasterCard`, `UnionPay`, `Visa`, or `Unknown`.
    pub brand: String,
    #[serde(rename = "country")]
    ///Two-letter ISO code representing the country of the card. You could use this attribute to get a sense of the international breakdown of cards you've collected.
    pub country: Option<String>,
    #[serde(rename = "currency")]
    ///Three-letter [ISO code for currency](https://stripe.com/docs/payouts). Only applicable on accounts (not customers or recipients). The card can be used as a transfer destination for funds in this currency.
    pub currency: Option<String>,
    #[serde(rename = "customer")]
    ///The customer that this card belongs to. This attribute will not be in the card object if the card belongs to an account or recipient instead.
    pub customer: Option<serde_json::Value>,
    #[serde(rename = "cvc_check")]
    ///If a CVC was provided, results of the check: `pass`, `fail`, `unavailable`, or `unchecked`. A result of unchecked indicates that CVC was provided but hasn't been checked yet. Checks are typically performed when attaching a card to a Customer object, or when creating a charge. For more details, see [Check if a card is valid without a charge](https://support.stripe.com/questions/check-if-a-card-is-valid-without-a-charge).
    pub cvc_check: Option<String>,
    #[serde(rename = "default_for_currency")]
    ///Whether this card is the default external account for its currency.
    pub default_for_currency: Option<bool>,
    #[serde(rename = "dynamic_last4")]
    ///(For tokenized numbers only.) The last four digits of the device account number.
    pub dynamic_last4: Option<String>,
    #[serde(rename = "exp_month")]
    ///Two-digit number representing the card's expiration month.
    pub exp_month: i64,
    #[serde(rename = "exp_year")]
    ///Four-digit number representing the card's expiration year.
    pub exp_year: i64,
    #[serde(rename = "fingerprint")]
    /**Uniquely identifies this particular card number. You can use this attribute to check whether two customers who’ve signed up with you are using the same card number, for example. For payment methods that tokenize card information (Apple Pay, Google Pay), the tokenized number might be provided instead of the underlying card number.

*Starting May 1, 2021, card fingerprint in India for Connect will change to allow two fingerprints for the same card --- one for India and one for the rest of the world.**/
    pub fingerprint: Option<String>,
    #[serde(rename = "funding")]
    ///Card funding type. Can be `credit`, `debit`, `prepaid`, or `unknown`.
    pub funding: String,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "last4")]
    ///The last four digits of the card.
    pub last4: String,
    #[serde(rename = "metadata")]
    ///Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<serde_json::Value>,
    #[serde(rename = "name")]
    ///Cardholder name.
    pub name: Option<String>,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "recipient")]
    ///The recipient that this card belongs to. This attribute will not be in the card object if the card belongs to a customer or account instead.
    pub recipient: Option<serde_json::Value>,
    #[serde(rename = "status")]
    ///For external accounts, possible values are `new` and `errored`. If a transfer fails, the status is set to `errored` and transfers are stopped until account details are updated.
    pub status: Option<String>,
    #[serde(rename = "tokenization_method")]
    ///If the card number is tokenized, this is the method that was used. Can be `android_pay` (includes Google Pay), `apple_pay`, `masterpass`, `visa_checkout`, or null.
    pub tokenization_method: Option<String>,
}
impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CardGeneratedFromPaymentMethodDetails {
    #[serde(rename = "card_present")]
    ///
    pub card_present: Option<PaymentMethodDetailsCardPresent>,
    #[serde(rename = "type")]
    ///The type of payment method transaction-specific details from the transaction that generated this `card` payment method. Always `card_present`.
    pub type_: String,
}
impl std::fmt::Display for CardGeneratedFromPaymentMethodDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CardIssuingAccountTermsOfService {
    #[serde(rename = "date")]
    ///The Unix timestamp marking when the account representative accepted the service agreement.
    pub date: Option<i64>,
    #[serde(rename = "ip")]
    ///The IP address from which the account representative accepted the service agreement.
    pub ip: Option<String>,
    #[serde(rename = "user_agent")]
    ///The user agent of the browser from which the account representative accepted the service agreement.
    pub user_agent: Option<String>,
}
impl std::fmt::Display for CardIssuingAccountTermsOfService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CardMandatePaymentMethodDetails {}
impl std::fmt::Display for CardMandatePaymentMethodDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CashBalance {
    #[serde(rename = "available")]
    ///A hash of all cash balances available to this customer. You cannot delete a customer with any cash balances, even if the balance is 0. Amounts are represented in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    pub available: Option<serde_json::Value>,
    #[serde(rename = "customer")]
    ///The ID of the customer whose cash balance this object represents.
    pub customer: String,
    #[serde(rename = "livemode")]
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "settings")]
    ///
    pub settings: CustomerBalanceCustomerBalanceSettings,
}
impl std::fmt::Display for CashBalance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Charge {
    #[serde(rename = "amount")]
    ///Amount intended to be collected by this payment. A positive integer representing how much to charge in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal) (e.g., 100 cents to charge $1.00 or 100 to charge ¥100, a zero-decimal currency). The minimum amount is $0.50 US or [equivalent in charge currency](https://stripe.com/docs/currencies#minimum-and-maximum-charge-amounts). The amount value supports up to eight digits (e.g., a value of 99999999 for a USD charge of $999,999.99).
    pub amount: i64,
    #[serde(rename = "amount_captured")]
    ///Amount in %s captured (can be less than the amount attribute on the charge if a partial capture was made).
    pub amount_captured: i64,
    #[serde(rename = "amount_refunded")]
    ///Amount in %s refunded (can be less than the amount attribute on the charge if a partial refund was issued).
    pub amount_refunded: i64,
    #[serde(rename = "application")]
    ///ID of the Connect application that created the charge.
    pub application: Option<serde_json::Value>,
    #[serde(rename = "application_fee")]
    ///The application fee (if any) for the charge. [See the Connect documentation](https://stripe.com/docs/connect/direct-charges#collecting-fees) for details.
    pub application_fee: Option<serde_json::Value>,
    #[serde(rename = "application_fee_amount")]
    ///The amount of the application fee (if any) requested for the charge. [See the Connect documentation](https://stripe.com/docs/connect/direct-charges#collecting-fees) for details.
    pub application_fee_amount: Option<i64>,
    #[serde(rename = "balance_transaction")]
    ///ID of the balance transaction that describes the impact of this charge on your account balance (not including refunds or disputes).
    pub balance_transaction: Option<serde_json::Value>,
    #[serde(rename = "billing_details")]
    ///
    pub billing_details: BillingDetails,
    #[serde(rename = "calculated_statement_descriptor")]
    ///The full statement descriptor that is passed to card networks, and that is displayed on your customers' credit card and bank statements. Allows you to see what the statement descriptor looks like after the static and dynamic portions are combined.
    pub calculated_statement_descriptor: Option<String>,
    #[serde(rename = "captured")]
    ///If the charge was created without capturing, this Boolean represents whether it is still uncaptured or has since been captured.
    pub captured: bool,
    #[serde(rename = "created")]
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    #[serde(rename = "currency")]
    ///Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: String,
    #[serde(rename = "customer")]
    ///ID of the customer this charge is for if one exists.
    pub customer: Option<serde_json::Value>,
    #[serde(rename = "description")]
    ///An arbitrary string attached to the object. Often useful for displaying to users.
    pub description: Option<String>,
    #[serde(rename = "disputed")]
    ///Whether the charge has been disputed.
    pub disputed: bool,
    #[serde(rename = "failure_balance_transaction")]
    ///ID of the balance transaction that describes the reversal of the balance on your account due to payment failure.
    pub failure_balance_transaction: Option<serde_json::Value>,
    #[serde(rename = "failure_code")]
    ///Error code explaining reason for charge failure if available (see [the errors section](https://stripe.com/docs/api#errors) for a list of codes).
    pub failure_code: Option<String>,
    #[serde(rename = "failure_message")]
    ///Message to user further explaining reason for charge failure if available.
    pub failure_message: Option<String>,
    #[serde(rename = "fraud_details")]
    ///Information on fraud assessments for the charge.
    pub fraud_details: Option<serde_json::Value>,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "invoice")]
    ///ID of the invoice this charge is for if one exists.
    pub invoice: Option<serde_json::Value>,
    #[serde(rename = "livemode")]
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    #[serde(rename = "metadata")]
    ///Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format.
    pub metadata: serde_json::Value,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "on_behalf_of")]
    ///The account (if any) the charge was made on behalf of without triggering an automatic transfer. See the [Connect documentation](https://stripe.com/docs/connect/charges-transfers) for details.
    pub on_behalf_of: Option<serde_json::Value>,
    #[serde(rename = "outcome")]
    ///Details about whether the payment was accepted, and why. See [understanding declines](https://stripe.com/docs/declines) for details.
    pub outcome: Option<serde_json::Value>,
    #[serde(rename = "paid")]
    ///`true` if the charge succeeded, or was successfully authorized for later capture.
    pub paid: bool,
    #[serde(rename = "payment_intent")]
    ///ID of the PaymentIntent associated with this charge, if one exists.
    pub payment_intent: Option<serde_json::Value>,
    #[serde(rename = "payment_method")]
    ///ID of the payment method used in this charge.
    pub payment_method: Option<String>,
    #[serde(rename = "payment_method_details")]
    ///Details about the payment method at the time of the transaction.
    pub payment_method_details: Option<serde_json::Value>,
    #[serde(rename = "radar_options")]
    ///Options to configure Radar. See [Radar Session](https://stripe.com/docs/radar/radar-session) for more information.
    pub radar_options: Option<RadarRadarOptions>,
    #[serde(rename = "receipt_email")]
    ///This is the email address that the receipt for this charge was sent to.
    pub receipt_email: Option<String>,
    #[serde(rename = "receipt_number")]
    ///This is the transaction number that appears on email receipts sent for this charge. This attribute will be `null` until a receipt has been sent.
    pub receipt_number: Option<String>,
    #[serde(rename = "receipt_url")]
    ///This is the URL to view the receipt for this charge. The receipt is kept up-to-date to the latest state of the charge, including any refunds. If the charge is for an Invoice, the receipt will be stylized as an Invoice receipt.
    pub receipt_url: Option<String>,
    #[serde(rename = "refunded")]
    ///Whether the charge has been fully refunded. If the charge is only partially refunded, this attribute will still be false.
    pub refunded: bool,
    #[serde(rename = "refunds")]
    ///A list of refunds that have been applied to the charge.
    pub refunds: serde_json::Value,
    #[serde(rename = "review")]
    ///ID of the review associated with this charge if one exists.
    pub review: Option<serde_json::Value>,
    #[serde(rename = "shipping")]
    ///Shipping information for the charge.
    pub shipping: Option<serde_json::Value>,
    #[serde(rename = "source_transfer")]
    ///The transfer ID which created this charge. Only present if the charge came from another Stripe account. [See the Connect documentation](https://stripe.com/docs/connect/destination-charges) for details.
    pub source_transfer: Option<serde_json::Value>,
    #[serde(rename = "statement_descriptor")]
    ///For card charges, use `statement_descriptor_suffix` instead. Otherwise, you can use this value as the complete description of a charge on your customers’ statements. Must contain at least one letter, maximum 22 characters.
    pub statement_descriptor: Option<String>,
    #[serde(rename = "statement_descriptor_suffix")]
    ///Provides information about the charge that customers see on their statements. Concatenated with the prefix (shortened descriptor) or statement descriptor that’s set on the account to form the complete statement descriptor. Maximum 22 characters for the concatenated descriptor.
    pub statement_descriptor_suffix: Option<String>,
    #[serde(rename = "status")]
    ///The status of the payment is either `succeeded`, `pending`, or `failed`.
    pub status: String,
    #[serde(rename = "transfer")]
    ///ID of the transfer to the `destination` account (only applicable if the charge was created using the `destination` parameter).
    pub transfer: Option<serde_json::Value>,
    #[serde(rename = "transfer_data")]
    ///An optional dictionary including the account to automatically transfer to as part of a destination charge. [See the Connect documentation](https://stripe.com/docs/connect/destination-charges) for details.
    pub transfer_data: Option<serde_json::Value>,
    #[serde(rename = "transfer_group")]
    ///A string that identifies this transaction as part of a group. See the [Connect documentation](https://stripe.com/docs/connect/charges-transfers#transfer-options) for details.
    pub transfer_group: Option<String>,
}
impl std::fmt::Display for Charge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ChargeFraudDetails {
    #[serde(rename = "stripe_report")]
    ///Assessments from Stripe. If set, the value is `fraudulent`.
    pub stripe_report: Option<String>,
    #[serde(rename = "user_report")]
    ///Assessments reported by you. If set, possible values of are `safe` and `fraudulent`.
    pub user_report: Option<String>,
}
impl std::fmt::Display for ChargeFraudDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ChargeOutcome {
    #[serde(rename = "network_status")]
    ///Possible values are `approved_by_network`, `declined_by_network`, `not_sent_to_network`, and `reversed_after_approval`. The value `reversed_after_approval` indicates the payment was [blocked by Stripe](https://stripe.com/docs/declines#blocked-payments) after bank authorization, and may temporarily appear as "pending" on a cardholder's statement.
    pub network_status: Option<String>,
    #[serde(rename = "reason")]
    ///An enumerated value providing a more detailed explanation of the outcome's `type`. Charges blocked by Radar's default block rule have the value `highest_risk_level`. Charges placed in review by Radar's default review rule have the value `elevated_risk_level`. Charges authorized, blocked, or placed in review by custom rules have the value `rule`. See [understanding declines](https://stripe.com/docs/declines) for more details.
    pub reason: Option<String>,
    #[serde(rename = "risk_level")]
    ///Stripe Radar's evaluation of the riskiness of the payment. Possible values for evaluated payments are `normal`, `elevated`, `highest`. For non-card payments, and card-based payments predating the public assignment of risk levels, this field will have the value `not_assessed`. In the event of an error in the evaluation, this field will have the value `unknown`. This field is only available with Radar.
    pub risk_level: Option<String>,
    #[serde(rename = "risk_score")]
    ///Stripe Radar's evaluation of the riskiness of the payment. Possible values for evaluated payments are between 0 and 100. For non-card payments, card-based payments predating the public assignment of risk scores, or in the event of an error during evaluation, this field will not be present. This field is only available with Radar for Fraud Teams.
    pub risk_score: Option<i64>,
    #[serde(rename = "rule")]
    ///The ID of the Radar rule that matched the payment, if applicable.
    pub rule: Option<serde_json::Value>,
    #[serde(rename = "seller_message")]
    ///A human-readable description of the outcome type and reason, designed for you (the recipient of the payment), not your customer.
    pub seller_message: Option<String>,
    #[serde(rename = "type")]
    ///Possible values are `authorized`, `manual_review`, `issuer_declined`, `blocked`, and `invalid`. See [understanding declines](https://stripe.com/docs/declines) and [Radar reviews](https://stripe.com/docs/radar/reviews) for details.
    pub type_: String,
}
impl std::fmt::Display for ChargeOutcome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ChargeTransferData {
    #[serde(rename = "amount")]
    ///The amount transferred to the destination account, if specified. By default, the entire charge amount is transferred to the destination account.
    pub amount: Option<i64>,
    #[serde(rename = "destination")]
    ///ID of an existing, connected Stripe account to transfer funds to if `transfer_data` was specified in the charge request.
    pub destination: serde_json::Value,
}
impl std::fmt::Display for ChargeTransferData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CheckoutSession {
    #[serde(rename = "after_expiration")]
    ///When set, provides configuration for actions to take if this Checkout Session expires.
    pub after_expiration: Option<serde_json::Value>,
    #[serde(rename = "allow_promotion_codes")]
    ///Enables user redeemable promotion codes.
    pub allow_promotion_codes: Option<bool>,
    #[serde(rename = "amount_subtotal")]
    ///Total of all items before discounts or taxes are applied.
    pub amount_subtotal: Option<i64>,
    #[serde(rename = "amount_total")]
    ///Total of all items after discounts and taxes are applied.
    pub amount_total: Option<i64>,
    #[serde(rename = "automatic_tax")]
    ///
    pub automatic_tax: PaymentPagesCheckoutSessionAutomaticTax,
    #[serde(rename = "billing_address_collection")]
    ///Describes whether Checkout should collect the customer's billing address.
    pub billing_address_collection: Option<String>,
    #[serde(rename = "cancel_url")]
    ///The URL the customer will be directed to if they decide to cancel payment and return to your website.
    pub cancel_url: String,
    #[serde(rename = "client_reference_id")]
    /**A unique string to reference the Checkout Session. This can be a
customer ID, a cart ID, or similar, and can be used to reconcile the
Session with your internal systems.*/
    pub client_reference_id: Option<String>,
    #[serde(rename = "consent")]
    ///Results of `consent_collection` for this session.
    pub consent: Option<serde_json::Value>,
    #[serde(rename = "consent_collection")]
    ///When set, provides configuration for the Checkout Session to gather active consent from customers.
    pub consent_collection: Option<serde_json::Value>,
    #[serde(rename = "currency")]
    ///Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Option<String>,
    #[serde(rename = "customer")]
    /**The ID of the customer for this Session.
For Checkout Sessions in `payment` or `subscription` mode, Checkout
will create a new customer object based on information provided
during the payment flow unless an existing customer was provided when
the Session was created.*/
    pub customer: Option<serde_json::Value>,
    #[serde(rename = "customer_creation")]
    ///Configure whether a Checkout Session creates a Customer when the Checkout Session completes.
    pub customer_creation: Option<String>,
    #[serde(rename = "customer_details")]
    ///The customer details including the customer's tax exempt status and the customer's tax IDs. Only the customer's email is present on Sessions in `setup` mode.
    pub customer_details: Option<serde_json::Value>,
    #[serde(rename = "customer_email")]
    /**If provided, this value will be used when the Customer object is created.
If not provided, customers will be asked to enter their email address.
Use this parameter to prefill customer data if you already have an email
on file. To access information about the customer once the payment flow is
complete, use the `customer` attribute.*/
    pub customer_email: Option<String>,
    #[serde(rename = "expires_at")]
    ///The timestamp at which the Checkout Session will expire.
    pub expires_at: i64,
    #[serde(rename = "id")]
    /**Unique identifier for the object. Used to pass to `redirectToCheckout`
in Stripe.js.*/
    pub id: String,
    #[serde(rename = "line_items")]
    ///The line items purchased by the customer.
    pub line_items: Option<serde_json::Value>,
    #[serde(rename = "livemode")]
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    #[serde(rename = "locale")]
    ///The IETF language tag of the locale Checkout is displayed in. If blank or `auto`, the browser's locale is used.
    pub locale: Option<String>,
    #[serde(rename = "metadata")]
    ///Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<serde_json::Value>,
    #[serde(rename = "mode")]
    ///The mode of the Checkout Session.
    pub mode: String,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "payment_intent")]
    ///The ID of the PaymentIntent for Checkout Sessions in `payment` mode.
    pub payment_intent: Option<serde_json::Value>,
    #[serde(rename = "payment_link")]
    ///The ID of the Payment Link that created this Session.
    pub payment_link: Option<serde_json::Value>,
    #[serde(rename = "payment_method_collection")]
    ///Configure whether a Checkout Session should collect a payment method.
    pub payment_method_collection: Option<String>,
    #[serde(rename = "payment_method_options")]
    ///Payment-method-specific configuration for the PaymentIntent or SetupIntent of this CheckoutSession.
    pub payment_method_options: Option<serde_json::Value>,
    #[serde(rename = "payment_method_types")]
    /**A list of the types of payment methods (e.g. card) this Checkout
Session is allowed to accept.*/
    pub payment_method_types: Vec<String>,
    #[serde(rename = "payment_status")]
    /**The payment status of the Checkout Session, one of `paid`, `unpaid`, or `no_payment_required`.
You can use this value to decide when to fulfill your customer's order.*/
    pub payment_status: String,
    #[serde(rename = "phone_number_collection")]
    ///
    pub phone_number_collection: Option<
        PaymentPagesCheckoutSessionPhoneNumberCollection,
    >,
    #[serde(rename = "recovered_from")]
    ///The ID of the original expired Checkout Session that triggered the recovery flow.
    pub recovered_from: Option<String>,
    #[serde(rename = "setup_intent")]
    ///The ID of the SetupIntent for Checkout Sessions in `setup` mode.
    pub setup_intent: Option<serde_json::Value>,
    #[serde(rename = "shipping_address_collection")]
    ///When set, provides configuration for Checkout to collect a shipping address from a customer.
    pub shipping_address_collection: Option<serde_json::Value>,
    #[serde(rename = "shipping_cost")]
    ///The details of the customer cost of shipping, including the customer chosen ShippingRate.
    pub shipping_cost: Option<serde_json::Value>,
    #[serde(rename = "shipping_details")]
    ///Shipping information for this Checkout Session.
    pub shipping_details: Option<serde_json::Value>,
    #[serde(rename = "shipping_options")]
    ///The shipping rate options applied to this Session.
    pub shipping_options: Vec<PaymentPagesCheckoutSessionShippingOption>,
    #[serde(rename = "status")]
    ///The status of the Checkout Session, one of `open`, `complete`, or `expired`.
    pub status: Option<String>,
    #[serde(rename = "submit_type")]
    /**Describes the type of transaction being performed by Checkout in order to customize
relevant text on the page, such as the submit button. `submit_type` can only be
specified on Checkout Sessions in `payment` mode, but not Checkout Sessions
in `subscription` or `setup` mode.*/
    pub submit_type: Option<String>,
    #[serde(rename = "subscription")]
    ///The ID of the subscription for Checkout Sessions in `subscription` mode.
    pub subscription: Option<serde_json::Value>,
    #[serde(rename = "success_url")]
    /**The URL the customer will be directed to after the payment or
subscription creation is successful.*/
    pub success_url: String,
    #[serde(rename = "tax_id_collection")]
    ///
    pub tax_id_collection: Option<PaymentPagesCheckoutSessionTaxIdCollection>,
    #[serde(rename = "total_details")]
    ///Tax and discount details for the computed total amount.
    pub total_details: Option<serde_json::Value>,
    #[serde(rename = "url")]
    ///The URL to the Checkout Session. Redirect customers to this URL to take them to Checkout. If you’re using [Custom Domains](https://stripe.com/docs/payments/checkout/custom-domains), the URL will use your subdomain. Otherwise, it’ll use `checkout.stripe.com.`
    pub url: Option<String>,
}
impl std::fmt::Display for CheckoutSession {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CheckoutAcssDebitMandateOptions {
    #[serde(rename = "custom_mandate_url")]
    ///A URL for custom mandate text
    pub custom_mandate_url: Option<String>,
    #[serde(rename = "default_for")]
    ///List of Stripe products where this mandate can be selected automatically. Returned when the Session is in `setup` mode.
    pub default_for: Option<Vec<String>>,
    #[serde(rename = "interval_description")]
    ///Description of the interval. Only required if the 'payment_schedule' parameter is 'interval' or 'combined'.
    pub interval_description: Option<String>,
    #[serde(rename = "payment_schedule")]
    ///Payment schedule for the mandate.
    pub payment_schedule: Option<String>,
    #[serde(rename = "transaction_type")]
    ///Transaction type of the mandate.
    pub transaction_type: Option<String>,
}
impl std::fmt::Display for CheckoutAcssDebitMandateOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CheckoutAcssDebitPaymentMethodOptions {
    #[serde(rename = "currency")]
    ///Currency supported by the bank account. Returned when the Session is in `setup` mode.
    pub currency: Option<String>,
    #[serde(rename = "mandate_options")]
    ///
    pub mandate_options: Option<CheckoutAcssDebitMandateOptions>,
    #[serde(rename = "setup_future_usage")]
    /**Indicates that you intend to make future payments with this PaymentIntent's payment method.

Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete. If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.

When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).*/
    pub setup_future_usage: Option<String>,
    #[serde(rename = "verification_method")]
    ///Bank account verification method.
    pub verification_method: Option<String>,
}
impl std::fmt::Display for CheckoutAcssDebitPaymentMethodOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CheckoutAffirmPaymentMethodOptions {
    #[serde(rename = "setup_future_usage")]
    /**Indicates that you intend to make future payments with this PaymentIntent's payment method.

Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete. If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.

When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).*/
    pub setup_future_usage: Option<String>,
}
impl std::fmt::Display for CheckoutAffirmPaymentMethodOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CheckoutAfterpayClearpayPaymentMethodOptions {
    #[serde(rename = "setup_future_usage")]
    /**Indicates that you intend to make future payments with this PaymentIntent's payment method.

Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete. If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.

When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).*/
    pub setup_future_usage: Option<String>,
}
impl std::fmt::Display for CheckoutAfterpayClearpayPaymentMethodOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CheckoutAlipayPaymentMethodOptions {
    #[serde(rename = "setup_future_usage")]
    /**Indicates that you intend to make future payments with this PaymentIntent's payment method.

Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete. If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.

When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).*/
    pub setup_future_usage: Option<String>,
}
impl std::fmt::Display for CheckoutAlipayPaymentMethodOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CheckoutAuBecsDebitPaymentMethodOptions {
    #[serde(rename = "setup_future_usage")]
    /**Indicates that you intend to make future payments with this PaymentIntent's payment method.

Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete. If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.

When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).*/
    pub setup_future_usage: Option<String>,
}
impl std::fmt::Display for CheckoutAuBecsDebitPaymentMethodOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CheckoutBacsDebitPaymentMethodOptions {
    #[serde(rename = "setup_future_usage")]
    /**Indicates that you intend to make future payments with this PaymentIntent's payment method.

Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete. If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.

When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).*/
    pub setup_future_usage: Option<String>,
}
impl std::fmt::Display for CheckoutBacsDebitPaymentMethodOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CheckoutBancontactPaymentMethodOptions {
    #[serde(rename = "setup_future_usage")]
    /**Indicates that you intend to make future payments with this PaymentIntent's payment method.

Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete. If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.

When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).*/
    pub setup_future_usage: Option<String>,
}
impl std::fmt::Display for CheckoutBancontactPaymentMethodOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CheckoutBoletoPaymentMethodOptions {
    #[serde(rename = "expires_after_days")]
    ///The number of calendar days before a Boleto voucher expires. For example, if you create a Boleto voucher on Monday and you set expires_after_days to 2, the Boleto voucher will expire on Wednesday at 23:59 America/Sao_Paulo time.
    pub expires_after_days: i64,
    #[serde(rename = "setup_future_usage")]
    /**Indicates that you intend to make future payments with this PaymentIntent's payment method.

Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete. If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.

When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).*/
    pub setup_future_usage: Option<String>,
}
impl std::fmt::Display for CheckoutBoletoPaymentMethodOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CheckoutCardInstallmentsOptions {
    #[serde(rename = "enabled")]
    ///Indicates if installments are enabled
    pub enabled: Option<bool>,
}
impl std::fmt::Display for CheckoutCardInstallmentsOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CheckoutCardPaymentMethodOptions {
    #[serde(rename = "installments")]
    ///
    pub installments: Option<CheckoutCardInstallmentsOptions>,
    #[serde(rename = "setup_future_usage")]
    /**Indicates that you intend to make future payments with this PaymentIntent's payment method.

Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete. If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.

When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).*/
    pub setup_future_usage: Option<String>,
    #[serde(rename = "statement_descriptor_suffix_kana")]
    ///Provides information about a card payment that customers see on their statements. Concatenated with the Kana prefix (shortened Kana descriptor) or Kana statement descriptor that’s set on the account to form the complete statement descriptor. Maximum 22 characters. On card statements, the *concatenation* of both prefix and suffix (including separators) will appear truncated to 22 characters.
    pub statement_descriptor_suffix_kana: Option<String>,
    #[serde(rename = "statement_descriptor_suffix_kanji")]
    ///Provides information about a card payment that customers see on their statements. Concatenated with the Kanji prefix (shortened Kanji descriptor) or Kanji statement descriptor that’s set on the account to form the complete statement descriptor. Maximum 17 characters. On card statements, the *concatenation* of both prefix and suffix (including separators) will appear truncated to 17 characters.
    pub statement_descriptor_suffix_kanji: Option<String>,
}
impl std::fmt::Display for CheckoutCardPaymentMethodOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CheckoutCustomerBalanceBankTransferPaymentMethodOptions {
    #[serde(rename = "eu_bank_transfer")]
    ///
    pub eu_bank_transfer: Option<PaymentMethodOptionsCustomerBalanceEuBankAccount>,
    #[serde(rename = "requested_address_types")]
    /**List of address types that should be returned in the financial_addresses response. If not specified, all valid types will be returned.

Permitted values include: `sort_code`, `zengin`, `iban`, or `spei`.*/
    pub requested_address_types: Option<Vec<String>>,
    #[serde(rename = "type")]
    ///The bank transfer type that this PaymentIntent is allowed to use for funding Permitted values include: `eu_bank_transfer`, `gb_bank_transfer`, `jp_bank_transfer`, or `mx_bank_transfer`.
    pub type_: Option<String>,
}
impl std::fmt::Display for CheckoutCustomerBalanceBankTransferPaymentMethodOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CheckoutCustomerBalancePaymentMethodOptions {
    #[serde(rename = "bank_transfer")]
    ///
    pub bank_transfer: Option<CheckoutCustomerBalanceBankTransferPaymentMethodOptions>,
    #[serde(rename = "funding_type")]
    ///The funding method type to be used when there are not enough funds in the customer balance. Permitted values include: `bank_transfer`.
    pub funding_type: Option<String>,
    #[serde(rename = "setup_future_usage")]
    /**Indicates that you intend to make future payments with this PaymentIntent's payment method.

Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete. If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.

When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).*/
    pub setup_future_usage: Option<String>,
}
impl std::fmt::Display for CheckoutCustomerBalancePaymentMethodOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CheckoutEpsPaymentMethodOptions {
    #[serde(rename = "setup_future_usage")]
    /**Indicates that you intend to make future payments with this PaymentIntent's payment method.

Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete. If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.

When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).*/
    pub setup_future_usage: Option<String>,
}
impl std::fmt::Display for CheckoutEpsPaymentMethodOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CheckoutFpxPaymentMethodOptions {
    #[serde(rename = "setup_future_usage")]
    /**Indicates that you intend to make future payments with this PaymentIntent's payment method.

Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete. If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.

When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).*/
    pub setup_future_usage: Option<String>,
}
impl std::fmt::Display for CheckoutFpxPaymentMethodOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CheckoutGiropayPaymentMethodOptions {
    #[serde(rename = "setup_future_usage")]
    /**Indicates that you intend to make future payments with this PaymentIntent's payment method.

Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete. If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.

When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).*/
    pub setup_future_usage: Option<String>,
}
impl std::fmt::Display for CheckoutGiropayPaymentMethodOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CheckoutGrabPayPaymentMethodOptions {
    #[serde(rename = "setup_future_usage")]
    /**Indicates that you intend to make future payments with this PaymentIntent's payment method.

Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete. If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.

When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).*/
    pub setup_future_usage: Option<String>,
}
impl std::fmt::Display for CheckoutGrabPayPaymentMethodOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CheckoutIdealPaymentMethodOptions {
    #[serde(rename = "setup_future_usage")]
    /**Indicates that you intend to make future payments with this PaymentIntent's payment method.

Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete. If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.

When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).*/
    pub setup_future_usage: Option<String>,
}
impl std::fmt::Display for CheckoutIdealPaymentMethodOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CheckoutKlarnaPaymentMethodOptions {
    #[serde(rename = "setup_future_usage")]
    /**Indicates that you intend to make future payments with this PaymentIntent's payment method.

Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete. If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.

When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).*/
    pub setup_future_usage: Option<String>,
}
impl std::fmt::Display for CheckoutKlarnaPaymentMethodOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CheckoutKonbiniPaymentMethodOptions {
    #[serde(rename = "expires_after_days")]
    ///The number of calendar days (between 1 and 60) after which Konbini payment instructions will expire. For example, if a PaymentIntent is confirmed with Konbini and `expires_after_days` set to 2 on Monday JST, the instructions will expire on Wednesday 23:59:59 JST.
    pub expires_after_days: Option<i64>,
    #[serde(rename = "setup_future_usage")]
    /**Indicates that you intend to make future payments with this PaymentIntent's payment method.

Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete. If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.

When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).*/
    pub setup_future_usage: Option<String>,
}
impl std::fmt::Display for CheckoutKonbiniPaymentMethodOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CheckoutOxxoPaymentMethodOptions {
    #[serde(rename = "expires_after_days")]
    ///The number of calendar days before an OXXO invoice expires. For example, if you create an OXXO invoice on Monday and you set expires_after_days to 2, the OXXO invoice will expire on Wednesday at 23:59 America/Mexico_City time.
    pub expires_after_days: i64,
    #[serde(rename = "setup_future_usage")]
    /**Indicates that you intend to make future payments with this PaymentIntent's payment method.

Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete. If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.

When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).*/
    pub setup_future_usage: Option<String>,
}
impl std::fmt::Display for CheckoutOxxoPaymentMethodOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CheckoutP24PaymentMethodOptions {
    #[serde(rename = "setup_future_usage")]
    /**Indicates that you intend to make future payments with this PaymentIntent's payment method.

Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete. If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.

When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).*/
    pub setup_future_usage: Option<String>,
}
impl std::fmt::Display for CheckoutP24PaymentMethodOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CheckoutPaynowPaymentMethodOptions {
    #[serde(rename = "setup_future_usage")]
    /**Indicates that you intend to make future payments with this PaymentIntent's payment method.

Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete. If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.

When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).*/
    pub setup_future_usage: Option<String>,
}
impl std::fmt::Display for CheckoutPaynowPaymentMethodOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CheckoutSepaDebitPaymentMethodOptions {
    #[serde(rename = "setup_future_usage")]
    /**Indicates that you intend to make future payments with this PaymentIntent's payment method.

Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete. If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.

When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).*/
    pub setup_future_usage: Option<String>,
}
impl std::fmt::Display for CheckoutSepaDebitPaymentMethodOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CheckoutSessionPaymentMethodOptions {
    #[serde(rename = "acss_debit")]
    ///
    pub acss_debit: Option<CheckoutAcssDebitPaymentMethodOptions>,
    #[serde(rename = "affirm")]
    ///
    pub affirm: Option<CheckoutAffirmPaymentMethodOptions>,
    #[serde(rename = "afterpay_clearpay")]
    ///
    pub afterpay_clearpay: Option<CheckoutAfterpayClearpayPaymentMethodOptions>,
    #[serde(rename = "alipay")]
    ///
    pub alipay: Option<CheckoutAlipayPaymentMethodOptions>,
    #[serde(rename = "au_becs_debit")]
    ///
    pub au_becs_debit: Option<CheckoutAuBecsDebitPaymentMethodOptions>,
    #[serde(rename = "bacs_debit")]
    ///
    pub bacs_debit: Option<CheckoutBacsDebitPaymentMethodOptions>,
    #[serde(rename = "bancontact")]
    ///
    pub bancontact: Option<CheckoutBancontactPaymentMethodOptions>,
    #[serde(rename = "boleto")]
    ///
    pub boleto: Option<CheckoutBoletoPaymentMethodOptions>,
    #[serde(rename = "card")]
    ///
    pub card: Option<CheckoutCardPaymentMethodOptions>,
    #[serde(rename = "customer_balance")]
    ///
    pub customer_balance: Option<CheckoutCustomerBalancePaymentMethodOptions>,
    #[serde(rename = "eps")]
    ///
    pub eps: Option<CheckoutEpsPaymentMethodOptions>,
    #[serde(rename = "fpx")]
    ///
    pub fpx: Option<CheckoutFpxPaymentMethodOptions>,
    #[serde(rename = "giropay")]
    ///
    pub giropay: Option<CheckoutGiropayPaymentMethodOptions>,
    #[serde(rename = "grabpay")]
    ///
    pub grabpay: Option<CheckoutGrabPayPaymentMethodOptions>,
    #[serde(rename = "ideal")]
    ///
    pub ideal: Option<CheckoutIdealPaymentMethodOptions>,
    #[serde(rename = "klarna")]
    ///
    pub klarna: Option<CheckoutKlarnaPaymentMethodOptions>,
    #[serde(rename = "konbini")]
    ///
    pub konbini: Option<CheckoutKonbiniPaymentMethodOptions>,
    #[serde(rename = "oxxo")]
    ///
    pub oxxo: Option<CheckoutOxxoPaymentMethodOptions>,
    #[serde(rename = "p24")]
    ///
    pub p24: Option<CheckoutP24PaymentMethodOptions>,
    #[serde(rename = "paynow")]
    ///
    pub paynow: Option<CheckoutPaynowPaymentMethodOptions>,
    #[serde(rename = "sepa_debit")]
    ///
    pub sepa_debit: Option<CheckoutSepaDebitPaymentMethodOptions>,
    #[serde(rename = "sofort")]
    ///
    pub sofort: Option<CheckoutSofortPaymentMethodOptions>,
    #[serde(rename = "us_bank_account")]
    ///
    pub us_bank_account: Option<CheckoutUsBankAccountPaymentMethodOptions>,
}
impl std::fmt::Display for CheckoutSessionPaymentMethodOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CheckoutSofortPaymentMethodOptions {
    #[serde(rename = "setup_future_usage")]
    /**Indicates that you intend to make future payments with this PaymentIntent's payment method.

Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete. If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.

When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).*/
    pub setup_future_usage: Option<String>,
}
impl std::fmt::Display for CheckoutSofortPaymentMethodOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CheckoutUsBankAccountPaymentMethodOptions {
    #[serde(rename = "financial_connections")]
    ///
    pub financial_connections: Option<LinkedAccountOptionsUsBankAccount>,
    #[serde(rename = "setup_future_usage")]
    /**Indicates that you intend to make future payments with this PaymentIntent's payment method.

Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete. If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.

When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).*/
    pub setup_future_usage: Option<String>,
    #[serde(rename = "verification_method")]
    ///Bank account verification method.
    pub verification_method: Option<String>,
}
impl std::fmt::Display for CheckoutUsBankAccountPaymentMethodOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ConnectCollectionTransfer {
    #[serde(rename = "amount")]
    ///Amount transferred, in %s.
    pub amount: i64,
    #[serde(rename = "currency")]
    ///Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: String,
    #[serde(rename = "destination")]
    ///ID of the account that funds are being collected for.
    pub destination: serde_json::Value,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "livemode")]
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
}
impl std::fmt::Display for ConnectCollectionTransfer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CountrySpec {
    #[serde(rename = "default_currency")]
    ///The default currency for this country. This applies to both payment methods and bank accounts.
    pub default_currency: String,
    #[serde(rename = "id")]
    ///Unique identifier for the object. Represented as the ISO country code for this country.
    pub id: String,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "supported_bank_account_currencies")]
    ///Currencies that can be accepted in the specific country (for transfers).
    pub supported_bank_account_currencies: serde_json::Value,
    #[serde(rename = "supported_payment_currencies")]
    ///Currencies that can be accepted in the specified country (for payments).
    pub supported_payment_currencies: Vec<String>,
    #[serde(rename = "supported_payment_methods")]
    ///Payment methods available in the specified country. You may need to enable some payment methods (e.g., [ACH](https://stripe.com/docs/ach)) on your account before they appear in this list. The `stripe` payment method refers to [charging through your platform](https://stripe.com/docs/connect/destination-charges).
    pub supported_payment_methods: Vec<String>,
    #[serde(rename = "supported_transfer_countries")]
    ///Countries that can accept transfers from the specified country.
    pub supported_transfer_countries: Vec<String>,
    #[serde(rename = "verification_fields")]
    ///
    pub verification_fields: CountrySpecVerificationFields,
}
impl std::fmt::Display for CountrySpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CountrySpecVerificationFieldDetails {
    #[serde(rename = "additional")]
    ///Additional fields which are only required for some users.
    pub additional: Vec<String>,
    #[serde(rename = "minimum")]
    ///Fields which every account must eventually provide.
    pub minimum: Vec<String>,
}
impl std::fmt::Display for CountrySpecVerificationFieldDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CountrySpecVerificationFields {
    #[serde(rename = "company")]
    ///
    pub company: CountrySpecVerificationFieldDetails,
    #[serde(rename = "individual")]
    ///
    pub individual: CountrySpecVerificationFieldDetails,
}
impl std::fmt::Display for CountrySpecVerificationFields {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Coupon {
    #[serde(rename = "amount_off")]
    ///Amount (in the `currency` specified) that will be taken off the subtotal of any invoices for this customer.
    pub amount_off: Option<i64>,
    #[serde(rename = "applies_to")]
    ///
    pub applies_to: Option<CouponAppliesTo>,
    #[serde(rename = "created")]
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    #[serde(rename = "currency")]
    ///If `amount_off` has been set, the three-letter [ISO code for the currency](https://stripe.com/docs/currencies) of the amount to take off.
    pub currency: Option<String>,
    #[serde(rename = "currency_options")]
    ///Coupons defined in each available currency option. Each key must be a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html) and a [supported currency](https://stripe.com/docs/currencies).
    pub currency_options: Option<serde_json::Value>,
    #[serde(rename = "duration")]
    ///One of `forever`, `once`, and `repeating`. Describes how long a customer who applies this coupon will get the discount.
    pub duration: String,
    #[serde(rename = "duration_in_months")]
    ///If `duration` is `repeating`, the number of months the coupon applies. Null if coupon `duration` is `forever` or `once`.
    pub duration_in_months: Option<i64>,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "livemode")]
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    #[serde(rename = "max_redemptions")]
    ///Maximum number of times this coupon can be redeemed, in total, across all customers, before it is no longer valid.
    pub max_redemptions: Option<i64>,
    #[serde(rename = "metadata")]
    ///Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<serde_json::Value>,
    #[serde(rename = "name")]
    ///Name of the coupon displayed to customers on for instance invoices or receipts.
    pub name: Option<String>,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "percent_off")]
    ///Percent that will be taken off the subtotal of any invoices for this customer for the duration of the coupon. For example, a coupon with percent_off of 50 will make a %s100 invoice %s50 instead.
    pub percent_off: Option<f64>,
    #[serde(rename = "redeem_by")]
    ///Date after which the coupon can no longer be redeemed.
    pub redeem_by: Option<i64>,
    #[serde(rename = "times_redeemed")]
    ///Number of times this coupon has been applied to a customer.
    pub times_redeemed: i64,
    #[serde(rename = "valid")]
    ///Taking account of the above properties, whether this coupon can still be applied to a customer.
    pub valid: bool,
}
impl std::fmt::Display for Coupon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CouponAppliesTo {
    #[serde(rename = "products")]
    ///A list of product IDs this coupon applies to
    pub products: Vec<String>,
}
impl std::fmt::Display for CouponAppliesTo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CouponCurrencyOption {
    #[serde(rename = "amount_off")]
    ///Amount (in the `currency` specified) that will be taken off the subtotal of any invoices for this customer.
    pub amount_off: i64,
}
impl std::fmt::Display for CouponCurrencyOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditNote {
    #[serde(rename = "amount")]
    ///The integer amount in %s representing the total amount of the credit note, including tax.
    pub amount: i64,
    #[serde(rename = "created")]
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    #[serde(rename = "currency")]
    ///Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: String,
    #[serde(rename = "customer")]
    ///ID of the customer.
    pub customer: serde_json::Value,
    #[serde(rename = "customer_balance_transaction")]
    ///Customer balance transaction related to this credit note.
    pub customer_balance_transaction: Option<serde_json::Value>,
    #[serde(rename = "discount_amount")]
    ///The integer amount in %s representing the total amount of discount that was credited.
    pub discount_amount: i64,
    #[serde(rename = "discount_amounts")]
    ///The aggregate amounts calculated per discount for all line items.
    pub discount_amounts: Vec<DiscountsResourceDiscountAmount>,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "invoice")]
    ///ID of the invoice.
    pub invoice: serde_json::Value,
    #[serde(rename = "lines")]
    ///Line items that make up the credit note
    pub lines: serde_json::Value,
    #[serde(rename = "livemode")]
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    #[serde(rename = "memo")]
    ///Customer-facing text that appears on the credit note PDF.
    pub memo: Option<String>,
    #[serde(rename = "metadata")]
    ///Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<serde_json::Value>,
    #[serde(rename = "number")]
    ///A unique number that identifies this particular credit note and appears on the PDF of the credit note and its associated invoice.
    pub number: String,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "out_of_band_amount")]
    ///Amount that was credited outside of Stripe.
    pub out_of_band_amount: Option<i64>,
    #[serde(rename = "pdf")]
    ///The link to download the PDF of the credit note.
    pub pdf: String,
    #[serde(rename = "reason")]
    ///Reason for issuing this credit note, one of `duplicate`, `fraudulent`, `order_change`, or `product_unsatisfactory`
    pub reason: Option<String>,
    #[serde(rename = "refund")]
    ///Refund related to this credit note.
    pub refund: Option<serde_json::Value>,
    #[serde(rename = "status")]
    ///Status of this credit note, one of `issued` or `void`. Learn more about [voiding credit notes](https://stripe.com/docs/billing/invoices/credit-notes#voiding).
    pub status: String,
    #[serde(rename = "subtotal")]
    ///The integer amount in %s representing the amount of the credit note, excluding exclusive tax and invoice level discounts.
    pub subtotal: i64,
    #[serde(rename = "subtotal_excluding_tax")]
    ///The integer amount in %s representing the amount of the credit note, excluding all tax and invoice level discounts.
    pub subtotal_excluding_tax: Option<i64>,
    #[serde(rename = "tax_amounts")]
    ///The aggregate amounts calculated per tax rate for all line items.
    pub tax_amounts: Vec<CreditNoteTaxAmount>,
    #[serde(rename = "total")]
    ///The integer amount in %s representing the total amount of the credit note, including tax and all discount.
    pub total: i64,
    #[serde(rename = "total_excluding_tax")]
    ///The integer amount in %s representing the total amount of the credit note, excluding tax, but including discounts.
    pub total_excluding_tax: Option<i64>,
    #[serde(rename = "type")]
    ///Type of this credit note, one of `pre_payment` or `post_payment`. A `pre_payment` credit note means it was issued when the invoice was open. A `post_payment` credit note means it was issued when the invoice was paid.
    pub type_: String,
    #[serde(rename = "voided_at")]
    ///The time that the credit note was voided.
    pub voided_at: Option<i64>,
}
impl std::fmt::Display for CreditNote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditNoteLineItem {
    #[serde(rename = "amount")]
    ///The integer amount in %s representing the gross amount being credited for this line item, excluding (exclusive) tax and discounts.
    pub amount: i64,
    #[serde(rename = "amount_excluding_tax")]
    ///The integer amount in %s representing the amount being credited for this line item, excluding all tax and discounts.
    pub amount_excluding_tax: Option<i64>,
    #[serde(rename = "description")]
    ///Description of the item being credited.
    pub description: Option<String>,
    #[serde(rename = "discount_amount")]
    ///The integer amount in %s representing the discount being credited for this line item.
    pub discount_amount: i64,
    #[serde(rename = "discount_amounts")]
    ///The amount of discount calculated per discount for this line item
    pub discount_amounts: Vec<DiscountsResourceDiscountAmount>,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "invoice_line_item")]
    ///ID of the invoice line item being credited
    pub invoice_line_item: Option<String>,
    #[serde(rename = "livemode")]
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "quantity")]
    ///The number of units of product being credited.
    pub quantity: Option<i64>,
    #[serde(rename = "tax_amounts")]
    ///The amount of tax calculated per tax rate for this line item
    pub tax_amounts: Vec<CreditNoteTaxAmount>,
    #[serde(rename = "tax_rates")]
    ///The tax rates which apply to the line item.
    pub tax_rates: Vec<TaxRate>,
    #[serde(rename = "type")]
    ///The type of the credit note line item, one of `invoice_line_item` or `custom_line_item`. When the type is `invoice_line_item` there is an additional `invoice_line_item` property on the resource the value of which is the id of the credited line item on the invoice.
    pub type_: String,
    #[serde(rename = "unit_amount")]
    ///The cost of each unit of product being credited.
    pub unit_amount: Option<i64>,
    #[serde(rename = "unit_amount_decimal")]
    ///Same as `unit_amount`, but contains a decimal value with at most 12 decimal places.
    pub unit_amount_decimal: Option<String>,
    #[serde(rename = "unit_amount_excluding_tax")]
    ///The amount in %s representing the unit amount being credited for this line item, excluding all tax and discounts.
    pub unit_amount_excluding_tax: Option<String>,
}
impl std::fmt::Display for CreditNoteLineItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditNoteTaxAmount {
    #[serde(rename = "amount")]
    ///The amount, in %s, of the tax.
    pub amount: i64,
    #[serde(rename = "inclusive")]
    ///Whether this tax amount is inclusive or exclusive.
    pub inclusive: bool,
    #[serde(rename = "tax_rate")]
    ///The tax rate that was applied to get this tax amount.
    pub tax_rate: serde_json::Value,
}
impl std::fmt::Display for CreditNoteTaxAmount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CurrencyOption {
    #[serde(rename = "custom_unit_amount")]
    ///When set, provides configuration for the amount to be adjusted by the customer during Checkout Sessions and Payment Links.
    pub custom_unit_amount: Option<serde_json::Value>,
    #[serde(rename = "tax_behavior")]
    ///Specifies whether the price is considered inclusive of taxes or exclusive of taxes. One of `inclusive`, `exclusive`, or `unspecified`. Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    pub tax_behavior: Option<String>,
    #[serde(rename = "tiers")]
    ///Each element represents a pricing tier. This parameter requires `billing_scheme` to be set to `tiered`. See also the documentation for `billing_scheme`.
    pub tiers: Option<Vec<PriceTier>>,
    #[serde(rename = "unit_amount")]
    ///The unit amount in %s to be charged, represented as a whole integer if possible. Only set if `billing_scheme=per_unit`.
    pub unit_amount: Option<i64>,
    #[serde(rename = "unit_amount_decimal")]
    ///The unit amount in %s to be charged, represented as a decimal string with at most 12 decimal places. Only set if `billing_scheme=per_unit`.
    pub unit_amount_decimal: Option<String>,
}
impl std::fmt::Display for CurrencyOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CustomUnitAmount {
    #[serde(rename = "maximum")]
    ///The maximum unit amount the customer can specify for this item.
    pub maximum: Option<i64>,
    #[serde(rename = "minimum")]
    ///The minimum unit amount the customer can specify for this item. Must be at least the minimum charge amount.
    pub minimum: Option<i64>,
    #[serde(rename = "preset")]
    ///The starting unit amount which can be updated by the customer.
    pub preset: Option<i64>,
}
impl std::fmt::Display for CustomUnitAmount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Customer {
    #[serde(rename = "address")]
    ///The customer's address.
    pub address: Option<serde_json::Value>,
    #[serde(rename = "balance")]
    ///Current balance, if any, being stored on the customer. If negative, the customer has credit to apply to their next invoice. If positive, the customer has an amount owed that will be added to their next invoice. The balance does not refer to any unpaid invoices; it solely takes into account amounts that have yet to be successfully applied to any invoice. This balance is only taken into account as invoices are finalized.
    pub balance: Option<i64>,
    #[serde(rename = "cash_balance")]
    ///The current funds being held by Stripe on behalf of the customer. These funds can be applied towards payment intents with source "cash_balance".The settings[reconciliation_mode] field describes whether these funds are applied to such payment intents manually or automatically.
    pub cash_balance: Option<serde_json::Value>,
    #[serde(rename = "created")]
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    #[serde(rename = "currency")]
    ///Three-letter [ISO code for the currency](https://stripe.com/docs/currencies) the customer can be charged in for recurring billing purposes.
    pub currency: Option<String>,
    #[serde(rename = "default_source")]
    /**ID of the default payment source for the customer.

If you are using payment methods created via the PaymentMethods API, see the [invoice_settings.default_payment_method](https://stripe.com/docs/api/customers/object#customer_object-invoice_settings-default_payment_method) field instead.*/
    pub default_source: Option<serde_json::Value>,
    #[serde(rename = "delinquent")]
    /**When the customer's latest invoice is billed by charging automatically, `delinquent` is `true` if the invoice's latest charge failed. When the customer's latest invoice is billed by sending an invoice, `delinquent` is `true` if the invoice isn't paid by its due date.

If an invoice is marked uncollectible by [dunning](https://stripe.com/docs/billing/automatic-collection), `delinquent` doesn't get reset to `false`.*/
    pub delinquent: Option<bool>,
    #[serde(rename = "description")]
    ///An arbitrary string attached to the object. Often useful for displaying to users.
    pub description: Option<String>,
    #[serde(rename = "discount")]
    ///Describes the current discount active on the customer, if there is one.
    pub discount: Option<serde_json::Value>,
    #[serde(rename = "email")]
    ///The customer's email address.
    pub email: Option<String>,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "invoice_credit_balance")]
    ///The current multi-currency balances, if any, being stored on the customer.If positive in a currency, the customer has a credit to apply to their next invoice denominated in that currency.If negative, the customer has an amount owed that will be added to their next invoice denominated in that currency. These balances do not refer to any unpaid invoices.They solely track amounts that have yet to be successfully applied to any invoice. A balance in a particular currency is only applied to any invoice as an invoice in that currency is finalized.
    pub invoice_credit_balance: Option<serde_json::Value>,
    #[serde(rename = "invoice_prefix")]
    ///The prefix for the customer used to generate unique invoice numbers.
    pub invoice_prefix: Option<String>,
    #[serde(rename = "invoice_settings")]
    ///
    pub invoice_settings: Option<InvoiceSettingCustomerSetting>,
    #[serde(rename = "livemode")]
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    #[serde(rename = "metadata")]
    ///Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<serde_json::Value>,
    #[serde(rename = "name")]
    ///The customer's full name or business name.
    pub name: Option<String>,
    #[serde(rename = "next_invoice_sequence")]
    ///The suffix of the customer's next invoice number, e.g., 0001.
    pub next_invoice_sequence: Option<i64>,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "phone")]
    ///The customer's phone number.
    pub phone: Option<String>,
    #[serde(rename = "preferred_locales")]
    ///The customer's preferred locales (languages), ordered by preference.
    pub preferred_locales: Option<Vec<String>>,
    #[serde(rename = "shipping")]
    ///Mailing and shipping address for the customer. Appears on invoices emailed to this customer.
    pub shipping: Option<serde_json::Value>,
    #[serde(rename = "sources")]
    ///The customer's payment sources, if any.
    pub sources: Option<serde_json::Value>,
    #[serde(rename = "subscriptions")]
    ///The customer's current subscriptions, if any.
    pub subscriptions: Option<serde_json::Value>,
    #[serde(rename = "tax")]
    ///
    pub tax: Option<CustomerTax>,
    #[serde(rename = "tax_exempt")]
    ///Describes the customer's tax exemption status. One of `none`, `exempt`, or `reverse`. When set to `reverse`, invoice and receipt PDFs include the text **"Reverse charge"**.
    pub tax_exempt: Option<String>,
    #[serde(rename = "tax_ids")]
    ///The customer's tax IDs.
    pub tax_ids: Option<serde_json::Value>,
    #[serde(rename = "test_clock")]
    ///ID of the test clock this customer belongs to.
    pub test_clock: Option<serde_json::Value>,
}
impl std::fmt::Display for Customer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CustomerAcceptance {
    #[serde(rename = "accepted_at")]
    ///The time at which the customer accepted the Mandate.
    pub accepted_at: Option<i64>,
    #[serde(rename = "offline")]
    ///
    pub offline: Option<OfflineAcceptance>,
    #[serde(rename = "online")]
    ///
    pub online: Option<OnlineAcceptance>,
    #[serde(rename = "type")]
    ///The type of customer acceptance information included with the Mandate. One of `online` or `offline`.
    pub type_: String,
}
impl std::fmt::Display for CustomerAcceptance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CustomerBalanceCustomerBalanceSettings {
    #[serde(rename = "reconciliation_mode")]
    ///The configuration for how funds that land in the customer cash balance are reconciled.
    pub reconciliation_mode: String,
}
impl std::fmt::Display for CustomerBalanceCustomerBalanceSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CustomerBalanceTransaction {
    #[serde(rename = "amount")]
    ///The amount of the transaction. A negative value is a credit for the customer's balance, and a positive value is a debit to the customer's `balance`.
    pub amount: i64,
    #[serde(rename = "created")]
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    #[serde(rename = "credit_note")]
    ///The ID of the credit note (if any) related to the transaction.
    pub credit_note: Option<serde_json::Value>,
    #[serde(rename = "currency")]
    ///Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: String,
    #[serde(rename = "customer")]
    ///The ID of the customer the transaction belongs to.
    pub customer: serde_json::Value,
    #[serde(rename = "description")]
    ///An arbitrary string attached to the object. Often useful for displaying to users.
    pub description: Option<String>,
    #[serde(rename = "ending_balance")]
    ///The customer's `balance` after the transaction was applied. A negative value decreases the amount due on the customer's next invoice. A positive value increases the amount due on the customer's next invoice.
    pub ending_balance: i64,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "invoice")]
    ///The ID of the invoice (if any) related to the transaction.
    pub invoice: Option<serde_json::Value>,
    #[serde(rename = "livemode")]
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    #[serde(rename = "metadata")]
    ///Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<serde_json::Value>,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "type")]
    ///Transaction type: `adjustment`, `applied_to_invoice`, `credit_note`, `initial`, `invoice_too_large`, `invoice_too_small`, `unspent_receiver_credit`, or `unapplied_from_invoice`. See the [Customer Balance page](https://stripe.com/docs/billing/customer/balance#types) to learn more about transaction types.
    pub type_: String,
}
impl std::fmt::Display for CustomerBalanceTransaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CustomerTax {
    #[serde(rename = "automatic_tax")]
    ///Surfaces if automatic tax computation is possible given the current customer location information.
    pub automatic_tax: String,
    #[serde(rename = "ip_address")]
    ///A recent IP address of the customer used for tax reporting and tax location inference.
    pub ip_address: Option<String>,
    #[serde(rename = "location")]
    ///The customer's location as identified by Stripe Tax.
    pub location: Option<serde_json::Value>,
}
impl std::fmt::Display for CustomerTax {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CustomerTaxLocation {
    #[serde(rename = "country")]
    ///The customer's country as identified by Stripe Tax.
    pub country: String,
    #[serde(rename = "source")]
    ///The data source used to infer the customer's location.
    pub source: String,
    #[serde(rename = "state")]
    ///The customer's state, county, province, or region as identified by Stripe Tax.
    pub state: Option<String>,
}
impl std::fmt::Display for CustomerTaxLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DeletedAccount {
    #[serde(rename = "deleted")]
    ///Always true for a deleted object
    pub deleted: bool,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
}
impl std::fmt::Display for DeletedAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DeletedApplePayDomain {
    #[serde(rename = "deleted")]
    ///Always true for a deleted object
    pub deleted: bool,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
}
impl std::fmt::Display for DeletedApplePayDomain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DeletedApplication {
    #[serde(rename = "deleted")]
    ///Always true for a deleted object
    pub deleted: bool,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "name")]
    ///The name of the application.
    pub name: Option<String>,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
}
impl std::fmt::Display for DeletedApplication {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DeletedBankAccount {
    #[serde(rename = "currency")]
    ///Three-letter [ISO code for the currency](https://stripe.com/docs/payouts) paid out to the bank account.
    pub currency: Option<String>,
    #[serde(rename = "deleted")]
    ///Always true for a deleted object
    pub deleted: bool,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
}
impl std::fmt::Display for DeletedBankAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DeletedCard {
    #[serde(rename = "currency")]
    ///Three-letter [ISO code for the currency](https://stripe.com/docs/payouts) paid out to the bank account.
    pub currency: Option<String>,
    #[serde(rename = "deleted")]
    ///Always true for a deleted object
    pub deleted: bool,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
}
impl std::fmt::Display for DeletedCard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DeletedCoupon {
    #[serde(rename = "deleted")]
    ///Always true for a deleted object
    pub deleted: bool,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
}
impl std::fmt::Display for DeletedCoupon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DeletedCustomer {
    #[serde(rename = "deleted")]
    ///Always true for a deleted object
    pub deleted: bool,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
}
impl std::fmt::Display for DeletedCustomer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DeletedDiscount {
    #[serde(rename = "checkout_session")]
    ///The Checkout session that this coupon is applied to, if it is applied to a particular session in payment mode. Will not be present for subscription mode.
    pub checkout_session: Option<String>,
    #[serde(rename = "coupon")]
    /**A coupon contains information about a percent-off or amount-off discount you
might want to apply to a customer. Coupons may be applied to [subscriptions](https://stripe.com/docs/api#subscriptions), [invoices](https://stripe.com/docs/api#invoices),
[checkout sessions](https://stripe.com/docs/api/checkout/sessions), [quotes](https://stripe.com/docs/api#quotes), and more. Coupons do not work with conventional one-off [charges](https://stripe.com/docs/api#create_charge) or [payment intents](https://stripe.com/docs/api/payment_intents).*/
    pub coupon: Coupon,
    #[serde(rename = "customer")]
    ///The ID of the customer associated with this discount.
    pub customer: Option<serde_json::Value>,
    #[serde(rename = "deleted")]
    ///Always true for a deleted object
    pub deleted: bool,
    #[serde(rename = "id")]
    ///The ID of the discount object. Discounts cannot be fetched by ID. Use `expand[]=discounts` in API calls to expand discount IDs in an array.
    pub id: String,
    #[serde(rename = "invoice")]
    ///The invoice that the discount's coupon was applied to, if it was applied directly to a particular invoice.
    pub invoice: Option<String>,
    #[serde(rename = "invoice_item")]
    ///The invoice item `id` (or invoice line item `id` for invoice line items of type='subscription') that the discount's coupon was applied to, if it was applied directly to a particular invoice item or invoice line item.
    pub invoice_item: Option<String>,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "promotion_code")]
    ///The promotion code applied to create this discount.
    pub promotion_code: Option<serde_json::Value>,
    #[serde(rename = "start")]
    ///Date that the coupon was applied.
    pub start: i64,
    #[serde(rename = "subscription")]
    ///The subscription that this coupon is applied to, if it is applied to a particular subscription.
    pub subscription: Option<String>,
}
impl std::fmt::Display for DeletedDiscount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DeletedExternalAccount(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct DeletedInvoice {
    #[serde(rename = "deleted")]
    ///Always true for a deleted object
    pub deleted: bool,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
}
impl std::fmt::Display for DeletedInvoice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DeletedInvoiceitem {
    #[serde(rename = "deleted")]
    ///Always true for a deleted object
    pub deleted: bool,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
}
impl std::fmt::Display for DeletedInvoiceitem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DeletedPaymentSource(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct DeletedPerson {
    #[serde(rename = "deleted")]
    ///Always true for a deleted object
    pub deleted: bool,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
}
impl std::fmt::Display for DeletedPerson {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DeletedPlan {
    #[serde(rename = "deleted")]
    ///Always true for a deleted object
    pub deleted: bool,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
}
impl std::fmt::Display for DeletedPlan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DeletedPrice {
    #[serde(rename = "deleted")]
    ///Always true for a deleted object
    pub deleted: bool,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
}
impl std::fmt::Display for DeletedPrice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DeletedProduct {
    #[serde(rename = "deleted")]
    ///Always true for a deleted object
    pub deleted: bool,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
}
impl std::fmt::Display for DeletedProduct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DeletedRadarValueList {
    #[serde(rename = "deleted")]
    ///Always true for a deleted object
    pub deleted: bool,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
}
impl std::fmt::Display for DeletedRadarValueList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DeletedRadarValueListItem {
    #[serde(rename = "deleted")]
    ///Always true for a deleted object
    pub deleted: bool,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
}
impl std::fmt::Display for DeletedRadarValueListItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DeletedSku {
    #[serde(rename = "deleted")]
    ///Always true for a deleted object
    pub deleted: bool,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
}
impl std::fmt::Display for DeletedSku {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DeletedSubscriptionItem {
    #[serde(rename = "deleted")]
    ///Always true for a deleted object
    pub deleted: bool,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
}
impl std::fmt::Display for DeletedSubscriptionItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DeletedTaxId {
    #[serde(rename = "deleted")]
    ///Always true for a deleted object
    pub deleted: bool,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
}
impl std::fmt::Display for DeletedTaxId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DeletedTerminalConfiguration {
    #[serde(rename = "deleted")]
    ///Always true for a deleted object
    pub deleted: bool,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
}
impl std::fmt::Display for DeletedTerminalConfiguration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DeletedTerminalLocation {
    #[serde(rename = "deleted")]
    ///Always true for a deleted object
    pub deleted: bool,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
}
impl std::fmt::Display for DeletedTerminalLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DeletedTerminalReader {
    #[serde(rename = "deleted")]
    ///Always true for a deleted object
    pub deleted: bool,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
}
impl std::fmt::Display for DeletedTerminalReader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DeletedTestHelpersTestClock {
    #[serde(rename = "deleted")]
    ///Always true for a deleted object
    pub deleted: bool,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
}
impl std::fmt::Display for DeletedTestHelpersTestClock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DeletedWebhookEndpoint {
    #[serde(rename = "deleted")]
    ///Always true for a deleted object
    pub deleted: bool,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
}
impl std::fmt::Display for DeletedWebhookEndpoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Discount {
    #[serde(rename = "checkout_session")]
    ///The Checkout session that this coupon is applied to, if it is applied to a particular session in payment mode. Will not be present for subscription mode.
    pub checkout_session: Option<String>,
    #[serde(rename = "coupon")]
    /**A coupon contains information about a percent-off or amount-off discount you
might want to apply to a customer. Coupons may be applied to [subscriptions](https://stripe.com/docs/api#subscriptions), [invoices](https://stripe.com/docs/api#invoices),
[checkout sessions](https://stripe.com/docs/api/checkout/sessions), [quotes](https://stripe.com/docs/api#quotes), and more. Coupons do not work with conventional one-off [charges](https://stripe.com/docs/api#create_charge) or [payment intents](https://stripe.com/docs/api/payment_intents).*/
    pub coupon: Coupon,
    #[serde(rename = "customer")]
    ///The ID of the customer associated with this discount.
    pub customer: Option<serde_json::Value>,
    #[serde(rename = "end")]
    ///If the coupon has a duration of `repeating`, the date that this discount will end. If the coupon has a duration of `once` or `forever`, this attribute will be null.
    pub end: Option<i64>,
    #[serde(rename = "id")]
    ///The ID of the discount object. Discounts cannot be fetched by ID. Use `expand[]=discounts` in API calls to expand discount IDs in an array.
    pub id: String,
    #[serde(rename = "invoice")]
    ///The invoice that the discount's coupon was applied to, if it was applied directly to a particular invoice.
    pub invoice: Option<String>,
    #[serde(rename = "invoice_item")]
    ///The invoice item `id` (or invoice line item `id` for invoice line items of type='subscription') that the discount's coupon was applied to, if it was applied directly to a particular invoice item or invoice line item.
    pub invoice_item: Option<String>,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "promotion_code")]
    ///The promotion code applied to create this discount.
    pub promotion_code: Option<serde_json::Value>,
    #[serde(rename = "start")]
    ///Date that the coupon was applied.
    pub start: i64,
    #[serde(rename = "subscription")]
    ///The subscription that this coupon is applied to, if it is applied to a particular subscription.
    pub subscription: Option<String>,
}
impl std::fmt::Display for Discount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DiscountsResourceDiscountAmount {
    #[serde(rename = "amount")]
    ///The amount, in %s, of the discount.
    pub amount: i64,
    #[serde(rename = "discount")]
    ///The discount that was applied to get this discount amount.
    pub discount: serde_json::Value,
}
impl std::fmt::Display for DiscountsResourceDiscountAmount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Dispute {
    #[serde(rename = "amount")]
    ///Disputed amount. Usually the amount of the charge, but can differ (usually because of currency fluctuation or because only part of the order is disputed).
    pub amount: i64,
    #[serde(rename = "balance_transactions")]
    ///List of zero, one, or two balance transactions that show funds withdrawn and reinstated to your Stripe account as a result of this dispute.
    pub balance_transactions: Vec<BalanceTransaction>,
    #[serde(rename = "charge")]
    ///ID of the charge that was disputed.
    pub charge: serde_json::Value,
    #[serde(rename = "created")]
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    #[serde(rename = "currency")]
    ///Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: String,
    #[serde(rename = "evidence")]
    ///
    pub evidence: DisputeEvidence,
    #[serde(rename = "evidence_details")]
    ///
    pub evidence_details: DisputeEvidenceDetails,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "is_charge_refundable")]
    ///If true, it is still possible to refund the disputed payment. Once the payment has been fully refunded, no further funds will be withdrawn from your Stripe account as a result of this dispute.
    pub is_charge_refundable: bool,
    #[serde(rename = "livemode")]
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    #[serde(rename = "metadata")]
    ///Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format.
    pub metadata: serde_json::Value,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "payment_intent")]
    ///ID of the PaymentIntent that was disputed.
    pub payment_intent: Option<serde_json::Value>,
    #[serde(rename = "reason")]
    ///Reason given by cardholder for dispute. Possible values are `bank_cannot_process`, `check_returned`, `credit_not_processed`, `customer_initiated`, `debit_not_authorized`, `duplicate`, `fraudulent`, `general`, `incorrect_account_details`, `insufficient_funds`, `product_not_received`, `product_unacceptable`, `subscription_canceled`, or `unrecognized`. Read more about [dispute reasons](https://stripe.com/docs/disputes/categories).
    pub reason: String,
    #[serde(rename = "status")]
    ///Current status of dispute. Possible values are `warning_needs_response`, `warning_under_review`, `warning_closed`, `needs_response`, `under_review`, `charge_refunded`, `won`, or `lost`.
    pub status: String,
}
impl std::fmt::Display for Dispute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DisputeEvidence {
    #[serde(rename = "access_activity_log")]
    ///Any server or activity logs showing proof that the customer accessed or downloaded the purchased digital product. This information should include IP addresses, corresponding timestamps, and any detailed recorded activity.
    pub access_activity_log: Option<String>,
    #[serde(rename = "billing_address")]
    ///The billing address provided by the customer.
    pub billing_address: Option<String>,
    #[serde(rename = "cancellation_policy")]
    ///(ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Your subscription cancellation policy, as shown to the customer.
    pub cancellation_policy: Option<serde_json::Value>,
    #[serde(rename = "cancellation_policy_disclosure")]
    ///An explanation of how and when the customer was shown your refund policy prior to purchase.
    pub cancellation_policy_disclosure: Option<String>,
    #[serde(rename = "cancellation_rebuttal")]
    ///A justification for why the customer's subscription was not canceled.
    pub cancellation_rebuttal: Option<String>,
    #[serde(rename = "customer_communication")]
    ///(ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Any communication with the customer that you feel is relevant to your case. Examples include emails proving that the customer received the product or service, or demonstrating their use of or satisfaction with the product or service.
    pub customer_communication: Option<serde_json::Value>,
    #[serde(rename = "customer_email_address")]
    ///The email address of the customer.
    pub customer_email_address: Option<String>,
    #[serde(rename = "customer_name")]
    ///The name of the customer.
    pub customer_name: Option<String>,
    #[serde(rename = "customer_purchase_ip")]
    ///The IP address that the customer used when making the purchase.
    pub customer_purchase_ip: Option<String>,
    #[serde(rename = "customer_signature")]
    ///(ID of a [file upload](https://stripe.com/docs/guides/file-upload)) A relevant document or contract showing the customer's signature.
    pub customer_signature: Option<serde_json::Value>,
    #[serde(rename = "duplicate_charge_documentation")]
    ///(ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Documentation for the prior charge that can uniquely identify the charge, such as a receipt, shipping label, work order, etc. This document should be paired with a similar document from the disputed payment that proves the two payments are separate.
    pub duplicate_charge_documentation: Option<serde_json::Value>,
    #[serde(rename = "duplicate_charge_explanation")]
    ///An explanation of the difference between the disputed charge versus the prior charge that appears to be a duplicate.
    pub duplicate_charge_explanation: Option<String>,
    #[serde(rename = "duplicate_charge_id")]
    ///The Stripe ID for the prior charge which appears to be a duplicate of the disputed charge.
    pub duplicate_charge_id: Option<String>,
    #[serde(rename = "product_description")]
    ///A description of the product or service that was sold.
    pub product_description: Option<String>,
    #[serde(rename = "receipt")]
    ///(ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Any receipt or message sent to the customer notifying them of the charge.
    pub receipt: Option<serde_json::Value>,
    #[serde(rename = "refund_policy")]
    ///(ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Your refund policy, as shown to the customer.
    pub refund_policy: Option<serde_json::Value>,
    #[serde(rename = "refund_policy_disclosure")]
    ///Documentation demonstrating that the customer was shown your refund policy prior to purchase.
    pub refund_policy_disclosure: Option<String>,
    #[serde(rename = "refund_refusal_explanation")]
    ///A justification for why the customer is not entitled to a refund.
    pub refund_refusal_explanation: Option<String>,
    #[serde(rename = "service_date")]
    ///The date on which the customer received or began receiving the purchased service, in a clear human-readable format.
    pub service_date: Option<String>,
    #[serde(rename = "service_documentation")]
    ///(ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Documentation showing proof that a service was provided to the customer. This could include a copy of a signed contract, work order, or other form of written agreement.
    pub service_documentation: Option<serde_json::Value>,
    #[serde(rename = "shipping_address")]
    ///The address to which a physical product was shipped. You should try to include as complete address information as possible.
    pub shipping_address: Option<String>,
    #[serde(rename = "shipping_carrier")]
    ///The delivery service that shipped a physical product, such as Fedex, UPS, USPS, etc. If multiple carriers were used for this purchase, please separate them with commas.
    pub shipping_carrier: Option<String>,
    #[serde(rename = "shipping_date")]
    ///The date on which a physical product began its route to the shipping address, in a clear human-readable format.
    pub shipping_date: Option<String>,
    #[serde(rename = "shipping_documentation")]
    ///(ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Documentation showing proof that a product was shipped to the customer at the same address the customer provided to you. This could include a copy of the shipment receipt, shipping label, etc. It should show the customer's full shipping address, if possible.
    pub shipping_documentation: Option<serde_json::Value>,
    #[serde(rename = "shipping_tracking_number")]
    ///The tracking number for a physical product, obtained from the delivery service. If multiple tracking numbers were generated for this purchase, please separate them with commas.
    pub shipping_tracking_number: Option<String>,
    #[serde(rename = "uncategorized_file")]
    ///(ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Any additional evidence or statements.
    pub uncategorized_file: Option<serde_json::Value>,
    #[serde(rename = "uncategorized_text")]
    ///Any additional evidence or statements.
    pub uncategorized_text: Option<String>,
}
impl std::fmt::Display for DisputeEvidence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DisputeEvidenceDetails {
    #[serde(rename = "due_by")]
    ///Date by which evidence must be submitted in order to successfully challenge dispute. Will be null if the customer's bank or credit card company doesn't allow a response for this particular dispute.
    pub due_by: Option<i64>,
    #[serde(rename = "has_evidence")]
    ///Whether evidence has been staged for this dispute.
    pub has_evidence: bool,
    #[serde(rename = "past_due")]
    ///Whether the last evidence submission was submitted past the due date. Defaults to `false` if no evidence submissions have occurred. If `true`, then delivery of the latest evidence is *not* guaranteed.
    pub past_due: bool,
    #[serde(rename = "submission_count")]
    ///The number of times evidence has been submitted. Typically, you may only submit evidence once.
    pub submission_count: i64,
}
impl std::fmt::Display for DisputeEvidenceDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EmailSent {
    #[serde(rename = "email_sent_at")]
    ///The timestamp when the email was sent.
    pub email_sent_at: i64,
    #[serde(rename = "email_sent_to")]
    ///The recipient's email address.
    pub email_sent_to: String,
}
impl std::fmt::Display for EmailSent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EphemeralKey {
    #[serde(rename = "created")]
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    #[serde(rename = "expires")]
    ///Time at which the key will expire. Measured in seconds since the Unix epoch.
    pub expires: i64,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "livemode")]
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "secret")]
    ///The key's secret. You can use this value to make authorized requests to the Stripe API.
    pub secret: Option<String>,
}
impl std::fmt::Display for EphemeralKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
    #[serde(rename = "error")]
    ///
    pub error: ApiErrors,
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    #[serde(rename = "account")]
    ///The connected account that originated the event.
    pub account: Option<String>,
    #[serde(rename = "api_version")]
    ///The Stripe API version used to render `data`. *Note: This property is populated only for events on or after October 31, 2014*.
    pub api_version: Option<String>,
    #[serde(rename = "created")]
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    #[serde(rename = "data")]
    ///
    pub data: NotificationEventData,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "livemode")]
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "pending_webhooks")]
    ///Number of webhooks that have yet to be successfully delivered (i.e., to return a 20x response) to the URLs you've specified.
    pub pending_webhooks: i64,
    #[serde(rename = "request")]
    ///Information on the API request that instigated the event.
    pub request: Option<serde_json::Value>,
    #[serde(rename = "type")]
    ///Description of the event (e.g., `invoice.created` or `charge.refunded`).
    pub type_: String,
}
impl std::fmt::Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ExchangeRate {
    #[serde(rename = "id")]
    ///Unique identifier for the object. Represented as the three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html) in lowercase.
    pub id: String,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "rates")]
    ///Hash where the keys are supported currencies and the values are the exchange rate at which the base id currency converts to the key currency.
    pub rates: serde_json::Value,
}
impl std::fmt::Display for ExchangeRate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalAccount(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct Fee {
    #[serde(rename = "amount")]
    ///Amount of the fee, in cents.
    pub amount: i64,
    #[serde(rename = "application")]
    ///ID of the Connect application that earned the fee.
    pub application: Option<String>,
    #[serde(rename = "currency")]
    ///Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: String,
    #[serde(rename = "description")]
    ///An arbitrary string attached to the object. Often useful for displaying to users.
    pub description: Option<String>,
    #[serde(rename = "type")]
    ///Type of the fee, one of: `application_fee`, `stripe_fee` or `tax`.
    pub type_: String,
}
impl std::fmt::Display for Fee {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct FeeRefund {
    #[serde(rename = "amount")]
    ///Amount, in %s.
    pub amount: i64,
    #[serde(rename = "balance_transaction")]
    ///Balance transaction that describes the impact on your account balance.
    pub balance_transaction: Option<serde_json::Value>,
    #[serde(rename = "created")]
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    #[serde(rename = "currency")]
    ///Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: String,
    #[serde(rename = "fee")]
    ///ID of the application fee that was refunded.
    pub fee: serde_json::Value,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "metadata")]
    ///Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<serde_json::Value>,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
}
impl std::fmt::Display for FeeRefund {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct File {
    #[serde(rename = "created")]
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    #[serde(rename = "expires_at")]
    ///The time at which the file expires and is no longer available in epoch seconds.
    pub expires_at: Option<i64>,
    #[serde(rename = "filename")]
    ///A filename for the file, suitable for saving to a filesystem.
    pub filename: Option<String>,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "links")]
    ///A list of [file links](https://stripe.com/docs/api#file_links) that point at this file.
    pub links: Option<serde_json::Value>,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "purpose")]
    ///The [purpose](https://stripe.com/docs/file-upload#uploading-a-file) of the uploaded file.
    pub purpose: String,
    #[serde(rename = "size")]
    ///The size in bytes of the file object.
    pub size: i64,
    #[serde(rename = "title")]
    ///A user friendly title for the document.
    pub title: Option<String>,
    #[serde(rename = "type")]
    ///The type of the file returned (e.g., `csv`, `pdf`, `jpg`, or `png`).
    pub type_: Option<String>,
    #[serde(rename = "url")]
    ///The URL from which the file can be downloaded using your live secret API key.
    pub url: Option<String>,
}
impl std::fmt::Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct FileLink {
    #[serde(rename = "created")]
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    #[serde(rename = "expired")]
    ///Whether this link is already expired.
    pub expired: bool,
    #[serde(rename = "expires_at")]
    ///Time at which the link expires.
    pub expires_at: Option<i64>,
    #[serde(rename = "file")]
    ///The file object this link points to.
    pub file: serde_json::Value,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "livemode")]
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    #[serde(rename = "metadata")]
    ///Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format.
    pub metadata: serde_json::Value,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "url")]
    ///The publicly accessible URL to download the file.
    pub url: Option<String>,
}
impl std::fmt::Display for FileLink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct FinancialConnectionsAccount {
    #[serde(rename = "account_holder")]
    ///The account holder that this account belongs to.
    pub account_holder: Option<serde_json::Value>,
    #[serde(rename = "balance")]
    ///The most recent information about the account's balance.
    pub balance: Option<serde_json::Value>,
    #[serde(rename = "balance_refresh")]
    ///The state of the most recent attempt to refresh the account balance.
    pub balance_refresh: Option<serde_json::Value>,
    #[serde(rename = "category")]
    ///The type of the account. Account category is further divided in `subcategory`.
    pub category: String,
    #[serde(rename = "created")]
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    #[serde(rename = "display_name")]
    ///A human-readable name that has been assigned to this account, either by the account holder or by the institution.
    pub display_name: Option<String>,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "institution_name")]
    ///The name of the institution that holds this account.
    pub institution_name: String,
    #[serde(rename = "last4")]
    ///The last 4 digits of the account number. If present, this will be 4 numeric characters.
    pub last4: Option<String>,
    #[serde(rename = "livemode")]
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "ownership")]
    ///The most recent information about the account's owners.
    pub ownership: Option<serde_json::Value>,
    #[serde(rename = "ownership_refresh")]
    ///The state of the most recent attempt to refresh the account owners.
    pub ownership_refresh: Option<serde_json::Value>,
    #[serde(rename = "permissions")]
    ///The list of permissions granted by this account.
    pub permissions: Option<Vec<String>>,
    #[serde(rename = "status")]
    ///The status of the link to the account.
    pub status: String,
    #[serde(rename = "subcategory")]
    /**If `category` is `cash`, one of:

 - `checking`
 - `savings`
 - `other`

If `category` is `credit`, one of:

 - `mortgage`
 - `line_of_credit`
 - `credit_card`
 - `other`

If `category` is `investment` or `other`, this will be `other`.*/
    pub subcategory: String,
    #[serde(rename = "supported_payment_method_types")]
    ///The [PaymentMethod type](https://stripe.com/docs/api/payment_methods/object#payment_method_object-type)(s) that can be created from this account.
    pub supported_payment_method_types: Vec<String>,
}
impl std::fmt::Display for FinancialConnectionsAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct FinancialConnectionsAccountOwner {
    #[serde(rename = "email")]
    ///The email address of the owner.
    pub email: Option<String>,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "name")]
    ///The full name of the owner.
    pub name: String,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "ownership")]
    ///The ownership object that this owner belongs to.
    pub ownership: String,
    #[serde(rename = "phone")]
    ///The raw phone number of the owner.
    pub phone: Option<String>,
    #[serde(rename = "raw_address")]
    ///The raw physical address of the owner.
    pub raw_address: Option<String>,
    #[serde(rename = "refreshed_at")]
    ///The timestamp of the refresh that updated this owner.
    pub refreshed_at: Option<i64>,
}
impl std::fmt::Display for FinancialConnectionsAccountOwner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct FinancialConnectionsAccountOwnership {
    #[serde(rename = "created")]
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "owners")]
    ///A paginated list of owners for this account.
    pub owners: serde_json::Value,
}
impl std::fmt::Display for FinancialConnectionsAccountOwnership {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct FinancialConnectionsSession {
    #[serde(rename = "account_holder")]
    ///The account holder for whom accounts are collected in this session.
    pub account_holder: Option<serde_json::Value>,
    #[serde(rename = "accounts")]
    ///The accounts that were collected as part of this Session.
    pub accounts: serde_json::Value,
    #[serde(rename = "client_secret")]
    ///A value that will be passed to the client to launch the authentication flow.
    pub client_secret: String,
    #[serde(rename = "filters")]
    ///
    pub filters: Option<BankConnectionsResourceLinkAccountSessionFilters>,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "livemode")]
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "permissions")]
    ///Permissions requested for accounts collected during this session.
    pub permissions: Vec<String>,
    #[serde(rename = "return_url")]
    ///For webview integrations only. Upon completing OAuth login in the native browser, the user will be redirected to this URL to return to your app.
    pub return_url: Option<String>,
}
impl std::fmt::Display for FinancialConnectionsSession {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct FinancialReportingFinanceReportRunRunParameters {
    #[serde(rename = "columns")]
    ///The set of output columns requested for inclusion in the report run.
    pub columns: Option<Vec<String>>,
    #[serde(rename = "connected_account")]
    ///Connected account ID by which to filter the report run.
    pub connected_account: Option<String>,
    #[serde(rename = "currency")]
    ///Currency of objects to be included in the report run.
    pub currency: Option<String>,
    #[serde(rename = "interval_end")]
    ///Ending timestamp of data to be included in the report run (exclusive).
    pub interval_end: Option<i64>,
    #[serde(rename = "interval_start")]
    ///Starting timestamp of data to be included in the report run.
    pub interval_start: Option<i64>,
    #[serde(rename = "payout")]
    ///Payout ID by which to filter the report run.
    pub payout: Option<String>,
    #[serde(rename = "reporting_category")]
    ///Category of balance transactions to be included in the report run.
    pub reporting_category: Option<String>,
    #[serde(rename = "timezone")]
    ///Defaults to `Etc/UTC`. The output timezone for all timestamps in the report. A list of possible time zone values is maintained at the [IANA Time Zone Database](http://www.iana.org/time-zones). Has no effect on `interval_start` or `interval_end`.
    pub timezone: Option<String>,
}
impl std::fmt::Display for FinancialReportingFinanceReportRunRunParameters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct FundingInstructions {
    #[serde(rename = "bank_transfer")]
    ///
    pub bank_transfer: FundingInstructionsBankTransfer,
    #[serde(rename = "currency")]
    ///Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: String,
    #[serde(rename = "funding_type")]
    ///The `funding_type` of the returned instructions
    pub funding_type: String,
    #[serde(rename = "livemode")]
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
}
impl std::fmt::Display for FundingInstructions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct FundingInstructionsBankTransfer {
    #[serde(rename = "country")]
    ///The country of the bank account to fund
    pub country: String,
    #[serde(rename = "financial_addresses")]
    ///A list of financial addresses that can be used to fund a particular balance
    pub financial_addresses: Vec<FundingInstructionsBankTransferFinancialAddress>,
    #[serde(rename = "type")]
    ///The bank_transfer type
    pub type_: String,
}
impl std::fmt::Display for FundingInstructionsBankTransfer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct FundingInstructionsBankTransferFinancialAddress {
    #[serde(rename = "iban")]
    ///Iban Records contain E.U. bank account details per the SEPA format.
    pub iban: Option<FundingInstructionsBankTransferIbanRecord>,
    #[serde(rename = "sort_code")]
    ///Sort Code Records contain U.K. bank account details per the sort code format.
    pub sort_code: Option<FundingInstructionsBankTransferSortCodeRecord>,
    #[serde(rename = "spei")]
    ///SPEI Records contain Mexico bank account details per the SPEI format.
    pub spei: Option<FundingInstructionsBankTransferSpeiRecord>,
    #[serde(rename = "supported_networks")]
    ///The payment networks supported by this FinancialAddress
    pub supported_networks: Option<Vec<String>>,
    #[serde(rename = "type")]
    ///The type of financial address
    pub type_: String,
    #[serde(rename = "zengin")]
    ///Zengin Records contain Japan bank account details per the Zengin format.
    pub zengin: Option<FundingInstructionsBankTransferZenginRecord>,
}
impl std::fmt::Display for FundingInstructionsBankTransferFinancialAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct FundingInstructionsBankTransferIbanRecord {
    #[serde(rename = "account_holder_name")]
    ///The name of the person or business that owns the bank account
    pub account_holder_name: String,
    #[serde(rename = "bic")]
    ///The BIC/SWIFT code of the account.
    pub bic: String,
    #[serde(rename = "country")]
    ///Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    pub country: String,
    #[serde(rename = "iban")]
    ///The IBAN of the account.
    pub iban: String,
}
impl std::fmt::Display for FundingInstructionsBankTransferIbanRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct FundingInstructionsBankTransferSortCodeRecord {
    #[serde(rename = "account_holder_name")]
    ///The name of the person or business that owns the bank account
    pub account_holder_name: String,
    #[serde(rename = "account_number")]
    ///The account number
    pub account_number: String,
    #[serde(rename = "sort_code")]
    ///The six-digit sort code
    pub sort_code: String,
}
impl std::fmt::Display for FundingInstructionsBankTransferSortCodeRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct FundingInstructionsBankTransferSpeiRecord {
    #[serde(rename = "bank_code")]
    ///The three-digit bank code
    pub bank_code: String,
    #[serde(rename = "bank_name")]
    ///The short banking institution name
    pub bank_name: String,
    #[serde(rename = "clabe")]
    ///The CLABE number
    pub clabe: String,
}
impl std::fmt::Display for FundingInstructionsBankTransferSpeiRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct FundingInstructionsBankTransferZenginRecord {
    #[serde(rename = "account_holder_name")]
    ///The account holder name
    pub account_holder_name: Option<String>,
    #[serde(rename = "account_number")]
    ///The account number
    pub account_number: Option<String>,
    #[serde(rename = "account_type")]
    ///The bank account type. In Japan, this can only be `futsu` or `toza`.
    pub account_type: Option<String>,
    #[serde(rename = "bank_code")]
    ///The bank code of the account
    pub bank_code: Option<String>,
    #[serde(rename = "bank_name")]
    ///The bank name of the account
    pub bank_name: Option<String>,
    #[serde(rename = "branch_code")]
    ///The branch code of the account
    pub branch_code: Option<String>,
    #[serde(rename = "branch_name")]
    ///The branch name of the account
    pub branch_name: Option<String>,
}
impl std::fmt::Display for FundingInstructionsBankTransferZenginRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct GelatoDataDocumentReportDateOfBirth {
    #[serde(rename = "day")]
    ///Numerical day between 1 and 31.
    pub day: Option<i64>,
    #[serde(rename = "month")]
    ///Numerical month between 1 and 12.
    pub month: Option<i64>,
    #[serde(rename = "year")]
    ///The four-digit year.
    pub year: Option<i64>,
}
impl std::fmt::Display for GelatoDataDocumentReportDateOfBirth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct GelatoDataDocumentReportExpirationDate {
    #[serde(rename = "day")]
    ///Numerical day between 1 and 31.
    pub day: Option<i64>,
    #[serde(rename = "month")]
    ///Numerical month between 1 and 12.
    pub month: Option<i64>,
    #[serde(rename = "year")]
    ///The four-digit year.
    pub year: Option<i64>,
}
impl std::fmt::Display for GelatoDataDocumentReportExpirationDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct GelatoDataDocumentReportIssuedDate {
    #[serde(rename = "day")]
    ///Numerical day between 1 and 31.
    pub day: Option<i64>,
    #[serde(rename = "month")]
    ///Numerical month between 1 and 12.
    pub month: Option<i64>,
    #[serde(rename = "year")]
    ///The four-digit year.
    pub year: Option<i64>,
}
impl std::fmt::Display for GelatoDataDocumentReportIssuedDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct GelatoDataIdNumberReportDate {
    #[serde(rename = "day")]
    ///Numerical day between 1 and 31.
    pub day: Option<i64>,
    #[serde(rename = "month")]
    ///Numerical month between 1 and 12.
    pub month: Option<i64>,
    #[serde(rename = "year")]
    ///The four-digit year.
    pub year: Option<i64>,
}
impl std::fmt::Display for GelatoDataIdNumberReportDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct GelatoDataVerifiedOutputsDate {
    #[serde(rename = "day")]
    ///Numerical day between 1 and 31.
    pub day: Option<i64>,
    #[serde(rename = "month")]
    ///Numerical month between 1 and 12.
    pub month: Option<i64>,
    #[serde(rename = "year")]
    ///The four-digit year.
    pub year: Option<i64>,
}
impl std::fmt::Display for GelatoDataVerifiedOutputsDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct GelatoDocumentReport {
    #[serde(rename = "address")]
    ///Address as it appears in the document.
    pub address: Option<serde_json::Value>,
    #[serde(rename = "dob")]
    ///Date of birth as it appears in the document.
    pub dob: Option<serde_json::Value>,
    #[serde(rename = "error")]
    ///Details on the verification error. Present when status is `unverified`.
    pub error: Option<serde_json::Value>,
    #[serde(rename = "expiration_date")]
    ///Expiration date of the document.
    pub expiration_date: Option<serde_json::Value>,
    #[serde(rename = "files")]
    ///Array of [File](https://stripe.com/docs/api/files) ids containing images for this document.
    pub files: Option<Vec<String>>,
    #[serde(rename = "first_name")]
    ///First name as it appears in the document.
    pub first_name: Option<String>,
    #[serde(rename = "issued_date")]
    ///Issued date of the document.
    pub issued_date: Option<serde_json::Value>,
    #[serde(rename = "issuing_country")]
    ///Issuing country of the document.
    pub issuing_country: Option<String>,
    #[serde(rename = "last_name")]
    ///Last name as it appears in the document.
    pub last_name: Option<String>,
    #[serde(rename = "number")]
    ///Document ID number.
    pub number: Option<String>,
    #[serde(rename = "status")]
    ///Status of this `document` check.
    pub status: String,
    #[serde(rename = "type")]
    ///Type of the document.
    pub type_: Option<String>,
}
impl std::fmt::Display for GelatoDocumentReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct GelatoDocumentReportError {
    #[serde(rename = "code")]
    ///A short machine-readable string giving the reason for the verification failure.
    pub code: Option<String>,
    #[serde(rename = "reason")]
    ///A human-readable message giving the reason for the failure. These messages can be shown to your users.
    pub reason: Option<String>,
}
impl std::fmt::Display for GelatoDocumentReportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct GelatoIdNumberReport {
    #[serde(rename = "dob")]
    ///Date of birth.
    pub dob: Option<serde_json::Value>,
    #[serde(rename = "error")]
    ///Details on the verification error. Present when status is `unverified`.
    pub error: Option<serde_json::Value>,
    #[serde(rename = "first_name")]
    ///First name.
    pub first_name: Option<String>,
    #[serde(rename = "id_number")]
    ///ID number.
    pub id_number: Option<String>,
    #[serde(rename = "id_number_type")]
    ///Type of ID number.
    pub id_number_type: Option<String>,
    #[serde(rename = "last_name")]
    ///Last name.
    pub last_name: Option<String>,
    #[serde(rename = "status")]
    ///Status of this `id_number` check.
    pub status: String,
}
impl std::fmt::Display for GelatoIdNumberReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct GelatoIdNumberReportError {
    #[serde(rename = "code")]
    ///A short machine-readable string giving the reason for the verification failure.
    pub code: Option<String>,
    #[serde(rename = "reason")]
    ///A human-readable message giving the reason for the failure. These messages can be shown to your users.
    pub reason: Option<String>,
}
impl std::fmt::Display for GelatoIdNumberReportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct GelatoReportDocumentOptions {
    #[serde(rename = "allowed_types")]
    ///Array of strings of allowed identity document types. If the provided identity document isn’t one of the allowed types, the verification check will fail with a document_type_not_allowed error code.
    pub allowed_types: Option<Vec<String>>,
    #[serde(rename = "require_id_number")]
    ///Collect an ID number and perform an [ID number check](https://stripe.com/docs/identity/verification-checks?type=id-number) with the document’s extracted name and date of birth.
    pub require_id_number: Option<bool>,
    #[serde(rename = "require_live_capture")]
    ///Disable image uploads, identity document images have to be captured using the device’s camera.
    pub require_live_capture: Option<bool>,
    #[serde(rename = "require_matching_selfie")]
    ///Capture a face image and perform a [selfie check](https://stripe.com/docs/identity/verification-checks?type=selfie) comparing a photo ID and a picture of your user’s face. [Learn more](https://stripe.com/docs/identity/selfie).
    pub require_matching_selfie: Option<bool>,
}
impl std::fmt::Display for GelatoReportDocumentOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct GelatoReportIdNumberOptions {}
impl std::fmt::Display for GelatoReportIdNumberOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct GelatoSelfieReport {
    #[serde(rename = "document")]
    ///ID of the [File](https://stripe.com/docs/api/files) holding the image of the identity document used in this check.
    pub document: Option<String>,
    #[serde(rename = "error")]
    ///Details on the verification error. Present when status is `unverified`.
    pub error: Option<serde_json::Value>,
    #[serde(rename = "selfie")]
    ///ID of the [File](https://stripe.com/docs/api/files) holding the image of the selfie used in this check.
    pub selfie: Option<String>,
    #[serde(rename = "status")]
    ///Status of this `selfie` check.
    pub status: String,
}
impl std::fmt::Display for GelatoSelfieReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct GelatoSelfieReportError {
    #[serde(rename = "code")]
    ///A short machine-readable string giving the reason for the verification failure.
    pub code: Option<String>,
    #[serde(rename = "reason")]
    ///A human-readable message giving the reason for the failure. These messages can be shown to your users.
    pub reason: Option<String>,
}
impl std::fmt::Display for GelatoSelfieReportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct GelatoSessionDocumentOptions {
    #[serde(rename = "allowed_types")]
    ///Array of strings of allowed identity document types. If the provided identity document isn’t one of the allowed types, the verification check will fail with a document_type_not_allowed error code.
    pub allowed_types: Option<Vec<String>>,
    #[serde(rename = "require_id_number")]
    ///Collect an ID number and perform an [ID number check](https://stripe.com/docs/identity/verification-checks?type=id-number) with the document’s extracted name and date of birth.
    pub require_id_number: Option<bool>,
    #[serde(rename = "require_live_capture")]
    ///Disable image uploads, identity document images have to be captured using the device’s camera.
    pub require_live_capture: Option<bool>,
    #[serde(rename = "require_matching_selfie")]
    ///Capture a face image and perform a [selfie check](https://stripe.com/docs/identity/verification-checks?type=selfie) comparing a photo ID and a picture of your user’s face. [Learn more](https://stripe.com/docs/identity/selfie).
    pub require_matching_selfie: Option<bool>,
}
impl std::fmt::Display for GelatoSessionDocumentOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct GelatoSessionIdNumberOptions {}
impl std::fmt::Display for GelatoSessionIdNumberOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct GelatoSessionLastError {
    #[serde(rename = "code")]
    ///A short machine-readable string giving the reason for the verification or user-session failure.
    pub code: Option<String>,
    #[serde(rename = "reason")]
    ///A message that explains the reason for verification or user-session failure.
    pub reason: Option<String>,
}
impl std::fmt::Display for GelatoSessionLastError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct GelatoVerificationReportOptions {
    #[serde(rename = "document")]
    ///
    pub document: Option<GelatoReportDocumentOptions>,
    #[serde(rename = "id_number")]
    ///
    pub id_number: Option<GelatoReportIdNumberOptions>,
}
impl std::fmt::Display for GelatoVerificationReportOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct GelatoVerificationSessionOptions {
    #[serde(rename = "document")]
    ///
    pub document: Option<GelatoSessionDocumentOptions>,
    #[serde(rename = "id_number")]
    ///
    pub id_number: Option<GelatoSessionIdNumberOptions>,
}
impl std::fmt::Display for GelatoVerificationSessionOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct GelatoVerifiedOutputs {
    #[serde(rename = "address")]
    ///The user's verified address.
    pub address: Option<serde_json::Value>,
    #[serde(rename = "dob")]
    ///The user’s verified date of birth.
    pub dob: Option<serde_json::Value>,
    #[serde(rename = "first_name")]
    ///The user's verified first name.
    pub first_name: Option<String>,
    #[serde(rename = "id_number")]
    ///The user's verified id number.
    pub id_number: Option<String>,
    #[serde(rename = "id_number_type")]
    ///The user's verified id number type.
    pub id_number_type: Option<String>,
    #[serde(rename = "last_name")]
    ///The user's verified last name.
    pub last_name: Option<String>,
}
impl std::fmt::Display for GelatoVerifiedOutputs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IdentityVerificationReport {
    #[serde(rename = "created")]
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    #[serde(rename = "document")]
    ///Result from a document check
    pub document: Option<GelatoDocumentReport>,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "id_number")]
    ///Result from an id_number check
    pub id_number: Option<GelatoIdNumberReport>,
    #[serde(rename = "livemode")]
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "options")]
    ///
    pub options: GelatoVerificationReportOptions,
    #[serde(rename = "selfie")]
    ///Result from a selfie check
    pub selfie: Option<GelatoSelfieReport>,
    #[serde(rename = "type")]
    ///Type of report.
    pub type_: String,
    #[serde(rename = "verification_session")]
    ///ID of the VerificationSession that created this report.
    pub verification_session: Option<String>,
}
impl std::fmt::Display for IdentityVerificationReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IdentityVerificationSession {
    #[serde(rename = "client_secret")]
    ///The short-lived client secret used by Stripe.js to [show a verification modal](https://stripe.com/docs/js/identity/modal) inside your app. This client secret expires after 24 hours and can only be used once. Don’t store it, log it, embed it in a URL, or expose it to anyone other than the user. Make sure that you have TLS enabled on any page that includes the client secret. Refer to our docs on [passing the client secret to the frontend](https://stripe.com/docs/identity/verification-sessions#client-secret) to learn more.
    pub client_secret: Option<String>,
    #[serde(rename = "created")]
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "last_error")]
    ///If present, this property tells you the last error encountered when processing the verification.
    pub last_error: Option<serde_json::Value>,
    #[serde(rename = "last_verification_report")]
    ///ID of the most recent VerificationReport. [Learn more about accessing detailed verification results.](https://stripe.com/docs/identity/verification-sessions#results)
    pub last_verification_report: Option<serde_json::Value>,
    #[serde(rename = "livemode")]
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    #[serde(rename = "metadata")]
    ///Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format.
    pub metadata: serde_json::Value,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "options")]
    ///
    pub options: GelatoVerificationSessionOptions,
    #[serde(rename = "redaction")]
    ///Redaction status of this VerificationSession. If the VerificationSession is not redacted, this field will be null.
    pub redaction: Option<serde_json::Value>,
    #[serde(rename = "status")]
    ///Status of this VerificationSession. [Learn more about the lifecycle of sessions](https://stripe.com/docs/identity/how-sessions-work).
    pub status: String,
    #[serde(rename = "type")]
    ///The type of [verification check](https://stripe.com/docs/identity/verification-checks) to be performed.
    pub type_: String,
    #[serde(rename = "url")]
    ///The short-lived URL that you use to redirect a user to Stripe to submit their identity information. This URL expires after 48 hours and can only be used once. Don’t store it, log it, send it in emails or expose it to anyone other than the user. Refer to our docs on [verifying identity documents](https://stripe.com/docs/identity/verify-identity-documents?platform=web&type=redirect) to learn how to redirect users to Stripe.
    pub url: Option<String>,
    #[serde(rename = "verified_outputs")]
    ///The user’s verified data.
    pub verified_outputs: Option<serde_json::Value>,
}
impl std::fmt::Display for IdentityVerificationSession {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InboundTransfers {
    #[serde(rename = "billing_details")]
    ///
    pub billing_details: TreasurySharedResourceBillingDetails,
    #[serde(rename = "type")]
    ///The type of the payment method used in the InboundTransfer.
    pub type_: String,
    #[serde(rename = "us_bank_account")]
    ///
    pub us_bank_account: Option<InboundTransfersPaymentMethodDetailsUsBankAccount>,
}
impl std::fmt::Display for InboundTransfers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InboundTransfersPaymentMethodDetailsUsBankAccount {
    #[serde(rename = "account_holder_type")]
    ///Account holder type: individual or company.
    pub account_holder_type: Option<String>,
    #[serde(rename = "account_type")]
    ///Account type: checkings or savings. Defaults to checking if omitted.
    pub account_type: Option<String>,
    #[serde(rename = "bank_name")]
    ///Name of the bank associated with the bank account.
    pub bank_name: Option<String>,
    #[serde(rename = "fingerprint")]
    ///Uniquely identifies this particular bank account. You can use this attribute to check whether two bank accounts are the same.
    pub fingerprint: Option<String>,
    #[serde(rename = "last4")]
    ///Last four digits of the bank account number.
    pub last4: Option<String>,
    #[serde(rename = "network")]
    ///The US bank account network used to debit funds.
    pub network: String,
    #[serde(rename = "routing_number")]
    ///Routing number of the bank account.
    pub routing_number: Option<String>,
}
impl std::fmt::Display for InboundTransfersPaymentMethodDetailsUsBankAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Invoice {
    #[serde(rename = "account_country")]
    ///The country of the business associated with this invoice, most often the business creating the invoice.
    pub account_country: Option<String>,
    #[serde(rename = "account_name")]
    ///The public name of the business associated with this invoice, most often the business creating the invoice.
    pub account_name: Option<String>,
    #[serde(rename = "account_tax_ids")]
    ///The account tax IDs associated with the invoice. Only editable when the invoice is a draft.
    pub account_tax_ids: Option<Vec<serde_json::Value>>,
    #[serde(rename = "amount_due")]
    ///Final amount due at this time for this invoice. If the invoice's total is smaller than the minimum charge amount, for example, or if there is account credit that can be applied to the invoice, the `amount_due` may be 0. If there is a positive `starting_balance` for the invoice (the customer owes money), the `amount_due` will also take that into account. The charge that gets generated for the invoice will be for the amount specified in `amount_due`.
    pub amount_due: i64,
    #[serde(rename = "amount_paid")]
    ///The amount, in %s, that was paid.
    pub amount_paid: i64,
    #[serde(rename = "amount_remaining")]
    ///The difference between amount_due and amount_paid, in %s.
    pub amount_remaining: i64,
    #[serde(rename = "application")]
    ///ID of the Connect Application that created the invoice.
    pub application: Option<serde_json::Value>,
    #[serde(rename = "application_fee_amount")]
    ///The fee in %s that will be applied to the invoice and transferred to the application owner's Stripe account when the invoice is paid.
    pub application_fee_amount: Option<i64>,
    #[serde(rename = "attempt_count")]
    ///Number of payment attempts made for this invoice, from the perspective of the payment retry schedule. Any payment attempt counts as the first attempt, and subsequently only automatic retries increment the attempt count. In other words, manual payment attempts after the first attempt do not affect the retry schedule.
    pub attempt_count: i64,
    #[serde(rename = "attempted")]
    ///Whether an attempt has been made to pay the invoice. An invoice is not attempted until 1 hour after the `invoice.created` webhook, for example, so you might not want to display that invoice as unpaid to your users.
    pub attempted: bool,
    #[serde(rename = "auto_advance")]
    ///Controls whether Stripe will perform [automatic collection](https://stripe.com/docs/billing/invoices/workflow/#auto_advance) of the invoice. When `false`, the invoice's state will not automatically advance without an explicit action.
    pub auto_advance: Option<bool>,
    #[serde(rename = "automatic_tax")]
    ///
    pub automatic_tax: AutomaticTax,
    #[serde(rename = "billing_reason")]
    ///Indicates the reason why the invoice was created. `subscription_cycle` indicates an invoice created by a subscription advancing into a new period. `subscription_create` indicates an invoice created due to creating a subscription. `subscription_update` indicates an invoice created due to updating a subscription. `subscription` is set for all old invoices to indicate either a change to a subscription or a period advancement. `manual` is set for all invoices unrelated to a subscription (for example: created via the invoice editor). The `upcoming` value is reserved for simulated invoices per the upcoming invoice endpoint. `subscription_threshold` indicates an invoice created due to a billing threshold being reached.
    pub billing_reason: Option<String>,
    #[serde(rename = "charge")]
    ///ID of the latest charge generated for this invoice, if any.
    pub charge: Option<serde_json::Value>,
    #[serde(rename = "collection_method")]
    ///Either `charge_automatically`, or `send_invoice`. When charging automatically, Stripe will attempt to pay this invoice using the default source attached to the customer. When sending an invoice, Stripe will email this invoice to the customer with payment instructions.
    pub collection_method: String,
    #[serde(rename = "created")]
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    #[serde(rename = "currency")]
    ///Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: String,
    #[serde(rename = "custom_fields")]
    ///Custom fields displayed on the invoice.
    pub custom_fields: Option<Vec<InvoiceSettingCustomField>>,
    #[serde(rename = "customer")]
    ///The ID of the customer who will be billed.
    pub customer: Option<serde_json::Value>,
    #[serde(rename = "customer_address")]
    ///The customer's address. Until the invoice is finalized, this field will equal `customer.address`. Once the invoice is finalized, this field will no longer be updated.
    pub customer_address: Option<serde_json::Value>,
    #[serde(rename = "customer_email")]
    ///The customer's email. Until the invoice is finalized, this field will equal `customer.email`. Once the invoice is finalized, this field will no longer be updated.
    pub customer_email: Option<String>,
    #[serde(rename = "customer_name")]
    ///The customer's name. Until the invoice is finalized, this field will equal `customer.name`. Once the invoice is finalized, this field will no longer be updated.
    pub customer_name: Option<String>,
    #[serde(rename = "customer_phone")]
    ///The customer's phone number. Until the invoice is finalized, this field will equal `customer.phone`. Once the invoice is finalized, this field will no longer be updated.
    pub customer_phone: Option<String>,
    #[serde(rename = "customer_shipping")]
    ///The customer's shipping information. Until the invoice is finalized, this field will equal `customer.shipping`. Once the invoice is finalized, this field will no longer be updated.
    pub customer_shipping: Option<serde_json::Value>,
    #[serde(rename = "customer_tax_exempt")]
    ///The customer's tax exempt status. Until the invoice is finalized, this field will equal `customer.tax_exempt`. Once the invoice is finalized, this field will no longer be updated.
    pub customer_tax_exempt: Option<String>,
    #[serde(rename = "customer_tax_ids")]
    ///The customer's tax IDs. Until the invoice is finalized, this field will contain the same tax IDs as `customer.tax_ids`. Once the invoice is finalized, this field will no longer be updated.
    pub customer_tax_ids: Option<Vec<InvoicesResourceInvoiceTaxId>>,
    #[serde(rename = "default_payment_method")]
    ///ID of the default payment method for the invoice. It must belong to the customer associated with the invoice. If not set, defaults to the subscription's default payment method, if any, or to the default payment method in the customer's invoice settings.
    pub default_payment_method: Option<serde_json::Value>,
    #[serde(rename = "default_source")]
    ///ID of the default payment source for the invoice. It must belong to the customer associated with the invoice and be in a chargeable state. If not set, defaults to the subscription's default source, if any, or to the customer's default source.
    pub default_source: Option<serde_json::Value>,
    #[serde(rename = "default_tax_rates")]
    ///The tax rates applied to this invoice, if any.
    pub default_tax_rates: Vec<TaxRate>,
    #[serde(rename = "description")]
    ///An arbitrary string attached to the object. Often useful for displaying to users. Referenced as 'memo' in the Dashboard.
    pub description: Option<String>,
    #[serde(rename = "discount")]
    ///Describes the current discount applied to this invoice, if there is one. Not populated if there are multiple discounts.
    pub discount: Option<serde_json::Value>,
    #[serde(rename = "discounts")]
    ///The discounts applied to the invoice. Line item discounts are applied before invoice discounts. Use `expand[]=discounts` to expand each discount.
    pub discounts: Option<Vec<serde_json::Value>>,
    #[serde(rename = "due_date")]
    ///The date on which payment for this invoice is due. This value will be `null` for invoices where `collection_method=charge_automatically`.
    pub due_date: Option<i64>,
    #[serde(rename = "ending_balance")]
    ///Ending customer balance after the invoice is finalized. Invoices are finalized approximately an hour after successful webhook delivery or when payment collection is attempted for the invoice. If the invoice has not been finalized yet, this will be null.
    pub ending_balance: Option<i64>,
    #[serde(rename = "footer")]
    ///Footer displayed on the invoice.
    pub footer: Option<String>,
    #[serde(rename = "hosted_invoice_url")]
    ///The URL for the hosted invoice page, which allows customers to view and pay an invoice. If the invoice has not been finalized yet, this will be null.
    pub hosted_invoice_url: Option<String>,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: Option<String>,
    #[serde(rename = "invoice_pdf")]
    ///The link to download the PDF for the invoice. If the invoice has not been finalized yet, this will be null.
    pub invoice_pdf: Option<String>,
    #[serde(rename = "last_finalization_error")]
    ///The error encountered during the previous attempt to finalize the invoice. This field is cleared when the invoice is successfully finalized.
    pub last_finalization_error: Option<serde_json::Value>,
    #[serde(rename = "lines")]
    ///The individual line items that make up the invoice. `lines` is sorted as follows: invoice items in reverse chronological order, followed by the subscription, if any.
    pub lines: serde_json::Value,
    #[serde(rename = "livemode")]
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    #[serde(rename = "metadata")]
    ///Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<serde_json::Value>,
    #[serde(rename = "next_payment_attempt")]
    ///The time at which payment will next be attempted. This value will be `null` for invoices where `collection_method=send_invoice`.
    pub next_payment_attempt: Option<i64>,
    #[serde(rename = "number")]
    ///A unique, identifying string that appears on emails sent to the customer for this invoice. This starts with the customer's unique invoice_prefix if it is specified.
    pub number: Option<String>,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "on_behalf_of")]
    ///The account (if any) for which the funds of the invoice payment are intended. If set, the invoice will be presented with the branding and support information of the specified account. See the [Invoices with Connect](https://stripe.com/docs/billing/invoices/connect) documentation for details.
    pub on_behalf_of: Option<serde_json::Value>,
    #[serde(rename = "paid")]
    ///Whether payment was successfully collected for this invoice. An invoice can be paid (most commonly) with a charge or with credit from the customer's account balance.
    pub paid: bool,
    #[serde(rename = "paid_out_of_band")]
    ///Returns true if the invoice was manually marked paid, returns false if the invoice hasn't been paid yet or was paid on Stripe.
    pub paid_out_of_band: bool,
    #[serde(rename = "payment_intent")]
    ///The PaymentIntent associated with this invoice. The PaymentIntent is generated when the invoice is finalized, and can then be used to pay the invoice. Note that voiding an invoice will cancel the PaymentIntent.
    pub payment_intent: Option<serde_json::Value>,
    #[serde(rename = "payment_settings")]
    ///
    pub payment_settings: InvoicesPaymentSettings,
    #[serde(rename = "period_end")]
    ///End of the usage period during which invoice items were added to this invoice.
    pub period_end: i64,
    #[serde(rename = "period_start")]
    ///Start of the usage period during which invoice items were added to this invoice.
    pub period_start: i64,
    #[serde(rename = "post_payment_credit_notes_amount")]
    ///Total amount of all post-payment credit notes issued for this invoice.
    pub post_payment_credit_notes_amount: i64,
    #[serde(rename = "pre_payment_credit_notes_amount")]
    ///Total amount of all pre-payment credit notes issued for this invoice.
    pub pre_payment_credit_notes_amount: i64,
    #[serde(rename = "quote")]
    ///The quote this invoice was generated from.
    pub quote: Option<serde_json::Value>,
    #[serde(rename = "receipt_number")]
    ///This is the transaction number that appears on email receipts sent for this invoice.
    pub receipt_number: Option<String>,
    #[serde(rename = "rendering_options")]
    ///Options for invoice PDF rendering.
    pub rendering_options: Option<serde_json::Value>,
    #[serde(rename = "starting_balance")]
    ///Starting customer balance before the invoice is finalized. If the invoice has not been finalized yet, this will be the current customer balance.
    pub starting_balance: i64,
    #[serde(rename = "statement_descriptor")]
    ///Extra information about an invoice for the customer's credit card statement.
    pub statement_descriptor: Option<String>,
    #[serde(rename = "status")]
    ///The status of the invoice, one of `draft`, `open`, `paid`, `uncollectible`, or `void`. [Learn more](https://stripe.com/docs/billing/invoices/workflow#workflow-overview)
    pub status: Option<String>,
    #[serde(rename = "status_transitions")]
    ///
    pub status_transitions: InvoicesStatusTransitions,
    #[serde(rename = "subscription")]
    ///The subscription that this invoice was prepared for, if any.
    pub subscription: Option<serde_json::Value>,
    #[serde(rename = "subscription_proration_date")]
    ///Only set for upcoming invoices that preview prorations. The time used to calculate prorations.
    pub subscription_proration_date: Option<i64>,
    #[serde(rename = "subtotal")]
    ///Total of all subscriptions, invoice items, and prorations on the invoice before any invoice level discount or exclusive tax is applied. Item discounts are already incorporated
    pub subtotal: i64,
    #[serde(rename = "subtotal_excluding_tax")]
    ///The integer amount in %s representing the subtotal of the invoice before any invoice level discount or tax is applied. Item discounts are already incorporated
    pub subtotal_excluding_tax: Option<i64>,
    #[serde(rename = "tax")]
    ///The amount of tax on this invoice. This is the sum of all the tax amounts on this invoice.
    pub tax: Option<i64>,
    #[serde(rename = "test_clock")]
    ///ID of the test clock this invoice belongs to.
    pub test_clock: Option<serde_json::Value>,
    #[serde(rename = "threshold_reason")]
    ///
    pub threshold_reason: Option<InvoiceThresholdReason>,
    #[serde(rename = "total")]
    ///Total after discounts and taxes.
    pub total: i64,
    #[serde(rename = "total_discount_amounts")]
    ///The aggregate amounts calculated per discount across all line items.
    pub total_discount_amounts: Option<Vec<DiscountsResourceDiscountAmount>>,
    #[serde(rename = "total_excluding_tax")]
    ///The integer amount in %s representing the total amount of the invoice including all discounts but excluding all tax.
    pub total_excluding_tax: Option<i64>,
    #[serde(rename = "total_tax_amounts")]
    ///The aggregate amounts calculated per tax rate for all line items.
    pub total_tax_amounts: Vec<InvoiceTaxAmount>,
    #[serde(rename = "transfer_data")]
    ///The account (if any) the payment will be attributed to for tax reporting, and where funds from the payment will be transferred to for the invoice.
    pub transfer_data: Option<serde_json::Value>,
    #[serde(rename = "webhooks_delivered_at")]
    ///Invoices are automatically paid or sent 1 hour after webhooks are delivered, or until all webhook delivery attempts have [been exhausted](https://stripe.com/docs/billing/webhooks#understand). This field tracks the time when webhooks for this invoice were successfully delivered. If the invoice had no webhooks to deliver, this will be set while the invoice is being created.
    pub webhooks_delivered_at: Option<i64>,
}
impl std::fmt::Display for Invoice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InvoiceInstallmentsCard {
    #[serde(rename = "enabled")]
    ///Whether Installments are enabled for this Invoice.
    pub enabled: Option<bool>,
}
impl std::fmt::Display for InvoiceInstallmentsCard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InvoiceItemThresholdReason {
    #[serde(rename = "line_item_ids")]
    ///The IDs of the line items that triggered the threshold invoice.
    pub line_item_ids: Vec<String>,
    #[serde(rename = "usage_gte")]
    ///The quantity threshold boundary that applied to the given line item.
    pub usage_gte: i64,
}
impl std::fmt::Display for InvoiceItemThresholdReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InvoiceLineItemPeriod {
    #[serde(rename = "end")]
    ///The end of the period, which must be greater than or equal to the start.
    pub end: i64,
    #[serde(rename = "start")]
    ///The start of the period.
    pub start: i64,
}
impl std::fmt::Display for InvoiceLineItemPeriod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InvoiceMandateOptionsCard {
    #[serde(rename = "amount")]
    ///Amount to be charged for future payments.
    pub amount: Option<i64>,
    #[serde(rename = "amount_type")]
    ///One of `fixed` or `maximum`. If `fixed`, the `amount` param refers to the exact amount to be charged in future payments. If `maximum`, the amount charged can be up to the value passed for the `amount` param.
    pub amount_type: Option<String>,
    #[serde(rename = "description")]
    ///A description of the mandate or subscription that is meant to be displayed to the customer.
    pub description: Option<String>,
}
impl std::fmt::Display for InvoiceMandateOptionsCard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InvoicePaymentMethodOptionsAcssDebit {
    #[serde(rename = "mandate_options")]
    ///
    pub mandate_options: Option<InvoicePaymentMethodOptionsAcssDebitMandateOptions>,
    #[serde(rename = "verification_method")]
    ///Bank account verification method.
    pub verification_method: Option<String>,
}
impl std::fmt::Display for InvoicePaymentMethodOptionsAcssDebit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InvoicePaymentMethodOptionsAcssDebitMandateOptions {
    #[serde(rename = "transaction_type")]
    ///Transaction type of the mandate.
    pub transaction_type: Option<String>,
}
impl std::fmt::Display for InvoicePaymentMethodOptionsAcssDebitMandateOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InvoicePaymentMethodOptionsBancontact {
    #[serde(rename = "preferred_language")]
    ///Preferred language of the Bancontact authorization page that the customer is redirected to.
    pub preferred_language: String,
}
impl std::fmt::Display for InvoicePaymentMethodOptionsBancontact {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InvoicePaymentMethodOptionsCard {
    #[serde(rename = "installments")]
    ///
    pub installments: Option<InvoiceInstallmentsCard>,
    #[serde(rename = "request_three_d_secure")]
    ///We strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based on risk level and [other requirements](https://stripe.com/docs/strong-customer-authentication). However, if you wish to request 3D Secure based on logic from your own fraud engine, provide this option. Read our guide on [manually requesting 3D Secure](https://stripe.com/docs/payments/3d-secure#manual-three-ds) for more information on how this configuration interacts with Radar and our SCA Engine.
    pub request_three_d_secure: Option<String>,
}
impl std::fmt::Display for InvoicePaymentMethodOptionsCard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InvoicePaymentMethodOptionsCustomerBalance {
    #[serde(rename = "bank_transfer")]
    ///
    pub bank_transfer: Option<InvoicePaymentMethodOptionsCustomerBalanceBankTransfer>,
    #[serde(rename = "funding_type")]
    ///The funding method type to be used when there are not enough funds in the customer balance. Permitted values include: `bank_transfer`.
    pub funding_type: Option<String>,
}
impl std::fmt::Display for InvoicePaymentMethodOptionsCustomerBalance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InvoicePaymentMethodOptionsCustomerBalanceBankTransfer {
    #[serde(rename = "eu_bank_transfer")]
    ///
    pub eu_bank_transfer: Option<
        InvoicePaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer,
    >,
    #[serde(rename = "type")]
    ///The bank transfer type that can be used for funding. Permitted values include: `eu_bank_transfer`, `gb_bank_transfer`, `jp_bank_transfer`, or `mx_bank_transfer`.
    pub type_: Option<String>,
}
impl std::fmt::Display for InvoicePaymentMethodOptionsCustomerBalanceBankTransfer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InvoicePaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer {
    #[serde(rename = "country")]
    ///The desired country code of the bank account information. Permitted values include: `DE`, `ES`, `FR`, `IE`, or `NL`.
    pub country: String,
}
impl std::fmt::Display
for InvoicePaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InvoicePaymentMethodOptionsKonbini {}
impl std::fmt::Display for InvoicePaymentMethodOptionsKonbini {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InvoicePaymentMethodOptionsUsBankAccount {
    #[serde(rename = "financial_connections")]
    ///
    pub financial_connections: Option<
        InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptions,
    >,
    #[serde(rename = "verification_method")]
    ///Bank account verification method.
    pub verification_method: Option<String>,
}
impl std::fmt::Display for InvoicePaymentMethodOptionsUsBankAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptions {
    #[serde(rename = "permissions")]
    ///The list of permissions to request. The `payment_method` permission must be included.
    pub permissions: Option<Vec<String>>,
}
impl std::fmt::Display for InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InvoiceSettingCustomField {
    #[serde(rename = "name")]
    ///The name of the custom field.
    pub name: String,
    #[serde(rename = "value")]
    ///The value of the custom field.
    pub value: String,
}
impl std::fmt::Display for InvoiceSettingCustomField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InvoiceSettingCustomerSetting {
    #[serde(rename = "custom_fields")]
    ///Default custom fields to be displayed on invoices for this customer.
    pub custom_fields: Option<Vec<InvoiceSettingCustomField>>,
    #[serde(rename = "default_payment_method")]
    ///ID of a payment method that's attached to the customer, to be used as the customer's default payment method for subscriptions and invoices.
    pub default_payment_method: Option<serde_json::Value>,
    #[serde(rename = "footer")]
    ///Default footer to be displayed on invoices for this customer.
    pub footer: Option<String>,
    #[serde(rename = "rendering_options")]
    ///Default options for invoice PDF rendering for this customer.
    pub rendering_options: Option<serde_json::Value>,
}
impl std::fmt::Display for InvoiceSettingCustomerSetting {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InvoiceSettingQuoteSetting {
    #[serde(rename = "days_until_due")]
    ///Number of days within which a customer must pay invoices generated by this quote. This value will be `null` for quotes where `collection_method=charge_automatically`.
    pub days_until_due: Option<i64>,
}
impl std::fmt::Display for InvoiceSettingQuoteSetting {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InvoiceSettingRenderingOptions {
    #[serde(rename = "amount_tax_display")]
    ///How line-item prices and amounts will be displayed with respect to tax on invoice PDFs.
    pub amount_tax_display: Option<String>,
}
impl std::fmt::Display for InvoiceSettingRenderingOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InvoiceSettingSubscriptionScheduleSetting {
    #[serde(rename = "days_until_due")]
    ///Number of days within which a customer must pay invoices generated by this subscription schedule. This value will be `null` for subscription schedules where `billing=charge_automatically`.
    pub days_until_due: Option<i64>,
}
impl std::fmt::Display for InvoiceSettingSubscriptionScheduleSetting {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InvoiceTaxAmount {
    #[serde(rename = "amount")]
    ///The amount, in %s, of the tax.
    pub amount: i64,
    #[serde(rename = "inclusive")]
    ///Whether this tax amount is inclusive or exclusive.
    pub inclusive: bool,
    #[serde(rename = "tax_rate")]
    ///The tax rate that was applied to get this tax amount.
    pub tax_rate: serde_json::Value,
}
impl std::fmt::Display for InvoiceTaxAmount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InvoiceThresholdReason {
    #[serde(rename = "amount_gte")]
    ///The total invoice amount threshold boundary if it triggered the threshold invoice.
    pub amount_gte: Option<i64>,
    #[serde(rename = "item_reasons")]
    ///Indicates which line items triggered a threshold invoice.
    pub item_reasons: Vec<InvoiceItemThresholdReason>,
}
impl std::fmt::Display for InvoiceThresholdReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InvoiceTransferData {
    #[serde(rename = "amount")]
    ///The amount in %s that will be transferred to the destination account when the invoice is paid. By default, the entire amount is transferred to the destination.
    pub amount: Option<i64>,
    #[serde(rename = "destination")]
    ///The account where funds from the payment will be transferred to upon payment success.
    pub destination: serde_json::Value,
}
impl std::fmt::Display for InvoiceTransferData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Invoiceitem {
    #[serde(rename = "amount")]
    ///Amount (in the `currency` specified) of the invoice item. This should always be equal to `unit_amount * quantity`.
    pub amount: i64,
    #[serde(rename = "currency")]
    ///Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: String,
    #[serde(rename = "customer")]
    ///The ID of the customer who will be billed when this invoice item is billed.
    pub customer: serde_json::Value,
    #[serde(rename = "date")]
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub date: i64,
    #[serde(rename = "description")]
    ///An arbitrary string attached to the object. Often useful for displaying to users.
    pub description: Option<String>,
    #[serde(rename = "discountable")]
    ///If true, discounts will apply to this invoice item. Always false for prorations.
    pub discountable: bool,
    #[serde(rename = "discounts")]
    ///The discounts which apply to the invoice item. Item discounts are applied before invoice discounts. Use `expand[]=discounts` to expand each discount.
    pub discounts: Option<Vec<serde_json::Value>>,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "invoice")]
    ///The ID of the invoice this invoice item belongs to.
    pub invoice: Option<serde_json::Value>,
    #[serde(rename = "livemode")]
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    #[serde(rename = "metadata")]
    ///Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<serde_json::Value>,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "period")]
    ///
    pub period: InvoiceLineItemPeriod,
    #[serde(rename = "price")]
    ///The price of the invoice item.
    pub price: Option<serde_json::Value>,
    #[serde(rename = "proration")]
    ///Whether the invoice item was created automatically as a proration adjustment when the customer switched plans.
    pub proration: bool,
    #[serde(rename = "quantity")]
    ///Quantity of units for the invoice item. If the invoice item is a proration, the quantity of the subscription that the proration was computed for.
    pub quantity: i64,
    #[serde(rename = "subscription")]
    ///The subscription that this invoice item has been created for, if any.
    pub subscription: Option<serde_json::Value>,
    #[serde(rename = "subscription_item")]
    ///The subscription item that this invoice item has been created for, if any.
    pub subscription_item: Option<String>,
    #[serde(rename = "tax_rates")]
    ///The tax rates which apply to the invoice item. When set, the `default_tax_rates` on the invoice do not apply to this invoice item.
    pub tax_rates: Option<Vec<TaxRate>>,
    #[serde(rename = "test_clock")]
    ///ID of the test clock this invoice item belongs to.
    pub test_clock: Option<serde_json::Value>,
    #[serde(rename = "unit_amount")]
    ///Unit amount (in the `currency` specified) of the invoice item.
    pub unit_amount: Option<i64>,
    #[serde(rename = "unit_amount_decimal")]
    ///Same as `unit_amount`, but contains a decimal value with at most 12 decimal places.
    pub unit_amount_decimal: Option<String>,
}
impl std::fmt::Display for Invoiceitem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InvoicesLineItemsCreditedItems {
    #[serde(rename = "invoice")]
    ///Invoice containing the credited invoice line items
    pub invoice: String,
    #[serde(rename = "invoice_line_items")]
    ///Credited invoice line items
    pub invoice_line_items: Vec<String>,
}
impl std::fmt::Display for InvoicesLineItemsCreditedItems {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InvoicesLineItemsProrationDetails {
    #[serde(rename = "credited_items")]
    ///For a credit proration `line_item`, the original debit line_items to which the credit proration applies.
    pub credited_items: Option<serde_json::Value>,
}
impl std::fmt::Display for InvoicesLineItemsProrationDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InvoicesPaymentMethodOptions {
    #[serde(rename = "acss_debit")]
    ///If paying by `acss_debit`, this sub-hash contains details about the Canadian pre-authorized debit payment method options to pass to the invoice’s PaymentIntent.
    pub acss_debit: Option<serde_json::Value>,
    #[serde(rename = "bancontact")]
    ///If paying by `bancontact`, this sub-hash contains details about the Bancontact payment method options to pass to the invoice’s PaymentIntent.
    pub bancontact: Option<serde_json::Value>,
    #[serde(rename = "card")]
    ///If paying by `card`, this sub-hash contains details about the Card payment method options to pass to the invoice’s PaymentIntent.
    pub card: Option<serde_json::Value>,
    #[serde(rename = "customer_balance")]
    ///If paying by `customer_balance`, this sub-hash contains details about the Bank transfer payment method options to pass to the invoice’s PaymentIntent.
    pub customer_balance: Option<serde_json::Value>,
    #[serde(rename = "konbini")]
    ///If paying by `konbini`, this sub-hash contains details about the Konbini payment method options to pass to the invoice’s PaymentIntent.
    pub konbini: Option<serde_json::Value>,
    #[serde(rename = "us_bank_account")]
    ///If paying by `us_bank_account`, this sub-hash contains details about the ACH direct debit payment method options to pass to the invoice’s PaymentIntent.
    pub us_bank_account: Option<serde_json::Value>,
}
impl std::fmt::Display for InvoicesPaymentMethodOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InvoicesPaymentSettings {
    #[serde(rename = "default_mandate")]
    ///ID of the mandate to be used for this invoice. It must correspond to the payment method used to pay the invoice, including the invoice's default_payment_method or default_source, if set.
    pub default_mandate: Option<String>,
    #[serde(rename = "payment_method_options")]
    ///Payment-method-specific configuration to provide to the invoice’s PaymentIntent.
    pub payment_method_options: Option<serde_json::Value>,
    #[serde(rename = "payment_method_types")]
    ///The list of payment method types (e.g. card) to provide to the invoice’s PaymentIntent. If not set, Stripe attempts to automatically determine the types to use by looking at the invoice’s default payment method, the subscription’s default payment method, the customer’s default payment method, and your [invoice template settings](https://dashboard.stripe.com/settings/billing/invoice).
    pub payment_method_types: Option<Vec<String>>,
}
impl std::fmt::Display for InvoicesPaymentSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InvoicesResourceInvoiceTaxId {
    #[serde(rename = "type")]
    ///The type of the tax ID, one of `eu_vat`, `br_cnpj`, `br_cpf`, `eu_oss_vat`, `gb_vat`, `nz_gst`, `au_abn`, `au_arn`, `in_gst`, `no_vat`, `za_vat`, `ch_vat`, `mx_rfc`, `sg_uen`, `ru_inn`, `ru_kpp`, `ca_bn`, `hk_br`, `es_cif`, `tw_vat`, `th_vat`, `jp_cn`, `jp_rn`, `li_uid`, `my_itn`, `us_ein`, `kr_brn`, `ca_qst`, `ca_gst_hst`, `ca_pst_bc`, `ca_pst_mb`, `ca_pst_sk`, `my_sst`, `sg_gst`, `ae_trn`, `cl_tin`, `sa_vat`, `id_npwp`, `my_frp`, `il_vat`, `ge_vat`, `ua_vat`, `is_vat`, `bg_uic`, `hu_tin`, `si_tin`, or `unknown`
    pub type_: String,
    #[serde(rename = "value")]
    ///The value of the tax ID.
    pub value: Option<String>,
}
impl std::fmt::Display for InvoicesResourceInvoiceTaxId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InvoicesStatusTransitions {
    #[serde(rename = "finalized_at")]
    ///The time that the invoice draft was finalized.
    pub finalized_at: Option<i64>,
    #[serde(rename = "marked_uncollectible_at")]
    ///The time that the invoice was marked uncollectible.
    pub marked_uncollectible_at: Option<i64>,
    #[serde(rename = "paid_at")]
    ///The time that the invoice was paid.
    pub paid_at: Option<i64>,
    #[serde(rename = "voided_at")]
    ///The time that the invoice was voided.
    pub voided_at: Option<i64>,
}
impl std::fmt::Display for InvoicesStatusTransitions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IssuingAuthorization {
    #[serde(rename = "amount")]
    ///The total amount that was authorized or rejected. This amount is in the card's currency and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    pub amount: i64,
    #[serde(rename = "amount_details")]
    ///Detailed breakdown of amount components. These amounts are denominated in `currency` and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    pub amount_details: Option<serde_json::Value>,
    #[serde(rename = "approved")]
    ///Whether the authorization has been approved.
    pub approved: bool,
    #[serde(rename = "authorization_method")]
    ///How the card details were provided.
    pub authorization_method: String,
    #[serde(rename = "balance_transactions")]
    ///List of balance transactions associated with this authorization.
    pub balance_transactions: Vec<BalanceTransaction>,
    #[serde(rename = "card")]
    ///You can [create physical or virtual cards](https://stripe.com/docs/issuing/cards) that are issued to cardholders.
    pub card: IssuingCard,
    #[serde(rename = "cardholder")]
    ///The cardholder to whom this authorization belongs.
    pub cardholder: Option<serde_json::Value>,
    #[serde(rename = "created")]
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    #[serde(rename = "currency")]
    ///Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: String,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "livemode")]
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    #[serde(rename = "merchant_amount")]
    ///The total amount that was authorized or rejected. This amount is in the `merchant_currency` and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    pub merchant_amount: i64,
    #[serde(rename = "merchant_currency")]
    ///The currency that was presented to the cardholder for the authorization. Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
    pub merchant_currency: String,
    #[serde(rename = "merchant_data")]
    ///
    pub merchant_data: IssuingAuthorizationMerchantData,
    #[serde(rename = "metadata")]
    ///Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format.
    pub metadata: serde_json::Value,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "pending_request")]
    ///The pending authorization request. This field will only be non-null during an `issuing_authorization.request` webhook.
    pub pending_request: Option<serde_json::Value>,
    #[serde(rename = "request_history")]
    ///History of every time `pending_request` was approved/denied, either by you directly or by Stripe (e.g. based on your `spending_controls`). If the merchant changes the authorization by performing an [incremental authorization](https://stripe.com/docs/issuing/purchases/authorizations), you can look at this field to see the previous requests for the authorization.
    pub request_history: Vec<IssuingAuthorizationRequest>,
    #[serde(rename = "status")]
    ///The current status of the authorization in its lifecycle.
    pub status: String,
    #[serde(rename = "transactions")]
    ///List of [transactions](https://stripe.com/docs/api/issuing/transactions) associated with this authorization.
    pub transactions: Vec<IssuingTransaction>,
    #[serde(rename = "treasury")]
    ///[Treasury](https://stripe.com/docs/api/treasury) details related to this authorization if it was created on a [FinancialAccount](https://stripe.com/docs/api/treasury/financial_accounts).
    pub treasury: Option<serde_json::Value>,
    #[serde(rename = "verification_data")]
    ///
    pub verification_data: IssuingAuthorizationVerificationData,
    #[serde(rename = "wallet")]
    ///The digital wallet used for this authorization. One of `apple_pay`, `google_pay`, or `samsung_pay`.
    pub wallet: Option<String>,
}
impl std::fmt::Display for IssuingAuthorization {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IssuingCard {
    #[serde(rename = "brand")]
    ///The brand of the card.
    pub brand: String,
    #[serde(rename = "cancellation_reason")]
    ///The reason why the card was canceled.
    pub cancellation_reason: Option<String>,
    #[serde(rename = "cardholder")]
    /**An Issuing `Cardholder` object represents an individual or business entity who is [issued](https://stripe.com/docs/issuing) cards.

Related guide: [How to create a Cardholder](https://stripe.com/docs/issuing/cards#create-cardholder)*/
    pub cardholder: IssuingCardholder,
    #[serde(rename = "created")]
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    #[serde(rename = "currency")]
    ///Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Supported currencies are `usd` in the US, `eur` in the EU, and `gbp` in the UK.
    pub currency: String,
    #[serde(rename = "cvc")]
    ///The card's CVC. For security reasons, this is only available for virtual cards, and will be omitted unless you explicitly request it with [the `expand` parameter](https://stripe.com/docs/api/expanding_objects). Additionally, it's only available via the ["Retrieve a card" endpoint](https://stripe.com/docs/api/issuing/cards/retrieve), not via "List all cards" or any other endpoint.
    pub cvc: Option<String>,
    #[serde(rename = "exp_month")]
    ///The expiration month of the card.
    pub exp_month: i64,
    #[serde(rename = "exp_year")]
    ///The expiration year of the card.
    pub exp_year: i64,
    #[serde(rename = "financial_account")]
    ///The financial account this card is attached to.
    pub financial_account: Option<String>,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "last4")]
    ///The last 4 digits of the card number.
    pub last4: String,
    #[serde(rename = "livemode")]
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    #[serde(rename = "metadata")]
    ///Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format.
    pub metadata: serde_json::Value,
    #[serde(rename = "number")]
    ///The full unredacted card number. For security reasons, this is only available for virtual cards, and will be omitted unless you explicitly request it with [the `expand` parameter](https://stripe.com/docs/api/expanding_objects). Additionally, it's only available via the ["Retrieve a card" endpoint](https://stripe.com/docs/api/issuing/cards/retrieve), not via "List all cards" or any other endpoint.
    pub number: Option<String>,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "replaced_by")]
    ///The latest card that replaces this card, if any.
    pub replaced_by: Option<serde_json::Value>,
    #[serde(rename = "replacement_for")]
    ///The card this card replaces, if any.
    pub replacement_for: Option<serde_json::Value>,
    #[serde(rename = "replacement_reason")]
    ///The reason why the previous card needed to be replaced.
    pub replacement_reason: Option<String>,
    #[serde(rename = "shipping")]
    ///Where and how the card will be shipped.
    pub shipping: Option<serde_json::Value>,
    #[serde(rename = "spending_controls")]
    ///
    pub spending_controls: IssuingCardAuthorizationControls,
    #[serde(rename = "status")]
    ///Whether authorizations can be approved on this card.
    pub status: String,
    #[serde(rename = "type")]
    ///The type of the card.
    pub type_: String,
    #[serde(rename = "wallets")]
    ///Information relating to digital wallets (like Apple Pay and Google Pay).
    pub wallets: Option<serde_json::Value>,
}
impl std::fmt::Display for IssuingCard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IssuingCardholder {
    #[serde(rename = "billing")]
    ///
    pub billing: IssuingCardholderAddress,
    #[serde(rename = "company")]
    ///Additional information about a `company` cardholder.
    pub company: Option<serde_json::Value>,
    #[serde(rename = "created")]
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    #[serde(rename = "email")]
    ///The cardholder's email address.
    pub email: Option<String>,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "individual")]
    ///Additional information about an `individual` cardholder.
    pub individual: Option<serde_json::Value>,
    #[serde(rename = "livemode")]
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    #[serde(rename = "metadata")]
    ///Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format.
    pub metadata: serde_json::Value,
    #[serde(rename = "name")]
    ///The cardholder's name. This will be printed on cards issued to them.
    pub name: String,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "phone_number")]
    ///The cardholder's phone number. This is required for all cardholders who will be creating EU cards. See the [3D Secure documentation](https://stripe.com/docs/issuing/3d-secure#when-is-3d-secure-applied) for more details.
    pub phone_number: Option<String>,
    #[serde(rename = "requirements")]
    ///
    pub requirements: IssuingCardholderRequirements,
    #[serde(rename = "spending_controls")]
    ///Rules that control spending across this cardholder's cards. Refer to our [documentation](https://stripe.com/docs/issuing/controls/spending-controls) for more details.
    pub spending_controls: Option<serde_json::Value>,
    #[serde(rename = "status")]
    ///Specifies whether to permit authorizations on this cardholder's cards.
    pub status: String,
    #[serde(rename = "type")]
    ///One of `individual` or `company`.
    pub type_: String,
}
impl std::fmt::Display for IssuingCardholder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IssuingDispute {
    #[serde(rename = "amount")]
    ///Disputed amount in the card's currency and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal). Usually the amount of the `transaction`, but can differ (usually because of currency fluctuation).
    pub amount: i64,
    #[serde(rename = "balance_transactions")]
    ///List of balance transactions associated with the dispute.
    pub balance_transactions: Option<Vec<BalanceTransaction>>,
    #[serde(rename = "created")]
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    #[serde(rename = "currency")]
    ///The currency the `transaction` was made in.
    pub currency: String,
    #[serde(rename = "evidence")]
    ///
    pub evidence: IssuingDisputeEvidence,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "livemode")]
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    #[serde(rename = "metadata")]
    ///Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format.
    pub metadata: serde_json::Value,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "status")]
    ///Current status of the dispute.
    pub status: String,
    #[serde(rename = "transaction")]
    ///The transaction being disputed.
    pub transaction: serde_json::Value,
    #[serde(rename = "treasury")]
    ///[Treasury](https://stripe.com/docs/api/treasury) details related to this dispute if it was created on a [FinancialAccount](/docs/api/treasury/financial_accounts
    pub treasury: Option<serde_json::Value>,
}
impl std::fmt::Display for IssuingDispute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IssuingSettlement {
    #[serde(rename = "bin")]
    ///The Bank Identification Number reflecting this settlement record.
    pub bin: String,
    #[serde(rename = "clearing_date")]
    ///The date that the transactions are cleared and posted to user's accounts.
    pub clearing_date: i64,
    #[serde(rename = "created")]
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    #[serde(rename = "currency")]
    ///Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: String,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "interchange_fees")]
    ///The total interchange received as reimbursement for the transactions.
    pub interchange_fees: i64,
    #[serde(rename = "livemode")]
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    #[serde(rename = "metadata")]
    ///Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format.
    pub metadata: serde_json::Value,
    #[serde(rename = "net_total")]
    ///The total net amount required to settle with the network.
    pub net_total: i64,
    #[serde(rename = "network")]
    ///The card network for this settlement report. One of ["visa"]
    pub network: String,
    #[serde(rename = "network_fees")]
    ///The total amount of fees owed to the network.
    pub network_fees: i64,
    #[serde(rename = "network_settlement_identifier")]
    ///The Settlement Identification Number assigned by the network.
    pub network_settlement_identifier: String,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "settlement_service")]
    ///One of `international` or `uk_national_net`.
    pub settlement_service: String,
    #[serde(rename = "transaction_count")]
    ///The total number of transactions reflected in this settlement.
    pub transaction_count: i64,
    #[serde(rename = "transaction_volume")]
    ///The total transaction amount reflected in this settlement.
    pub transaction_volume: i64,
}
impl std::fmt::Display for IssuingSettlement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IssuingTransaction {
    #[serde(rename = "amount")]
    ///The transaction amount, which will be reflected in your balance. This amount is in your currency and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    pub amount: i64,
    #[serde(rename = "amount_details")]
    ///Detailed breakdown of amount components. These amounts are denominated in `currency` and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    pub amount_details: Option<serde_json::Value>,
    #[serde(rename = "authorization")]
    ///The `Authorization` object that led to this transaction.
    pub authorization: Option<serde_json::Value>,
    #[serde(rename = "balance_transaction")]
    ///ID of the [balance transaction](https://stripe.com/docs/api/balance_transactions) associated with this transaction.
    pub balance_transaction: Option<serde_json::Value>,
    #[serde(rename = "card")]
    ///The card used to make this transaction.
    pub card: serde_json::Value,
    #[serde(rename = "cardholder")]
    ///The cardholder to whom this transaction belongs.
    pub cardholder: Option<serde_json::Value>,
    #[serde(rename = "created")]
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    #[serde(rename = "currency")]
    ///Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: String,
    #[serde(rename = "dispute")]
    ///If you've disputed the transaction, the ID of the dispute.
    pub dispute: Option<serde_json::Value>,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "livemode")]
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    #[serde(rename = "merchant_amount")]
    ///The amount that the merchant will receive, denominated in `merchant_currency` and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal). It will be different from `amount` if the merchant is taking payment in a different currency.
    pub merchant_amount: i64,
    #[serde(rename = "merchant_currency")]
    ///The currency with which the merchant is taking payment.
    pub merchant_currency: String,
    #[serde(rename = "merchant_data")]
    ///
    pub merchant_data: IssuingAuthorizationMerchantData,
    #[serde(rename = "metadata")]
    ///Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format.
    pub metadata: serde_json::Value,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "purchase_details")]
    ///Additional purchase information that is optionally provided by the merchant.
    pub purchase_details: Option<serde_json::Value>,
    #[serde(rename = "treasury")]
    ///[Treasury](https://stripe.com/docs/api/treasury) details related to this transaction if it was created on a [FinancialAccount](/docs/api/treasury/financial_accounts
    pub treasury: Option<serde_json::Value>,
    #[serde(rename = "type")]
    ///The nature of the transaction.
    pub type_: String,
    #[serde(rename = "wallet")]
    ///The digital wallet used for this transaction. One of `apple_pay`, `google_pay`, or `samsung_pay`.
    pub wallet: Option<String>,
}
impl std::fmt::Display for IssuingTransaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IssuingAuthorizationAmountDetails {
    #[serde(rename = "atm_fee")]
    ///The fee charged by the ATM for the cash withdrawal.
    pub atm_fee: Option<i64>,
}
impl std::fmt::Display for IssuingAuthorizationAmountDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IssuingAuthorizationMerchantData {
    #[serde(rename = "category")]
    ///A categorization of the seller's type of business. See our [merchant categories guide](https://stripe.com/docs/issuing/merchant-categories) for a list of possible values.
    pub category: String,
    #[serde(rename = "category_code")]
    ///The merchant category code for the seller’s business
    pub category_code: String,
    #[serde(rename = "city")]
    ///City where the seller is located
    pub city: Option<String>,
    #[serde(rename = "country")]
    ///Country where the seller is located
    pub country: Option<String>,
    #[serde(rename = "name")]
    ///Name of the seller
    pub name: Option<String>,
    #[serde(rename = "network_id")]
    ///Identifier assigned to the seller by the card brand
    pub network_id: String,
    #[serde(rename = "postal_code")]
    ///Postal code where the seller is located
    pub postal_code: Option<String>,
    #[serde(rename = "state")]
    ///State where the seller is located
    pub state: Option<String>,
}
impl std::fmt::Display for IssuingAuthorizationMerchantData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IssuingAuthorizationPendingRequest {
    #[serde(rename = "amount")]
    ///The additional amount Stripe will hold if the authorization is approved, in the card's [currency](https://stripe.com/docs/api#issuing_authorization_object-pending-request-currency) and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    pub amount: i64,
    #[serde(rename = "amount_details")]
    ///Detailed breakdown of amount components. These amounts are denominated in `currency` and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    pub amount_details: Option<serde_json::Value>,
    #[serde(rename = "currency")]
    ///Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: String,
    #[serde(rename = "is_amount_controllable")]
    ///If set `true`, you may provide [amount](https://stripe.com/docs/api/issuing/authorizations/approve#approve_issuing_authorization-amount) to control how much to hold for the authorization.
    pub is_amount_controllable: bool,
    #[serde(rename = "merchant_amount")]
    ///The amount the merchant is requesting to be authorized in the `merchant_currency`. The amount is in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    pub merchant_amount: i64,
    #[serde(rename = "merchant_currency")]
    ///The local currency the merchant is requesting to authorize.
    pub merchant_currency: String,
}
impl std::fmt::Display for IssuingAuthorizationPendingRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IssuingAuthorizationRequest {
    #[serde(rename = "amount")]
    ///The `pending_request.amount` at the time of the request, presented in your card's currency and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal). Stripe held this amount from your account to fund the authorization if the request was approved.
    pub amount: i64,
    #[serde(rename = "amount_details")]
    ///Detailed breakdown of amount components. These amounts are denominated in `currency` and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    pub amount_details: Option<serde_json::Value>,
    #[serde(rename = "approved")]
    ///Whether this request was approved.
    pub approved: bool,
    #[serde(rename = "created")]
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    #[serde(rename = "currency")]
    ///Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: String,
    #[serde(rename = "merchant_amount")]
    ///The `pending_request.merchant_amount` at the time of the request, presented in the `merchant_currency` and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    pub merchant_amount: i64,
    #[serde(rename = "merchant_currency")]
    ///The currency that was collected by the merchant and presented to the cardholder for the authorization. Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
    pub merchant_currency: String,
    #[serde(rename = "reason")]
    ///The reason for the approval or decline.
    pub reason: String,
}
impl std::fmt::Display for IssuingAuthorizationRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IssuingAuthorizationTreasury {
    #[serde(rename = "received_credits")]
    ///The array of [ReceivedCredits](https://stripe.com/docs/api/treasury/received_credits) associated with this authorization
    pub received_credits: Vec<String>,
    #[serde(rename = "received_debits")]
    ///The array of [ReceivedDebits](https://stripe.com/docs/api/treasury/received_debits) associated with this authorization
    pub received_debits: Vec<String>,
    #[serde(rename = "transaction")]
    ///The Treasury [Transaction](https://stripe.com/docs/api/treasury/transactions) associated with this authorization
    pub transaction: Option<String>,
}
impl std::fmt::Display for IssuingAuthorizationTreasury {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IssuingAuthorizationVerificationData {
    #[serde(rename = "address_line1_check")]
    ///Whether the cardholder provided an address first line and if it matched the cardholder’s `billing.address.line1`.
    pub address_line1_check: String,
    #[serde(rename = "address_postal_code_check")]
    ///Whether the cardholder provided a postal code and if it matched the cardholder’s `billing.address.postal_code`.
    pub address_postal_code_check: String,
    #[serde(rename = "cvc_check")]
    ///Whether the cardholder provided a CVC and if it matched Stripe’s record.
    pub cvc_check: String,
    #[serde(rename = "expiry_check")]
    ///Whether the cardholder provided an expiry date and if it matched Stripe’s record.
    pub expiry_check: String,
}
impl std::fmt::Display for IssuingAuthorizationVerificationData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IssuingCardApplePay {
    #[serde(rename = "eligible")]
    ///Apple Pay Eligibility
    pub eligible: bool,
    #[serde(rename = "ineligible_reason")]
    ///Reason the card is ineligible for Apple Pay
    pub ineligible_reason: Option<String>,
}
impl std::fmt::Display for IssuingCardApplePay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IssuingCardAuthorizationControls {
    #[serde(rename = "allowed_categories")]
    ///Array of strings containing [categories](https://stripe.com/docs/api#issuing_authorization_object-merchant_data-category) of authorizations to allow. All other categories will be blocked. Cannot be set with `blocked_categories`.
    pub allowed_categories: Option<Vec<String>>,
    #[serde(rename = "blocked_categories")]
    ///Array of strings containing [categories](https://stripe.com/docs/api#issuing_authorization_object-merchant_data-category) of authorizations to decline. All other categories will be allowed. Cannot be set with `allowed_categories`.
    pub blocked_categories: Option<Vec<String>>,
    #[serde(rename = "spending_limits")]
    ///Limit spending with amount-based rules that apply across any cards this card replaced (i.e., its `replacement_for` card and _that_ card's `replacement_for` card, up the chain).
    pub spending_limits: Option<Vec<IssuingCardSpendingLimit>>,
    #[serde(rename = "spending_limits_currency")]
    ///Currency of the amounts within `spending_limits`. Always the same as the currency of the card.
    pub spending_limits_currency: Option<String>,
}
impl std::fmt::Display for IssuingCardAuthorizationControls {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IssuingCardGooglePay {
    #[serde(rename = "eligible")]
    ///Google Pay Eligibility
    pub eligible: bool,
    #[serde(rename = "ineligible_reason")]
    ///Reason the card is ineligible for Google Pay
    pub ineligible_reason: Option<String>,
}
impl std::fmt::Display for IssuingCardGooglePay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IssuingCardShipping {
    #[serde(rename = "address")]
    ///
    pub address: Address,
    #[serde(rename = "carrier")]
    ///The delivery company that shipped a card.
    pub carrier: Option<String>,
    #[serde(rename = "eta")]
    ///A unix timestamp representing a best estimate of when the card will be delivered.
    pub eta: Option<i64>,
    #[serde(rename = "name")]
    ///Recipient name.
    pub name: String,
    #[serde(rename = "service")]
    ///Shipment service, such as `standard` or `express`.
    pub service: String,
    #[serde(rename = "status")]
    ///The delivery status of the card.
    pub status: Option<String>,
    #[serde(rename = "tracking_number")]
    ///A tracking number for a card shipment.
    pub tracking_number: Option<String>,
    #[serde(rename = "tracking_url")]
    ///A link to the shipping carrier's site where you can view detailed information about a card shipment.
    pub tracking_url: Option<String>,
    #[serde(rename = "type")]
    ///Packaging options.
    pub type_: String,
}
impl std::fmt::Display for IssuingCardShipping {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IssuingCardSpendingLimit {
    #[serde(rename = "amount")]
    ///Maximum amount allowed to spend per interval. This amount is in the card's currency and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    pub amount: i64,
    #[serde(rename = "categories")]
    ///Array of strings containing [categories](https://stripe.com/docs/api#issuing_authorization_object-merchant_data-category) this limit applies to. Omitting this field will apply the limit to all categories.
    pub categories: Option<Vec<String>>,
    #[serde(rename = "interval")]
    ///Interval (or event) to which the amount applies.
    pub interval: String,
}
impl std::fmt::Display for IssuingCardSpendingLimit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IssuingCardWallets {
    #[serde(rename = "apple_pay")]
    ///
    pub apple_pay: IssuingCardApplePay,
    #[serde(rename = "google_pay")]
    ///
    pub google_pay: IssuingCardGooglePay,
    #[serde(rename = "primary_account_identifier")]
    ///Unique identifier for a card used with digital wallets
    pub primary_account_identifier: Option<String>,
}
impl std::fmt::Display for IssuingCardWallets {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IssuingCardholderAddress {
    #[serde(rename = "address")]
    ///
    pub address: Address,
}
impl std::fmt::Display for IssuingCardholderAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IssuingCardholderAuthorizationControls {
    #[serde(rename = "allowed_categories")]
    ///Array of strings containing [categories](https://stripe.com/docs/api#issuing_authorization_object-merchant_data-category) of authorizations to allow. All other categories will be blocked. Cannot be set with `blocked_categories`.
    pub allowed_categories: Option<Vec<String>>,
    #[serde(rename = "blocked_categories")]
    ///Array of strings containing [categories](https://stripe.com/docs/api#issuing_authorization_object-merchant_data-category) of authorizations to decline. All other categories will be allowed. Cannot be set with `allowed_categories`.
    pub blocked_categories: Option<Vec<String>>,
    #[serde(rename = "spending_limits")]
    ///Limit spending with amount-based rules that apply across this cardholder's cards.
    pub spending_limits: Option<Vec<IssuingCardholderSpendingLimit>>,
    #[serde(rename = "spending_limits_currency")]
    ///Currency of the amounts within `spending_limits`.
    pub spending_limits_currency: Option<String>,
}
impl std::fmt::Display for IssuingCardholderAuthorizationControls {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IssuingCardholderCompany {
    #[serde(rename = "tax_id_provided")]
    ///Whether the company's business ID number was provided.
    pub tax_id_provided: bool,
}
impl std::fmt::Display for IssuingCardholderCompany {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IssuingCardholderIdDocument {
    #[serde(rename = "back")]
    ///The back of a document returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `identity_document`.
    pub back: Option<serde_json::Value>,
    #[serde(rename = "front")]
    ///The front of a document returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `identity_document`.
    pub front: Option<serde_json::Value>,
}
impl std::fmt::Display for IssuingCardholderIdDocument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IssuingCardholderIndividual {
    #[serde(rename = "dob")]
    ///The date of birth of this cardholder.
    pub dob: Option<serde_json::Value>,
    #[serde(rename = "first_name")]
    ///The first name of this cardholder.
    pub first_name: String,
    #[serde(rename = "last_name")]
    ///The last name of this cardholder.
    pub last_name: String,
    #[serde(rename = "verification")]
    ///Government-issued ID document for this cardholder.
    pub verification: Option<serde_json::Value>,
}
impl std::fmt::Display for IssuingCardholderIndividual {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IssuingCardholderIndividualDob {
    #[serde(rename = "day")]
    ///The day of birth, between 1 and 31.
    pub day: Option<i64>,
    #[serde(rename = "month")]
    ///The month of birth, between 1 and 12.
    pub month: Option<i64>,
    #[serde(rename = "year")]
    ///The four-digit year of birth.
    pub year: Option<i64>,
}
impl std::fmt::Display for IssuingCardholderIndividualDob {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IssuingCardholderRequirements {
    #[serde(rename = "disabled_reason")]
    ///If `disabled_reason` is present, all cards will decline authorizations with `cardholder_verification_required` reason.
    pub disabled_reason: Option<String>,
    #[serde(rename = "past_due")]
    ///Array of fields that need to be collected in order to verify and re-enable the cardholder.
    pub past_due: Option<Vec<String>>,
}
impl std::fmt::Display for IssuingCardholderRequirements {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IssuingCardholderSpendingLimit {
    #[serde(rename = "amount")]
    ///Maximum amount allowed to spend per interval. This amount is in the card's currency and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    pub amount: i64,
    #[serde(rename = "categories")]
    ///Array of strings containing [categories](https://stripe.com/docs/api#issuing_authorization_object-merchant_data-category) this limit applies to. Omitting this field will apply the limit to all categories.
    pub categories: Option<Vec<String>>,
    #[serde(rename = "interval")]
    ///Interval (or event) to which the amount applies.
    pub interval: String,
}
impl std::fmt::Display for IssuingCardholderSpendingLimit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IssuingCardholderVerification {
    #[serde(rename = "document")]
    ///An identifying document, either a passport or local ID card.
    pub document: Option<serde_json::Value>,
}
impl std::fmt::Display for IssuingCardholderVerification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IssuingDisputeCanceledEvidence {
    #[serde(rename = "additional_documentation")]
    ///(ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute.
    pub additional_documentation: Option<serde_json::Value>,
    #[serde(rename = "canceled_at")]
    ///Date when order was canceled.
    pub canceled_at: Option<i64>,
    #[serde(rename = "cancellation_policy_provided")]
    ///Whether the cardholder was provided with a cancellation policy.
    pub cancellation_policy_provided: Option<bool>,
    #[serde(rename = "cancellation_reason")]
    ///Reason for canceling the order.
    pub cancellation_reason: Option<String>,
    #[serde(rename = "expected_at")]
    ///Date when the cardholder expected to receive the product.
    pub expected_at: Option<i64>,
    #[serde(rename = "explanation")]
    ///Explanation of why the cardholder is disputing this transaction.
    pub explanation: Option<String>,
    #[serde(rename = "product_description")]
    ///Description of the merchandise or service that was purchased.
    pub product_description: Option<String>,
    #[serde(rename = "product_type")]
    ///Whether the product was a merchandise or service.
    pub product_type: Option<String>,
    #[serde(rename = "return_status")]
    ///Result of cardholder's attempt to return the product.
    pub return_status: Option<String>,
    #[serde(rename = "returned_at")]
    ///Date when the product was returned or attempted to be returned.
    pub returned_at: Option<i64>,
}
impl std::fmt::Display for IssuingDisputeCanceledEvidence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IssuingDisputeDuplicateEvidence {
    #[serde(rename = "additional_documentation")]
    ///(ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute.
    pub additional_documentation: Option<serde_json::Value>,
    #[serde(rename = "card_statement")]
    ///(ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Copy of the card statement showing that the product had already been paid for.
    pub card_statement: Option<serde_json::Value>,
    #[serde(rename = "cash_receipt")]
    ///(ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Copy of the receipt showing that the product had been paid for in cash.
    pub cash_receipt: Option<serde_json::Value>,
    #[serde(rename = "check_image")]
    ///(ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Image of the front and back of the check that was used to pay for the product.
    pub check_image: Option<serde_json::Value>,
    #[serde(rename = "explanation")]
    ///Explanation of why the cardholder is disputing this transaction.
    pub explanation: Option<String>,
    #[serde(rename = "original_transaction")]
    ///Transaction (e.g., ipi_...) that the disputed transaction is a duplicate of. Of the two or more transactions that are copies of each other, this is original undisputed one.
    pub original_transaction: Option<String>,
}
impl std::fmt::Display for IssuingDisputeDuplicateEvidence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IssuingDisputeEvidence {
    #[serde(rename = "canceled")]
    ///
    pub canceled: Option<IssuingDisputeCanceledEvidence>,
    #[serde(rename = "duplicate")]
    ///
    pub duplicate: Option<IssuingDisputeDuplicateEvidence>,
    #[serde(rename = "fraudulent")]
    ///
    pub fraudulent: Option<IssuingDisputeFraudulentEvidence>,
    #[serde(rename = "merchandise_not_as_described")]
    ///
    pub merchandise_not_as_described: Option<
        IssuingDisputeMerchandiseNotAsDescribedEvidence,
    >,
    #[serde(rename = "not_received")]
    ///
    pub not_received: Option<IssuingDisputeNotReceivedEvidence>,
    #[serde(rename = "other")]
    ///
    pub other: Option<IssuingDisputeOtherEvidence>,
    #[serde(rename = "reason")]
    ///The reason for filing the dispute. Its value will match the field containing the evidence.
    pub reason: String,
    #[serde(rename = "service_not_as_described")]
    ///
    pub service_not_as_described: Option<IssuingDisputeServiceNotAsDescribedEvidence>,
}
impl std::fmt::Display for IssuingDisputeEvidence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IssuingDisputeFraudulentEvidence {
    #[serde(rename = "additional_documentation")]
    ///(ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute.
    pub additional_documentation: Option<serde_json::Value>,
    #[serde(rename = "explanation")]
    ///Explanation of why the cardholder is disputing this transaction.
    pub explanation: Option<String>,
}
impl std::fmt::Display for IssuingDisputeFraudulentEvidence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IssuingDisputeMerchandiseNotAsDescribedEvidence {
    #[serde(rename = "additional_documentation")]
    ///(ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute.
    pub additional_documentation: Option<serde_json::Value>,
    #[serde(rename = "explanation")]
    ///Explanation of why the cardholder is disputing this transaction.
    pub explanation: Option<String>,
    #[serde(rename = "received_at")]
    ///Date when the product was received.
    pub received_at: Option<i64>,
    #[serde(rename = "return_description")]
    ///Description of the cardholder's attempt to return the product.
    pub return_description: Option<String>,
    #[serde(rename = "return_status")]
    ///Result of cardholder's attempt to return the product.
    pub return_status: Option<String>,
    #[serde(rename = "returned_at")]
    ///Date when the product was returned or attempted to be returned.
    pub returned_at: Option<i64>,
}
impl std::fmt::Display for IssuingDisputeMerchandiseNotAsDescribedEvidence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IssuingDisputeNotReceivedEvidence {
    #[serde(rename = "additional_documentation")]
    ///(ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute.
    pub additional_documentation: Option<serde_json::Value>,
    #[serde(rename = "expected_at")]
    ///Date when the cardholder expected to receive the product.
    pub expected_at: Option<i64>,
    #[serde(rename = "explanation")]
    ///Explanation of why the cardholder is disputing this transaction.
    pub explanation: Option<String>,
    #[serde(rename = "product_description")]
    ///Description of the merchandise or service that was purchased.
    pub product_description: Option<String>,
    #[serde(rename = "product_type")]
    ///Whether the product was a merchandise or service.
    pub product_type: Option<String>,
}
impl std::fmt::Display for IssuingDisputeNotReceivedEvidence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IssuingDisputeOtherEvidence {
    #[serde(rename = "additional_documentation")]
    ///(ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute.
    pub additional_documentation: Option<serde_json::Value>,
    #[serde(rename = "explanation")]
    ///Explanation of why the cardholder is disputing this transaction.
    pub explanation: Option<String>,
    #[serde(rename = "product_description")]
    ///Description of the merchandise or service that was purchased.
    pub product_description: Option<String>,
    #[serde(rename = "product_type")]
    ///Whether the product was a merchandise or service.
    pub product_type: Option<String>,
}
impl std::fmt::Display for IssuingDisputeOtherEvidence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IssuingDisputeServiceNotAsDescribedEvidence {
    #[serde(rename = "additional_documentation")]
    ///(ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute.
    pub additional_documentation: Option<serde_json::Value>,
    #[serde(rename = "canceled_at")]
    ///Date when order was canceled.
    pub canceled_at: Option<i64>,
    #[serde(rename = "cancellation_reason")]
    ///Reason for canceling the order.
    pub cancellation_reason: Option<String>,
    #[serde(rename = "explanation")]
    ///Explanation of why the cardholder is disputing this transaction.
    pub explanation: Option<String>,
    #[serde(rename = "received_at")]
    ///Date when the product was received.
    pub received_at: Option<i64>,
}
impl std::fmt::Display for IssuingDisputeServiceNotAsDescribedEvidence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IssuingDisputeTreasury {
    #[serde(rename = "debit_reversal")]
    ///The Treasury [DebitReversal](https://stripe.com/docs/api/treasury/debit_reversals) representing this Issuing dispute
    pub debit_reversal: Option<String>,
    #[serde(rename = "received_debit")]
    ///The Treasury [ReceivedDebit](https://stripe.com/docs/api/treasury/received_debits) that is being disputed.
    pub received_debit: String,
}
impl std::fmt::Display for IssuingDisputeTreasury {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IssuingTransactionAmountDetails {
    #[serde(rename = "atm_fee")]
    ///The fee charged by the ATM for the cash withdrawal.
    pub atm_fee: Option<i64>,
}
impl std::fmt::Display for IssuingTransactionAmountDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IssuingTransactionFlightData {
    #[serde(rename = "departure_at")]
    ///The time that the flight departed.
    pub departure_at: Option<i64>,
    #[serde(rename = "passenger_name")]
    ///The name of the passenger.
    pub passenger_name: Option<String>,
    #[serde(rename = "refundable")]
    ///Whether the ticket is refundable.
    pub refundable: Option<bool>,
    #[serde(rename = "segments")]
    ///The legs of the trip.
    pub segments: Option<Vec<IssuingTransactionFlightDataLeg>>,
    #[serde(rename = "travel_agency")]
    ///The travel agency that issued the ticket.
    pub travel_agency: Option<String>,
}
impl std::fmt::Display for IssuingTransactionFlightData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IssuingTransactionFlightDataLeg {
    #[serde(rename = "arrival_airport_code")]
    ///The three-letter IATA airport code of the flight's destination.
    pub arrival_airport_code: Option<String>,
    #[serde(rename = "carrier")]
    ///The airline carrier code.
    pub carrier: Option<String>,
    #[serde(rename = "departure_airport_code")]
    ///The three-letter IATA airport code that the flight departed from.
    pub departure_airport_code: Option<String>,
    #[serde(rename = "flight_number")]
    ///The flight number.
    pub flight_number: Option<String>,
    #[serde(rename = "service_class")]
    ///The flight's service class.
    pub service_class: Option<String>,
    #[serde(rename = "stopover_allowed")]
    ///Whether a stopover is allowed on this flight.
    pub stopover_allowed: Option<bool>,
}
impl std::fmt::Display for IssuingTransactionFlightDataLeg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IssuingTransactionFuelData {
    #[serde(rename = "type")]
    ///The type of fuel that was purchased. One of `diesel`, `unleaded_plus`, `unleaded_regular`, `unleaded_super`, or `other`.
    pub type_: String,
    #[serde(rename = "unit")]
    ///The units for `volume_decimal`. One of `us_gallon` or `liter`.
    pub unit: String,
    #[serde(rename = "unit_cost_decimal")]
    ///The cost in cents per each unit of fuel, represented as a decimal string with at most 12 decimal places.
    pub unit_cost_decimal: String,
    #[serde(rename = "volume_decimal")]
    ///The volume of the fuel that was pumped, represented as a decimal string with at most 12 decimal places.
    pub volume_decimal: Option<String>,
}
impl std::fmt::Display for IssuingTransactionFuelData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IssuingTransactionLodgingData {
    #[serde(rename = "check_in_at")]
    ///The time of checking into the lodging.
    pub check_in_at: Option<i64>,
    #[serde(rename = "nights")]
    ///The number of nights stayed at the lodging.
    pub nights: Option<i64>,
}
impl std::fmt::Display for IssuingTransactionLodgingData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IssuingTransactionPurchaseDetails {
    #[serde(rename = "flight")]
    ///Information about the flight that was purchased with this transaction.
    pub flight: Option<serde_json::Value>,
    #[serde(rename = "fuel")]
    ///Information about fuel that was purchased with this transaction.
    pub fuel: Option<serde_json::Value>,
    #[serde(rename = "lodging")]
    ///Information about lodging that was purchased with this transaction.
    pub lodging: Option<serde_json::Value>,
    #[serde(rename = "receipt")]
    ///The line items in the purchase.
    pub receipt: Option<Vec<IssuingTransactionReceiptData>>,
    #[serde(rename = "reference")]
    ///A merchant-specific order number.
    pub reference: Option<String>,
}
impl std::fmt::Display for IssuingTransactionPurchaseDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IssuingTransactionReceiptData {
    #[serde(rename = "description")]
    ///The description of the item. The maximum length of this field is 26 characters.
    pub description: Option<String>,
    #[serde(rename = "quantity")]
    ///The quantity of the item.
    pub quantity: Option<f64>,
    #[serde(rename = "total")]
    ///The total for this line item in cents.
    pub total: Option<i64>,
    #[serde(rename = "unit_cost")]
    ///The unit cost of the item in cents.
    pub unit_cost: Option<i64>,
}
impl std::fmt::Display for IssuingTransactionReceiptData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IssuingTransactionTreasury {
    #[serde(rename = "received_credit")]
    ///The Treasury [ReceivedCredit](https://stripe.com/docs/api/treasury/received_debits) representing this Issuing transaction if it is a refund
    pub received_credit: Option<String>,
    #[serde(rename = "received_debit")]
    ///The Treasury [ReceivedDebit](https://stripe.com/docs/api/treasury/received_credits) representing this Issuing transaction if it is a capture
    pub received_debit: Option<String>,
}
impl std::fmt::Display for IssuingTransactionTreasury {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    #[serde(rename = "amount_discount")]
    ///Total discount amount applied. If no discounts were applied, defaults to 0.
    pub amount_discount: i64,
    #[serde(rename = "amount_subtotal")]
    ///Total before any discounts or taxes are applied.
    pub amount_subtotal: i64,
    #[serde(rename = "amount_tax")]
    ///Total tax amount applied. If no tax was applied, defaults to 0.
    pub amount_tax: i64,
    #[serde(rename = "amount_total")]
    ///Total after discounts and taxes.
    pub amount_total: i64,
    #[serde(rename = "currency")]
    ///Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: String,
    #[serde(rename = "description")]
    ///An arbitrary string attached to the object. Often useful for displaying to users. Defaults to product name.
    pub description: String,
    #[serde(rename = "discounts")]
    ///The discounts applied to the line item.
    pub discounts: Option<Vec<LineItemsDiscountAmount>>,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "price")]
    ///The price used to generate the line item.
    pub price: Option<serde_json::Value>,
    #[serde(rename = "product")]
    /**The ID of the product for this line item.

This will always be the same as `price.product`.*/
    pub product: Option<serde_json::Value>,
    #[serde(rename = "quantity")]
    ///The quantity of products being purchased.
    pub quantity: Option<i64>,
    #[serde(rename = "taxes")]
    ///The taxes applied to the line item.
    pub taxes: Option<Vec<LineItemsTaxAmount>>,
}
impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LegalEntityCompany {
    #[serde(rename = "address")]
    ///
    pub address: Option<Address>,
    #[serde(rename = "address_kana")]
    ///The Kana variation of the company's primary address (Japan only).
    pub address_kana: Option<serde_json::Value>,
    #[serde(rename = "address_kanji")]
    ///The Kanji variation of the company's primary address (Japan only).
    pub address_kanji: Option<serde_json::Value>,
    #[serde(rename = "directors_provided")]
    ///Whether the company's directors have been provided. This Boolean will be `true` if you've manually indicated that all directors are provided via [the `directors_provided` parameter](https://stripe.com/docs/api/accounts/update#update_account-company-directors_provided).
    pub directors_provided: Option<bool>,
    #[serde(rename = "executives_provided")]
    ///Whether the company's executives have been provided. This Boolean will be `true` if you've manually indicated that all executives are provided via [the `executives_provided` parameter](https://stripe.com/docs/api/accounts/update#update_account-company-executives_provided), or if Stripe determined that sufficient executives were provided.
    pub executives_provided: Option<bool>,
    #[serde(rename = "name")]
    ///The company's legal name.
    pub name: Option<String>,
    #[serde(rename = "name_kana")]
    ///The Kana variation of the company's legal name (Japan only).
    pub name_kana: Option<String>,
    #[serde(rename = "name_kanji")]
    ///The Kanji variation of the company's legal name (Japan only).
    pub name_kanji: Option<String>,
    #[serde(rename = "owners_provided")]
    ///Whether the company's owners have been provided. This Boolean will be `true` if you've manually indicated that all owners are provided via [the `owners_provided` parameter](https://stripe.com/docs/api/accounts/update#update_account-company-owners_provided), or if Stripe determined that sufficient owners were provided. Stripe determines ownership requirements using both the number of owners provided and their total percent ownership (calculated by adding the `percent_ownership` of each owner together).
    pub owners_provided: Option<bool>,
    #[serde(rename = "ownership_declaration")]
    ///This hash is used to attest that the beneficial owner information provided to Stripe is both current and correct.
    pub ownership_declaration: Option<serde_json::Value>,
    #[serde(rename = "phone")]
    ///The company's phone number (used for verification).
    pub phone: Option<String>,
    #[serde(rename = "structure")]
    ///The category identifying the legal structure of the company or legal entity. See [Business structure](https://stripe.com/docs/connect/identity-verification#business-structure) for more details.
    pub structure: Option<String>,
    #[serde(rename = "tax_id_provided")]
    ///Whether the company's business ID number was provided.
    pub tax_id_provided: Option<bool>,
    #[serde(rename = "tax_id_registrar")]
    ///The jurisdiction in which the `tax_id` is registered (Germany-based companies only).
    pub tax_id_registrar: Option<String>,
    #[serde(rename = "vat_id_provided")]
    ///Whether the company's business VAT number was provided.
    pub vat_id_provided: Option<bool>,
    #[serde(rename = "verification")]
    ///Information on the verification state of the company.
    pub verification: Option<serde_json::Value>,
}
impl std::fmt::Display for LegalEntityCompany {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LegalEntityCompanyVerification {
    #[serde(rename = "document")]
    ///
    pub document: LegalEntityCompanyVerificationDocument,
}
impl std::fmt::Display for LegalEntityCompanyVerification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LegalEntityCompanyVerificationDocument {
    #[serde(rename = "back")]
    ///The back of a document returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `additional_verification`.
    pub back: Option<serde_json::Value>,
    #[serde(rename = "details")]
    ///A user-displayable string describing the verification state of this document.
    pub details: Option<String>,
    #[serde(rename = "details_code")]
    ///One of `document_corrupt`, `document_expired`, `document_failed_copy`, `document_failed_greyscale`, `document_failed_other`, `document_failed_test_mode`, `document_fraudulent`, `document_incomplete`, `document_invalid`, `document_manipulated`, `document_not_readable`, `document_not_uploaded`, `document_type_not_supported`, or `document_too_large`. A machine-readable code specifying the verification state for this document.
    pub details_code: Option<String>,
    #[serde(rename = "front")]
    ///The front of a document returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `additional_verification`.
    pub front: Option<serde_json::Value>,
}
impl std::fmt::Display for LegalEntityCompanyVerificationDocument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LegalEntityDob {
    #[serde(rename = "day")]
    ///The day of birth, between 1 and 31.
    pub day: Option<i64>,
    #[serde(rename = "month")]
    ///The month of birth, between 1 and 12.
    pub month: Option<i64>,
    #[serde(rename = "year")]
    ///The four-digit year of birth.
    pub year: Option<i64>,
}
impl std::fmt::Display for LegalEntityDob {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LegalEntityJapanAddress {
    #[serde(rename = "city")]
    ///City/Ward.
    pub city: Option<String>,
    #[serde(rename = "country")]
    ///Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    pub country: Option<String>,
    #[serde(rename = "line1")]
    ///Block/Building number.
    pub line1: Option<String>,
    #[serde(rename = "line2")]
    ///Building details.
    pub line2: Option<String>,
    #[serde(rename = "postal_code")]
    ///ZIP or postal code.
    pub postal_code: Option<String>,
    #[serde(rename = "state")]
    ///Prefecture.
    pub state: Option<String>,
    #[serde(rename = "town")]
    ///Town/cho-me.
    pub town: Option<String>,
}
impl std::fmt::Display for LegalEntityJapanAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LegalEntityPersonVerification {
    #[serde(rename = "additional_document")]
    ///A document showing address, either a passport, local ID card, or utility bill from a well-known utility company.
    pub additional_document: Option<serde_json::Value>,
    #[serde(rename = "details")]
    ///A user-displayable string describing the verification state for the person. For example, this may say "Provided identity information could not be verified".
    pub details: Option<String>,
    #[serde(rename = "details_code")]
    ///One of `document_address_mismatch`, `document_dob_mismatch`, `document_duplicate_type`, `document_id_number_mismatch`, `document_name_mismatch`, `document_nationality_mismatch`, `failed_keyed_identity`, or `failed_other`. A machine-readable code specifying the verification state for the person.
    pub details_code: Option<String>,
    #[serde(rename = "document")]
    ///
    pub document: Option<LegalEntityPersonVerificationDocument>,
    #[serde(rename = "status")]
    ///The state of verification for the person. Possible values are `unverified`, `pending`, or `verified`.
    pub status: String,
}
impl std::fmt::Display for LegalEntityPersonVerification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LegalEntityPersonVerificationDocument {
    #[serde(rename = "back")]
    ///The back of an ID returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `identity_document`.
    pub back: Option<serde_json::Value>,
    #[serde(rename = "details")]
    ///A user-displayable string describing the verification state of this document. For example, if a document is uploaded and the picture is too fuzzy, this may say "Identity document is too unclear to read".
    pub details: Option<String>,
    #[serde(rename = "details_code")]
    ///One of `document_corrupt`, `document_country_not_supported`, `document_expired`, `document_failed_copy`, `document_failed_other`, `document_failed_test_mode`, `document_fraudulent`, `document_failed_greyscale`, `document_incomplete`, `document_invalid`, `document_manipulated`, `document_missing_back`, `document_missing_front`, `document_not_readable`, `document_not_uploaded`, `document_photo_mismatch`, `document_too_large`, or `document_type_not_supported`. A machine-readable code specifying the verification state for this document.
    pub details_code: Option<String>,
    #[serde(rename = "front")]
    ///The front of an ID returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `identity_document`.
    pub front: Option<serde_json::Value>,
}
impl std::fmt::Display for LegalEntityPersonVerificationDocument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LegalEntityUboDeclaration {
    #[serde(rename = "date")]
    ///The Unix timestamp marking when the beneficial owner attestation was made.
    pub date: Option<i64>,
    #[serde(rename = "ip")]
    ///The IP address from which the beneficial owner attestation was made.
    pub ip: Option<String>,
    #[serde(rename = "user_agent")]
    ///The user-agent string from the browser where the beneficial owner attestation was made.
    pub user_agent: Option<String>,
}
impl std::fmt::Display for LegalEntityUboDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LineItem {
    #[serde(rename = "amount")]
    ///The amount, in %s.
    pub amount: i64,
    #[serde(rename = "amount_excluding_tax")]
    ///The integer amount in %s representing the amount for this line item, excluding all tax and discounts.
    pub amount_excluding_tax: Option<i64>,
    #[serde(rename = "currency")]
    ///Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: String,
    #[serde(rename = "description")]
    ///An arbitrary string attached to the object. Often useful for displaying to users.
    pub description: Option<String>,
    #[serde(rename = "discount_amounts")]
    ///The amount of discount calculated per discount for this line item.
    pub discount_amounts: Option<Vec<DiscountsResourceDiscountAmount>>,
    #[serde(rename = "discountable")]
    ///If true, discounts will apply to this line item. Always false for prorations.
    pub discountable: bool,
    #[serde(rename = "discounts")]
    ///The discounts applied to the invoice line item. Line item discounts are applied before invoice discounts. Use `expand[]=discounts` to expand each discount.
    pub discounts: Option<Vec<serde_json::Value>>,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "invoice_item")]
    ///The ID of the [invoice item](https://stripe.com/docs/api/invoiceitems) associated with this line item if any.
    pub invoice_item: Option<String>,
    #[serde(rename = "livemode")]
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    #[serde(rename = "metadata")]
    ///Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format. Note that for line items with `type=subscription` this will reflect the metadata of the subscription that caused the line item to be created.
    pub metadata: serde_json::Value,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "period")]
    ///
    pub period: InvoiceLineItemPeriod,
    #[serde(rename = "price")]
    ///The price of the line item.
    pub price: Option<serde_json::Value>,
    #[serde(rename = "proration")]
    ///Whether this is a proration.
    pub proration: bool,
    #[serde(rename = "proration_details")]
    ///Additional details for proration line items
    pub proration_details: Option<serde_json::Value>,
    #[serde(rename = "quantity")]
    ///The quantity of the subscription, if the line item is a subscription or a proration.
    pub quantity: Option<i64>,
    #[serde(rename = "subscription")]
    ///The subscription that the invoice item pertains to, if any.
    pub subscription: Option<String>,
    #[serde(rename = "subscription_item")]
    ///The subscription item that generated this invoice item. Left empty if the line item is not an explicit result of a subscription.
    pub subscription_item: Option<String>,
    #[serde(rename = "tax_amounts")]
    ///The amount of tax calculated per tax rate for this line item
    pub tax_amounts: Option<Vec<InvoiceTaxAmount>>,
    #[serde(rename = "tax_rates")]
    ///The tax rates which apply to the line item.
    pub tax_rates: Option<Vec<TaxRate>>,
    #[serde(rename = "type")]
    ///A string identifying the type of the source of this line item, either an `invoiceitem` or a `subscription`.
    pub type_: String,
    #[serde(rename = "unit_amount_excluding_tax")]
    ///The amount in %s representing the unit amount for this line item, excluding all tax and discounts.
    pub unit_amount_excluding_tax: Option<String>,
}
impl std::fmt::Display for LineItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LineItemsDiscountAmount {
    #[serde(rename = "amount")]
    ///The amount discounted.
    pub amount: i64,
    #[serde(rename = "discount")]
    /**A discount represents the actual application of a [coupon](https://stripe.com/docs/api#coupons) or [promotion code](https://stripe.com/docs/api#promotion_codes).
It contains information about when the discount began, when it will end, and what it is applied to.

Related guide: [Applying Discounts to Subscriptions](https://stripe.com/docs/billing/subscriptions/discounts).*/
    pub discount: Discount,
}
impl std::fmt::Display for LineItemsDiscountAmount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LineItemsTaxAmount {
    #[serde(rename = "amount")]
    ///Amount of tax applied for this rate.
    pub amount: i64,
    #[serde(rename = "rate")]
    /**Tax rates can be applied to [invoices](https://stripe.com/docs/billing/invoices/tax-rates), [subscriptions](https://stripe.com/docs/billing/subscriptions/taxes) and [Checkout Sessions](https://stripe.com/docs/payments/checkout/set-up-a-subscription#tax-rates) to collect tax.

Related guide: [Tax Rates](https://stripe.com/docs/billing/taxes/tax-rates).*/
    pub rate: TaxRate,
}
impl std::fmt::Display for LineItemsTaxAmount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LinkedAccountOptionsUsBankAccount {
    #[serde(rename = "permissions")]
    ///The list of permissions to request. The `payment_method` permission must be included.
    pub permissions: Option<Vec<String>>,
    #[serde(rename = "return_url")]
    ///For webview integrations only. Upon completing OAuth login in the native browser, the user will be redirected to this URL to return to your app.
    pub return_url: Option<String>,
}
impl std::fmt::Display for LinkedAccountOptionsUsBankAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginLink {
    #[serde(rename = "created")]
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "url")]
    ///The URL for the login link.
    pub url: String,
}
impl std::fmt::Display for LoginLink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Mandate {
    #[serde(rename = "customer_acceptance")]
    ///
    pub customer_acceptance: CustomerAcceptance,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "livemode")]
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    #[serde(rename = "multi_use")]
    ///
    pub multi_use: Option<MandateMultiUse>,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "payment_method")]
    ///ID of the payment method associated with this mandate.
    pub payment_method: serde_json::Value,
    #[serde(rename = "payment_method_details")]
    ///
    pub payment_method_details: MandatePaymentMethodDetails,
    #[serde(rename = "single_use")]
    ///
    pub single_use: Option<MandateSingleUse>,
    #[serde(rename = "status")]
    ///The status of the mandate, which indicates whether it can be used to initiate a payment.
    pub status: String,
    #[serde(rename = "type")]
    ///The type of the mandate.
    pub type_: String,
}
impl std::fmt::Display for Mandate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct MandateAcssDebit {
    #[serde(rename = "default_for")]
    ///List of Stripe products where this mandate can be selected automatically.
    pub default_for: Option<Vec<String>>,
    #[serde(rename = "interval_description")]
    ///Description of the interval. Only required if the 'payment_schedule' parameter is 'interval' or 'combined'.
    pub interval_description: Option<String>,
    #[serde(rename = "payment_schedule")]
    ///Payment schedule for the mandate.
    pub payment_schedule: String,
    #[serde(rename = "transaction_type")]
    ///Transaction type of the mandate.
    pub transaction_type: String,
}
impl std::fmt::Display for MandateAcssDebit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct MandateAuBecsDebit {
    #[serde(rename = "url")]
    ///The URL of the mandate. This URL generally contains sensitive information about the customer and should be shared with them exclusively.
    pub url: String,
}
impl std::fmt::Display for MandateAuBecsDebit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct MandateBacsDebit {
    #[serde(rename = "network_status")]
    ///The status of the mandate on the Bacs network. Can be one of `pending`, `revoked`, `refused`, or `accepted`.
    pub network_status: String,
    #[serde(rename = "reference")]
    ///The unique reference identifying the mandate on the Bacs network.
    pub reference: String,
    #[serde(rename = "url")]
    ///The URL that will contain the mandate that the customer has signed.
    pub url: String,
}
impl std::fmt::Display for MandateBacsDebit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct MandateBlik {
    #[serde(rename = "expires_after")]
    ///Date at which the mandate expires.
    pub expires_after: Option<i64>,
    #[serde(rename = "off_session")]
    ///
    pub off_session: Option<MandateOptionsOffSessionDetailsBlik>,
    #[serde(rename = "type")]
    ///Type of the mandate.
    pub type_: Option<String>,
}
impl std::fmt::Display for MandateBlik {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct MandateLink {}
impl std::fmt::Display for MandateLink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct MandateMultiUse {}
impl std::fmt::Display for MandateMultiUse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct MandateOptionsOffSessionDetailsBlik {
    #[serde(rename = "amount")]
    ///Amount of each recurring payment.
    pub amount: Option<i64>,
    #[serde(rename = "currency")]
    ///Currency of each recurring payment.
    pub currency: Option<String>,
    #[serde(rename = "interval")]
    ///Frequency interval of each recurring payment.
    pub interval: Option<String>,
    #[serde(rename = "interval_count")]
    ///Frequency indicator of each recurring payment.
    pub interval_count: Option<i64>,
}
impl std::fmt::Display for MandateOptionsOffSessionDetailsBlik {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct MandatePaymentMethodDetails {
    #[serde(rename = "acss_debit")]
    ///
    pub acss_debit: Option<MandateAcssDebit>,
    #[serde(rename = "au_becs_debit")]
    ///
    pub au_becs_debit: Option<MandateAuBecsDebit>,
    #[serde(rename = "bacs_debit")]
    ///
    pub bacs_debit: Option<MandateBacsDebit>,
    #[serde(rename = "blik")]
    ///
    pub blik: Option<MandateBlik>,
    #[serde(rename = "card")]
    ///
    pub card: Option<CardMandatePaymentMethodDetails>,
    #[serde(rename = "link")]
    ///
    pub link: Option<MandateLink>,
    #[serde(rename = "sepa_debit")]
    ///
    pub sepa_debit: Option<MandateSepaDebit>,
    #[serde(rename = "type")]
    ///The type of the payment method associated with this mandate. An additional hash is included on `payment_method_details` with a name matching this value. It contains mandate information specific to the payment method.
    pub type_: String,
    #[serde(rename = "us_bank_account")]
    ///
    pub us_bank_account: Option<MandateUsBankAccount>,
}
impl std::fmt::Display for MandatePaymentMethodDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct MandateSepaDebit {
    #[serde(rename = "reference")]
    ///The unique reference of the mandate.
    pub reference: String,
    #[serde(rename = "url")]
    ///The URL of the mandate. This URL generally contains sensitive information about the customer and should be shared with them exclusively.
    pub url: String,
}
impl std::fmt::Display for MandateSepaDebit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct MandateSingleUse {
    #[serde(rename = "amount")]
    ///On a single use mandate, the amount of the payment.
    pub amount: i64,
    #[serde(rename = "currency")]
    ///On a single use mandate, the currency of the payment.
    pub currency: String,
}
impl std::fmt::Display for MandateSingleUse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct MandateUsBankAccount {}
impl std::fmt::Display for MandateUsBankAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Networks {
    #[serde(rename = "available")]
    ///All available networks for the card.
    pub available: Vec<String>,
    #[serde(rename = "preferred")]
    ///The preferred network for the card.
    pub preferred: Option<String>,
}
impl std::fmt::Display for Networks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationEventData {
    #[serde(rename = "object")]
    ///Object containing the API resource relevant to the event. For example, an `invoice.created` event will have a full [invoice object](https://stripe.com/docs/api#invoice_object) as the value of the object key.
    pub object: serde_json::Value,
    #[serde(rename = "previous_attributes")]
    ///Object containing the names of the attributes that have changed, and their previous values (sent along only with *.updated events).
    pub previous_attributes: Option<serde_json::Value>,
}
impl std::fmt::Display for NotificationEventData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationEventRequest {
    #[serde(rename = "id")]
    ///ID of the API request that caused the event. If null, the event was automatic (e.g., Stripe's automatic subscription handling). Request logs are available in the [dashboard](https://dashboard.stripe.com/logs), but currently not in the API.
    pub id: Option<String>,
    #[serde(rename = "idempotency_key")]
    ///The idempotency key transmitted during the request, if any. *Note: This property is populated only for events on or after May 23, 2017*.
    pub idempotency_key: Option<String>,
}
impl std::fmt::Display for NotificationEventRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct OfflineAcceptance {}
impl std::fmt::Display for OfflineAcceptance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct OnlineAcceptance {
    #[serde(rename = "ip_address")]
    ///The IP address from which the Mandate was accepted by the customer.
    pub ip_address: Option<String>,
    #[serde(rename = "user_agent")]
    ///The user agent of the browser from which the Mandate was accepted by the customer.
    pub user_agent: Option<String>,
}
impl std::fmt::Display for OnlineAcceptance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
    #[serde(rename = "amount_subtotal")]
    ///Order cost before any discounts or taxes are applied. A positive integer representing the subtotal of the order in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal) (e.g., 100 cents to charge $1.00 or 100 to charge ¥100, a zero-decimal currency).
    pub amount_subtotal: i64,
    #[serde(rename = "amount_total")]
    ///Total order cost after discounts and taxes are applied. A positive integer representing the cost of the order in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal) (e.g., 100 cents to charge $1.00 or 100 to charge ¥100, a zero-decimal currency). To submit an order, the total must be either 0 or at least $0.50 USD or [equivalent in charge currency](https://stripe.com/docs/currencies#minimum-and-maximum-charge-amounts).
    pub amount_total: i64,
    #[serde(rename = "application")]
    ///ID of the Connect application that created the Order, if any.
    pub application: Option<serde_json::Value>,
    #[serde(rename = "automatic_tax")]
    ///
    pub automatic_tax: Option<OrdersV2ResourceAutomaticTax>,
    #[serde(rename = "billing_details")]
    ///Customer billing details associated with the order.
    pub billing_details: Option<serde_json::Value>,
    #[serde(rename = "client_secret")]
    /**The client secret of this Order. Used for client-side retrieval using a publishable key.

The client secret can be used to complete a payment for an Order from your frontend. It should not be stored, logged, embedded in URLs, or exposed to anyone other than the customer. Make sure that you have TLS enabled on any page that includes the client secret.

Refer to our docs for [creating and processing an order](https://stripe.com/docs/orders-beta/create-and-process) to learn about how client_secret should be handled.*/
    pub client_secret: Option<String>,
    #[serde(rename = "created")]
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    #[serde(rename = "currency")]
    ///Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: String,
    #[serde(rename = "customer")]
    ///The customer which this orders belongs to.
    pub customer: Option<serde_json::Value>,
    #[serde(rename = "description")]
    ///An arbitrary string attached to the object. Often useful for displaying to users.
    pub description: Option<String>,
    #[serde(rename = "discounts")]
    ///The discounts applied to the order. Use `expand[]=discounts` to expand each discount.
    pub discounts: Option<Vec<serde_json::Value>>,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "ip_address")]
    ///A recent IP address of the purchaser used for tax reporting and tax location inference.
    pub ip_address: Option<String>,
    #[serde(rename = "line_items")]
    ///A list of line items the customer is ordering. Each line item includes information about the product, the quantity, and the resulting cost. There is a maximum of 100 line items.
    pub line_items: Option<serde_json::Value>,
    #[serde(rename = "livemode")]
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    #[serde(rename = "metadata")]
    ///Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<serde_json::Value>,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "payment")]
    ///
    pub payment: OrdersV2ResourcePayment,
    #[serde(rename = "shipping_cost")]
    ///The details of the customer cost of shipping, including the customer chosen ShippingRate.
    pub shipping_cost: Option<serde_json::Value>,
    #[serde(rename = "shipping_details")]
    ///Customer shipping information associated with the order.
    pub shipping_details: Option<serde_json::Value>,
    #[serde(rename = "status")]
    ///The overall status of the order.
    pub status: String,
    #[serde(rename = "tax_details")]
    ///
    pub tax_details: Option<OrdersV2ResourceTaxDetails>,
    #[serde(rename = "total_details")]
    ///
    pub total_details: OrdersV2ResourceTotalDetails,
}
impl std::fmt::Display for Order {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct OrdersPaymentMethodOptionsAfterpayClearpay {
    #[serde(rename = "capture_method")]
    ///Controls when the funds will be captured from the customer's account.
    pub capture_method: Option<String>,
    #[serde(rename = "reference")]
    ///Order identifier shown to the user in Afterpay's online portal. We recommend using a value that helps you answer any questions a customer might have about the payment. The identifier is limited to 128 characters and may contain only letters, digits, underscores, backslashes and dashes.
    pub reference: Option<String>,
    #[serde(rename = "setup_future_usage")]
    /**Indicates that you intend to make future payments with the payment method.

Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the order's Customer, if present, after the order's PaymentIntent is confirmed and any required actions from the user are complete. If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.

When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).

If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.*/
    pub setup_future_usage: Option<String>,
}
impl std::fmt::Display for OrdersPaymentMethodOptionsAfterpayClearpay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct OrdersV2ResourceAutomaticPaymentMethods {
    #[serde(rename = "enabled")]
    ///Whether this Order has been opted into managing payment method types via the [Stripe Dashboard](https://dashboard.stripe.com/settings/payment_methods).
    pub enabled: bool,
}
impl std::fmt::Display for OrdersV2ResourceAutomaticPaymentMethods {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct OrdersV2ResourceAutomaticTax {
    #[serde(rename = "enabled")]
    ///Whether Stripe automatically computes tax on this Order.
    pub enabled: bool,
    #[serde(rename = "status")]
    ///The status of the most recent automated tax calculation for this Order.
    pub status: Option<String>,
}
impl std::fmt::Display for OrdersV2ResourceAutomaticTax {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct OrdersV2ResourceBillingDetails {
    #[serde(rename = "address")]
    ///Billing address for the order.
    pub address: Option<serde_json::Value>,
    #[serde(rename = "email")]
    ///Email address for the order.
    pub email: Option<String>,
    #[serde(rename = "name")]
    ///Full name for the order.
    pub name: Option<String>,
    #[serde(rename = "phone")]
    ///Billing phone number for the order (including extension).
    pub phone: Option<String>,
}
impl std::fmt::Display for OrdersV2ResourceBillingDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct OrdersV2ResourceCardPaymentMethodOptions {
    #[serde(rename = "capture_method")]
    ///Controls when the funds will be captured from the customer's account.
    pub capture_method: String,
    #[serde(rename = "setup_future_usage")]
    /**Indicates that you intend to make future payments with the payment method.

Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the order's Customer, if present, after the order's PaymentIntent is confirmed and any required actions from the user are complete. If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.

When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).

If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.*/
    pub setup_future_usage: Option<String>,
}
impl std::fmt::Display for OrdersV2ResourceCardPaymentMethodOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct OrdersV2ResourcePayment {
    #[serde(rename = "payment_intent")]
    ///ID of the payment intent associated with this order. Null when the order is `open`.
    pub payment_intent: Option<serde_json::Value>,
    #[serde(rename = "settings")]
    ///Settings describing how the order should configure generated PaymentIntents.
    pub settings: Option<serde_json::Value>,
    #[serde(rename = "status")]
    ///The status of the underlying payment associated with this order, if any. Null when the order is `open`.
    pub status: Option<String>,
}
impl std::fmt::Display for OrdersV2ResourcePayment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct OrdersV2ResourcePaymentMethodOptions {
    #[serde(rename = "acss_debit")]
    ///
    pub acss_debit: Option<PaymentIntentPaymentMethodOptionsAcssDebit>,
    #[serde(rename = "afterpay_clearpay")]
    ///
    pub afterpay_clearpay: Option<OrdersPaymentMethodOptionsAfterpayClearpay>,
    #[serde(rename = "alipay")]
    ///
    pub alipay: Option<PaymentMethodOptionsAlipay>,
    #[serde(rename = "bancontact")]
    ///
    pub bancontact: Option<PaymentMethodOptionsBancontact>,
    #[serde(rename = "card")]
    ///
    pub card: Option<OrdersV2ResourceCardPaymentMethodOptions>,
    #[serde(rename = "customer_balance")]
    ///
    pub customer_balance: Option<PaymentMethodOptionsCustomerBalance>,
    #[serde(rename = "ideal")]
    ///
    pub ideal: Option<PaymentMethodOptionsIdeal>,
    #[serde(rename = "klarna")]
    ///
    pub klarna: Option<PaymentMethodOptionsKlarna>,
    #[serde(rename = "link")]
    ///
    pub link: Option<PaymentIntentPaymentMethodOptionsLink>,
    #[serde(rename = "oxxo")]
    ///
    pub oxxo: Option<PaymentMethodOptionsOxxo>,
    #[serde(rename = "p24")]
    ///
    pub p24: Option<PaymentMethodOptionsP24>,
    #[serde(rename = "paypal")]
    ///
    pub paypal: Option<PaymentMethodOptionsPaypal>,
    #[serde(rename = "sepa_debit")]
    ///
    pub sepa_debit: Option<PaymentIntentPaymentMethodOptionsSepaDebit>,
    #[serde(rename = "sofort")]
    ///
    pub sofort: Option<PaymentMethodOptionsSofort>,
    #[serde(rename = "wechat_pay")]
    ///
    pub wechat_pay: Option<PaymentMethodOptionsWechatPay>,
}
impl std::fmt::Display for OrdersV2ResourcePaymentMethodOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct OrdersV2ResourcePaymentSettings {
    #[serde(rename = "application_fee_amount")]
    ///The amount of the application fee (if any) that will be requested to be applied to the payment and transferred to the application owner's Stripe account.
    pub application_fee_amount: Option<i64>,
    #[serde(rename = "automatic_payment_methods")]
    ///Indicates whether order has been opted into using [Stripe Dashboard](https://dashboard.stripe.com/settings/payment_methods) to manage payment method types.
    pub automatic_payment_methods: Option<serde_json::Value>,
    #[serde(rename = "payment_method_options")]
    ///PaymentMethod-specific configuration to provide to the order's PaymentIntent.
    pub payment_method_options: Option<serde_json::Value>,
    #[serde(rename = "payment_method_types")]
    ///The list of [payment method types](https://stripe.com/docs/payments/payment-methods/overview) to provide to the order's PaymentIntent. Do not include this attribute if you prefer to manage your payment methods from the [Stripe Dashboard](https://dashboard.stripe.com/settings/payment_methods).
    pub payment_method_types: Option<Vec<String>>,
    #[serde(rename = "return_url")]
    ///The URL to redirect the customer to after they authenticate their payment.
    pub return_url: Option<String>,
    #[serde(rename = "statement_descriptor")]
    ///For non-card charges, you can use this value as the complete description that appears on your customers' statements. Must contain at least one letter, maximum 22 characters.
    pub statement_descriptor: Option<String>,
    #[serde(rename = "statement_descriptor_suffix")]
    ///Provides information about a card payment that customers see on their statements. Concatenated with the prefix (shortened descriptor) or statement descriptor that’s set on the account to form the complete statement descriptor. Maximum 22 characters for the concatenated descriptor.
    pub statement_descriptor_suffix: Option<String>,
    #[serde(rename = "transfer_data")]
    ///Provides configuration for completing a transfer for the order after it is paid.
    pub transfer_data: Option<serde_json::Value>,
}
impl std::fmt::Display for OrdersV2ResourcePaymentSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct OrdersV2ResourceShippingCost {
    #[serde(rename = "amount_subtotal")]
    ///Total shipping cost before any discounts or taxes are applied.
    pub amount_subtotal: i64,
    #[serde(rename = "amount_tax")]
    ///Total tax amount applied due to shipping costs. If no tax was applied, defaults to 0.
    pub amount_tax: i64,
    #[serde(rename = "amount_total")]
    ///Total shipping cost after discounts and taxes are applied.
    pub amount_total: i64,
    #[serde(rename = "shipping_rate")]
    ///The ID of the ShippingRate for this order.
    pub shipping_rate: Option<serde_json::Value>,
    #[serde(rename = "taxes")]
    ///The taxes applied to the shipping rate.
    pub taxes: Option<Vec<LineItemsTaxAmount>>,
}
impl std::fmt::Display for OrdersV2ResourceShippingCost {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct OrdersV2ResourceShippingDetails {
    #[serde(rename = "address")]
    ///Recipient shipping address. Required if the order includes products that are shippable.
    pub address: Option<serde_json::Value>,
    #[serde(rename = "name")]
    ///Recipient name.
    pub name: Option<String>,
    #[serde(rename = "phone")]
    ///Recipient phone (including extension).
    pub phone: Option<String>,
}
impl std::fmt::Display for OrdersV2ResourceShippingDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct OrdersV2ResourceTaxDetails {
    #[serde(rename = "tax_exempt")]
    ///Describes the purchaser's tax exemption status. One of `none`, `exempt`, or `reverse`.
    pub tax_exempt: String,
    #[serde(rename = "tax_ids")]
    ///The purchaser's tax IDs to be used in calculation of tax for this Order.
    pub tax_ids: Vec<OrdersV2ResourceTaxDetailsResourceTaxId>,
}
impl std::fmt::Display for OrdersV2ResourceTaxDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct OrdersV2ResourceTaxDetailsResourceTaxId {
    #[serde(rename = "type")]
    ///The type of the tax ID, one of `eu_vat`, `br_cnpj`, `br_cpf`, `eu_oss_vat`, `gb_vat`, `nz_gst`, `au_abn`, `au_arn`, `in_gst`, `no_vat`, `za_vat`, `ch_vat`, `mx_rfc`, `sg_uen`, `ru_inn`, `ru_kpp`, `ca_bn`, `hk_br`, `es_cif`, `tw_vat`, `th_vat`, `jp_cn`, `jp_rn`, `li_uid`, `my_itn`, `us_ein`, `kr_brn`, `ca_qst`, `ca_gst_hst`, `ca_pst_bc`, `ca_pst_mb`, `ca_pst_sk`, `my_sst`, `sg_gst`, `ae_trn`, `cl_tin`, `sa_vat`, `id_npwp`, `my_frp`, `il_vat`, `ge_vat`, `ua_vat`, `is_vat`, `bg_uic`, `hu_tin`, `si_tin`, or `unknown`
    pub type_: String,
    #[serde(rename = "value")]
    ///The value of the tax ID.
    pub value: Option<String>,
}
impl std::fmt::Display for OrdersV2ResourceTaxDetailsResourceTaxId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct OrdersV2ResourceTotalDetails {
    #[serde(rename = "amount_discount")]
    ///This is the sum of all the discounts.
    pub amount_discount: i64,
    #[serde(rename = "amount_shipping")]
    ///This is the sum of all the shipping amounts.
    pub amount_shipping: Option<i64>,
    #[serde(rename = "amount_tax")]
    ///This is the sum of all the tax amounts.
    pub amount_tax: i64,
    #[serde(rename = "breakdown")]
    ///
    pub breakdown: Option<OrdersV2ResourceTotalDetailsApiResourceBreakdown>,
}
impl std::fmt::Display for OrdersV2ResourceTotalDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct OrdersV2ResourceTotalDetailsApiResourceBreakdown {
    #[serde(rename = "discounts")]
    ///The aggregated discounts.
    pub discounts: Vec<LineItemsDiscountAmount>,
    #[serde(rename = "taxes")]
    ///The aggregated tax amounts by rate.
    pub taxes: Vec<LineItemsTaxAmount>,
}
impl std::fmt::Display for OrdersV2ResourceTotalDetailsApiResourceBreakdown {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct OrdersV2ResourceTransferData {
    #[serde(rename = "amount")]
    ///The amount that will be transferred automatically when the order is paid. If no amount is set, the full amount is transferred. There cannot be any line items with recurring prices when using this field.
    pub amount: Option<i64>,
    #[serde(rename = "destination")]
    ///ID of the Connected account receiving the transfer.
    pub destination: serde_json::Value,
}
impl std::fmt::Display for OrdersV2ResourceTransferData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct OutboundPaymentsPaymentMethodDetails {
    #[serde(rename = "billing_details")]
    ///
    pub billing_details: TreasurySharedResourceBillingDetails,
    #[serde(rename = "financial_account")]
    ///
    pub financial_account: Option<OutboundPaymentsPaymentMethodDetailsFinancialAccount>,
    #[serde(rename = "type")]
    ///The type of the payment method used in the OutboundPayment.
    pub type_: String,
    #[serde(rename = "us_bank_account")]
    ///
    pub us_bank_account: Option<OutboundPaymentsPaymentMethodDetailsUsBankAccount>,
}
impl std::fmt::Display for OutboundPaymentsPaymentMethodDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct OutboundPaymentsPaymentMethodDetailsFinancialAccount {
    #[serde(rename = "id")]
    ///Token of the FinancialAccount.
    pub id: String,
    #[serde(rename = "network")]
    ///The rails used to send funds.
    pub network: String,
}
impl std::fmt::Display for OutboundPaymentsPaymentMethodDetailsFinancialAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct OutboundPaymentsPaymentMethodDetailsUsBankAccount {
    #[serde(rename = "account_holder_type")]
    ///Account holder type: individual or company.
    pub account_holder_type: Option<String>,
    #[serde(rename = "account_type")]
    ///Account type: checkings or savings. Defaults to checking if omitted.
    pub account_type: Option<String>,
    #[serde(rename = "bank_name")]
    ///Name of the bank associated with the bank account.
    pub bank_name: Option<String>,
    #[serde(rename = "fingerprint")]
    ///Uniquely identifies this particular bank account. You can use this attribute to check whether two bank accounts are the same.
    pub fingerprint: Option<String>,
    #[serde(rename = "last4")]
    ///Last four digits of the bank account number.
    pub last4: Option<String>,
    #[serde(rename = "network")]
    ///The US bank account network used to send funds.
    pub network: String,
    #[serde(rename = "routing_number")]
    ///Routing number of the bank account.
    pub routing_number: Option<String>,
}
impl std::fmt::Display for OutboundPaymentsPaymentMethodDetailsUsBankAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct OutboundTransfersPaymentMethodDetails {
    #[serde(rename = "billing_details")]
    ///
    pub billing_details: TreasurySharedResourceBillingDetails,
    #[serde(rename = "type")]
    ///The type of the payment method used in the OutboundTransfer.
    pub type_: String,
    #[serde(rename = "us_bank_account")]
    ///
    pub us_bank_account: Option<OutboundTransfersPaymentMethodDetailsUsBankAccount>,
}
impl std::fmt::Display for OutboundTransfersPaymentMethodDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct OutboundTransfersPaymentMethodDetailsUsBankAccount {
    #[serde(rename = "account_holder_type")]
    ///Account holder type: individual or company.
    pub account_holder_type: Option<String>,
    #[serde(rename = "account_type")]
    ///Account type: checkings or savings. Defaults to checking if omitted.
    pub account_type: Option<String>,
    #[serde(rename = "bank_name")]
    ///Name of the bank associated with the bank account.
    pub bank_name: Option<String>,
    #[serde(rename = "fingerprint")]
    ///Uniquely identifies this particular bank account. You can use this attribute to check whether two bank accounts are the same.
    pub fingerprint: Option<String>,
    #[serde(rename = "last4")]
    ///Last four digits of the bank account number.
    pub last4: Option<String>,
    #[serde(rename = "network")]
    ///The US bank account network used to send funds.
    pub network: String,
    #[serde(rename = "routing_number")]
    ///Routing number of the bank account.
    pub routing_number: Option<String>,
}
impl std::fmt::Display for OutboundTransfersPaymentMethodDetailsUsBankAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PackageDimensions {
    #[serde(rename = "height")]
    ///Height, in inches.
    pub height: f64,
    #[serde(rename = "length")]
    ///Length, in inches.
    pub length: f64,
    #[serde(rename = "weight")]
    ///Weight, in ounces.
    pub weight: f64,
    #[serde(rename = "width")]
    ///Width, in inches.
    pub width: f64,
}
impl std::fmt::Display for PackageDimensions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentFlowsAmountDetails {
    #[serde(rename = "tip")]
    ///
    pub tip: Option<PaymentFlowsAmountDetailsResourceTip>,
}
impl std::fmt::Display for PaymentFlowsAmountDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentFlowsAmountDetailsResourceTip {
    #[serde(rename = "amount")]
    ///Portion of the amount that corresponds to a tip.
    pub amount: Option<i64>,
}
impl std::fmt::Display for PaymentFlowsAmountDetailsResourceTip {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentFlowsAutomaticPaymentMethodsPaymentIntent {
    #[serde(rename = "enabled")]
    ///Automatically calculates compatible payment methods
    pub enabled: bool,
}
impl std::fmt::Display for PaymentFlowsAutomaticPaymentMethodsPaymentIntent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentFlowsInstallmentOptions {
    #[serde(rename = "enabled")]
    pub enabled: bool,
    #[serde(rename = "plan")]
    ///
    pub plan: Option<PaymentMethodDetailsCardInstallmentsPlan>,
}
impl std::fmt::Display for PaymentFlowsInstallmentOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentFlowsPrivatePaymentMethodsAlipay {}
impl std::fmt::Display for PaymentFlowsPrivatePaymentMethodsAlipay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentFlowsPrivatePaymentMethodsAlipayDetails {
    #[serde(rename = "buyer_id")]
    ///Uniquely identifies this particular Alipay account. You can use this attribute to check whether two Alipay accounts are the same.
    pub buyer_id: Option<String>,
    #[serde(rename = "fingerprint")]
    ///Uniquely identifies this particular Alipay account. You can use this attribute to check whether two Alipay accounts are the same.
    pub fingerprint: Option<String>,
    #[serde(rename = "transaction_id")]
    ///Transaction ID of this particular Alipay transaction.
    pub transaction_id: Option<String>,
}
impl std::fmt::Display for PaymentFlowsPrivatePaymentMethodsAlipayDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentFlowsPrivatePaymentMethodsKlarnaDob {
    #[serde(rename = "day")]
    ///The day of birth, between 1 and 31.
    pub day: Option<i64>,
    #[serde(rename = "month")]
    ///The month of birth, between 1 and 12.
    pub month: Option<i64>,
    #[serde(rename = "year")]
    ///The four-digit year of birth.
    pub year: Option<i64>,
}
impl std::fmt::Display for PaymentFlowsPrivatePaymentMethodsKlarnaDob {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentIntent {
    #[serde(rename = "amount")]
    ///Amount intended to be collected by this PaymentIntent. A positive integer representing how much to charge in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal) (e.g., 100 cents to charge $1.00 or 100 to charge ¥100, a zero-decimal currency). The minimum amount is $0.50 US or [equivalent in charge currency](https://stripe.com/docs/currencies#minimum-and-maximum-charge-amounts). The amount value supports up to eight digits (e.g., a value of 99999999 for a USD charge of $999,999.99).
    pub amount: i64,
    #[serde(rename = "amount_capturable")]
    ///Amount that can be captured from this PaymentIntent.
    pub amount_capturable: Option<i64>,
    #[serde(rename = "amount_details")]
    ///
    pub amount_details: Option<PaymentFlowsAmountDetails>,
    #[serde(rename = "amount_received")]
    ///Amount that was collected by this PaymentIntent.
    pub amount_received: Option<i64>,
    #[serde(rename = "application")]
    ///ID of the Connect application that created the PaymentIntent.
    pub application: Option<serde_json::Value>,
    #[serde(rename = "application_fee_amount")]
    ///The amount of the application fee (if any) that will be requested to be applied to the payment and transferred to the application owner's Stripe account. The amount of the application fee collected will be capped at the total payment amount. For more information, see the PaymentIntents [use case for connected accounts](https://stripe.com/docs/payments/connected-accounts).
    pub application_fee_amount: Option<i64>,
    #[serde(rename = "automatic_payment_methods")]
    ///Settings to configure compatible payment methods from the [Stripe Dashboard](https://dashboard.stripe.com/settings/payment_methods)
    pub automatic_payment_methods: Option<serde_json::Value>,
    #[serde(rename = "canceled_at")]
    ///Populated when `status` is `canceled`, this is the time at which the PaymentIntent was canceled. Measured in seconds since the Unix epoch.
    pub canceled_at: Option<i64>,
    #[serde(rename = "cancellation_reason")]
    ///Reason for cancellation of this PaymentIntent, either user-provided (`duplicate`, `fraudulent`, `requested_by_customer`, or `abandoned`) or generated by Stripe internally (`failed_invoice`, `void_invoice`, or `automatic`).
    pub cancellation_reason: Option<String>,
    #[serde(rename = "capture_method")]
    ///Controls when the funds will be captured from the customer's account.
    pub capture_method: String,
    #[serde(rename = "charges")]
    ///Charges that were created by this PaymentIntent, if any.
    pub charges: Option<serde_json::Value>,
    #[serde(rename = "client_secret")]
    /**The client secret of this PaymentIntent. Used for client-side retrieval using a publishable key.

The client secret can be used to complete a payment from your frontend. It should not be stored, logged, or exposed to anyone other than the customer. Make sure that you have TLS enabled on any page that includes the client secret.

Refer to our docs to [accept a payment](https://stripe.com/docs/payments/accept-a-payment?ui=elements) and learn about how `client_secret` should be handled.*/
    pub client_secret: Option<String>,
    #[serde(rename = "confirmation_method")]
    pub confirmation_method: String,
    #[serde(rename = "created")]
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    #[serde(rename = "currency")]
    ///Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: String,
    #[serde(rename = "customer")]
    /**ID of the Customer this PaymentIntent belongs to, if one exists.

Payment methods attached to other Customers cannot be used with this PaymentIntent.

If present in combination with [setup_future_usage](https://stripe.com/docs/api#payment_intent_object-setup_future_usage), this PaymentIntent's payment method will be attached to the Customer after the PaymentIntent has been confirmed and any required actions from the user are complete.*/
    pub customer: Option<serde_json::Value>,
    #[serde(rename = "description")]
    ///An arbitrary string attached to the object. Often useful for displaying to users.
    pub description: Option<String>,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "invoice")]
    ///ID of the invoice that created this PaymentIntent, if it exists.
    pub invoice: Option<serde_json::Value>,
    #[serde(rename = "last_payment_error")]
    ///The payment error encountered in the previous PaymentIntent confirmation. It will be cleared if the PaymentIntent is later updated for any reason.
    pub last_payment_error: Option<serde_json::Value>,
    #[serde(rename = "livemode")]
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    #[serde(rename = "metadata")]
    ///Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format. For more information, see the [documentation](https://stripe.com/docs/payments/payment-intents/creating-payment-intents#storing-information-in-metadata).
    pub metadata: Option<serde_json::Value>,
    #[serde(rename = "next_action")]
    ///If present, this property tells you what actions you need to take in order for your customer to fulfill a payment using the provided source.
    pub next_action: Option<serde_json::Value>,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "on_behalf_of")]
    ///The account (if any) for which the funds of the PaymentIntent are intended. See the PaymentIntents [use case for connected accounts](https://stripe.com/docs/payments/connected-accounts) for details.
    pub on_behalf_of: Option<serde_json::Value>,
    #[serde(rename = "payment_method")]
    ///ID of the payment method used in this PaymentIntent.
    pub payment_method: Option<serde_json::Value>,
    #[serde(rename = "payment_method_options")]
    ///Payment-method-specific configuration for this PaymentIntent.
    pub payment_method_options: Option<serde_json::Value>,
    #[serde(rename = "payment_method_types")]
    ///The list of payment method types (e.g. card) that this PaymentIntent is allowed to use.
    pub payment_method_types: Vec<String>,
    #[serde(rename = "processing")]
    ///If present, this property tells you about the processing state of the payment.
    pub processing: Option<serde_json::Value>,
    #[serde(rename = "receipt_email")]
    ///Email address that the receipt for the resulting payment will be sent to. If `receipt_email` is specified for a payment in live mode, a receipt will be sent regardless of your [email settings](https://dashboard.stripe.com/account/emails).
    pub receipt_email: Option<String>,
    #[serde(rename = "review")]
    ///ID of the review associated with this PaymentIntent, if any.
    pub review: Option<serde_json::Value>,
    #[serde(rename = "setup_future_usage")]
    /**Indicates that you intend to make future payments with this PaymentIntent's payment method.

Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete. If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.

When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).*/
    pub setup_future_usage: Option<String>,
    #[serde(rename = "shipping")]
    ///Shipping information for this PaymentIntent.
    pub shipping: Option<serde_json::Value>,
    #[serde(rename = "statement_descriptor")]
    ///For non-card charges, you can use this value as the complete description that appears on your customers’ statements. Must contain at least one letter, maximum 22 characters.
    pub statement_descriptor: Option<String>,
    #[serde(rename = "statement_descriptor_suffix")]
    ///Provides information about a card payment that customers see on their statements. Concatenated with the prefix (shortened descriptor) or statement descriptor that’s set on the account to form the complete statement descriptor. Maximum 22 characters for the concatenated descriptor.
    pub statement_descriptor_suffix: Option<String>,
    #[serde(rename = "status")]
    ///Status of this PaymentIntent, one of `requires_payment_method`, `requires_confirmation`, `requires_action`, `processing`, `requires_capture`, `canceled`, or `succeeded`. Read more about each PaymentIntent [status](https://stripe.com/docs/payments/intents#intent-statuses).
    pub status: String,
    #[serde(rename = "transfer_data")]
    ///The data with which to automatically create a Transfer when the payment is finalized. See the PaymentIntents [use case for connected accounts](https://stripe.com/docs/payments/connected-accounts) for details.
    pub transfer_data: Option<serde_json::Value>,
    #[serde(rename = "transfer_group")]
    ///A string that identifies the resulting payment as part of a group. See the PaymentIntents [use case for connected accounts](https://stripe.com/docs/payments/connected-accounts) for details.
    pub transfer_group: Option<String>,
}
impl std::fmt::Display for PaymentIntent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentIntentCardProcessing {
    #[serde(rename = "customer_notification")]
    ///
    pub customer_notification: Option<PaymentIntentProcessingCustomerNotification>,
}
impl std::fmt::Display for PaymentIntentCardProcessing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentIntentNextAction {
    #[serde(rename = "alipay_handle_redirect")]
    ///
    pub alipay_handle_redirect: Option<PaymentIntentNextActionAlipayHandleRedirect>,
    #[serde(rename = "boleto_display_details")]
    ///
    pub boleto_display_details: Option<PaymentIntentNextActionBoleto>,
    #[serde(rename = "card_await_notification")]
    ///
    pub card_await_notification: Option<PaymentIntentNextActionCardAwaitNotification>,
    #[serde(rename = "display_bank_transfer_instructions")]
    ///
    pub display_bank_transfer_instructions: Option<
        PaymentIntentNextActionDisplayBankTransferInstructions,
    >,
    #[serde(rename = "konbini_display_details")]
    ///
    pub konbini_display_details: Option<PaymentIntentNextActionKonbini>,
    #[serde(rename = "oxxo_display_details")]
    ///
    pub oxxo_display_details: Option<PaymentIntentNextActionDisplayOxxoDetails>,
    #[serde(rename = "paynow_display_qr_code")]
    ///
    pub paynow_display_qr_code: Option<PaymentIntentNextActionPaynowDisplayQrCode>,
    #[serde(rename = "promptpay_display_qr_code")]
    ///
    pub promptpay_display_qr_code: Option<PaymentIntentNextActionPromptpayDisplayQrCode>,
    #[serde(rename = "redirect_to_url")]
    ///
    pub redirect_to_url: Option<PaymentIntentNextActionRedirectToUrl>,
    #[serde(rename = "type")]
    ///Type of the next action to perform, one of `redirect_to_url`, `use_stripe_sdk`, `alipay_handle_redirect`, `oxxo_display_details`, or `verify_with_microdeposits`.
    pub type_: String,
    #[serde(rename = "use_stripe_sdk")]
    ///When confirming a PaymentIntent with Stripe.js, Stripe.js depends on the contents of this dictionary to invoke authentication flows. The shape of the contents is subject to change and is only intended to be used by Stripe.js.
    pub use_stripe_sdk: Option<serde_json::Value>,
    #[serde(rename = "verify_with_microdeposits")]
    ///
    pub verify_with_microdeposits: Option<
        PaymentIntentNextActionVerifyWithMicrodeposits,
    >,
    #[serde(rename = "wechat_pay_display_qr_code")]
    ///
    pub wechat_pay_display_qr_code: Option<
        PaymentIntentNextActionWechatPayDisplayQrCode,
    >,
    #[serde(rename = "wechat_pay_redirect_to_android_app")]
    ///
    pub wechat_pay_redirect_to_android_app: Option<
        PaymentIntentNextActionWechatPayRedirectToAndroidApp,
    >,
    #[serde(rename = "wechat_pay_redirect_to_ios_app")]
    ///
    pub wechat_pay_redirect_to_ios_app: Option<
        PaymentIntentNextActionWechatPayRedirectToIosApp,
    >,
}
impl std::fmt::Display for PaymentIntentNextAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentIntentNextActionAlipayHandleRedirect {
    #[serde(rename = "native_data")]
    ///The native data to be used with Alipay SDK you must redirect your customer to in order to authenticate the payment in an Android App.
    pub native_data: Option<String>,
    #[serde(rename = "native_url")]
    ///The native URL you must redirect your customer to in order to authenticate the payment in an iOS App.
    pub native_url: Option<String>,
    #[serde(rename = "return_url")]
    ///If the customer does not exit their browser while authenticating, they will be redirected to this specified URL after completion.
    pub return_url: Option<String>,
    #[serde(rename = "url")]
    ///The URL you must redirect your customer to in order to authenticate the payment.
    pub url: Option<String>,
}
impl std::fmt::Display for PaymentIntentNextActionAlipayHandleRedirect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentIntentNextActionBoleto {
    #[serde(rename = "expires_at")]
    ///The timestamp after which the boleto expires.
    pub expires_at: Option<i64>,
    #[serde(rename = "hosted_voucher_url")]
    ///The URL to the hosted boleto voucher page, which allows customers to view the boleto voucher.
    pub hosted_voucher_url: Option<String>,
    #[serde(rename = "number")]
    ///The boleto number.
    pub number: Option<String>,
    #[serde(rename = "pdf")]
    ///The URL to the downloadable boleto voucher PDF.
    pub pdf: Option<String>,
}
impl std::fmt::Display for PaymentIntentNextActionBoleto {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentIntentNextActionCardAwaitNotification {
    #[serde(rename = "charge_attempt_at")]
    ///The time that payment will be attempted. If customer approval is required, they need to provide approval before this time.
    pub charge_attempt_at: Option<i64>,
    #[serde(rename = "customer_approval_required")]
    ///For payments greater than INR 15000, the customer must provide explicit approval of the payment with their bank. For payments of lower amount, no customer action is required.
    pub customer_approval_required: Option<bool>,
}
impl std::fmt::Display for PaymentIntentNextActionCardAwaitNotification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentIntentNextActionDisplayBankTransferInstructions {
    #[serde(rename = "amount_remaining")]
    ///The remaining amount that needs to be transferred to complete the payment.
    pub amount_remaining: Option<i64>,
    #[serde(rename = "currency")]
    ///Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Option<String>,
    #[serde(rename = "financial_addresses")]
    ///A list of financial addresses that can be used to fund the customer balance
    pub financial_addresses: Option<
        Vec<FundingInstructionsBankTransferFinancialAddress>,
    >,
    #[serde(rename = "hosted_instructions_url")]
    ///A link to a hosted page that guides your customer through completing the transfer.
    pub hosted_instructions_url: Option<String>,
    #[serde(rename = "reference")]
    ///A string identifying this payment. Instruct your customer to include this code in the reference or memo field of their bank transfer.
    pub reference: Option<String>,
    #[serde(rename = "type")]
    ///Type of bank transfer
    pub type_: String,
}
impl std::fmt::Display for PaymentIntentNextActionDisplayBankTransferInstructions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentIntentNextActionDisplayOxxoDetails {
    #[serde(rename = "expires_after")]
    ///The timestamp after which the OXXO voucher expires.
    pub expires_after: Option<i64>,
    #[serde(rename = "hosted_voucher_url")]
    ///The URL for the hosted OXXO voucher page, which allows customers to view and print an OXXO voucher.
    pub hosted_voucher_url: Option<String>,
    #[serde(rename = "number")]
    ///OXXO reference number.
    pub number: Option<String>,
}
impl std::fmt::Display for PaymentIntentNextActionDisplayOxxoDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentIntentNextActionKonbini {
    #[serde(rename = "expires_at")]
    ///The timestamp at which the pending Konbini payment expires.
    pub expires_at: i64,
    #[serde(rename = "hosted_voucher_url")]
    ///The URL for the Konbini payment instructions page, which allows customers to view and print a Konbini voucher.
    pub hosted_voucher_url: Option<String>,
    #[serde(rename = "stores")]
    ///
    pub stores: PaymentIntentNextActionKonbiniStores,
}
impl std::fmt::Display for PaymentIntentNextActionKonbini {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentIntentNextActionKonbiniFamilymart {
    #[serde(rename = "confirmation_number")]
    ///The confirmation number.
    pub confirmation_number: Option<String>,
    #[serde(rename = "payment_code")]
    ///The payment code.
    pub payment_code: String,
}
impl std::fmt::Display for PaymentIntentNextActionKonbiniFamilymart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentIntentNextActionKonbiniLawson {
    #[serde(rename = "confirmation_number")]
    ///The confirmation number.
    pub confirmation_number: Option<String>,
    #[serde(rename = "payment_code")]
    ///The payment code.
    pub payment_code: String,
}
impl std::fmt::Display for PaymentIntentNextActionKonbiniLawson {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentIntentNextActionKonbiniMinistop {
    #[serde(rename = "confirmation_number")]
    ///The confirmation number.
    pub confirmation_number: Option<String>,
    #[serde(rename = "payment_code")]
    ///The payment code.
    pub payment_code: String,
}
impl std::fmt::Display for PaymentIntentNextActionKonbiniMinistop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentIntentNextActionKonbiniSeicomart {
    #[serde(rename = "confirmation_number")]
    ///The confirmation number.
    pub confirmation_number: Option<String>,
    #[serde(rename = "payment_code")]
    ///The payment code.
    pub payment_code: String,
}
impl std::fmt::Display for PaymentIntentNextActionKonbiniSeicomart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentIntentNextActionKonbiniStores {
    #[serde(rename = "familymart")]
    ///FamilyMart instruction details.
    pub familymart: Option<serde_json::Value>,
    #[serde(rename = "lawson")]
    ///Lawson instruction details.
    pub lawson: Option<serde_json::Value>,
    #[serde(rename = "ministop")]
    ///Ministop instruction details.
    pub ministop: Option<serde_json::Value>,
    #[serde(rename = "seicomart")]
    ///Seicomart instruction details.
    pub seicomart: Option<serde_json::Value>,
}
impl std::fmt::Display for PaymentIntentNextActionKonbiniStores {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentIntentNextActionPaynowDisplayQrCode {
    #[serde(rename = "data")]
    ///The raw data string used to generate QR code, it should be used together with QR code library.
    pub data: String,
    #[serde(rename = "image_url_png")]
    ///The image_url_png string used to render QR code
    pub image_url_png: String,
    #[serde(rename = "image_url_svg")]
    ///The image_url_svg string used to render QR code
    pub image_url_svg: String,
}
impl std::fmt::Display for PaymentIntentNextActionPaynowDisplayQrCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentIntentNextActionPromptpayDisplayQrCode {
    #[serde(rename = "data")]
    ///The raw data string used to generate QR code, it should be used together with QR code library.
    pub data: String,
    #[serde(rename = "hosted_instructions_url")]
    ///The URL to the hosted PromptPay instructions page, which allows customers to view the PromptPay QR code.
    pub hosted_instructions_url: String,
    #[serde(rename = "image_url_png")]
    ///The image_url_png string used to render QR code, can be used as <img src="…" />
    pub image_url_png: String,
    #[serde(rename = "image_url_svg")]
    ///The image_url_svg string used to render QR code, can be used as <img src="…" />
    pub image_url_svg: String,
}
impl std::fmt::Display for PaymentIntentNextActionPromptpayDisplayQrCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentIntentNextActionRedirectToUrl {
    #[serde(rename = "return_url")]
    ///If the customer does not exit their browser while authenticating, they will be redirected to this specified URL after completion.
    pub return_url: Option<String>,
    #[serde(rename = "url")]
    ///The URL you must redirect your customer to in order to authenticate the payment.
    pub url: Option<String>,
}
impl std::fmt::Display for PaymentIntentNextActionRedirectToUrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentIntentNextActionVerifyWithMicrodeposits {
    #[serde(rename = "arrival_date")]
    ///The timestamp when the microdeposits are expected to land.
    pub arrival_date: i64,
    #[serde(rename = "hosted_verification_url")]
    ///The URL for the hosted verification page, which allows customers to verify their bank account.
    pub hosted_verification_url: String,
    #[serde(rename = "microdeposit_type")]
    ///The type of the microdeposit sent to the customer. Used to distinguish between different verification methods.
    pub microdeposit_type: Option<String>,
}
impl std::fmt::Display for PaymentIntentNextActionVerifyWithMicrodeposits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentIntentNextActionWechatPayDisplayQrCode {
    #[serde(rename = "data")]
    ///The data being used to generate QR code
    pub data: String,
    #[serde(rename = "image_data_url")]
    ///The base64 image data for a pre-generated QR code
    pub image_data_url: String,
    #[serde(rename = "image_url_png")]
    ///The image_url_png string used to render QR code
    pub image_url_png: String,
    #[serde(rename = "image_url_svg")]
    ///The image_url_svg string used to render QR code
    pub image_url_svg: String,
}
impl std::fmt::Display for PaymentIntentNextActionWechatPayDisplayQrCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentIntentNextActionWechatPayRedirectToAndroidApp {
    #[serde(rename = "app_id")]
    ///app_id is the APP ID registered on WeChat open platform
    pub app_id: String,
    #[serde(rename = "nonce_str")]
    ///nonce_str is a random string
    pub nonce_str: String,
    #[serde(rename = "package")]
    ///package is static value
    pub package: String,
    #[serde(rename = "partner_id")]
    ///an unique merchant ID assigned by WeChat Pay
    pub partner_id: String,
    #[serde(rename = "prepay_id")]
    ///an unique trading ID assigned by WeChat Pay
    pub prepay_id: String,
    #[serde(rename = "sign")]
    ///A signature
    pub sign: String,
    #[serde(rename = "timestamp")]
    ///Specifies the current time in epoch format
    pub timestamp: String,
}
impl std::fmt::Display for PaymentIntentNextActionWechatPayRedirectToAndroidApp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentIntentNextActionWechatPayRedirectToIosApp {
    #[serde(rename = "native_url")]
    ///An universal link that redirect to WeChat Pay app
    pub native_url: String,
}
impl std::fmt::Display for PaymentIntentNextActionWechatPayRedirectToIosApp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentIntentPaymentMethodOptions {
    #[serde(rename = "acss_debit")]
    pub acss_debit: Option<serde_json::Value>,
    #[serde(rename = "affirm")]
    pub affirm: Option<serde_json::Value>,
    #[serde(rename = "afterpay_clearpay")]
    pub afterpay_clearpay: Option<serde_json::Value>,
    #[serde(rename = "alipay")]
    pub alipay: Option<serde_json::Value>,
    #[serde(rename = "au_becs_debit")]
    pub au_becs_debit: Option<serde_json::Value>,
    #[serde(rename = "bacs_debit")]
    pub bacs_debit: Option<serde_json::Value>,
    #[serde(rename = "bancontact")]
    pub bancontact: Option<serde_json::Value>,
    #[serde(rename = "blik")]
    pub blik: Option<serde_json::Value>,
    #[serde(rename = "boleto")]
    pub boleto: Option<serde_json::Value>,
    #[serde(rename = "card")]
    pub card: Option<serde_json::Value>,
    #[serde(rename = "card_present")]
    pub card_present: Option<serde_json::Value>,
    #[serde(rename = "customer_balance")]
    pub customer_balance: Option<serde_json::Value>,
    #[serde(rename = "eps")]
    pub eps: Option<serde_json::Value>,
    #[serde(rename = "fpx")]
    pub fpx: Option<serde_json::Value>,
    #[serde(rename = "giropay")]
    pub giropay: Option<serde_json::Value>,
    #[serde(rename = "grabpay")]
    pub grabpay: Option<serde_json::Value>,
    #[serde(rename = "ideal")]
    pub ideal: Option<serde_json::Value>,
    #[serde(rename = "interac_present")]
    pub interac_present: Option<serde_json::Value>,
    #[serde(rename = "klarna")]
    pub klarna: Option<serde_json::Value>,
    #[serde(rename = "konbini")]
    pub konbini: Option<serde_json::Value>,
    #[serde(rename = "link")]
    pub link: Option<serde_json::Value>,
    #[serde(rename = "oxxo")]
    pub oxxo: Option<serde_json::Value>,
    #[serde(rename = "p24")]
    pub p24: Option<serde_json::Value>,
    #[serde(rename = "paynow")]
    pub paynow: Option<serde_json::Value>,
    #[serde(rename = "promptpay")]
    pub promptpay: Option<serde_json::Value>,
    #[serde(rename = "sepa_debit")]
    pub sepa_debit: Option<serde_json::Value>,
    #[serde(rename = "sofort")]
    pub sofort: Option<serde_json::Value>,
    #[serde(rename = "us_bank_account")]
    pub us_bank_account: Option<serde_json::Value>,
    #[serde(rename = "wechat_pay")]
    pub wechat_pay: Option<serde_json::Value>,
}
impl std::fmt::Display for PaymentIntentPaymentMethodOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentIntentPaymentMethodOptionsAcssDebit {
    #[serde(rename = "mandate_options")]
    ///
    pub mandate_options: Option<
        PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebit,
    >,
    #[serde(rename = "setup_future_usage")]
    /**Indicates that you intend to make future payments with this PaymentIntent's payment method.

Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete. If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.

When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).*/
    pub setup_future_usage: Option<String>,
    #[serde(rename = "verification_method")]
    ///Bank account verification method.
    pub verification_method: Option<String>,
}
impl std::fmt::Display for PaymentIntentPaymentMethodOptionsAcssDebit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentIntentPaymentMethodOptionsAuBecsDebit {
    #[serde(rename = "setup_future_usage")]
    /**Indicates that you intend to make future payments with this PaymentIntent's payment method.

Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete. If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.

When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).*/
    pub setup_future_usage: Option<String>,
}
impl std::fmt::Display for PaymentIntentPaymentMethodOptionsAuBecsDebit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentIntentPaymentMethodOptionsBlik {}
impl std::fmt::Display for PaymentIntentPaymentMethodOptionsBlik {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentIntentPaymentMethodOptionsCard {
    #[serde(rename = "capture_method")]
    ///Controls when the funds will be captured from the customer's account.
    pub capture_method: Option<String>,
    #[serde(rename = "installments")]
    /**Installment details for this payment (Mexico only).

For more information, see the [installments integration guide](https://stripe.com/docs/payments/installments).*/
    pub installments: Option<serde_json::Value>,
    #[serde(rename = "mandate_options")]
    ///Configuration options for setting up an eMandate for cards issued in India.
    pub mandate_options: Option<serde_json::Value>,
    #[serde(rename = "network")]
    ///Selected network to process this payment intent on. Depends on the available networks of the card attached to the payment intent. Can be only set confirm-time.
    pub network: Option<String>,
    #[serde(rename = "request_three_d_secure")]
    ///We strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based on risk level and [other requirements](https://stripe.com/docs/strong-customer-authentication). However, if you wish to request 3D Secure based on logic from your own fraud engine, provide this option. Permitted values include: `automatic` or `any`. If not provided, defaults to `automatic`. Read our guide on [manually requesting 3D Secure](https://stripe.com/docs/payments/3d-secure#manual-three-ds) for more information on how this configuration interacts with Radar and our SCA Engine.
    pub request_three_d_secure: Option<String>,
    #[serde(rename = "setup_future_usage")]
    /**Indicates that you intend to make future payments with this PaymentIntent's payment method.

Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete. If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.

When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).*/
    pub setup_future_usage: Option<String>,
    #[serde(rename = "statement_descriptor_suffix_kana")]
    ///Provides information about a card payment that customers see on their statements. Concatenated with the Kana prefix (shortened Kana descriptor) or Kana statement descriptor that’s set on the account to form the complete statement descriptor. Maximum 22 characters. On card statements, the *concatenation* of both prefix and suffix (including separators) will appear truncated to 22 characters.
    pub statement_descriptor_suffix_kana: Option<String>,
    #[serde(rename = "statement_descriptor_suffix_kanji")]
    ///Provides information about a card payment that customers see on their statements. Concatenated with the Kanji prefix (shortened Kanji descriptor) or Kanji statement descriptor that’s set on the account to form the complete statement descriptor. Maximum 17 characters. On card statements, the *concatenation* of both prefix and suffix (including separators) will appear truncated to 17 characters.
    pub statement_descriptor_suffix_kanji: Option<String>,
}
impl std::fmt::Display for PaymentIntentPaymentMethodOptionsCard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentIntentPaymentMethodOptionsEps {
    #[serde(rename = "setup_future_usage")]
    /**Indicates that you intend to make future payments with this PaymentIntent's payment method.

Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete. If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.

When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).*/
    pub setup_future_usage: Option<String>,
}
impl std::fmt::Display for PaymentIntentPaymentMethodOptionsEps {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentIntentPaymentMethodOptionsLink {
    #[serde(rename = "capture_method")]
    ///Controls when the funds will be captured from the customer's account.
    pub capture_method: Option<String>,
    #[serde(rename = "persistent_token")]
    ///Token used for persistent Link logins.
    pub persistent_token: Option<String>,
    #[serde(rename = "setup_future_usage")]
    /**Indicates that you intend to make future payments with this PaymentIntent's payment method.

Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete. If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.

When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).*/
    pub setup_future_usage: Option<String>,
}
impl std::fmt::Display for PaymentIntentPaymentMethodOptionsLink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebit {
    #[serde(rename = "custom_mandate_url")]
    ///A URL for custom mandate text
    pub custom_mandate_url: Option<String>,
    #[serde(rename = "interval_description")]
    ///Description of the interval. Only required if the 'payment_schedule' parameter is 'interval' or 'combined'.
    pub interval_description: Option<String>,
    #[serde(rename = "payment_schedule")]
    ///Payment schedule for the mandate.
    pub payment_schedule: Option<String>,
    #[serde(rename = "transaction_type")]
    ///Transaction type of the mandate.
    pub transaction_type: Option<String>,
}
impl std::fmt::Display for PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentIntentPaymentMethodOptionsMandateOptionsSepaDebit {}
impl std::fmt::Display for PaymentIntentPaymentMethodOptionsMandateOptionsSepaDebit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentIntentPaymentMethodOptionsSepaDebit {
    #[serde(rename = "mandate_options")]
    ///
    pub mandate_options: Option<
        PaymentIntentPaymentMethodOptionsMandateOptionsSepaDebit,
    >,
    #[serde(rename = "setup_future_usage")]
    /**Indicates that you intend to make future payments with this PaymentIntent's payment method.

Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete. If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.

When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).*/
    pub setup_future_usage: Option<String>,
}
impl std::fmt::Display for PaymentIntentPaymentMethodOptionsSepaDebit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentIntentPaymentMethodOptionsUsBankAccount {
    #[serde(rename = "financial_connections")]
    ///
    pub financial_connections: Option<LinkedAccountOptionsUsBankAccount>,
    #[serde(rename = "setup_future_usage")]
    /**Indicates that you intend to make future payments with this PaymentIntent's payment method.

Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete. If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.

When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).*/
    pub setup_future_usage: Option<String>,
    #[serde(rename = "verification_method")]
    ///Bank account verification method.
    pub verification_method: Option<String>,
}
impl std::fmt::Display for PaymentIntentPaymentMethodOptionsUsBankAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentIntentProcessing {
    #[serde(rename = "card")]
    ///
    pub card: Option<PaymentIntentCardProcessing>,
    #[serde(rename = "type")]
    ///Type of the payment method for which payment is in `processing` state, one of `card`.
    pub type_: String,
}
impl std::fmt::Display for PaymentIntentProcessing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentIntentProcessingCustomerNotification {
    #[serde(rename = "approval_requested")]
    ///Whether customer approval has been requested for this payment. For payments greater than INR 15000 or mandate amount, the customer must provide explicit approval of the payment with their bank.
    pub approval_requested: Option<bool>,
    #[serde(rename = "completes_at")]
    ///If customer approval is required, they need to provide approval before this time.
    pub completes_at: Option<i64>,
}
impl std::fmt::Display for PaymentIntentProcessingCustomerNotification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentIntentTypeSpecificPaymentMethodOptionsClient {
    #[serde(rename = "capture_method")]
    ///Controls when the funds will be captured from the customer's account.
    pub capture_method: Option<String>,
    #[serde(rename = "installments")]
    ///
    pub installments: Option<PaymentFlowsInstallmentOptions>,
    #[serde(rename = "setup_future_usage")]
    /**Indicates that you intend to make future payments with this PaymentIntent's payment method.

Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete. If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.

When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).*/
    pub setup_future_usage: Option<String>,
    #[serde(rename = "verification_method")]
    ///Bank account verification method.
    pub verification_method: Option<String>,
}
impl std::fmt::Display for PaymentIntentTypeSpecificPaymentMethodOptionsClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentLink {
    #[serde(rename = "active")]
    ///Whether the payment link's `url` is active. If `false`, customers visiting the URL will be shown a page saying that the link has been deactivated.
    pub active: bool,
    #[serde(rename = "after_completion")]
    ///
    pub after_completion: PaymentLinksResourceAfterCompletion,
    #[serde(rename = "allow_promotion_codes")]
    ///Whether user redeemable promotion codes are enabled.
    pub allow_promotion_codes: bool,
    #[serde(rename = "application_fee_amount")]
    ///The amount of the application fee (if any) that will be requested to be applied to the payment and transferred to the application owner's Stripe account.
    pub application_fee_amount: Option<i64>,
    #[serde(rename = "application_fee_percent")]
    ///This represents the percentage of the subscription invoice subtotal that will be transferred to the application owner's Stripe account.
    pub application_fee_percent: Option<f64>,
    #[serde(rename = "automatic_tax")]
    ///
    pub automatic_tax: PaymentLinksResourceAutomaticTax,
    #[serde(rename = "billing_address_collection")]
    ///Configuration for collecting the customer's billing address.
    pub billing_address_collection: String,
    #[serde(rename = "consent_collection")]
    ///When set, provides configuration to gather active consent from customers.
    pub consent_collection: Option<serde_json::Value>,
    #[serde(rename = "customer_creation")]
    ///Configuration for Customer creation during checkout.
    pub customer_creation: String,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "line_items")]
    ///The line items representing what is being sold.
    pub line_items: Option<serde_json::Value>,
    #[serde(rename = "livemode")]
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    #[serde(rename = "metadata")]
    ///Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format.
    pub metadata: serde_json::Value,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "on_behalf_of")]
    ///The account on behalf of which to charge. See the [Connect documentation](https://support.stripe.com/questions/sending-invoices-on-behalf-of-connected-accounts) for details.
    pub on_behalf_of: Option<serde_json::Value>,
    #[serde(rename = "payment_intent_data")]
    ///Indicates the parameters to be passed to PaymentIntent creation during checkout.
    pub payment_intent_data: Option<serde_json::Value>,
    #[serde(rename = "payment_method_collection")]
    ///Configuration for collecting a payment method during checkout.
    pub payment_method_collection: String,
    #[serde(rename = "payment_method_types")]
    ///The list of payment method types that customers can use. When `null`, Stripe will dynamically show relevant payment methods you've enabled in your [payment method settings](https://dashboard.stripe.com/settings/payment_methods).
    pub payment_method_types: Option<Vec<String>>,
    #[serde(rename = "phone_number_collection")]
    ///
    pub phone_number_collection: PaymentLinksResourcePhoneNumberCollection,
    #[serde(rename = "shipping_address_collection")]
    ///Configuration for collecting the customer's shipping address.
    pub shipping_address_collection: Option<serde_json::Value>,
    #[serde(rename = "shipping_options")]
    ///The shipping rate options applied to the session.
    pub shipping_options: Vec<PaymentLinksResourceShippingOption>,
    #[serde(rename = "submit_type")]
    ///Indicates the type of transaction being performed which customizes relevant text on the page, such as the submit button.
    pub submit_type: String,
    #[serde(rename = "subscription_data")]
    ///When creating a subscription, the specified configuration data will be used. There must be at least one line item with a recurring price to use `subscription_data`.
    pub subscription_data: Option<serde_json::Value>,
    #[serde(rename = "tax_id_collection")]
    ///
    pub tax_id_collection: PaymentLinksResourceTaxIdCollection,
    #[serde(rename = "transfer_data")]
    ///The account (if any) the payments will be attributed to for tax reporting, and where funds from each payment will be transferred to.
    pub transfer_data: Option<serde_json::Value>,
    #[serde(rename = "url")]
    ///The public URL that can be shared with customers.
    pub url: String,
}
impl std::fmt::Display for PaymentLink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentLinksResourceAfterCompletion {
    #[serde(rename = "hosted_confirmation")]
    ///
    pub hosted_confirmation: Option<
        PaymentLinksResourceCompletionBehaviorConfirmationPage,
    >,
    #[serde(rename = "redirect")]
    ///
    pub redirect: Option<PaymentLinksResourceCompletionBehaviorRedirect>,
    #[serde(rename = "type")]
    ///The specified behavior after the purchase is complete.
    pub type_: String,
}
impl std::fmt::Display for PaymentLinksResourceAfterCompletion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentLinksResourceAutomaticTax {
    #[serde(rename = "enabled")]
    ///If `true`, tax will be calculated automatically using the customer's location.
    pub enabled: bool,
}
impl std::fmt::Display for PaymentLinksResourceAutomaticTax {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentLinksResourceCompletionBehaviorConfirmationPage {
    #[serde(rename = "custom_message")]
    ///The custom message that is displayed to the customer after the purchase is complete.
    pub custom_message: Option<String>,
}
impl std::fmt::Display for PaymentLinksResourceCompletionBehaviorConfirmationPage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentLinksResourceCompletionBehaviorRedirect {
    #[serde(rename = "url")]
    ///The URL the customer will be redirected to after the purchase is complete.
    pub url: String,
}
impl std::fmt::Display for PaymentLinksResourceCompletionBehaviorRedirect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentLinksResourceConsentCollection {
    #[serde(rename = "promotions")]
    ///If set to `auto`, enables the collection of customer consent for promotional communications.
    pub promotions: Option<String>,
}
impl std::fmt::Display for PaymentLinksResourceConsentCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentLinksResourcePaymentIntentData {
    #[serde(rename = "capture_method")]
    ///Indicates when the funds will be captured from the customer's account.
    pub capture_method: Option<String>,
    #[serde(rename = "setup_future_usage")]
    ///Indicates that you intend to make future payments with the payment method collected during checkout.
    pub setup_future_usage: Option<String>,
}
impl std::fmt::Display for PaymentLinksResourcePaymentIntentData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentLinksResourcePhoneNumberCollection {
    #[serde(rename = "enabled")]
    ///If `true`, a phone number will be collected during checkout.
    pub enabled: bool,
}
impl std::fmt::Display for PaymentLinksResourcePhoneNumberCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentLinksResourceShippingAddressCollection {
    #[serde(rename = "allowed_countries")]
    ///An array of two-letter ISO country codes representing which countries Checkout should provide as options for shipping locations. Unsupported country codes: `AS, CX, CC, CU, HM, IR, KP, MH, FM, NF, MP, PW, SD, SY, UM, VI`.
    pub allowed_countries: Vec<String>,
}
impl std::fmt::Display for PaymentLinksResourceShippingAddressCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentLinksResourceShippingOption {
    #[serde(rename = "shipping_amount")]
    ///A non-negative integer in cents representing how much to charge.
    pub shipping_amount: i64,
    #[serde(rename = "shipping_rate")]
    ///The ID of the Shipping Rate to use for this shipping option.
    pub shipping_rate: serde_json::Value,
}
impl std::fmt::Display for PaymentLinksResourceShippingOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentLinksResourceSubscriptionData {
    #[serde(rename = "trial_period_days")]
    ///Integer representing the number of trial period days before the customer is charged for the first time.
    pub trial_period_days: Option<i64>,
}
impl std::fmt::Display for PaymentLinksResourceSubscriptionData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentLinksResourceTaxIdCollection {
    #[serde(rename = "enabled")]
    ///Indicates whether tax ID collection is enabled for the session.
    pub enabled: bool,
}
impl std::fmt::Display for PaymentLinksResourceTaxIdCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentLinksResourceTransferData {
    #[serde(rename = "amount")]
    ///The amount in %s that will be transferred to the destination account. By default, the entire amount is transferred to the destination.
    pub amount: Option<i64>,
    #[serde(rename = "destination")]
    ///The connected account receiving the transfer.
    pub destination: serde_json::Value,
}
impl std::fmt::Display for PaymentLinksResourceTransferData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethod {
    #[serde(rename = "acss_debit")]
    ///
    pub acss_debit: Option<PaymentMethodAcssDebit>,
    #[serde(rename = "affirm")]
    ///
    pub affirm: Option<PaymentMethodAffirm>,
    #[serde(rename = "afterpay_clearpay")]
    ///
    pub afterpay_clearpay: Option<PaymentMethodAfterpayClearpay>,
    #[serde(rename = "alipay")]
    ///
    pub alipay: Option<PaymentFlowsPrivatePaymentMethodsAlipay>,
    #[serde(rename = "au_becs_debit")]
    ///
    pub au_becs_debit: Option<PaymentMethodAuBecsDebit>,
    #[serde(rename = "bacs_debit")]
    ///
    pub bacs_debit: Option<PaymentMethodBacsDebit>,
    #[serde(rename = "bancontact")]
    ///
    pub bancontact: Option<PaymentMethodBancontact>,
    #[serde(rename = "billing_details")]
    ///
    pub billing_details: BillingDetails,
    #[serde(rename = "blik")]
    ///
    pub blik: Option<PaymentMethodBlik>,
    #[serde(rename = "boleto")]
    ///
    pub boleto: Option<PaymentMethodBoleto>,
    #[serde(rename = "card")]
    ///
    pub card: Option<PaymentMethodCard>,
    #[serde(rename = "card_present")]
    ///
    pub card_present: Option<PaymentMethodCardPresent>,
    #[serde(rename = "created")]
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    #[serde(rename = "customer")]
    ///The ID of the Customer to which this PaymentMethod is saved. This will not be set when the PaymentMethod has not been saved to a Customer.
    pub customer: Option<serde_json::Value>,
    #[serde(rename = "customer_balance")]
    ///
    pub customer_balance: Option<PaymentMethodCustomerBalance>,
    #[serde(rename = "eps")]
    ///
    pub eps: Option<PaymentMethodEps>,
    #[serde(rename = "fpx")]
    ///
    pub fpx: Option<PaymentMethodFpx>,
    #[serde(rename = "giropay")]
    ///
    pub giropay: Option<PaymentMethodGiropay>,
    #[serde(rename = "grabpay")]
    ///
    pub grabpay: Option<PaymentMethodGrabpay>,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "ideal")]
    ///
    pub ideal: Option<PaymentMethodIdeal>,
    #[serde(rename = "interac_present")]
    ///
    pub interac_present: Option<PaymentMethodInteracPresent>,
    #[serde(rename = "klarna")]
    ///
    pub klarna: Option<PaymentMethodKlarna>,
    #[serde(rename = "konbini")]
    ///
    pub konbini: Option<PaymentMethodKonbini>,
    #[serde(rename = "link")]
    ///
    pub link: Option<PaymentMethodLink>,
    #[serde(rename = "livemode")]
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    #[serde(rename = "metadata")]
    ///Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<serde_json::Value>,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "oxxo")]
    ///
    pub oxxo: Option<PaymentMethodOxxo>,
    #[serde(rename = "p24")]
    ///
    pub p24: Option<PaymentMethodP24>,
    #[serde(rename = "paynow")]
    ///
    pub paynow: Option<PaymentMethodPaynow>,
    #[serde(rename = "promptpay")]
    ///
    pub promptpay: Option<PaymentMethodPromptpay>,
    #[serde(rename = "radar_options")]
    ///Options to configure Radar. See [Radar Session](https://stripe.com/docs/radar/radar-session) for more information.
    pub radar_options: Option<RadarRadarOptions>,
    #[serde(rename = "sepa_debit")]
    ///
    pub sepa_debit: Option<PaymentMethodSepaDebit>,
    #[serde(rename = "sofort")]
    ///
    pub sofort: Option<PaymentMethodSofort>,
    #[serde(rename = "type")]
    ///The type of the PaymentMethod. An additional hash is included on the PaymentMethod with a name matching this value. It contains additional information specific to the PaymentMethod type.
    pub type_: String,
    #[serde(rename = "us_bank_account")]
    ///
    pub us_bank_account: Option<PaymentMethodUsBankAccount>,
    #[serde(rename = "wechat_pay")]
    ///
    pub wechat_pay: Option<PaymentMethodWechatPay>,
}
impl std::fmt::Display for PaymentMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodAcssDebit {
    #[serde(rename = "bank_name")]
    ///Name of the bank associated with the bank account.
    pub bank_name: Option<String>,
    #[serde(rename = "fingerprint")]
    ///Uniquely identifies this particular bank account. You can use this attribute to check whether two bank accounts are the same.
    pub fingerprint: Option<String>,
    #[serde(rename = "institution_number")]
    ///Institution number of the bank account.
    pub institution_number: Option<String>,
    #[serde(rename = "last4")]
    ///Last four digits of the bank account number.
    pub last4: Option<String>,
    #[serde(rename = "transit_number")]
    ///Transit number of the bank account.
    pub transit_number: Option<String>,
}
impl std::fmt::Display for PaymentMethodAcssDebit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodAffirm {}
impl std::fmt::Display for PaymentMethodAffirm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodAfterpayClearpay {}
impl std::fmt::Display for PaymentMethodAfterpayClearpay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodAuBecsDebit {
    #[serde(rename = "bsb_number")]
    ///Six-digit number identifying bank and branch associated with this bank account.
    pub bsb_number: Option<String>,
    #[serde(rename = "fingerprint")]
    ///Uniquely identifies this particular bank account. You can use this attribute to check whether two bank accounts are the same.
    pub fingerprint: Option<String>,
    #[serde(rename = "last4")]
    ///Last four digits of the bank account number.
    pub last4: Option<String>,
}
impl std::fmt::Display for PaymentMethodAuBecsDebit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodBacsDebit {
    #[serde(rename = "fingerprint")]
    ///Uniquely identifies this particular bank account. You can use this attribute to check whether two bank accounts are the same.
    pub fingerprint: Option<String>,
    #[serde(rename = "last4")]
    ///Last four digits of the bank account number.
    pub last4: Option<String>,
    #[serde(rename = "sort_code")]
    ///Sort code of the bank account. (e.g., `10-20-30`)
    pub sort_code: Option<String>,
}
impl std::fmt::Display for PaymentMethodBacsDebit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodBancontact {}
impl std::fmt::Display for PaymentMethodBancontact {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodBlik {}
impl std::fmt::Display for PaymentMethodBlik {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodBoleto {
    #[serde(rename = "tax_id")]
    ///Uniquely identifies the customer tax id (CNPJ or CPF)
    pub tax_id: String,
}
impl std::fmt::Display for PaymentMethodBoleto {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodCard {
    #[serde(rename = "brand")]
    ///Card brand. Can be `amex`, `diners`, `discover`, `jcb`, `mastercard`, `unionpay`, `visa`, or `unknown`.
    pub brand: String,
    #[serde(rename = "checks")]
    ///Checks on Card address and CVC if provided.
    pub checks: Option<serde_json::Value>,
    #[serde(rename = "country")]
    ///Two-letter ISO code representing the country of the card. You could use this attribute to get a sense of the international breakdown of cards you've collected.
    pub country: Option<String>,
    #[serde(rename = "exp_month")]
    ///Two-digit number representing the card's expiration month.
    pub exp_month: i64,
    #[serde(rename = "exp_year")]
    ///Four-digit number representing the card's expiration year.
    pub exp_year: i64,
    #[serde(rename = "fingerprint")]
    /**Uniquely identifies this particular card number. You can use this attribute to check whether two customers who’ve signed up with you are using the same card number, for example. For payment methods that tokenize card information (Apple Pay, Google Pay), the tokenized number might be provided instead of the underlying card number.

*Starting May 1, 2021, card fingerprint in India for Connect will change to allow two fingerprints for the same card --- one for India and one for the rest of the world.**/
    pub fingerprint: Option<String>,
    #[serde(rename = "funding")]
    ///Card funding type. Can be `credit`, `debit`, `prepaid`, or `unknown`.
    pub funding: String,
    #[serde(rename = "generated_from")]
    ///Details of the original PaymentMethod that created this object.
    pub generated_from: Option<serde_json::Value>,
    #[serde(rename = "last4")]
    ///The last four digits of the card.
    pub last4: String,
    #[serde(rename = "networks")]
    ///Contains information about card networks that can be used to process the payment.
    pub networks: Option<serde_json::Value>,
    #[serde(rename = "three_d_secure_usage")]
    ///Contains details on how this Card maybe be used for 3D Secure authentication.
    pub three_d_secure_usage: Option<serde_json::Value>,
    #[serde(rename = "wallet")]
    ///If this Card is part of a card wallet, this contains the details of the card wallet.
    pub wallet: Option<serde_json::Value>,
}
impl std::fmt::Display for PaymentMethodCard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodCardChecks {
    #[serde(rename = "address_line1_check")]
    ///If a address line1 was provided, results of the check, one of `pass`, `fail`, `unavailable`, or `unchecked`.
    pub address_line1_check: Option<String>,
    #[serde(rename = "address_postal_code_check")]
    ///If a address postal code was provided, results of the check, one of `pass`, `fail`, `unavailable`, or `unchecked`.
    pub address_postal_code_check: Option<String>,
    #[serde(rename = "cvc_check")]
    ///If a CVC was provided, results of the check, one of `pass`, `fail`, `unavailable`, or `unchecked`.
    pub cvc_check: Option<String>,
}
impl std::fmt::Display for PaymentMethodCardChecks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodCardGeneratedCard {
    #[serde(rename = "charge")]
    ///The charge that created this object.
    pub charge: Option<String>,
    #[serde(rename = "payment_method_details")]
    ///Transaction-specific details of the payment method used in the payment.
    pub payment_method_details: Option<serde_json::Value>,
    #[serde(rename = "setup_attempt")]
    ///The ID of the SetupAttempt that generated this PaymentMethod, if any.
    pub setup_attempt: Option<serde_json::Value>,
}
impl std::fmt::Display for PaymentMethodCardGeneratedCard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodCardPresent {}
impl std::fmt::Display for PaymentMethodCardPresent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodCardWallet {
    #[serde(rename = "amex_express_checkout")]
    ///
    pub amex_express_checkout: Option<PaymentMethodCardWalletAmexExpressCheckout>,
    #[serde(rename = "apple_pay")]
    ///
    pub apple_pay: Option<PaymentMethodCardWalletApplePay>,
    #[serde(rename = "dynamic_last4")]
    ///(For tokenized numbers only.) The last four digits of the device account number.
    pub dynamic_last4: Option<String>,
    #[serde(rename = "google_pay")]
    ///
    pub google_pay: Option<PaymentMethodCardWalletGooglePay>,
    #[serde(rename = "masterpass")]
    ///
    pub masterpass: Option<PaymentMethodCardWalletMasterpass>,
    #[serde(rename = "samsung_pay")]
    ///
    pub samsung_pay: Option<PaymentMethodCardWalletSamsungPay>,
    #[serde(rename = "type")]
    ///The type of the card wallet, one of `amex_express_checkout`, `apple_pay`, `google_pay`, `masterpass`, `samsung_pay`, or `visa_checkout`. An additional hash is included on the Wallet subhash with a name matching this value. It contains additional information specific to the card wallet type.
    pub type_: String,
    #[serde(rename = "visa_checkout")]
    ///
    pub visa_checkout: Option<PaymentMethodCardWalletVisaCheckout>,
}
impl std::fmt::Display for PaymentMethodCardWallet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodCardWalletAmexExpressCheckout {}
impl std::fmt::Display for PaymentMethodCardWalletAmexExpressCheckout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodCardWalletApplePay {}
impl std::fmt::Display for PaymentMethodCardWalletApplePay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodCardWalletGooglePay {}
impl std::fmt::Display for PaymentMethodCardWalletGooglePay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodCardWalletMasterpass {
    #[serde(rename = "billing_address")]
    ///Owner's verified billing address. Values are verified or provided by the wallet directly (if supported) at the time of authorization or settlement. They cannot be set or mutated.
    pub billing_address: Option<serde_json::Value>,
    #[serde(rename = "email")]
    ///Owner's verified email. Values are verified or provided by the wallet directly (if supported) at the time of authorization or settlement. They cannot be set or mutated.
    pub email: Option<String>,
    #[serde(rename = "name")]
    ///Owner's verified full name. Values are verified or provided by the wallet directly (if supported) at the time of authorization or settlement. They cannot be set or mutated.
    pub name: Option<String>,
    #[serde(rename = "shipping_address")]
    ///Owner's verified shipping address. Values are verified or provided by the wallet directly (if supported) at the time of authorization or settlement. They cannot be set or mutated.
    pub shipping_address: Option<serde_json::Value>,
}
impl std::fmt::Display for PaymentMethodCardWalletMasterpass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodCardWalletSamsungPay {}
impl std::fmt::Display for PaymentMethodCardWalletSamsungPay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodCardWalletVisaCheckout {
    #[serde(rename = "billing_address")]
    ///Owner's verified billing address. Values are verified or provided by the wallet directly (if supported) at the time of authorization or settlement. They cannot be set or mutated.
    pub billing_address: Option<serde_json::Value>,
    #[serde(rename = "email")]
    ///Owner's verified email. Values are verified or provided by the wallet directly (if supported) at the time of authorization or settlement. They cannot be set or mutated.
    pub email: Option<String>,
    #[serde(rename = "name")]
    ///Owner's verified full name. Values are verified or provided by the wallet directly (if supported) at the time of authorization or settlement. They cannot be set or mutated.
    pub name: Option<String>,
    #[serde(rename = "shipping_address")]
    ///Owner's verified shipping address. Values are verified or provided by the wallet directly (if supported) at the time of authorization or settlement. They cannot be set or mutated.
    pub shipping_address: Option<serde_json::Value>,
}
impl std::fmt::Display for PaymentMethodCardWalletVisaCheckout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodCustomerBalance {}
impl std::fmt::Display for PaymentMethodCustomerBalance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodDetails {
    #[serde(rename = "ach_credit_transfer")]
    ///
    pub ach_credit_transfer: Option<PaymentMethodDetailsAchCreditTransfer>,
    #[serde(rename = "ach_debit")]
    ///
    pub ach_debit: Option<PaymentMethodDetailsAchDebit>,
    #[serde(rename = "acss_debit")]
    ///
    pub acss_debit: Option<PaymentMethodDetailsAcssDebit>,
    #[serde(rename = "affirm")]
    ///
    pub affirm: Option<PaymentMethodDetailsAffirm>,
    #[serde(rename = "afterpay_clearpay")]
    ///
    pub afterpay_clearpay: Option<PaymentMethodDetailsAfterpayClearpay>,
    #[serde(rename = "alipay")]
    ///
    pub alipay: Option<PaymentFlowsPrivatePaymentMethodsAlipayDetails>,
    #[serde(rename = "au_becs_debit")]
    ///
    pub au_becs_debit: Option<PaymentMethodDetailsAuBecsDebit>,
    #[serde(rename = "bacs_debit")]
    ///
    pub bacs_debit: Option<PaymentMethodDetailsBacsDebit>,
    #[serde(rename = "bancontact")]
    ///
    pub bancontact: Option<PaymentMethodDetailsBancontact>,
    #[serde(rename = "blik")]
    ///
    pub blik: Option<PaymentMethodDetailsBlik>,
    #[serde(rename = "boleto")]
    ///
    pub boleto: Option<PaymentMethodDetailsBoleto>,
    #[serde(rename = "card")]
    ///
    pub card: Option<PaymentMethodDetailsCard>,
    #[serde(rename = "card_present")]
    ///
    pub card_present: Option<PaymentMethodDetailsCardPresent>,
    #[serde(rename = "customer_balance")]
    ///
    pub customer_balance: Option<PaymentMethodDetailsCustomerBalance>,
    #[serde(rename = "eps")]
    ///
    pub eps: Option<PaymentMethodDetailsEps>,
    #[serde(rename = "fpx")]
    ///
    pub fpx: Option<PaymentMethodDetailsFpx>,
    #[serde(rename = "giropay")]
    ///
    pub giropay: Option<PaymentMethodDetailsGiropay>,
    #[serde(rename = "grabpay")]
    ///
    pub grabpay: Option<PaymentMethodDetailsGrabpay>,
    #[serde(rename = "ideal")]
    ///
    pub ideal: Option<PaymentMethodDetailsIdeal>,
    #[serde(rename = "interac_present")]
    ///
    pub interac_present: Option<PaymentMethodDetailsInteracPresent>,
    #[serde(rename = "klarna")]
    ///
    pub klarna: Option<PaymentMethodDetailsKlarna>,
    #[serde(rename = "konbini")]
    ///
    pub konbini: Option<PaymentMethodDetailsKonbini>,
    #[serde(rename = "link")]
    ///
    pub link: Option<PaymentMethodDetailsLink>,
    #[serde(rename = "multibanco")]
    ///
    pub multibanco: Option<PaymentMethodDetailsMultibanco>,
    #[serde(rename = "oxxo")]
    ///
    pub oxxo: Option<PaymentMethodDetailsOxxo>,
    #[serde(rename = "p24")]
    ///
    pub p24: Option<PaymentMethodDetailsP24>,
    #[serde(rename = "paynow")]
    ///
    pub paynow: Option<PaymentMethodDetailsPaynow>,
    #[serde(rename = "promptpay")]
    ///
    pub promptpay: Option<PaymentMethodDetailsPromptpay>,
    #[serde(rename = "sepa_debit")]
    ///
    pub sepa_debit: Option<PaymentMethodDetailsSepaDebit>,
    #[serde(rename = "sofort")]
    ///
    pub sofort: Option<PaymentMethodDetailsSofort>,
    #[serde(rename = "stripe_account")]
    ///
    pub stripe_account: Option<PaymentMethodDetailsStripeAccount>,
    #[serde(rename = "type")]
    /**The type of transaction-specific details of the payment method used in the payment, one of `ach_credit_transfer`, `ach_debit`, `acss_debit`, `alipay`, `au_becs_debit`, `bancontact`, `card`, `card_present`, `eps`, `giropay`, `ideal`, `klarna`, `multibanco`, `p24`, `sepa_debit`, `sofort`, `stripe_account`, or `wechat`.
An additional hash is included on `payment_method_details` with a name matching this value.
It contains information specific to the payment method.*/
    pub type_: String,
    #[serde(rename = "us_bank_account")]
    ///
    pub us_bank_account: Option<PaymentMethodDetailsUsBankAccount>,
    #[serde(rename = "wechat")]
    ///
    pub wechat: Option<PaymentMethodDetailsWechat>,
    #[serde(rename = "wechat_pay")]
    ///
    pub wechat_pay: Option<PaymentMethodDetailsWechatPay>,
}
impl std::fmt::Display for PaymentMethodDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodDetailsAchCreditTransfer {
    #[serde(rename = "account_number")]
    ///Account number to transfer funds to.
    pub account_number: Option<String>,
    #[serde(rename = "bank_name")]
    ///Name of the bank associated with the routing number.
    pub bank_name: Option<String>,
    #[serde(rename = "routing_number")]
    ///Routing transit number for the bank account to transfer funds to.
    pub routing_number: Option<String>,
    #[serde(rename = "swift_code")]
    ///SWIFT code of the bank associated with the routing number.
    pub swift_code: Option<String>,
}
impl std::fmt::Display for PaymentMethodDetailsAchCreditTransfer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodDetailsAchDebit {
    #[serde(rename = "account_holder_type")]
    ///Type of entity that holds the account. This can be either `individual` or `company`.
    pub account_holder_type: Option<String>,
    #[serde(rename = "bank_name")]
    ///Name of the bank associated with the bank account.
    pub bank_name: Option<String>,
    #[serde(rename = "country")]
    ///Two-letter ISO code representing the country the bank account is located in.
    pub country: Option<String>,
    #[serde(rename = "fingerprint")]
    ///Uniquely identifies this particular bank account. You can use this attribute to check whether two bank accounts are the same.
    pub fingerprint: Option<String>,
    #[serde(rename = "last4")]
    ///Last four digits of the bank account number.
    pub last4: Option<String>,
    #[serde(rename = "routing_number")]
    ///Routing transit number of the bank account.
    pub routing_number: Option<String>,
}
impl std::fmt::Display for PaymentMethodDetailsAchDebit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodDetailsAcssDebit {
    #[serde(rename = "bank_name")]
    ///Name of the bank associated with the bank account.
    pub bank_name: Option<String>,
    #[serde(rename = "fingerprint")]
    ///Uniquely identifies this particular bank account. You can use this attribute to check whether two bank accounts are the same.
    pub fingerprint: Option<String>,
    #[serde(rename = "institution_number")]
    ///Institution number of the bank account
    pub institution_number: Option<String>,
    #[serde(rename = "last4")]
    ///Last four digits of the bank account number.
    pub last4: Option<String>,
    #[serde(rename = "mandate")]
    ///ID of the mandate used to make this payment.
    pub mandate: Option<String>,
    #[serde(rename = "transit_number")]
    ///Transit number of the bank account.
    pub transit_number: Option<String>,
}
impl std::fmt::Display for PaymentMethodDetailsAcssDebit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodDetailsAffirm {}
impl std::fmt::Display for PaymentMethodDetailsAffirm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodDetailsAfterpayClearpay {
    #[serde(rename = "reference")]
    ///Order identifier shown to the merchant in Afterpay’s online portal.
    pub reference: Option<String>,
}
impl std::fmt::Display for PaymentMethodDetailsAfterpayClearpay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodDetailsAuBecsDebit {
    #[serde(rename = "bsb_number")]
    ///Bank-State-Branch number of the bank account.
    pub bsb_number: Option<String>,
    #[serde(rename = "fingerprint")]
    ///Uniquely identifies this particular bank account. You can use this attribute to check whether two bank accounts are the same.
    pub fingerprint: Option<String>,
    #[serde(rename = "last4")]
    ///Last four digits of the bank account number.
    pub last4: Option<String>,
    #[serde(rename = "mandate")]
    ///ID of the mandate used to make this payment.
    pub mandate: Option<String>,
}
impl std::fmt::Display for PaymentMethodDetailsAuBecsDebit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodDetailsBacsDebit {
    #[serde(rename = "fingerprint")]
    ///Uniquely identifies this particular bank account. You can use this attribute to check whether two bank accounts are the same.
    pub fingerprint: Option<String>,
    #[serde(rename = "last4")]
    ///Last four digits of the bank account number.
    pub last4: Option<String>,
    #[serde(rename = "mandate")]
    ///ID of the mandate used to make this payment.
    pub mandate: Option<String>,
    #[serde(rename = "sort_code")]
    ///Sort code of the bank account. (e.g., `10-20-30`)
    pub sort_code: Option<String>,
}
impl std::fmt::Display for PaymentMethodDetailsBacsDebit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodDetailsBancontact {
    #[serde(rename = "bank_code")]
    ///Bank code of bank associated with the bank account.
    pub bank_code: Option<String>,
    #[serde(rename = "bank_name")]
    ///Name of the bank associated with the bank account.
    pub bank_name: Option<String>,
    #[serde(rename = "bic")]
    ///Bank Identifier Code of the bank associated with the bank account.
    pub bic: Option<String>,
    #[serde(rename = "generated_sepa_debit")]
    ///The ID of the SEPA Direct Debit PaymentMethod which was generated by this Charge.
    pub generated_sepa_debit: Option<serde_json::Value>,
    #[serde(rename = "generated_sepa_debit_mandate")]
    ///The mandate for the SEPA Direct Debit PaymentMethod which was generated by this Charge.
    pub generated_sepa_debit_mandate: Option<serde_json::Value>,
    #[serde(rename = "iban_last4")]
    ///Last four characters of the IBAN.
    pub iban_last4: Option<String>,
    #[serde(rename = "preferred_language")]
    /**Preferred language of the Bancontact authorization page that the customer is redirected to.
Can be one of `en`, `de`, `fr`, or `nl`*/
    pub preferred_language: Option<String>,
    #[serde(rename = "verified_name")]
    /**Owner's verified full name. Values are verified or provided by Bancontact directly
(if supported) at the time of authorization or settlement. They cannot be set or mutated.*/
    pub verified_name: Option<String>,
}
impl std::fmt::Display for PaymentMethodDetailsBancontact {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodDetailsBlik {}
impl std::fmt::Display for PaymentMethodDetailsBlik {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodDetailsBoleto {
    #[serde(rename = "tax_id")]
    ///The tax ID of the customer (CPF for individuals consumers or CNPJ for businesses consumers)
    pub tax_id: String,
}
impl std::fmt::Display for PaymentMethodDetailsBoleto {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodDetailsCard {
    #[serde(rename = "brand")]
    ///Card brand. Can be `amex`, `diners`, `discover`, `jcb`, `mastercard`, `unionpay`, `visa`, or `unknown`.
    pub brand: Option<String>,
    #[serde(rename = "checks")]
    ///Check results by Card networks on Card address and CVC at time of payment.
    pub checks: Option<serde_json::Value>,
    #[serde(rename = "country")]
    ///Two-letter ISO code representing the country of the card. You could use this attribute to get a sense of the international breakdown of cards you've collected.
    pub country: Option<String>,
    #[serde(rename = "exp_month")]
    ///Two-digit number representing the card's expiration month.
    pub exp_month: i64,
    #[serde(rename = "exp_year")]
    ///Four-digit number representing the card's expiration year.
    pub exp_year: i64,
    #[serde(rename = "fingerprint")]
    /**Uniquely identifies this particular card number. You can use this attribute to check whether two customers who’ve signed up with you are using the same card number, for example. For payment methods that tokenize card information (Apple Pay, Google Pay), the tokenized number might be provided instead of the underlying card number.

*Starting May 1, 2021, card fingerprint in India for Connect will change to allow two fingerprints for the same card --- one for India and one for the rest of the world.**/
    pub fingerprint: Option<String>,
    #[serde(rename = "funding")]
    ///Card funding type. Can be `credit`, `debit`, `prepaid`, or `unknown`.
    pub funding: Option<String>,
    #[serde(rename = "installments")]
    /**Installment details for this payment (Mexico only).

For more information, see the [installments integration guide](https://stripe.com/docs/payments/installments).*/
    pub installments: Option<serde_json::Value>,
    #[serde(rename = "last4")]
    ///The last four digits of the card.
    pub last4: Option<String>,
    #[serde(rename = "mandate")]
    ///ID of the mandate used to make this payment or created by it.
    pub mandate: Option<String>,
    #[serde(rename = "network")]
    ///Identifies which network this charge was processed on. Can be `amex`, `cartes_bancaires`, `diners`, `discover`, `interac`, `jcb`, `mastercard`, `unionpay`, `visa`, or `unknown`.
    pub network: Option<String>,
    #[serde(rename = "three_d_secure")]
    ///Populated if this transaction used 3D Secure authentication.
    pub three_d_secure: Option<serde_json::Value>,
    #[serde(rename = "wallet")]
    ///If this Card is part of a card wallet, this contains the details of the card wallet.
    pub wallet: Option<serde_json::Value>,
}
impl std::fmt::Display for PaymentMethodDetailsCard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodDetailsCardChecks {
    #[serde(rename = "address_line1_check")]
    ///If a address line1 was provided, results of the check, one of `pass`, `fail`, `unavailable`, or `unchecked`.
    pub address_line1_check: Option<String>,
    #[serde(rename = "address_postal_code_check")]
    ///If a address postal code was provided, results of the check, one of `pass`, `fail`, `unavailable`, or `unchecked`.
    pub address_postal_code_check: Option<String>,
    #[serde(rename = "cvc_check")]
    ///If a CVC was provided, results of the check, one of `pass`, `fail`, `unavailable`, or `unchecked`.
    pub cvc_check: Option<String>,
}
impl std::fmt::Display for PaymentMethodDetailsCardChecks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodDetailsCardInstallments {
    #[serde(rename = "plan")]
    ///Installment plan selected for the payment.
    pub plan: Option<serde_json::Value>,
}
impl std::fmt::Display for PaymentMethodDetailsCardInstallments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodDetailsCardInstallmentsPlan {
    #[serde(rename = "count")]
    ///For `fixed_count` installment plans, this is the number of installment payments your customer will make to their credit card.
    pub count: Option<i64>,
    #[serde(rename = "interval")]
    /**For `fixed_count` installment plans, this is the interval between installment payments your customer will make to their credit card.
One of `month`.*/
    pub interval: Option<String>,
    #[serde(rename = "type")]
    ///Type of installment plan, one of `fixed_count`.
    pub type_: String,
}
impl std::fmt::Display for PaymentMethodDetailsCardInstallmentsPlan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodDetailsCardPresent {
    #[serde(rename = "amount_authorized")]
    ///The authorized amount
    pub amount_authorized: Option<i64>,
    #[serde(rename = "brand")]
    ///Card brand. Can be `amex`, `diners`, `discover`, `jcb`, `mastercard`, `unionpay`, `visa`, or `unknown`.
    pub brand: Option<String>,
    #[serde(rename = "capture_before")]
    ///When using manual capture, a future timestamp after which the charge will be automatically refunded if uncaptured.
    pub capture_before: Option<i64>,
    #[serde(rename = "cardholder_name")]
    ///The cardholder name as read from the card, in [ISO 7813](https://en.wikipedia.org/wiki/ISO/IEC_7813) format. May include alphanumeric characters, special characters and first/last name separator (`/`). In some cases, the cardholder name may not be available depending on how the issuer has configured the card. Cardholder name is typically not available on swipe or contactless payments, such as those made with Apple Pay and Google Pay.
    pub cardholder_name: Option<String>,
    #[serde(rename = "country")]
    ///Two-letter ISO code representing the country of the card. You could use this attribute to get a sense of the international breakdown of cards you've collected.
    pub country: Option<String>,
    #[serde(rename = "emv_auth_data")]
    ///Authorization response cryptogram.
    pub emv_auth_data: Option<String>,
    #[serde(rename = "exp_month")]
    ///Two-digit number representing the card's expiration month.
    pub exp_month: i64,
    #[serde(rename = "exp_year")]
    ///Four-digit number representing the card's expiration year.
    pub exp_year: i64,
    #[serde(rename = "fingerprint")]
    /**Uniquely identifies this particular card number. You can use this attribute to check whether two customers who’ve signed up with you are using the same card number, for example. For payment methods that tokenize card information (Apple Pay, Google Pay), the tokenized number might be provided instead of the underlying card number.

*Starting May 1, 2021, card fingerprint in India for Connect will change to allow two fingerprints for the same card --- one for India and one for the rest of the world.**/
    pub fingerprint: Option<String>,
    #[serde(rename = "funding")]
    ///Card funding type. Can be `credit`, `debit`, `prepaid`, or `unknown`.
    pub funding: Option<String>,
    #[serde(rename = "generated_card")]
    ///ID of a card PaymentMethod generated from the card_present PaymentMethod that may be attached to a Customer for future transactions. Only present if it was possible to generate a card PaymentMethod.
    pub generated_card: Option<String>,
    #[serde(rename = "incremental_authorization_supported")]
    ///Whether this [PaymentIntent](https://stripe.com/docs/api/payment_intents) is eligible for incremental authorizations. Request support using [request_incremental_authorization_support](https://stripe.com/docs/api/payment_intents/create#create_payment_intent-payment_method_options-card_present-request_incremental_authorization_support).
    pub incremental_authorization_supported: Option<bool>,
    #[serde(rename = "last4")]
    ///The last four digits of the card.
    pub last4: Option<String>,
    #[serde(rename = "network")]
    ///Identifies which network this charge was processed on. Can be `amex`, `cartes_bancaires`, `diners`, `discover`, `interac`, `jcb`, `mastercard`, `unionpay`, `visa`, or `unknown`.
    pub network: Option<String>,
    #[serde(rename = "overcapture_supported")]
    ///Defines whether the authorized amount can be over-captured or not
    pub overcapture_supported: Option<bool>,
    #[serde(rename = "read_method")]
    ///How card details were read in this transaction.
    pub read_method: Option<String>,
    #[serde(rename = "receipt")]
    ///A collection of fields required to be displayed on receipts. Only required for EMV transactions.
    pub receipt: Option<serde_json::Value>,
}
impl std::fmt::Display for PaymentMethodDetailsCardPresent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodDetailsCardPresentReceipt {
    #[serde(rename = "account_type")]
    ///The type of account being debited or credited
    pub account_type: Option<String>,
    #[serde(rename = "application_cryptogram")]
    ///EMV tag 9F26, cryptogram generated by the integrated circuit chip.
    pub application_cryptogram: Option<String>,
    #[serde(rename = "application_preferred_name")]
    ///Mnenomic of the Application Identifier.
    pub application_preferred_name: Option<String>,
    #[serde(rename = "authorization_code")]
    ///Identifier for this transaction.
    pub authorization_code: Option<String>,
    #[serde(rename = "authorization_response_code")]
    ///EMV tag 8A. A code returned by the card issuer.
    pub authorization_response_code: Option<String>,
    #[serde(rename = "cardholder_verification_method")]
    ///How the cardholder verified ownership of the card.
    pub cardholder_verification_method: Option<String>,
    #[serde(rename = "dedicated_file_name")]
    ///EMV tag 84. Similar to the application identifier stored on the integrated circuit chip.
    pub dedicated_file_name: Option<String>,
    #[serde(rename = "terminal_verification_results")]
    ///The outcome of a series of EMV functions performed by the card reader.
    pub terminal_verification_results: Option<String>,
    #[serde(rename = "transaction_status_information")]
    ///An indication of various EMV functions performed during the transaction.
    pub transaction_status_information: Option<String>,
}
impl std::fmt::Display for PaymentMethodDetailsCardPresentReceipt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodDetailsCardWallet {
    #[serde(rename = "amex_express_checkout")]
    ///
    pub amex_express_checkout: Option<PaymentMethodDetailsCardWalletAmexExpressCheckout>,
    #[serde(rename = "apple_pay")]
    ///
    pub apple_pay: Option<PaymentMethodDetailsCardWalletApplePay>,
    #[serde(rename = "dynamic_last4")]
    ///(For tokenized numbers only.) The last four digits of the device account number.
    pub dynamic_last4: Option<String>,
    #[serde(rename = "google_pay")]
    ///
    pub google_pay: Option<PaymentMethodDetailsCardWalletGooglePay>,
    #[serde(rename = "masterpass")]
    ///
    pub masterpass: Option<PaymentMethodDetailsCardWalletMasterpass>,
    #[serde(rename = "samsung_pay")]
    ///
    pub samsung_pay: Option<PaymentMethodDetailsCardWalletSamsungPay>,
    #[serde(rename = "type")]
    ///The type of the card wallet, one of `amex_express_checkout`, `apple_pay`, `google_pay`, `masterpass`, `samsung_pay`, or `visa_checkout`. An additional hash is included on the Wallet subhash with a name matching this value. It contains additional information specific to the card wallet type.
    pub type_: String,
    #[serde(rename = "visa_checkout")]
    ///
    pub visa_checkout: Option<PaymentMethodDetailsCardWalletVisaCheckout>,
}
impl std::fmt::Display for PaymentMethodDetailsCardWallet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodDetailsCardWalletAmexExpressCheckout {}
impl std::fmt::Display for PaymentMethodDetailsCardWalletAmexExpressCheckout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodDetailsCardWalletApplePay {}
impl std::fmt::Display for PaymentMethodDetailsCardWalletApplePay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodDetailsCardWalletGooglePay {}
impl std::fmt::Display for PaymentMethodDetailsCardWalletGooglePay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodDetailsCardWalletMasterpass {
    #[serde(rename = "billing_address")]
    ///Owner's verified billing address. Values are verified or provided by the wallet directly (if supported) at the time of authorization or settlement. They cannot be set or mutated.
    pub billing_address: Option<serde_json::Value>,
    #[serde(rename = "email")]
    ///Owner's verified email. Values are verified or provided by the wallet directly (if supported) at the time of authorization or settlement. They cannot be set or mutated.
    pub email: Option<String>,
    #[serde(rename = "name")]
    ///Owner's verified full name. Values are verified or provided by the wallet directly (if supported) at the time of authorization or settlement. They cannot be set or mutated.
    pub name: Option<String>,
    #[serde(rename = "shipping_address")]
    ///Owner's verified shipping address. Values are verified or provided by the wallet directly (if supported) at the time of authorization or settlement. They cannot be set or mutated.
    pub shipping_address: Option<serde_json::Value>,
}
impl std::fmt::Display for PaymentMethodDetailsCardWalletMasterpass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodDetailsCardWalletSamsungPay {}
impl std::fmt::Display for PaymentMethodDetailsCardWalletSamsungPay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodDetailsCardWalletVisaCheckout {
    #[serde(rename = "billing_address")]
    ///Owner's verified billing address. Values are verified or provided by the wallet directly (if supported) at the time of authorization or settlement. They cannot be set or mutated.
    pub billing_address: Option<serde_json::Value>,
    #[serde(rename = "email")]
    ///Owner's verified email. Values are verified or provided by the wallet directly (if supported) at the time of authorization or settlement. They cannot be set or mutated.
    pub email: Option<String>,
    #[serde(rename = "name")]
    ///Owner's verified full name. Values are verified or provided by the wallet directly (if supported) at the time of authorization or settlement. They cannot be set or mutated.
    pub name: Option<String>,
    #[serde(rename = "shipping_address")]
    ///Owner's verified shipping address. Values are verified or provided by the wallet directly (if supported) at the time of authorization or settlement. They cannot be set or mutated.
    pub shipping_address: Option<serde_json::Value>,
}
impl std::fmt::Display for PaymentMethodDetailsCardWalletVisaCheckout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodDetailsCustomerBalance {}
impl std::fmt::Display for PaymentMethodDetailsCustomerBalance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodDetailsEps {
    #[serde(rename = "bank")]
    ///The customer's bank. Should be one of `arzte_und_apotheker_bank`, `austrian_anadi_bank_ag`, `bank_austria`, `bankhaus_carl_spangler`, `bankhaus_schelhammer_und_schattera_ag`, `bawag_psk_ag`, `bks_bank_ag`, `brull_kallmus_bank_ag`, `btv_vier_lander_bank`, `capital_bank_grawe_gruppe_ag`, `dolomitenbank`, `easybank_ag`, `erste_bank_und_sparkassen`, `hypo_alpeadriabank_international_ag`, `hypo_noe_lb_fur_niederosterreich_u_wien`, `hypo_oberosterreich_salzburg_steiermark`, `hypo_tirol_bank_ag`, `hypo_vorarlberg_bank_ag`, `hypo_bank_burgenland_aktiengesellschaft`, `marchfelder_bank`, `oberbank_ag`, `raiffeisen_bankengruppe_osterreich`, `schoellerbank_ag`, `sparda_bank_wien`, `volksbank_gruppe`, `volkskreditbank_ag`, or `vr_bank_braunau`.
    pub bank: Option<String>,
    #[serde(rename = "verified_name")]
    /**Owner's verified full name. Values are verified or provided by EPS directly
(if supported) at the time of authorization or settlement. They cannot be set or mutated.
EPS rarely provides this information so the attribute is usually empty.*/
    pub verified_name: Option<String>,
}
impl std::fmt::Display for PaymentMethodDetailsEps {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodDetailsFpx {
    #[serde(rename = "bank")]
    ///The customer's bank. Can be one of `affin_bank`, `agrobank`, `alliance_bank`, `ambank`, `bank_islam`, `bank_muamalat`, `bank_rakyat`, `bsn`, `cimb`, `hong_leong_bank`, `hsbc`, `kfh`, `maybank2u`, `ocbc`, `public_bank`, `rhb`, `standard_chartered`, `uob`, `deutsche_bank`, `maybank2e`, or `pb_enterprise`.
    pub bank: String,
    #[serde(rename = "transaction_id")]
    ///Unique transaction id generated by FPX for every request from the merchant
    pub transaction_id: Option<String>,
}
impl std::fmt::Display for PaymentMethodDetailsFpx {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodDetailsGiropay {
    #[serde(rename = "bank_code")]
    ///Bank code of bank associated with the bank account.
    pub bank_code: Option<String>,
    #[serde(rename = "bank_name")]
    ///Name of the bank associated with the bank account.
    pub bank_name: Option<String>,
    #[serde(rename = "bic")]
    ///Bank Identifier Code of the bank associated with the bank account.
    pub bic: Option<String>,
    #[serde(rename = "verified_name")]
    /**Owner's verified full name. Values are verified or provided by Giropay directly
(if supported) at the time of authorization or settlement. They cannot be set or mutated.
Giropay rarely provides this information so the attribute is usually empty.*/
    pub verified_name: Option<String>,
}
impl std::fmt::Display for PaymentMethodDetailsGiropay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodDetailsGrabpay {
    #[serde(rename = "transaction_id")]
    ///Unique transaction id generated by GrabPay
    pub transaction_id: Option<String>,
}
impl std::fmt::Display for PaymentMethodDetailsGrabpay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodDetailsIdeal {
    #[serde(rename = "bank")]
    ///The customer's bank. Can be one of `abn_amro`, `asn_bank`, `bunq`, `handelsbanken`, `ing`, `knab`, `moneyou`, `rabobank`, `regiobank`, `revolut`, `sns_bank`, `triodos_bank`, or `van_lanschot`.
    pub bank: Option<String>,
    #[serde(rename = "bic")]
    ///The Bank Identifier Code of the customer's bank.
    pub bic: Option<String>,
    #[serde(rename = "generated_sepa_debit")]
    ///The ID of the SEPA Direct Debit PaymentMethod which was generated by this Charge.
    pub generated_sepa_debit: Option<serde_json::Value>,
    #[serde(rename = "generated_sepa_debit_mandate")]
    ///The mandate for the SEPA Direct Debit PaymentMethod which was generated by this Charge.
    pub generated_sepa_debit_mandate: Option<serde_json::Value>,
    #[serde(rename = "iban_last4")]
    ///Last four characters of the IBAN.
    pub iban_last4: Option<String>,
    #[serde(rename = "verified_name")]
    /**Owner's verified full name. Values are verified or provided by iDEAL directly
(if supported) at the time of authorization or settlement. They cannot be set or mutated.*/
    pub verified_name: Option<String>,
}
impl std::fmt::Display for PaymentMethodDetailsIdeal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodDetailsInteracPresent {
    #[serde(rename = "brand")]
    ///Card brand. Can be `interac`, `mastercard` or `visa`.
    pub brand: Option<String>,
    #[serde(rename = "cardholder_name")]
    ///The cardholder name as read from the card, in [ISO 7813](https://en.wikipedia.org/wiki/ISO/IEC_7813) format. May include alphanumeric characters, special characters and first/last name separator (`/`). In some cases, the cardholder name may not be available depending on how the issuer has configured the card. Cardholder name is typically not available on swipe or contactless payments, such as those made with Apple Pay and Google Pay.
    pub cardholder_name: Option<String>,
    #[serde(rename = "country")]
    ///Two-letter ISO code representing the country of the card. You could use this attribute to get a sense of the international breakdown of cards you've collected.
    pub country: Option<String>,
    #[serde(rename = "emv_auth_data")]
    ///Authorization response cryptogram.
    pub emv_auth_data: Option<String>,
    #[serde(rename = "exp_month")]
    ///Two-digit number representing the card's expiration month.
    pub exp_month: i64,
    #[serde(rename = "exp_year")]
    ///Four-digit number representing the card's expiration year.
    pub exp_year: i64,
    #[serde(rename = "fingerprint")]
    /**Uniquely identifies this particular card number. You can use this attribute to check whether two customers who’ve signed up with you are using the same card number, for example. For payment methods that tokenize card information (Apple Pay, Google Pay), the tokenized number might be provided instead of the underlying card number.

*Starting May 1, 2021, card fingerprint in India for Connect will change to allow two fingerprints for the same card --- one for India and one for the rest of the world.**/
    pub fingerprint: Option<String>,
    #[serde(rename = "funding")]
    ///Card funding type. Can be `credit`, `debit`, `prepaid`, or `unknown`.
    pub funding: Option<String>,
    #[serde(rename = "generated_card")]
    ///ID of a card PaymentMethod generated from the card_present PaymentMethod that may be attached to a Customer for future transactions. Only present if it was possible to generate a card PaymentMethod.
    pub generated_card: Option<String>,
    #[serde(rename = "last4")]
    ///The last four digits of the card.
    pub last4: Option<String>,
    #[serde(rename = "network")]
    ///Identifies which network this charge was processed on. Can be `amex`, `cartes_bancaires`, `diners`, `discover`, `interac`, `jcb`, `mastercard`, `unionpay`, `visa`, or `unknown`.
    pub network: Option<String>,
    #[serde(rename = "preferred_locales")]
    ///EMV tag 5F2D. Preferred languages specified by the integrated circuit chip.
    pub preferred_locales: Option<Vec<String>>,
    #[serde(rename = "read_method")]
    ///How card details were read in this transaction.
    pub read_method: Option<String>,
    #[serde(rename = "receipt")]
    ///A collection of fields required to be displayed on receipts. Only required for EMV transactions.
    pub receipt: Option<serde_json::Value>,
}
impl std::fmt::Display for PaymentMethodDetailsInteracPresent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodDetailsInteracPresentReceipt {
    #[serde(rename = "account_type")]
    ///The type of account being debited or credited
    pub account_type: Option<String>,
    #[serde(rename = "application_cryptogram")]
    ///EMV tag 9F26, cryptogram generated by the integrated circuit chip.
    pub application_cryptogram: Option<String>,
    #[serde(rename = "application_preferred_name")]
    ///Mnenomic of the Application Identifier.
    pub application_preferred_name: Option<String>,
    #[serde(rename = "authorization_code")]
    ///Identifier for this transaction.
    pub authorization_code: Option<String>,
    #[serde(rename = "authorization_response_code")]
    ///EMV tag 8A. A code returned by the card issuer.
    pub authorization_response_code: Option<String>,
    #[serde(rename = "cardholder_verification_method")]
    ///How the cardholder verified ownership of the card.
    pub cardholder_verification_method: Option<String>,
    #[serde(rename = "dedicated_file_name")]
    ///EMV tag 84. Similar to the application identifier stored on the integrated circuit chip.
    pub dedicated_file_name: Option<String>,
    #[serde(rename = "terminal_verification_results")]
    ///The outcome of a series of EMV functions performed by the card reader.
    pub terminal_verification_results: Option<String>,
    #[serde(rename = "transaction_status_information")]
    ///An indication of various EMV functions performed during the transaction.
    pub transaction_status_information: Option<String>,
}
impl std::fmt::Display for PaymentMethodDetailsInteracPresentReceipt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodDetailsKlarna {
    #[serde(rename = "payment_method_category")]
    /**The Klarna payment method used for this transaction.
Can be one of `pay_later`, `pay_now`, `pay_with_financing`, or `pay_in_installments`*/
    pub payment_method_category: Option<String>,
    #[serde(rename = "preferred_locale")]
    /**Preferred language of the Klarna authorization page that the customer is redirected to.
Can be one of `de-AT`, `en-AT`, `nl-BE`, `fr-BE`, `en-BE`, `de-DE`, `en-DE`, `da-DK`, `en-DK`, `es-ES`, `en-ES`, `fi-FI`, `sv-FI`, `en-FI`, `en-GB`, `en-IE`, `it-IT`, `en-IT`, `nl-NL`, `en-NL`, `nb-NO`, `en-NO`, `sv-SE`, `en-SE`, `en-US`, `es-US`, `fr-FR`, `en-FR`, `en-AU`, `en-NZ`, `en-CA`, or `fr-CA`*/
    pub preferred_locale: Option<String>,
}
impl std::fmt::Display for PaymentMethodDetailsKlarna {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodDetailsKonbini {
    #[serde(rename = "store")]
    ///If the payment succeeded, this contains the details of the convenience store where the payment was completed.
    pub store: Option<serde_json::Value>,
}
impl std::fmt::Display for PaymentMethodDetailsKonbini {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodDetailsKonbiniStore {
    #[serde(rename = "chain")]
    ///The name of the convenience store chain where the payment was completed.
    pub chain: Option<String>,
}
impl std::fmt::Display for PaymentMethodDetailsKonbiniStore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodDetailsLink {}
impl std::fmt::Display for PaymentMethodDetailsLink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodDetailsMultibanco {
    #[serde(rename = "entity")]
    ///Entity number associated with this Multibanco payment.
    pub entity: Option<String>,
    #[serde(rename = "reference")]
    ///Reference number associated with this Multibanco payment.
    pub reference: Option<String>,
}
impl std::fmt::Display for PaymentMethodDetailsMultibanco {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodDetailsOxxo {
    #[serde(rename = "number")]
    ///OXXO reference number
    pub number: Option<String>,
}
impl std::fmt::Display for PaymentMethodDetailsOxxo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodDetailsP24 {
    #[serde(rename = "bank")]
    ///The customer's bank. Can be one of `ing`, `citi_handlowy`, `tmobile_usbugi_bankowe`, `plus_bank`, `etransfer_pocztowy24`, `banki_spbdzielcze`, `bank_nowy_bfg_sa`, `getin_bank`, `blik`, `noble_pay`, `ideabank`, `envelobank`, `santander_przelew24`, `nest_przelew`, `mbank_mtransfer`, `inteligo`, `pbac_z_ipko`, `bnp_paribas`, `credit_agricole`, `toyota_bank`, `bank_pekao_sa`, `volkswagen_bank`, `bank_millennium`, `alior_bank`, or `boz`.
    pub bank: Option<String>,
    #[serde(rename = "reference")]
    ///Unique reference for this Przelewy24 payment.
    pub reference: Option<String>,
    #[serde(rename = "verified_name")]
    /**Owner's verified full name. Values are verified or provided by Przelewy24 directly
(if supported) at the time of authorization or settlement. They cannot be set or mutated.
Przelewy24 rarely provides this information so the attribute is usually empty.*/
    pub verified_name: Option<String>,
}
impl std::fmt::Display for PaymentMethodDetailsP24 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodDetailsPaynow {
    #[serde(rename = "reference")]
    ///Reference number associated with this PayNow payment
    pub reference: Option<String>,
}
impl std::fmt::Display for PaymentMethodDetailsPaynow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodDetailsPromptpay {
    #[serde(rename = "reference")]
    ///Bill reference generated by PromptPay
    pub reference: Option<String>,
}
impl std::fmt::Display for PaymentMethodDetailsPromptpay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodDetailsSepaDebit {
    #[serde(rename = "bank_code")]
    ///Bank code of bank associated with the bank account.
    pub bank_code: Option<String>,
    #[serde(rename = "branch_code")]
    ///Branch code of bank associated with the bank account.
    pub branch_code: Option<String>,
    #[serde(rename = "country")]
    ///Two-letter ISO code representing the country the bank account is located in.
    pub country: Option<String>,
    #[serde(rename = "fingerprint")]
    ///Uniquely identifies this particular bank account. You can use this attribute to check whether two bank accounts are the same.
    pub fingerprint: Option<String>,
    #[serde(rename = "last4")]
    ///Last four characters of the IBAN.
    pub last4: Option<String>,
    #[serde(rename = "mandate")]
    ///ID of the mandate used to make this payment.
    pub mandate: Option<String>,
}
impl std::fmt::Display for PaymentMethodDetailsSepaDebit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodDetailsSofort {
    #[serde(rename = "bank_code")]
    ///Bank code of bank associated with the bank account.
    pub bank_code: Option<String>,
    #[serde(rename = "bank_name")]
    ///Name of the bank associated with the bank account.
    pub bank_name: Option<String>,
    #[serde(rename = "bic")]
    ///Bank Identifier Code of the bank associated with the bank account.
    pub bic: Option<String>,
    #[serde(rename = "country")]
    ///Two-letter ISO code representing the country the bank account is located in.
    pub country: Option<String>,
    #[serde(rename = "generated_sepa_debit")]
    ///The ID of the SEPA Direct Debit PaymentMethod which was generated by this Charge.
    pub generated_sepa_debit: Option<serde_json::Value>,
    #[serde(rename = "generated_sepa_debit_mandate")]
    ///The mandate for the SEPA Direct Debit PaymentMethod which was generated by this Charge.
    pub generated_sepa_debit_mandate: Option<serde_json::Value>,
    #[serde(rename = "iban_last4")]
    ///Last four characters of the IBAN.
    pub iban_last4: Option<String>,
    #[serde(rename = "preferred_language")]
    /**Preferred language of the SOFORT authorization page that the customer is redirected to.
Can be one of `de`, `en`, `es`, `fr`, `it`, `nl`, or `pl`*/
    pub preferred_language: Option<String>,
    #[serde(rename = "verified_name")]
    /**Owner's verified full name. Values are verified or provided by SOFORT directly
(if supported) at the time of authorization or settlement. They cannot be set or mutated.*/
    pub verified_name: Option<String>,
}
impl std::fmt::Display for PaymentMethodDetailsSofort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodDetailsStripeAccount {}
impl std::fmt::Display for PaymentMethodDetailsStripeAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodDetailsUsBankAccount {
    #[serde(rename = "account_holder_type")]
    ///Account holder type: individual or company.
    pub account_holder_type: Option<String>,
    #[serde(rename = "account_type")]
    ///Account type: checkings or savings. Defaults to checking if omitted.
    pub account_type: Option<String>,
    #[serde(rename = "bank_name")]
    ///Name of the bank associated with the bank account.
    pub bank_name: Option<String>,
    #[serde(rename = "fingerprint")]
    ///Uniquely identifies this particular bank account. You can use this attribute to check whether two bank accounts are the same.
    pub fingerprint: Option<String>,
    #[serde(rename = "last4")]
    ///Last four digits of the bank account number.
    pub last4: Option<String>,
    #[serde(rename = "routing_number")]
    ///Routing number of the bank account.
    pub routing_number: Option<String>,
}
impl std::fmt::Display for PaymentMethodDetailsUsBankAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodDetailsWechat {}
impl std::fmt::Display for PaymentMethodDetailsWechat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodDetailsWechatPay {
    #[serde(rename = "fingerprint")]
    ///Uniquely identifies this particular WeChat Pay account. You can use this attribute to check whether two WeChat accounts are the same.
    pub fingerprint: Option<String>,
    #[serde(rename = "transaction_id")]
    ///Transaction ID of this particular WeChat Pay transaction.
    pub transaction_id: Option<String>,
}
impl std::fmt::Display for PaymentMethodDetailsWechatPay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodEps {
    #[serde(rename = "bank")]
    ///The customer's bank. Should be one of `arzte_und_apotheker_bank`, `austrian_anadi_bank_ag`, `bank_austria`, `bankhaus_carl_spangler`, `bankhaus_schelhammer_und_schattera_ag`, `bawag_psk_ag`, `bks_bank_ag`, `brull_kallmus_bank_ag`, `btv_vier_lander_bank`, `capital_bank_grawe_gruppe_ag`, `dolomitenbank`, `easybank_ag`, `erste_bank_und_sparkassen`, `hypo_alpeadriabank_international_ag`, `hypo_noe_lb_fur_niederosterreich_u_wien`, `hypo_oberosterreich_salzburg_steiermark`, `hypo_tirol_bank_ag`, `hypo_vorarlberg_bank_ag`, `hypo_bank_burgenland_aktiengesellschaft`, `marchfelder_bank`, `oberbank_ag`, `raiffeisen_bankengruppe_osterreich`, `schoellerbank_ag`, `sparda_bank_wien`, `volksbank_gruppe`, `volkskreditbank_ag`, or `vr_bank_braunau`.
    pub bank: Option<String>,
}
impl std::fmt::Display for PaymentMethodEps {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodFpx {
    #[serde(rename = "bank")]
    ///The customer's bank, if provided. Can be one of `affin_bank`, `agrobank`, `alliance_bank`, `ambank`, `bank_islam`, `bank_muamalat`, `bank_rakyat`, `bsn`, `cimb`, `hong_leong_bank`, `hsbc`, `kfh`, `maybank2u`, `ocbc`, `public_bank`, `rhb`, `standard_chartered`, `uob`, `deutsche_bank`, `maybank2e`, or `pb_enterprise`.
    pub bank: String,
}
impl std::fmt::Display for PaymentMethodFpx {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodGiropay {}
impl std::fmt::Display for PaymentMethodGiropay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodGrabpay {}
impl std::fmt::Display for PaymentMethodGrabpay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodIdeal {
    #[serde(rename = "bank")]
    ///The customer's bank, if provided. Can be one of `abn_amro`, `asn_bank`, `bunq`, `handelsbanken`, `ing`, `knab`, `moneyou`, `rabobank`, `regiobank`, `revolut`, `sns_bank`, `triodos_bank`, or `van_lanschot`.
    pub bank: Option<String>,
    #[serde(rename = "bic")]
    ///The Bank Identifier Code of the customer's bank, if the bank was provided.
    pub bic: Option<String>,
}
impl std::fmt::Display for PaymentMethodIdeal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodInteracPresent {}
impl std::fmt::Display for PaymentMethodInteracPresent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodKlarna {
    #[serde(rename = "dob")]
    ///The customer's date of birth, if provided.
    pub dob: Option<serde_json::Value>,
}
impl std::fmt::Display for PaymentMethodKlarna {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodKonbini {}
impl std::fmt::Display for PaymentMethodKonbini {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodLink {
    #[serde(rename = "email")]
    ///Account owner's email address.
    pub email: Option<String>,
    #[serde(rename = "persistent_token")]
    ///Token used for persistent Link logins.
    pub persistent_token: Option<String>,
}
impl std::fmt::Display for PaymentMethodLink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodOptionsAffirm {
    #[serde(rename = "capture_method")]
    ///Controls when the funds will be captured from the customer's account.
    pub capture_method: Option<String>,
    #[serde(rename = "setup_future_usage")]
    /**Indicates that you intend to make future payments with this PaymentIntent's payment method.

Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete. If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.

When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).*/
    pub setup_future_usage: Option<String>,
}
impl std::fmt::Display for PaymentMethodOptionsAffirm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodOptionsAfterpayClearpay {
    #[serde(rename = "capture_method")]
    ///Controls when the funds will be captured from the customer's account.
    pub capture_method: Option<String>,
    #[serde(rename = "reference")]
    /**Order identifier shown to the customer in Afterpay’s online portal. We recommend using a value that helps you answer any questions a customer might have about
the payment. The identifier is limited to 128 characters and may contain only letters, digits, underscores, backslashes and dashes.*/
    pub reference: Option<String>,
    #[serde(rename = "setup_future_usage")]
    /**Indicates that you intend to make future payments with this PaymentIntent's payment method.

Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete. If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.

When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).*/
    pub setup_future_usage: Option<String>,
}
impl std::fmt::Display for PaymentMethodOptionsAfterpayClearpay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodOptionsAlipay {
    #[serde(rename = "setup_future_usage")]
    /**Indicates that you intend to make future payments with this PaymentIntent's payment method.

Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete. If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.

When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).*/
    pub setup_future_usage: Option<String>,
}
impl std::fmt::Display for PaymentMethodOptionsAlipay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodOptionsBacsDebit {
    #[serde(rename = "setup_future_usage")]
    /**Indicates that you intend to make future payments with this PaymentIntent's payment method.

Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete. If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.

When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).*/
    pub setup_future_usage: Option<String>,
}
impl std::fmt::Display for PaymentMethodOptionsBacsDebit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodOptionsBancontact {
    #[serde(rename = "preferred_language")]
    ///Preferred language of the Bancontact authorization page that the customer is redirected to.
    pub preferred_language: String,
    #[serde(rename = "setup_future_usage")]
    /**Indicates that you intend to make future payments with this PaymentIntent's payment method.

Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete. If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.

When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).*/
    pub setup_future_usage: Option<String>,
}
impl std::fmt::Display for PaymentMethodOptionsBancontact {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodOptionsBoleto {
    #[serde(rename = "expires_after_days")]
    ///The number of calendar days before a Boleto voucher expires. For example, if you create a Boleto voucher on Monday and you set expires_after_days to 2, the Boleto voucher will expire on Wednesday at 23:59 America/Sao_Paulo time.
    pub expires_after_days: i64,
    #[serde(rename = "setup_future_usage")]
    /**Indicates that you intend to make future payments with this PaymentIntent's payment method.

Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete. If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.

When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).*/
    pub setup_future_usage: Option<String>,
}
impl std::fmt::Display for PaymentMethodOptionsBoleto {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodOptionsCardInstallments {
    #[serde(rename = "available_plans")]
    ///Installment plans that may be selected for this PaymentIntent.
    pub available_plans: Option<Vec<PaymentMethodDetailsCardInstallmentsPlan>>,
    #[serde(rename = "enabled")]
    ///Whether Installments are enabled for this PaymentIntent.
    pub enabled: bool,
    #[serde(rename = "plan")]
    ///Installment plan selected for this PaymentIntent.
    pub plan: Option<serde_json::Value>,
}
impl std::fmt::Display for PaymentMethodOptionsCardInstallments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodOptionsCardMandateOptions {
    #[serde(rename = "amount")]
    ///Amount to be charged for future payments.
    pub amount: i64,
    #[serde(rename = "amount_type")]
    ///One of `fixed` or `maximum`. If `fixed`, the `amount` param refers to the exact amount to be charged in future payments. If `maximum`, the amount charged can be up to the value passed for the `amount` param.
    pub amount_type: String,
    #[serde(rename = "description")]
    ///A description of the mandate or subscription that is meant to be displayed to the customer.
    pub description: Option<String>,
    #[serde(rename = "end_date")]
    ///End date of the mandate or subscription. If not provided, the mandate will be active until canceled. If provided, end date should be after start date.
    pub end_date: Option<i64>,
    #[serde(rename = "interval")]
    ///Specifies payment frequency. One of `day`, `week`, `month`, `year`, or `sporadic`.
    pub interval: String,
    #[serde(rename = "interval_count")]
    ///The number of intervals between payments. For example, `interval=month` and `interval_count=3` indicates one payment every three months. Maximum of one year interval allowed (1 year, 12 months, or 52 weeks). This parameter is optional when `interval=sporadic`.
    pub interval_count: Option<i64>,
    #[serde(rename = "reference")]
    ///Unique identifier for the mandate or subscription.
    pub reference: String,
    #[serde(rename = "start_date")]
    ///Start date of the mandate or subscription. Start date should not be lesser than yesterday.
    pub start_date: i64,
    #[serde(rename = "supported_types")]
    ///Specifies the type of mandates supported. Possible values are `india`.
    pub supported_types: Option<Vec<String>>,
}
impl std::fmt::Display for PaymentMethodOptionsCardMandateOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodOptionsCardPresent {
    #[serde(rename = "request_extended_authorization")]
    ///Request ability to capture this payment beyond the standard [authorization validity window](https://stripe.com/docs/terminal/features/extended-authorizations#authorization-validity)
    pub request_extended_authorization: Option<bool>,
    #[serde(rename = "request_incremental_authorization_support")]
    ///Request ability to [increment](https://stripe.com/docs/terminal/features/incremental-authorizations) this PaymentIntent if the combination of MCC and card brand is eligible. Check [incremental_authorization_supported](https://stripe.com/docs/api/charges/object#charge_object-payment_method_details-card_present-incremental_authorization_supported) in the [Confirm](https://stripe.com/docs/api/payment_intents/confirm) response to verify support.
    pub request_incremental_authorization_support: Option<bool>,
}
impl std::fmt::Display for PaymentMethodOptionsCardPresent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodOptionsCustomerBalance {
    #[serde(rename = "bank_transfer")]
    ///
    pub bank_transfer: Option<PaymentMethodOptionsCustomerBalanceBankTransfer>,
    #[serde(rename = "funding_type")]
    ///The funding method type to be used when there are not enough funds in the customer balance. Permitted values include: `bank_transfer`.
    pub funding_type: Option<String>,
    #[serde(rename = "setup_future_usage")]
    /**Indicates that you intend to make future payments with this PaymentIntent's payment method.

Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete. If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.

When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).*/
    pub setup_future_usage: Option<String>,
}
impl std::fmt::Display for PaymentMethodOptionsCustomerBalance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodOptionsCustomerBalanceBankTransfer {
    #[serde(rename = "eu_bank_transfer")]
    ///
    pub eu_bank_transfer: Option<PaymentMethodOptionsCustomerBalanceEuBankAccount>,
    #[serde(rename = "requested_address_types")]
    /**List of address types that should be returned in the financial_addresses response. If not specified, all valid types will be returned.

Permitted values include: `sort_code`, `zengin`, `iban`, or `spei`.*/
    pub requested_address_types: Option<Vec<String>>,
    #[serde(rename = "type")]
    ///The bank transfer type that this PaymentIntent is allowed to use for funding Permitted values include: `eu_bank_transfer`, `gb_bank_transfer`, `jp_bank_transfer`, or `mx_bank_transfer`.
    pub type_: Option<String>,
}
impl std::fmt::Display for PaymentMethodOptionsCustomerBalanceBankTransfer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodOptionsCustomerBalanceEuBankAccount {
    #[serde(rename = "country")]
    ///The desired country code of the bank account information. Permitted values include: `DE`, `ES`, `FR`, `IE`, or `NL`.
    pub country: String,
}
impl std::fmt::Display for PaymentMethodOptionsCustomerBalanceEuBankAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodOptionsFpx {
    #[serde(rename = "setup_future_usage")]
    /**Indicates that you intend to make future payments with this PaymentIntent's payment method.

Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete. If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.

When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).*/
    pub setup_future_usage: Option<String>,
}
impl std::fmt::Display for PaymentMethodOptionsFpx {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodOptionsGiropay {
    #[serde(rename = "setup_future_usage")]
    /**Indicates that you intend to make future payments with this PaymentIntent's payment method.

Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete. If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.

When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).*/
    pub setup_future_usage: Option<String>,
}
impl std::fmt::Display for PaymentMethodOptionsGiropay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodOptionsGrabpay {
    #[serde(rename = "setup_future_usage")]
    /**Indicates that you intend to make future payments with this PaymentIntent's payment method.

Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete. If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.

When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).*/
    pub setup_future_usage: Option<String>,
}
impl std::fmt::Display for PaymentMethodOptionsGrabpay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodOptionsIdeal {
    #[serde(rename = "setup_future_usage")]
    /**Indicates that you intend to make future payments with this PaymentIntent's payment method.

Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete. If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.

When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).*/
    pub setup_future_usage: Option<String>,
}
impl std::fmt::Display for PaymentMethodOptionsIdeal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodOptionsInteracPresent {}
impl std::fmt::Display for PaymentMethodOptionsInteracPresent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodOptionsKlarna {
    #[serde(rename = "capture_method")]
    ///Controls when the funds will be captured from the customer's account.
    pub capture_method: Option<String>,
    #[serde(rename = "preferred_locale")]
    ///Preferred locale of the Klarna checkout page that the customer is redirected to.
    pub preferred_locale: Option<String>,
    #[serde(rename = "setup_future_usage")]
    /**Indicates that you intend to make future payments with this PaymentIntent's payment method.

Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete. If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.

When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).*/
    pub setup_future_usage: Option<String>,
}
impl std::fmt::Display for PaymentMethodOptionsKlarna {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodOptionsKonbini {
    #[serde(rename = "confirmation_number")]
    ///An optional 10 to 11 digit numeric-only string determining the confirmation code at applicable convenience stores.
    pub confirmation_number: Option<String>,
    #[serde(rename = "expires_after_days")]
    ///The number of calendar days (between 1 and 60) after which Konbini payment instructions will expire. For example, if a PaymentIntent is confirmed with Konbini and `expires_after_days` set to 2 on Monday JST, the instructions will expire on Wednesday 23:59:59 JST.
    pub expires_after_days: Option<i64>,
    #[serde(rename = "expires_at")]
    ///The timestamp at which the Konbini payment instructions will expire. Only one of `expires_after_days` or `expires_at` may be set.
    pub expires_at: Option<i64>,
    #[serde(rename = "product_description")]
    ///A product descriptor of up to 22 characters, which will appear to customers at the convenience store.
    pub product_description: Option<String>,
    #[serde(rename = "setup_future_usage")]
    /**Indicates that you intend to make future payments with this PaymentIntent's payment method.

Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete. If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.

When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).*/
    pub setup_future_usage: Option<String>,
}
impl std::fmt::Display for PaymentMethodOptionsKonbini {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodOptionsOxxo {
    #[serde(rename = "expires_after_days")]
    ///The number of calendar days before an OXXO invoice expires. For example, if you create an OXXO invoice on Monday and you set expires_after_days to 2, the OXXO invoice will expire on Wednesday at 23:59 America/Mexico_City time.
    pub expires_after_days: i64,
    #[serde(rename = "setup_future_usage")]
    /**Indicates that you intend to make future payments with this PaymentIntent's payment method.

Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete. If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.

When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).*/
    pub setup_future_usage: Option<String>,
}
impl std::fmt::Display for PaymentMethodOptionsOxxo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodOptionsP24 {
    #[serde(rename = "setup_future_usage")]
    /**Indicates that you intend to make future payments with this PaymentIntent's payment method.

Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete. If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.

When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).*/
    pub setup_future_usage: Option<String>,
}
impl std::fmt::Display for PaymentMethodOptionsP24 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodOptionsPaynow {
    #[serde(rename = "setup_future_usage")]
    /**Indicates that you intend to make future payments with this PaymentIntent's payment method.

Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete. If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.

When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).*/
    pub setup_future_usage: Option<String>,
}
impl std::fmt::Display for PaymentMethodOptionsPaynow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodOptionsPaypal {
    #[serde(rename = "capture_method")]
    ///Controls when the funds will be captured from the customer's account.
    pub capture_method: Option<String>,
    #[serde(rename = "preferred_locale")]
    ///Preferred locale of the PayPal checkout page that the customer is redirected to.
    pub preferred_locale: Option<String>,
}
impl std::fmt::Display for PaymentMethodOptionsPaypal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodOptionsPromptpay {
    #[serde(rename = "setup_future_usage")]
    /**Indicates that you intend to make future payments with this PaymentIntent's payment method.

Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete. If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.

When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).*/
    pub setup_future_usage: Option<String>,
}
impl std::fmt::Display for PaymentMethodOptionsPromptpay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodOptionsSofort {
    #[serde(rename = "preferred_language")]
    ///Preferred language of the SOFORT authorization page that the customer is redirected to.
    pub preferred_language: Option<String>,
    #[serde(rename = "setup_future_usage")]
    /**Indicates that you intend to make future payments with this PaymentIntent's payment method.

Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete. If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.

When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).*/
    pub setup_future_usage: Option<String>,
}
impl std::fmt::Display for PaymentMethodOptionsSofort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodOptionsWechatPay {
    #[serde(rename = "app_id")]
    ///The app ID registered with WeChat Pay. Only required when client is ios or android.
    pub app_id: Option<String>,
    #[serde(rename = "client")]
    ///The client type that the end customer will pay from
    pub client: Option<String>,
    #[serde(rename = "setup_future_usage")]
    /**Indicates that you intend to make future payments with this PaymentIntent's payment method.

Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete. If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.

When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).*/
    pub setup_future_usage: Option<String>,
}
impl std::fmt::Display for PaymentMethodOptionsWechatPay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodOxxo {}
impl std::fmt::Display for PaymentMethodOxxo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodP24 {
    #[serde(rename = "bank")]
    ///The customer's bank, if provided.
    pub bank: Option<String>,
}
impl std::fmt::Display for PaymentMethodP24 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodPaynow {}
impl std::fmt::Display for PaymentMethodPaynow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodPromptpay {}
impl std::fmt::Display for PaymentMethodPromptpay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodSepaDebit {
    #[serde(rename = "bank_code")]
    ///Bank code of bank associated with the bank account.
    pub bank_code: Option<String>,
    #[serde(rename = "branch_code")]
    ///Branch code of bank associated with the bank account.
    pub branch_code: Option<String>,
    #[serde(rename = "country")]
    ///Two-letter ISO code representing the country the bank account is located in.
    pub country: Option<String>,
    #[serde(rename = "fingerprint")]
    ///Uniquely identifies this particular bank account. You can use this attribute to check whether two bank accounts are the same.
    pub fingerprint: Option<String>,
    #[serde(rename = "generated_from")]
    ///Information about the object that generated this PaymentMethod.
    pub generated_from: Option<serde_json::Value>,
    #[serde(rename = "last4")]
    ///Last four characters of the IBAN.
    pub last4: Option<String>,
}
impl std::fmt::Display for PaymentMethodSepaDebit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodSofort {
    #[serde(rename = "country")]
    ///Two-letter ISO code representing the country the bank account is located in.
    pub country: Option<String>,
}
impl std::fmt::Display for PaymentMethodSofort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodUsBankAccount {
    #[serde(rename = "account_holder_type")]
    ///Account holder type: individual or company.
    pub account_holder_type: Option<String>,
    #[serde(rename = "account_type")]
    ///Account type: checkings or savings. Defaults to checking if omitted.
    pub account_type: Option<String>,
    #[serde(rename = "bank_name")]
    ///The name of the bank.
    pub bank_name: Option<String>,
    #[serde(rename = "financial_connections_account")]
    ///The ID of the Financial Connections Account used to create the payment method.
    pub financial_connections_account: Option<String>,
    #[serde(rename = "fingerprint")]
    ///Uniquely identifies this particular bank account. You can use this attribute to check whether two bank accounts are the same.
    pub fingerprint: Option<String>,
    #[serde(rename = "last4")]
    ///Last four digits of the bank account number.
    pub last4: Option<String>,
    #[serde(rename = "networks")]
    ///Contains information about US bank account networks that can be used.
    pub networks: Option<serde_json::Value>,
    #[serde(rename = "routing_number")]
    ///Routing number of the bank account.
    pub routing_number: Option<String>,
}
impl std::fmt::Display for PaymentMethodUsBankAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodWechatPay {}
impl std::fmt::Display for PaymentMethodWechatPay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentPagesCheckoutSessionAfterExpiration {
    #[serde(rename = "recovery")]
    ///When set, configuration used to recover the Checkout Session on expiry.
    pub recovery: Option<serde_json::Value>,
}
impl std::fmt::Display for PaymentPagesCheckoutSessionAfterExpiration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentPagesCheckoutSessionAfterExpirationRecovery {
    #[serde(rename = "allow_promotion_codes")]
    ///Enables user redeemable promotion codes on the recovered Checkout Sessions. Defaults to `false`
    pub allow_promotion_codes: bool,
    #[serde(rename = "enabled")]
    /**If `true`, a recovery url will be generated to recover this Checkout Session if it
expires before a transaction is completed. It will be attached to the
Checkout Session object upon expiration.*/
    pub enabled: bool,
    #[serde(rename = "expires_at")]
    ///The timestamp at which the recovery URL will expire.
    pub expires_at: Option<i64>,
    #[serde(rename = "url")]
    ///URL that creates a new Checkout Session when clicked that is a copy of this expired Checkout Session
    pub url: Option<String>,
}
impl std::fmt::Display for PaymentPagesCheckoutSessionAfterExpirationRecovery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentPagesCheckoutSessionAutomaticTax {
    #[serde(rename = "enabled")]
    ///Indicates whether automatic tax is enabled for the session
    pub enabled: bool,
    #[serde(rename = "status")]
    ///The status of the most recent automated tax calculation for this session.
    pub status: Option<String>,
}
impl std::fmt::Display for PaymentPagesCheckoutSessionAutomaticTax {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentPagesCheckoutSessionConsent {
    #[serde(rename = "promotions")]
    /**If `opt_in`, the customer consents to receiving promotional communications
from the merchant about this Checkout Session.*/
    pub promotions: Option<String>,
}
impl std::fmt::Display for PaymentPagesCheckoutSessionConsent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentPagesCheckoutSessionConsentCollection {
    #[serde(rename = "promotions")]
    /**If set to `auto`, enables the collection of customer consent for promotional communications. The Checkout
Session will determine whether to display an option to opt into promotional communication
from the merchant depending on the customer's locale. Only available to US merchants.*/
    pub promotions: Option<String>,
}
impl std::fmt::Display for PaymentPagesCheckoutSessionConsentCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentPagesCheckoutSessionCustomerDetails {
    #[serde(rename = "address")]
    ///The customer's address after a completed Checkout Session. Note: This property is populated only for sessions on or after March 30, 2022.
    pub address: Option<serde_json::Value>,
    #[serde(rename = "email")]
    /**The email associated with the Customer, if one exists, on the Checkout Session after a completed Checkout Session or at time of session expiry.
Otherwise, if the customer has consented to promotional content, this value is the most recent valid email provided by the customer on the Checkout form.*/
    pub email: Option<String>,
    #[serde(rename = "name")]
    ///The customer's name after a completed Checkout Session. Note: This property is populated only for sessions on or after March 30, 2022.
    pub name: Option<String>,
    #[serde(rename = "phone")]
    ///The customer's phone number after a completed Checkout Session.
    pub phone: Option<String>,
    #[serde(rename = "tax_exempt")]
    ///The customer’s tax exempt status after a completed Checkout Session.
    pub tax_exempt: Option<String>,
    #[serde(rename = "tax_ids")]
    ///The customer’s tax IDs after a completed Checkout Session.
    pub tax_ids: Option<Vec<PaymentPagesCheckoutSessionTaxId>>,
}
impl std::fmt::Display for PaymentPagesCheckoutSessionCustomerDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentPagesCheckoutSessionPhoneNumberCollection {
    #[serde(rename = "enabled")]
    ///Indicates whether phone number collection is enabled for the session
    pub enabled: bool,
}
impl std::fmt::Display for PaymentPagesCheckoutSessionPhoneNumberCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentPagesCheckoutSessionShippingAddressCollection {
    #[serde(rename = "allowed_countries")]
    /**An array of two-letter ISO country codes representing which countries Checkout should provide as options for
shipping locations. Unsupported country codes: `AS, CX, CC, CU, HM, IR, KP, MH, FM, NF, MP, PW, SD, SY, UM, VI`.*/
    pub allowed_countries: Vec<String>,
}
impl std::fmt::Display for PaymentPagesCheckoutSessionShippingAddressCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentPagesCheckoutSessionShippingCost {
    #[serde(rename = "amount_subtotal")]
    ///Total shipping cost before any discounts or taxes are applied.
    pub amount_subtotal: i64,
    #[serde(rename = "amount_tax")]
    ///Total tax amount applied due to shipping costs. If no tax was applied, defaults to 0.
    pub amount_tax: i64,
    #[serde(rename = "amount_total")]
    ///Total shipping cost after discounts and taxes are applied.
    pub amount_total: i64,
    #[serde(rename = "shipping_rate")]
    ///The ID of the ShippingRate for this order.
    pub shipping_rate: Option<serde_json::Value>,
    #[serde(rename = "taxes")]
    ///The taxes applied to the shipping rate.
    pub taxes: Option<Vec<LineItemsTaxAmount>>,
}
impl std::fmt::Display for PaymentPagesCheckoutSessionShippingCost {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentPagesCheckoutSessionShippingOption {
    #[serde(rename = "shipping_amount")]
    ///A non-negative integer in cents representing how much to charge.
    pub shipping_amount: i64,
    #[serde(rename = "shipping_rate")]
    ///The shipping rate.
    pub shipping_rate: serde_json::Value,
}
impl std::fmt::Display for PaymentPagesCheckoutSessionShippingOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentPagesCheckoutSessionTaxId {
    #[serde(rename = "type")]
    ///The type of the tax ID, one of `eu_vat`, `br_cnpj`, `br_cpf`, `eu_oss_vat`, `gb_vat`, `nz_gst`, `au_abn`, `au_arn`, `in_gst`, `no_vat`, `za_vat`, `ch_vat`, `mx_rfc`, `sg_uen`, `ru_inn`, `ru_kpp`, `ca_bn`, `hk_br`, `es_cif`, `tw_vat`, `th_vat`, `jp_cn`, `jp_rn`, `li_uid`, `my_itn`, `us_ein`, `kr_brn`, `ca_qst`, `ca_gst_hst`, `ca_pst_bc`, `ca_pst_mb`, `ca_pst_sk`, `my_sst`, `sg_gst`, `ae_trn`, `cl_tin`, `sa_vat`, `id_npwp`, `my_frp`, `il_vat`, `ge_vat`, `ua_vat`, `is_vat`, `bg_uic`, `hu_tin`, `si_tin`, or `unknown`
    pub type_: String,
    #[serde(rename = "value")]
    ///The value of the tax ID.
    pub value: Option<String>,
}
impl std::fmt::Display for PaymentPagesCheckoutSessionTaxId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentPagesCheckoutSessionTaxIdCollection {
    #[serde(rename = "enabled")]
    ///Indicates whether tax ID collection is enabled for the session
    pub enabled: bool,
}
impl std::fmt::Display for PaymentPagesCheckoutSessionTaxIdCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentPagesCheckoutSessionTotalDetails {
    #[serde(rename = "amount_discount")]
    ///This is the sum of all the discounts.
    pub amount_discount: i64,
    #[serde(rename = "amount_shipping")]
    ///This is the sum of all the shipping amounts.
    pub amount_shipping: Option<i64>,
    #[serde(rename = "amount_tax")]
    ///This is the sum of all the tax amounts.
    pub amount_tax: i64,
    #[serde(rename = "breakdown")]
    ///
    pub breakdown: Option<PaymentPagesCheckoutSessionTotalDetailsResourceBreakdown>,
}
impl std::fmt::Display for PaymentPagesCheckoutSessionTotalDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentPagesCheckoutSessionTotalDetailsResourceBreakdown {
    #[serde(rename = "discounts")]
    ///The aggregated discounts.
    pub discounts: Vec<LineItemsDiscountAmount>,
    #[serde(rename = "taxes")]
    ///The aggregated tax amounts by rate.
    pub taxes: Vec<LineItemsTaxAmount>,
}
impl std::fmt::Display for PaymentPagesCheckoutSessionTotalDetailsResourceBreakdown {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentSource(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct Payout {
    #[serde(rename = "amount")]
    ///Amount (in %s) to be transferred to your bank account or debit card.
    pub amount: i64,
    #[serde(rename = "arrival_date")]
    ///Date the payout is expected to arrive in the bank. This factors in delays like weekends or bank holidays.
    pub arrival_date: i64,
    #[serde(rename = "automatic")]
    ///Returns `true` if the payout was created by an [automated payout schedule](https://stripe.com/docs/payouts#payout-schedule), and `false` if it was [requested manually](https://stripe.com/docs/payouts#manual-payouts).
    pub automatic: bool,
    #[serde(rename = "balance_transaction")]
    ///ID of the balance transaction that describes the impact of this payout on your account balance.
    pub balance_transaction: Option<serde_json::Value>,
    #[serde(rename = "created")]
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    #[serde(rename = "currency")]
    ///Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: String,
    #[serde(rename = "description")]
    ///An arbitrary string attached to the object. Often useful for displaying to users.
    pub description: Option<String>,
    #[serde(rename = "destination")]
    ///ID of the bank account or card the payout was sent to.
    pub destination: Option<serde_json::Value>,
    #[serde(rename = "failure_balance_transaction")]
    ///If the payout failed or was canceled, this will be the ID of the balance transaction that reversed the initial balance transaction, and puts the funds from the failed payout back in your balance.
    pub failure_balance_transaction: Option<serde_json::Value>,
    #[serde(rename = "failure_code")]
    ///Error code explaining reason for payout failure if available. See [Types of payout failures](https://stripe.com/docs/api#payout_failures) for a list of failure codes.
    pub failure_code: Option<String>,
    #[serde(rename = "failure_message")]
    ///Message to user further explaining reason for payout failure if available.
    pub failure_message: Option<String>,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "livemode")]
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    #[serde(rename = "metadata")]
    ///Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<serde_json::Value>,
    #[serde(rename = "method")]
    ///The method used to send this payout, which can be `standard` or `instant`. `instant` is only supported for payouts to debit cards. (See [Instant payouts for marketplaces](https://stripe.com/blog/instant-payouts-for-marketplaces) for more information.)
    pub method: String,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "original_payout")]
    ///If the payout reverses another, this is the ID of the original payout.
    pub original_payout: Option<serde_json::Value>,
    #[serde(rename = "reversed_by")]
    ///If the payout was reversed, this is the ID of the payout that reverses this payout.
    pub reversed_by: Option<serde_json::Value>,
    #[serde(rename = "source_type")]
    ///The source balance this payout came from. One of `card`, `fpx`, or `bank_account`.
    pub source_type: String,
    #[serde(rename = "statement_descriptor")]
    ///Extra information about a payout to be displayed on the user's bank statement.
    pub statement_descriptor: Option<String>,
    #[serde(rename = "status")]
    ///Current status of the payout: `paid`, `pending`, `in_transit`, `canceled` or `failed`. A payout is `pending` until it is submitted to the bank, when it becomes `in_transit`. The status then changes to `paid` if the transaction goes through, or to `failed` or `canceled` (within 5 business days). Some failed payouts may initially show as `paid` but then change to `failed`.
    pub status: String,
    #[serde(rename = "type")]
    ///Can be `bank_account` or `card`.
    pub type_: String,
}
impl std::fmt::Display for Payout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Period {
    #[serde(rename = "end")]
    ///The end date of this usage period. All usage up to and including this point in time is included.
    pub end: Option<i64>,
    #[serde(rename = "start")]
    ///The start date of this usage period. All usage after this point in time is included.
    pub start: Option<i64>,
}
impl std::fmt::Display for Period {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Person {
    #[serde(rename = "account")]
    ///The account the person is associated with.
    pub account: String,
    #[serde(rename = "address")]
    ///
    pub address: Option<Address>,
    #[serde(rename = "address_kana")]
    pub address_kana: Option<serde_json::Value>,
    #[serde(rename = "address_kanji")]
    pub address_kanji: Option<serde_json::Value>,
    #[serde(rename = "created")]
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    #[serde(rename = "dob")]
    ///
    pub dob: Option<LegalEntityDob>,
    #[serde(rename = "email")]
    ///The person's email address.
    pub email: Option<String>,
    #[serde(rename = "first_name")]
    ///The person's first name.
    pub first_name: Option<String>,
    #[serde(rename = "first_name_kana")]
    ///The Kana variation of the person's first name (Japan only).
    pub first_name_kana: Option<String>,
    #[serde(rename = "first_name_kanji")]
    ///The Kanji variation of the person's first name (Japan only).
    pub first_name_kanji: Option<String>,
    #[serde(rename = "full_name_aliases")]
    ///A list of alternate names or aliases that the person is known by.
    pub full_name_aliases: Option<Vec<String>>,
    #[serde(rename = "future_requirements")]
    pub future_requirements: Option<serde_json::Value>,
    #[serde(rename = "gender")]
    ///The person's gender (International regulations require either "male" or "female").
    pub gender: Option<String>,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "id_number_provided")]
    ///Whether the person's `id_number` was provided.
    pub id_number_provided: Option<bool>,
    #[serde(rename = "id_number_secondary_provided")]
    ///Whether the person's `id_number_secondary` was provided.
    pub id_number_secondary_provided: Option<bool>,
    #[serde(rename = "last_name")]
    ///The person's last name.
    pub last_name: Option<String>,
    #[serde(rename = "last_name_kana")]
    ///The Kana variation of the person's last name (Japan only).
    pub last_name_kana: Option<String>,
    #[serde(rename = "last_name_kanji")]
    ///The Kanji variation of the person's last name (Japan only).
    pub last_name_kanji: Option<String>,
    #[serde(rename = "maiden_name")]
    ///The person's maiden name.
    pub maiden_name: Option<String>,
    #[serde(rename = "metadata")]
    ///Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<serde_json::Value>,
    #[serde(rename = "nationality")]
    ///The country where the person is a national.
    pub nationality: Option<String>,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "phone")]
    ///The person's phone number.
    pub phone: Option<String>,
    #[serde(rename = "political_exposure")]
    ///Indicates if the person or any of their representatives, family members, or other closely related persons, declares that they hold or have held an important public job or function, in any jurisdiction.
    pub political_exposure: Option<String>,
    #[serde(rename = "registered_address")]
    ///
    pub registered_address: Option<Address>,
    #[serde(rename = "relationship")]
    ///
    pub relationship: Option<PersonRelationship>,
    #[serde(rename = "requirements")]
    pub requirements: Option<serde_json::Value>,
    #[serde(rename = "ssn_last_4_provided")]
    ///Whether the last four digits of the person's Social Security number have been provided (U.S. only).
    pub ssn_last4_provided: Option<bool>,
    #[serde(rename = "verification")]
    ///
    pub verification: Option<LegalEntityPersonVerification>,
}
impl std::fmt::Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PersonFutureRequirements {
    #[serde(rename = "alternatives")]
    ///Fields that are due and can be satisfied by providing the corresponding alternative fields instead.
    pub alternatives: Option<Vec<AccountRequirementsAlternative>>,
    #[serde(rename = "currently_due")]
    ///Fields that need to be collected to keep the person's account enabled. If not collected by the account's `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash, and may immediately become `past_due`, but the account may also be given a grace period depending on the account's enablement state prior to transition.
    pub currently_due: Vec<String>,
    #[serde(rename = "errors")]
    ///Fields that are `currently_due` and need to be collected again because validation or verification failed.
    pub errors: Vec<AccountRequirementsError>,
    #[serde(rename = "eventually_due")]
    ///Fields that need to be collected assuming all volume thresholds are reached. As they become required, they appear in `currently_due` as well, and the account's `future_requirements[current_deadline]` becomes set.
    pub eventually_due: Vec<String>,
    #[serde(rename = "past_due")]
    ///Fields that weren't collected by the account's `requirements.current_deadline`. These fields need to be collected to enable the person's account. New fields will never appear here; `future_requirements.past_due` will always be a subset of `requirements.past_due`.
    pub past_due: Vec<String>,
    #[serde(rename = "pending_verification")]
    ///Fields that may become required depending on the results of verification or review. Will be an empty array unless an asynchronous verification is pending. If verification fails, these fields move to `eventually_due` or `currently_due`.
    pub pending_verification: Vec<String>,
}
impl std::fmt::Display for PersonFutureRequirements {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PersonRelationship {
    #[serde(rename = "director")]
    ///Whether the person is a director of the account's legal entity. Directors are typically members of the governing board of the company, or responsible for ensuring the company meets its regulatory obligations.
    pub director: Option<bool>,
    #[serde(rename = "executive")]
    ///Whether the person has significant responsibility to control, manage, or direct the organization.
    pub executive: Option<bool>,
    #[serde(rename = "owner")]
    ///Whether the person is an owner of the account’s legal entity.
    pub owner: Option<bool>,
    #[serde(rename = "percent_ownership")]
    ///The percent owned by the person of the account's legal entity.
    pub percent_ownership: Option<f64>,
    #[serde(rename = "representative")]
    ///Whether the person is authorized as the primary representative of the account. This is the person nominated by the business to provide information about themselves, and general information about the account. There can only be one representative at any given time. At the time the account is created, this person should be set to the person responsible for opening the account.
    pub representative: Option<bool>,
    #[serde(rename = "title")]
    ///The person's title (e.g., CEO, Support Engineer).
    pub title: Option<String>,
}
impl std::fmt::Display for PersonRelationship {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PersonRequirements {
    #[serde(rename = "alternatives")]
    ///Fields that are due and can be satisfied by providing the corresponding alternative fields instead.
    pub alternatives: Option<Vec<AccountRequirementsAlternative>>,
    #[serde(rename = "currently_due")]
    ///Fields that need to be collected to keep the person's account enabled. If not collected by the account's `current_deadline`, these fields appear in `past_due` as well, and the account is disabled.
    pub currently_due: Vec<String>,
    #[serde(rename = "errors")]
    ///Fields that are `currently_due` and need to be collected again because validation or verification failed.
    pub errors: Vec<AccountRequirementsError>,
    #[serde(rename = "eventually_due")]
    ///Fields that need to be collected assuming all volume thresholds are reached. As they become required, they appear in `currently_due` as well, and the account's `current_deadline` becomes set.
    pub eventually_due: Vec<String>,
    #[serde(rename = "past_due")]
    ///Fields that weren't collected by the account's `current_deadline`. These fields need to be collected to enable the person's account.
    pub past_due: Vec<String>,
    #[serde(rename = "pending_verification")]
    ///Fields that may become required depending on the results of verification or review. Will be an empty array unless an asynchronous verification is pending. If verification fails, these fields move to `eventually_due`, `currently_due`, or `past_due`.
    pub pending_verification: Vec<String>,
}
impl std::fmt::Display for PersonRequirements {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Plan {
    #[serde(rename = "active")]
    ///Whether the plan can be used for new purchases.
    pub active: bool,
    #[serde(rename = "aggregate_usage")]
    ///Specifies a usage aggregation strategy for plans of `usage_type=metered`. Allowed values are `sum` for summing up all usage during a period, `last_during_period` for using the last usage record reported within a period, `last_ever` for using the last usage record ever (across period bounds) or `max` which uses the usage record with the maximum reported usage during a period. Defaults to `sum`.
    pub aggregate_usage: Option<String>,
    #[serde(rename = "amount")]
    ///The unit amount in %s to be charged, represented as a whole integer if possible. Only set if `billing_scheme=per_unit`.
    pub amount: Option<i64>,
    #[serde(rename = "amount_decimal")]
    ///The unit amount in %s to be charged, represented as a decimal string with at most 12 decimal places. Only set if `billing_scheme=per_unit`.
    pub amount_decimal: Option<String>,
    #[serde(rename = "billing_scheme")]
    ///Describes how to compute the price per period. Either `per_unit` or `tiered`. `per_unit` indicates that the fixed amount (specified in `amount`) will be charged per unit in `quantity` (for plans with `usage_type=licensed`), or per unit of total usage (for plans with `usage_type=metered`). `tiered` indicates that the unit pricing will be computed using a tiering strategy as defined using the `tiers` and `tiers_mode` attributes.
    pub billing_scheme: String,
    #[serde(rename = "created")]
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    #[serde(rename = "currency")]
    ///Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: String,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "interval")]
    ///The frequency at which a subscription is billed. One of `day`, `week`, `month` or `year`.
    pub interval: String,
    #[serde(rename = "interval_count")]
    ///The number of intervals (specified in the `interval` attribute) between subscription billings. For example, `interval=month` and `interval_count=3` bills every 3 months.
    pub interval_count: i64,
    #[serde(rename = "livemode")]
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    #[serde(rename = "metadata")]
    ///Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<serde_json::Value>,
    #[serde(rename = "nickname")]
    ///A brief description of the plan, hidden from customers.
    pub nickname: Option<String>,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "product")]
    ///The product whose pricing this plan determines.
    pub product: Option<serde_json::Value>,
    #[serde(rename = "tiers")]
    ///Each element represents a pricing tier. This parameter requires `billing_scheme` to be set to `tiered`. See also the documentation for `billing_scheme`.
    pub tiers: Option<Vec<PlanTier>>,
    #[serde(rename = "tiers_mode")]
    ///Defines if the tiering price should be `graduated` or `volume` based. In `volume`-based tiering, the maximum quantity within a period determines the per unit price. In `graduated` tiering, pricing can change as the quantity grows.
    pub tiers_mode: Option<String>,
    #[serde(rename = "transform_usage")]
    ///Apply a transformation to the reported usage or set quantity before computing the amount billed. Cannot be combined with `tiers`.
    pub transform_usage: Option<serde_json::Value>,
    #[serde(rename = "trial_period_days")]
    ///Default number of trial days when subscribing a customer to this plan using [`trial_from_plan=true`](https://stripe.com/docs/api#create_subscription-trial_from_plan).
    pub trial_period_days: Option<i64>,
    #[serde(rename = "usage_type")]
    ///Configures how the quantity per period should be determined. Can be either `metered` or `licensed`. `licensed` automatically bills the `quantity` set when adding it to a subscription. `metered` aggregates the total usage based on usage records. Defaults to `licensed`.
    pub usage_type: String,
}
impl std::fmt::Display for Plan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PlanTier {
    #[serde(rename = "flat_amount")]
    ///Price for the entire tier.
    pub flat_amount: Option<i64>,
    #[serde(rename = "flat_amount_decimal")]
    ///Same as `flat_amount`, but contains a decimal value with at most 12 decimal places.
    pub flat_amount_decimal: Option<String>,
    #[serde(rename = "unit_amount")]
    ///Per unit price for units relevant to the tier.
    pub unit_amount: Option<i64>,
    #[serde(rename = "unit_amount_decimal")]
    ///Same as `unit_amount`, but contains a decimal value with at most 12 decimal places.
    pub unit_amount_decimal: Option<String>,
    #[serde(rename = "up_to")]
    ///Up to and including to this quantity will be contained in the tier.
    pub up_to: Option<i64>,
}
impl std::fmt::Display for PlanTier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PlatformTaxFee {
    #[serde(rename = "account")]
    ///The Connected account that incurred this charge.
    pub account: String,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "source_transaction")]
    ///The payment object that caused this tax to be inflicted.
    pub source_transaction: String,
    #[serde(rename = "type")]
    ///The type of tax (VAT).
    pub type_: String,
}
impl std::fmt::Display for PlatformTaxFee {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PortalBusinessProfile {
    #[serde(rename = "headline")]
    ///The messaging shown to customers in the portal.
    pub headline: Option<String>,
    #[serde(rename = "privacy_policy_url")]
    ///A link to the business’s publicly available privacy policy.
    pub privacy_policy_url: Option<String>,
    #[serde(rename = "terms_of_service_url")]
    ///A link to the business’s publicly available terms of service.
    pub terms_of_service_url: Option<String>,
}
impl std::fmt::Display for PortalBusinessProfile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PortalCustomerUpdate {
    #[serde(rename = "allowed_updates")]
    ///The types of customer updates that are supported. When empty, customers are not updateable.
    pub allowed_updates: Vec<String>,
    #[serde(rename = "enabled")]
    ///Whether the feature is enabled.
    pub enabled: bool,
}
impl std::fmt::Display for PortalCustomerUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PortalFeatures {
    #[serde(rename = "customer_update")]
    ///
    pub customer_update: PortalCustomerUpdate,
    #[serde(rename = "invoice_history")]
    ///
    pub invoice_history: PortalInvoiceList,
    #[serde(rename = "payment_method_update")]
    ///
    pub payment_method_update: PortalPaymentMethodUpdate,
    #[serde(rename = "subscription_cancel")]
    ///
    pub subscription_cancel: PortalSubscriptionCancel,
    #[serde(rename = "subscription_pause")]
    ///
    pub subscription_pause: PortalSubscriptionPause,
    #[serde(rename = "subscription_update")]
    ///
    pub subscription_update: PortalSubscriptionUpdate,
}
impl std::fmt::Display for PortalFeatures {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PortalInvoiceList {
    #[serde(rename = "enabled")]
    ///Whether the feature is enabled.
    pub enabled: bool,
}
impl std::fmt::Display for PortalInvoiceList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PortalPaymentMethodUpdate {
    #[serde(rename = "enabled")]
    ///Whether the feature is enabled.
    pub enabled: bool,
}
impl std::fmt::Display for PortalPaymentMethodUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PortalSubscriptionCancel {
    #[serde(rename = "cancellation_reason")]
    ///
    pub cancellation_reason: PortalSubscriptionCancellationReason,
    #[serde(rename = "enabled")]
    ///Whether the feature is enabled.
    pub enabled: bool,
    #[serde(rename = "mode")]
    ///Whether to cancel subscriptions immediately or at the end of the billing period.
    pub mode: String,
    #[serde(rename = "proration_behavior")]
    ///Whether to create prorations when canceling subscriptions. Possible values are `none` and `create_prorations`.
    pub proration_behavior: String,
}
impl std::fmt::Display for PortalSubscriptionCancel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PortalSubscriptionCancellationReason {
    #[serde(rename = "enabled")]
    ///Whether the feature is enabled.
    pub enabled: bool,
    #[serde(rename = "options")]
    ///Which cancellation reasons will be given as options to the customer.
    pub options: Vec<String>,
}
impl std::fmt::Display for PortalSubscriptionCancellationReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PortalSubscriptionPause {
    #[serde(rename = "enabled")]
    ///Whether the feature is enabled.
    pub enabled: bool,
}
impl std::fmt::Display for PortalSubscriptionPause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PortalSubscriptionUpdate {
    #[serde(rename = "default_allowed_updates")]
    ///The types of subscription updates that are supported for items listed in the `products` attribute. When empty, subscriptions are not updateable.
    pub default_allowed_updates: Vec<String>,
    #[serde(rename = "enabled")]
    ///Whether the feature is enabled.
    pub enabled: bool,
    #[serde(rename = "products")]
    ///The list of products that support subscription updates.
    pub products: Option<Vec<PortalSubscriptionUpdateProduct>>,
    #[serde(rename = "proration_behavior")]
    ///Determines how to handle prorations resulting from subscription updates. Valid values are `none`, `create_prorations`, and `always_invoice`.
    pub proration_behavior: String,
}
impl std::fmt::Display for PortalSubscriptionUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PortalSubscriptionUpdateProduct {
    #[serde(rename = "prices")]
    ///The list of price IDs which, when subscribed to, a subscription can be updated.
    pub prices: Vec<String>,
    #[serde(rename = "product")]
    ///The product ID.
    pub product: String,
}
impl std::fmt::Display for PortalSubscriptionUpdateProduct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Price {
    #[serde(rename = "active")]
    ///Whether the price can be used for new purchases.
    pub active: bool,
    #[serde(rename = "billing_scheme")]
    ///Describes how to compute the price per period. Either `per_unit` or `tiered`. `per_unit` indicates that the fixed amount (specified in `unit_amount` or `unit_amount_decimal`) will be charged per unit in `quantity` (for prices with `usage_type=licensed`), or per unit of total usage (for prices with `usage_type=metered`). `tiered` indicates that the unit pricing will be computed using a tiering strategy as defined using the `tiers` and `tiers_mode` attributes.
    pub billing_scheme: String,
    #[serde(rename = "created")]
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    #[serde(rename = "currency")]
    ///Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: String,
    #[serde(rename = "currency_options")]
    ///Prices defined in each available currency option. Each key must be a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html) and a [supported currency](https://stripe.com/docs/currencies).
    pub currency_options: Option<serde_json::Value>,
    #[serde(rename = "custom_unit_amount")]
    ///When set, provides configuration for the amount to be adjusted by the customer during Checkout Sessions and Payment Links.
    pub custom_unit_amount: Option<serde_json::Value>,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "livemode")]
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    #[serde(rename = "lookup_key")]
    ///A lookup key used to retrieve prices dynamically from a static string. This may be up to 200 characters.
    pub lookup_key: Option<String>,
    #[serde(rename = "metadata")]
    ///Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format.
    pub metadata: serde_json::Value,
    #[serde(rename = "nickname")]
    ///A brief description of the price, hidden from customers.
    pub nickname: Option<String>,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "product")]
    ///The ID of the product this price is associated with.
    pub product: serde_json::Value,
    #[serde(rename = "recurring")]
    ///The recurring components of a price such as `interval` and `usage_type`.
    pub recurring: Option<serde_json::Value>,
    #[serde(rename = "tax_behavior")]
    ///Specifies whether the price is considered inclusive of taxes or exclusive of taxes. One of `inclusive`, `exclusive`, or `unspecified`. Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    pub tax_behavior: Option<String>,
    #[serde(rename = "tiers")]
    ///Each element represents a pricing tier. This parameter requires `billing_scheme` to be set to `tiered`. See also the documentation for `billing_scheme`.
    pub tiers: Option<Vec<PriceTier>>,
    #[serde(rename = "tiers_mode")]
    ///Defines if the tiering price should be `graduated` or `volume` based. In `volume`-based tiering, the maximum quantity within a period determines the per unit price. In `graduated` tiering, pricing can change as the quantity grows.
    pub tiers_mode: Option<String>,
    #[serde(rename = "transform_quantity")]
    ///Apply a transformation to the reported usage or set quantity before computing the amount billed. Cannot be combined with `tiers`.
    pub transform_quantity: Option<serde_json::Value>,
    #[serde(rename = "type")]
    ///One of `one_time` or `recurring` depending on whether the price is for a one-time purchase or a recurring (subscription) purchase.
    pub type_: String,
    #[serde(rename = "unit_amount")]
    ///The unit amount in %s to be charged, represented as a whole integer if possible. Only set if `billing_scheme=per_unit`.
    pub unit_amount: Option<i64>,
    #[serde(rename = "unit_amount_decimal")]
    ///The unit amount in %s to be charged, represented as a decimal string with at most 12 decimal places. Only set if `billing_scheme=per_unit`.
    pub unit_amount_decimal: Option<String>,
}
impl std::fmt::Display for Price {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PriceTier {
    #[serde(rename = "flat_amount")]
    ///Price for the entire tier.
    pub flat_amount: Option<i64>,
    #[serde(rename = "flat_amount_decimal")]
    ///Same as `flat_amount`, but contains a decimal value with at most 12 decimal places.
    pub flat_amount_decimal: Option<String>,
    #[serde(rename = "unit_amount")]
    ///Per unit price for units relevant to the tier.
    pub unit_amount: Option<i64>,
    #[serde(rename = "unit_amount_decimal")]
    ///Same as `unit_amount`, but contains a decimal value with at most 12 decimal places.
    pub unit_amount_decimal: Option<String>,
    #[serde(rename = "up_to")]
    ///Up to and including to this quantity will be contained in the tier.
    pub up_to: Option<i64>,
}
impl std::fmt::Display for PriceTier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    #[serde(rename = "active")]
    ///Whether the product is currently available for purchase.
    pub active: bool,
    #[serde(rename = "created")]
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    #[serde(rename = "default_price")]
    ///The ID of the [Price](https://stripe.com/docs/api/prices) object that is the default price for this product.
    pub default_price: Option<serde_json::Value>,
    #[serde(rename = "description")]
    ///The product's description, meant to be displayable to the customer. Use this field to optionally store a long form explanation of the product being sold for your own rendering purposes.
    pub description: Option<String>,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "images")]
    ///A list of up to 8 URLs of images for this product, meant to be displayable to the customer.
    pub images: Vec<String>,
    #[serde(rename = "livemode")]
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    #[serde(rename = "metadata")]
    ///Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format.
    pub metadata: serde_json::Value,
    #[serde(rename = "name")]
    ///The product's name, meant to be displayable to the customer.
    pub name: String,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "package_dimensions")]
    ///The dimensions of this product for shipping purposes.
    pub package_dimensions: Option<serde_json::Value>,
    #[serde(rename = "shippable")]
    ///Whether this product is shipped (i.e., physical goods).
    pub shippable: Option<bool>,
    #[serde(rename = "statement_descriptor")]
    ///Extra information about a product which will appear on your customer's credit card statement. In the case that multiple products are billed at once, the first statement descriptor will be used.
    pub statement_descriptor: Option<String>,
    #[serde(rename = "tax_code")]
    ///A [tax code](https://stripe.com/docs/tax/tax-categories) ID.
    pub tax_code: Option<serde_json::Value>,
    #[serde(rename = "unit_label")]
    ///A label that represents units of this product in Stripe and on customers’ receipts and invoices. When set, this will be included in associated invoice line item descriptions.
    pub unit_label: Option<String>,
    #[serde(rename = "updated")]
    ///Time at which the object was last updated. Measured in seconds since the Unix epoch.
    pub updated: i64,
    #[serde(rename = "url")]
    ///A URL of a publicly-accessible webpage for this product.
    pub url: Option<String>,
}
impl std::fmt::Display for Product {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PromotionCode {
    #[serde(rename = "active")]
    ///Whether the promotion code is currently active. A promotion code is only active if the coupon is also valid.
    pub active: bool,
    #[serde(rename = "code")]
    ///The customer-facing code. Regardless of case, this code must be unique across all active promotion codes for each customer.
    pub code: String,
    #[serde(rename = "coupon")]
    /**A coupon contains information about a percent-off or amount-off discount you
might want to apply to a customer. Coupons may be applied to [subscriptions](https://stripe.com/docs/api#subscriptions), [invoices](https://stripe.com/docs/api#invoices),
[checkout sessions](https://stripe.com/docs/api/checkout/sessions), [quotes](https://stripe.com/docs/api#quotes), and more. Coupons do not work with conventional one-off [charges](https://stripe.com/docs/api#create_charge) or [payment intents](https://stripe.com/docs/api/payment_intents).*/
    pub coupon: Coupon,
    #[serde(rename = "created")]
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    #[serde(rename = "customer")]
    ///The customer that this promotion code can be used by.
    pub customer: Option<serde_json::Value>,
    #[serde(rename = "expires_at")]
    ///Date at which the promotion code can no longer be redeemed.
    pub expires_at: Option<i64>,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "livemode")]
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    #[serde(rename = "max_redemptions")]
    ///Maximum number of times this promotion code can be redeemed.
    pub max_redemptions: Option<i64>,
    #[serde(rename = "metadata")]
    ///Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<serde_json::Value>,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "restrictions")]
    ///
    pub restrictions: PromotionCodesResourceRestrictions,
    #[serde(rename = "times_redeemed")]
    ///Number of times this promotion code has been used.
    pub times_redeemed: i64,
}
impl std::fmt::Display for PromotionCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PromotionCodeCurrencyOption {
    #[serde(rename = "minimum_amount")]
    ///Minimum amount required to redeem this Promotion Code into a Coupon (e.g., a purchase must be $100 or more to work).
    pub minimum_amount: i64,
}
impl std::fmt::Display for PromotionCodeCurrencyOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PromotionCodesResourceRestrictions {
    #[serde(rename = "currency_options")]
    ///Promotion code restrictions defined in each available currency option. Each key must be a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html) and a [supported currency](https://stripe.com/docs/currencies).
    pub currency_options: Option<serde_json::Value>,
    #[serde(rename = "first_time_transaction")]
    ///A Boolean indicating if the Promotion Code should only be redeemed for Customers without any successful payments or invoices
    pub first_time_transaction: bool,
    #[serde(rename = "minimum_amount")]
    ///Minimum amount required to redeem this Promotion Code into a Coupon (e.g., a purchase must be $100 or more to work).
    pub minimum_amount: Option<i64>,
    #[serde(rename = "minimum_amount_currency")]
    ///Three-letter [ISO code](https://stripe.com/docs/currencies) for minimum_amount
    pub minimum_amount_currency: Option<String>,
}
impl std::fmt::Display for PromotionCodesResourceRestrictions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Quote {
    #[serde(rename = "amount_subtotal")]
    ///Total before any discounts or taxes are applied.
    pub amount_subtotal: i64,
    #[serde(rename = "amount_total")]
    ///Total after discounts and taxes are applied.
    pub amount_total: i64,
    #[serde(rename = "application")]
    ///ID of the Connect Application that created the quote.
    pub application: Option<serde_json::Value>,
    #[serde(rename = "application_fee_amount")]
    ///The amount of the application fee (if any) that will be requested to be applied to the payment and transferred to the application owner's Stripe account. Only applicable if there are no line items with recurring prices on the quote.
    pub application_fee_amount: Option<i64>,
    #[serde(rename = "application_fee_percent")]
    ///A non-negative decimal between 0 and 100, with at most two decimal places. This represents the percentage of the subscription invoice subtotal that will be transferred to the application owner's Stripe account. Only applicable if there are line items with recurring prices on the quote.
    pub application_fee_percent: Option<f64>,
    #[serde(rename = "automatic_tax")]
    ///
    pub automatic_tax: QuotesResourceAutomaticTax,
    #[serde(rename = "collection_method")]
    ///Either `charge_automatically`, or `send_invoice`. When charging automatically, Stripe will attempt to pay invoices at the end of the subscription cycle or on finalization using the default payment method attached to the subscription or customer. When sending an invoice, Stripe will email your customer an invoice with payment instructions. Defaults to `charge_automatically`.
    pub collection_method: String,
    #[serde(rename = "computed")]
    ///
    pub computed: QuotesResourceComputed,
    #[serde(rename = "created")]
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    #[serde(rename = "currency")]
    ///Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Option<String>,
    #[serde(rename = "customer")]
    ///The customer which this quote belongs to. A customer is required before finalizing the quote. Once specified, it cannot be changed.
    pub customer: Option<serde_json::Value>,
    #[serde(rename = "default_tax_rates")]
    ///The tax rates applied to this quote.
    pub default_tax_rates: Option<Vec<serde_json::Value>>,
    #[serde(rename = "description")]
    ///A description that will be displayed on the quote PDF.
    pub description: Option<String>,
    #[serde(rename = "discounts")]
    ///The discounts applied to this quote.
    pub discounts: Vec<serde_json::Value>,
    #[serde(rename = "expires_at")]
    ///The date on which the quote will be canceled if in `open` or `draft` status. Measured in seconds since the Unix epoch.
    pub expires_at: i64,
    #[serde(rename = "footer")]
    ///A footer that will be displayed on the quote PDF.
    pub footer: Option<String>,
    #[serde(rename = "from_quote")]
    ///Details of the quote that was cloned. See the [cloning documentation](https://stripe.com/docs/quotes/clone) for more details.
    pub from_quote: Option<serde_json::Value>,
    #[serde(rename = "header")]
    ///A header that will be displayed on the quote PDF.
    pub header: Option<String>,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "invoice")]
    ///The invoice that was created from this quote.
    pub invoice: Option<serde_json::Value>,
    #[serde(rename = "invoice_settings")]
    ///All invoices will be billed using the specified settings.
    pub invoice_settings: Option<serde_json::Value>,
    #[serde(rename = "line_items")]
    ///A list of items the customer is being quoted for.
    pub line_items: Option<serde_json::Value>,
    #[serde(rename = "livemode")]
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    #[serde(rename = "metadata")]
    ///Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format.
    pub metadata: serde_json::Value,
    #[serde(rename = "number")]
    ///A unique number that identifies this particular quote. This number is assigned once the quote is [finalized](https://stripe.com/docs/quotes/overview#finalize).
    pub number: Option<String>,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "on_behalf_of")]
    ///The account on behalf of which to charge. See the [Connect documentation](https://support.stripe.com/questions/sending-invoices-on-behalf-of-connected-accounts) for details.
    pub on_behalf_of: Option<serde_json::Value>,
    #[serde(rename = "status")]
    ///The status of the quote.
    pub status: String,
    #[serde(rename = "status_transitions")]
    ///
    pub status_transitions: QuotesResourceStatusTransitions,
    #[serde(rename = "subscription")]
    ///The subscription that was created or updated from this quote.
    pub subscription: Option<serde_json::Value>,
    #[serde(rename = "subscription_data")]
    ///
    pub subscription_data: QuotesResourceSubscriptionData,
    #[serde(rename = "subscription_schedule")]
    ///The subscription schedule that was created or updated from this quote.
    pub subscription_schedule: Option<serde_json::Value>,
    #[serde(rename = "test_clock")]
    ///ID of the test clock this quote belongs to.
    pub test_clock: Option<serde_json::Value>,
    #[serde(rename = "total_details")]
    ///
    pub total_details: QuotesResourceTotalDetails,
    #[serde(rename = "transfer_data")]
    ///The account (if any) the payments will be attributed to for tax reporting, and where funds from each payment will be transferred to for each of the invoices.
    pub transfer_data: Option<serde_json::Value>,
}
impl std::fmt::Display for Quote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct QuotesResourceAutomaticTax {
    #[serde(rename = "enabled")]
    ///Automatically calculate taxes
    pub enabled: bool,
    #[serde(rename = "status")]
    ///The status of the most recent automated tax calculation for this quote.
    pub status: Option<String>,
}
impl std::fmt::Display for QuotesResourceAutomaticTax {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct QuotesResourceComputed {
    #[serde(rename = "recurring")]
    ///The definitive totals and line items the customer will be charged on a recurring basis. Takes into account the line items with recurring prices and discounts with `duration=forever` coupons only. Defaults to `null` if no inputted line items with recurring prices.
    pub recurring: Option<serde_json::Value>,
    #[serde(rename = "upfront")]
    ///
    pub upfront: QuotesResourceUpfront,
}
impl std::fmt::Display for QuotesResourceComputed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct QuotesResourceFromQuote {
    #[serde(rename = "is_revision")]
    ///Whether this quote is a revision of a different quote.
    pub is_revision: bool,
    #[serde(rename = "quote")]
    ///The quote that was cloned.
    pub quote: serde_json::Value,
}
impl std::fmt::Display for QuotesResourceFromQuote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct QuotesResourceRecurring {
    #[serde(rename = "amount_subtotal")]
    ///Total before any discounts or taxes are applied.
    pub amount_subtotal: i64,
    #[serde(rename = "amount_total")]
    ///Total after discounts and taxes are applied.
    pub amount_total: i64,
    #[serde(rename = "interval")]
    ///The frequency at which a subscription is billed. One of `day`, `week`, `month` or `year`.
    pub interval: String,
    #[serde(rename = "interval_count")]
    ///The number of intervals (specified in the `interval` attribute) between subscription billings. For example, `interval=month` and `interval_count=3` bills every 3 months.
    pub interval_count: i64,
    #[serde(rename = "total_details")]
    ///
    pub total_details: QuotesResourceTotalDetails,
}
impl std::fmt::Display for QuotesResourceRecurring {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct QuotesResourceStatusTransitions {
    #[serde(rename = "accepted_at")]
    ///The time that the quote was accepted. Measured in seconds since Unix epoch.
    pub accepted_at: Option<i64>,
    #[serde(rename = "canceled_at")]
    ///The time that the quote was canceled. Measured in seconds since Unix epoch.
    pub canceled_at: Option<i64>,
    #[serde(rename = "finalized_at")]
    ///The time that the quote was finalized. Measured in seconds since Unix epoch.
    pub finalized_at: Option<i64>,
}
impl std::fmt::Display for QuotesResourceStatusTransitions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct QuotesResourceSubscriptionData {
    #[serde(rename = "effective_date")]
    ///When creating a new subscription, the date of which the subscription schedule will start after the quote is accepted. This date is ignored if it is in the past when the quote is accepted. Measured in seconds since the Unix epoch.
    pub effective_date: Option<i64>,
    #[serde(rename = "trial_period_days")]
    ///Integer representing the number of trial period days before the customer is charged for the first time.
    pub trial_period_days: Option<i64>,
}
impl std::fmt::Display for QuotesResourceSubscriptionData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct QuotesResourceTotalDetails {
    #[serde(rename = "amount_discount")]
    ///This is the sum of all the discounts.
    pub amount_discount: i64,
    #[serde(rename = "amount_shipping")]
    ///This is the sum of all the shipping amounts.
    pub amount_shipping: Option<i64>,
    #[serde(rename = "amount_tax")]
    ///This is the sum of all the tax amounts.
    pub amount_tax: i64,
    #[serde(rename = "breakdown")]
    ///
    pub breakdown: Option<QuotesResourceTotalDetailsResourceBreakdown>,
}
impl std::fmt::Display for QuotesResourceTotalDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct QuotesResourceTotalDetailsResourceBreakdown {
    #[serde(rename = "discounts")]
    ///The aggregated discounts.
    pub discounts: Vec<LineItemsDiscountAmount>,
    #[serde(rename = "taxes")]
    ///The aggregated tax amounts by rate.
    pub taxes: Vec<LineItemsTaxAmount>,
}
impl std::fmt::Display for QuotesResourceTotalDetailsResourceBreakdown {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct QuotesResourceTransferData {
    #[serde(rename = "amount")]
    ///The amount in %s that will be transferred to the destination account when the invoice is paid. By default, the entire amount is transferred to the destination.
    pub amount: Option<i64>,
    #[serde(rename = "amount_percent")]
    ///A non-negative decimal between 0 and 100, with at most two decimal places. This represents the percentage of the subscription invoice subtotal that will be transferred to the destination account. By default, the entire amount will be transferred to the destination.
    pub amount_percent: Option<f64>,
    #[serde(rename = "destination")]
    ///The account where funds from the payment will be transferred to upon payment success.
    pub destination: serde_json::Value,
}
impl std::fmt::Display for QuotesResourceTransferData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct QuotesResourceUpfront {
    #[serde(rename = "amount_subtotal")]
    ///Total before any discounts or taxes are applied.
    pub amount_subtotal: i64,
    #[serde(rename = "amount_total")]
    ///Total after discounts and taxes are applied.
    pub amount_total: i64,
    #[serde(rename = "line_items")]
    ///The line items that will appear on the next invoice after this quote is accepted. This does not include pending invoice items that exist on the customer but may still be included in the next invoice.
    pub line_items: Option<serde_json::Value>,
    #[serde(rename = "total_details")]
    ///
    pub total_details: QuotesResourceTotalDetails,
}
impl std::fmt::Display for QuotesResourceUpfront {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RadarEarlyFraudWarning {
    #[serde(rename = "actionable")]
    ///An EFW is actionable if it has not received a dispute and has not been fully refunded. You may wish to proactively refund a charge that receives an EFW, in order to avoid receiving a dispute later.
    pub actionable: bool,
    #[serde(rename = "charge")]
    ///ID of the charge this early fraud warning is for, optionally expanded.
    pub charge: serde_json::Value,
    #[serde(rename = "created")]
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    #[serde(rename = "fraud_type")]
    ///The type of fraud labelled by the issuer. One of `card_never_received`, `fraudulent_card_application`, `made_with_counterfeit_card`, `made_with_lost_card`, `made_with_stolen_card`, `misc`, `unauthorized_use_of_card`.
    pub fraud_type: String,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "livemode")]
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "payment_intent")]
    ///ID of the Payment Intent this early fraud warning is for, optionally expanded.
    pub payment_intent: Option<serde_json::Value>,
}
impl std::fmt::Display for RadarEarlyFraudWarning {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RadarValueList {
    #[serde(rename = "alias")]
    ///The name of the value list for use in rules.
    pub alias: String,
    #[serde(rename = "created")]
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    #[serde(rename = "created_by")]
    ///The name or email address of the user who created this value list.
    pub created_by: String,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "item_type")]
    ///The type of items in the value list. One of `card_fingerprint`, `card_bin`, `email`, `ip_address`, `country`, `string`, `case_sensitive_string`, or `customer_id`.
    pub item_type: String,
    #[serde(rename = "list_items")]
    ///List of items contained within this value list.
    pub list_items: serde_json::Value,
    #[serde(rename = "livemode")]
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    #[serde(rename = "metadata")]
    ///Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format.
    pub metadata: serde_json::Value,
    #[serde(rename = "name")]
    ///The name of the value list.
    pub name: String,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
}
impl std::fmt::Display for RadarValueList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RadarValueListItem {
    #[serde(rename = "created")]
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    #[serde(rename = "created_by")]
    ///The name or email address of the user who added this item to the value list.
    pub created_by: String,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "livemode")]
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "value")]
    ///The value of the item.
    pub value: String,
    #[serde(rename = "value_list")]
    ///The identifier of the value list this item belongs to.
    pub value_list: String,
}
impl std::fmt::Display for RadarValueListItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RadarRadarOptions {
    #[serde(rename = "session")]
    ///A [Radar Session](https://stripe.com/docs/radar/radar-session) is a snapshot of the browser metadata and device details that help Radar make more accurate predictions on your payments.
    pub session: Option<String>,
}
impl std::fmt::Display for RadarRadarOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RadarReviewResourceLocation {
    #[serde(rename = "city")]
    ///The city where the payment originated.
    pub city: Option<String>,
    #[serde(rename = "country")]
    ///Two-letter ISO code representing the country where the payment originated.
    pub country: Option<String>,
    #[serde(rename = "latitude")]
    ///The geographic latitude where the payment originated.
    pub latitude: Option<f64>,
    #[serde(rename = "longitude")]
    ///The geographic longitude where the payment originated.
    pub longitude: Option<f64>,
    #[serde(rename = "region")]
    ///The state/county/province/region where the payment originated.
    pub region: Option<String>,
}
impl std::fmt::Display for RadarReviewResourceLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RadarReviewResourceSession {
    #[serde(rename = "browser")]
    ///The browser used in this browser session (e.g., `Chrome`).
    pub browser: Option<String>,
    #[serde(rename = "device")]
    ///Information about the device used for the browser session (e.g., `Samsung SM-G930T`).
    pub device: Option<String>,
    #[serde(rename = "platform")]
    ///The platform for the browser session (e.g., `Macintosh`).
    pub platform: Option<String>,
    #[serde(rename = "version")]
    ///The version for the browser session (e.g., `61.0.3163.100`).
    pub version: Option<String>,
}
impl std::fmt::Display for RadarReviewResourceSession {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ReceivedPaymentMethodDetailsFinancialAccount {
    #[serde(rename = "id")]
    ///The FinancialAccount ID.
    pub id: String,
    #[serde(rename = "network")]
    ///The rails the ReceivedCredit was sent over. A FinancialAccount can only send funds over `stripe`.
    pub network: String,
}
impl std::fmt::Display for ReceivedPaymentMethodDetailsFinancialAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Recipient {
    #[serde(rename = "active_account")]
    ///Hash describing the current account on the recipient, if there is one.
    pub active_account: Option<serde_json::Value>,
    #[serde(rename = "cards")]
    ///
    pub cards: Option<serde_json::Value>,
    #[serde(rename = "created")]
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    #[serde(rename = "default_card")]
    ///The default card to use for creating transfers to this recipient.
    pub default_card: Option<serde_json::Value>,
    #[serde(rename = "description")]
    ///An arbitrary string attached to the object. Often useful for displaying to users.
    pub description: Option<String>,
    #[serde(rename = "email")]
    pub email: Option<String>,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "livemode")]
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    #[serde(rename = "metadata")]
    ///Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format.
    pub metadata: serde_json::Value,
    #[serde(rename = "migrated_to")]
    ///The ID of the [Custom account](https://stripe.com/docs/connect/custom-accounts) this recipient was migrated to. If set, the recipient can no longer be updated, nor can transfers be made to it: use the Custom account instead.
    pub migrated_to: Option<serde_json::Value>,
    #[serde(rename = "name")]
    ///Full, legal name of the recipient.
    pub name: Option<String>,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "rolled_back_from")]
    pub rolled_back_from: Option<serde_json::Value>,
    #[serde(rename = "type")]
    ///Type of the recipient, one of `individual` or `corporation`.
    pub type_: String,
}
impl std::fmt::Display for Recipient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Recurring {
    #[serde(rename = "aggregate_usage")]
    ///Specifies a usage aggregation strategy for prices of `usage_type=metered`. Allowed values are `sum` for summing up all usage during a period, `last_during_period` for using the last usage record reported within a period, `last_ever` for using the last usage record ever (across period bounds) or `max` which uses the usage record with the maximum reported usage during a period. Defaults to `sum`.
    pub aggregate_usage: Option<String>,
    #[serde(rename = "interval")]
    ///The frequency at which a subscription is billed. One of `day`, `week`, `month` or `year`.
    pub interval: String,
    #[serde(rename = "interval_count")]
    ///The number of intervals (specified in the `interval` attribute) between subscription billings. For example, `interval=month` and `interval_count=3` bills every 3 months.
    pub interval_count: i64,
    #[serde(rename = "usage_type")]
    ///Configures how the quantity per period should be determined. Can be either `metered` or `licensed`. `licensed` automatically bills the `quantity` set when adding it to a subscription. `metered` aggregates the total usage based on usage records. Defaults to `licensed`.
    pub usage_type: String,
}
impl std::fmt::Display for Recurring {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Refund {
    #[serde(rename = "amount")]
    ///Amount, in %s.
    pub amount: i64,
    #[serde(rename = "balance_transaction")]
    ///Balance transaction that describes the impact on your account balance.
    pub balance_transaction: Option<serde_json::Value>,
    #[serde(rename = "charge")]
    ///ID of the charge that was refunded.
    pub charge: Option<serde_json::Value>,
    #[serde(rename = "created")]
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    #[serde(rename = "currency")]
    ///Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: String,
    #[serde(rename = "description")]
    ///An arbitrary string attached to the object. Often useful for displaying to users. (Available on non-card refunds only)
    pub description: Option<String>,
    #[serde(rename = "failure_balance_transaction")]
    ///If the refund failed, this balance transaction describes the adjustment made on your account balance that reverses the initial balance transaction.
    pub failure_balance_transaction: Option<serde_json::Value>,
    #[serde(rename = "failure_reason")]
    ///If the refund failed, the reason for refund failure if known. Possible values are `lost_or_stolen_card`, `expired_or_canceled_card`, or `unknown`.
    pub failure_reason: Option<String>,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "instructions_email")]
    ///Email to which refund instructions, if required, are sent to.
    pub instructions_email: Option<String>,
    #[serde(rename = "metadata")]
    ///Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<serde_json::Value>,
    #[serde(rename = "next_action")]
    ///
    pub next_action: Option<RefundNextAction>,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "payment_intent")]
    ///ID of the PaymentIntent that was refunded.
    pub payment_intent: Option<serde_json::Value>,
    #[serde(rename = "reason")]
    ///Reason for the refund, either user-provided (`duplicate`, `fraudulent`, or `requested_by_customer`) or generated by Stripe internally (`expired_uncaptured_charge`).
    pub reason: Option<String>,
    #[serde(rename = "receipt_number")]
    ///This is the transaction number that appears on email receipts sent for this refund.
    pub receipt_number: Option<String>,
    #[serde(rename = "source_transfer_reversal")]
    ///The transfer reversal that is associated with the refund. Only present if the charge came from another Stripe account. See the Connect documentation for details.
    pub source_transfer_reversal: Option<serde_json::Value>,
    #[serde(rename = "status")]
    ///Status of the refund. For credit card refunds, this can be `pending`, `succeeded`, or `failed`. For other types of refunds, it can be `pending`, `requires_action`, `succeeded`, `failed`, or `canceled`. Refer to our [refunds](https://stripe.com/docs/refunds#failed-refunds) documentation for more details.
    pub status: Option<String>,
    #[serde(rename = "transfer_reversal")]
    ///If the accompanying transfer was reversed, the transfer reversal object. Only applicable if the charge was created using the destination parameter.
    pub transfer_reversal: Option<serde_json::Value>,
}
impl std::fmt::Display for Refund {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RefundNextAction {
    #[serde(rename = "display_details")]
    ///Contains the refund details.
    pub display_details: Option<serde_json::Value>,
    #[serde(rename = "type")]
    ///Type of the next action to perform.
    pub type_: String,
}
impl std::fmt::Display for RefundNextAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RefundNextActionDisplayDetails {
    #[serde(rename = "email_sent")]
    ///
    pub email_sent: EmailSent,
    #[serde(rename = "expires_at")]
    ///The expiry timestamp.
    pub expires_at: i64,
}
impl std::fmt::Display for RefundNextActionDisplayDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ReportingReportRun {
    #[serde(rename = "created")]
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    #[serde(rename = "error")]
    /**If something should go wrong during the run, a message about the failure (populated when
 `status=failed`).*/
    pub error: Option<String>,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "livemode")]
    ///`true` if the report is run on live mode data and `false` if it is run on test mode data.
    pub livemode: bool,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "parameters")]
    ///
    pub parameters: FinancialReportingFinanceReportRunRunParameters,
    #[serde(rename = "report_type")]
    ///The ID of the [report type](https://stripe.com/docs/reports/report-types) to run, such as `"balance.summary.1"`.
    pub report_type: String,
    #[serde(rename = "result")]
    /**The file object representing the result of the report run (populated when
 `status=succeeded`).*/
    pub result: Option<serde_json::Value>,
    #[serde(rename = "status")]
    /**Status of this report run. This will be `pending` when the run is initially created.
 When the run finishes, this will be set to `succeeded` and the `result` field will be populated.
 Rarely, we may encounter an error, at which point this will be set to `failed` and the `error` field will be populated.*/
    pub status: String,
    #[serde(rename = "succeeded_at")]
    /**Timestamp at which this run successfully finished (populated when
 `status=succeeded`). Measured in seconds since the Unix epoch.*/
    pub succeeded_at: Option<i64>,
}
impl std::fmt::Display for ReportingReportRun {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ReportingReportType {
    #[serde(rename = "data_available_end")]
    ///Most recent time for which this Report Type is available. Measured in seconds since the Unix epoch.
    pub data_available_end: i64,
    #[serde(rename = "data_available_start")]
    ///Earliest time for which this Report Type is available. Measured in seconds since the Unix epoch.
    pub data_available_start: i64,
    #[serde(rename = "default_columns")]
    ///List of column names that are included by default when this Report Type gets run. (If the Report Type doesn't support the `columns` parameter, this will be null.)
    pub default_columns: Option<Vec<String>>,
    #[serde(rename = "id")]
    ///The [ID of the Report Type](https://stripe.com/docs/reporting/statements/api#available-report-types), such as `balance.summary.1`.
    pub id: String,
    #[serde(rename = "livemode")]
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    #[serde(rename = "name")]
    ///Human-readable name of the Report Type
    pub name: String,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "updated")]
    ///When this Report Type was latest updated. Measured in seconds since the Unix epoch.
    pub updated: i64,
    #[serde(rename = "version")]
    ///Version of the Report Type. Different versions report with the same ID will have the same purpose, but may take different run parameters or have different result schemas.
    pub version: i64,
}
impl std::fmt::Display for ReportingReportType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ReserveTransaction {
    #[serde(rename = "amount")]
    pub amount: i64,
    #[serde(rename = "currency")]
    ///Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: String,
    #[serde(rename = "description")]
    ///An arbitrary string attached to the object. Often useful for displaying to users.
    pub description: Option<String>,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
}
impl std::fmt::Display for ReserveTransaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Review {
    #[serde(rename = "billing_zip")]
    ///The ZIP or postal code of the card used, if applicable.
    pub billing_zip: Option<String>,
    #[serde(rename = "charge")]
    ///The charge associated with this review.
    pub charge: Option<serde_json::Value>,
    #[serde(rename = "closed_reason")]
    ///The reason the review was closed, or null if it has not yet been closed. One of `approved`, `refunded`, `refunded_as_fraud`, `disputed`, or `redacted`.
    pub closed_reason: Option<String>,
    #[serde(rename = "created")]
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "ip_address")]
    ///The IP address where the payment originated.
    pub ip_address: Option<String>,
    #[serde(rename = "ip_address_location")]
    ///Information related to the location of the payment. Note that this information is an approximation and attempts to locate the nearest population center - it should not be used to determine a specific address.
    pub ip_address_location: Option<serde_json::Value>,
    #[serde(rename = "livemode")]
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "open")]
    ///If `true`, the review needs action.
    pub open: bool,
    #[serde(rename = "opened_reason")]
    ///The reason the review was opened. One of `rule` or `manual`.
    pub opened_reason: String,
    #[serde(rename = "payment_intent")]
    ///The PaymentIntent ID associated with this review, if one exists.
    pub payment_intent: Option<serde_json::Value>,
    #[serde(rename = "reason")]
    ///The reason the review is currently open or closed. One of `rule`, `manual`, `approved`, `refunded`, `refunded_as_fraud`, `disputed`, or `redacted`.
    pub reason: String,
    #[serde(rename = "session")]
    ///Information related to the browsing session of the user who initiated the payment.
    pub session: Option<serde_json::Value>,
}
impl std::fmt::Display for Review {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Rule {
    #[serde(rename = "action")]
    ///The action taken on the payment.
    pub action: String,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "predicate")]
    ///The predicate to evaluate the payment against.
    pub predicate: String,
}
impl std::fmt::Display for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ScheduledQueryRun {
    #[serde(rename = "created")]
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    #[serde(rename = "data_load_time")]
    ///When the query was run, Sigma contained a snapshot of your Stripe data at this time.
    pub data_load_time: i64,
    #[serde(rename = "error")]
    ///
    pub error: Option<SigmaScheduledQueryRunError>,
    #[serde(rename = "file")]
    ///The file object representing the results of the query.
    pub file: Option<serde_json::Value>,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "livemode")]
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "result_available_until")]
    ///Time at which the result expires and is no longer available for download.
    pub result_available_until: i64,
    #[serde(rename = "sql")]
    ///SQL for the query.
    pub sql: String,
    #[serde(rename = "status")]
    ///The query's execution status, which will be `completed` for successful runs, and `canceled`, `failed`, or `timed_out` otherwise.
    pub status: String,
    #[serde(rename = "title")]
    ///Title of the query.
    pub title: String,
}
impl std::fmt::Display for ScheduledQueryRun {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SchedulesPhaseAutomaticTax {
    #[serde(rename = "enabled")]
    ///Whether Stripe automatically computes tax on invoices created during this phase.
    pub enabled: bool,
}
impl std::fmt::Display for SchedulesPhaseAutomaticTax {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SecretServiceResourceScope {
    #[serde(rename = "type")]
    ///The secret scope type.
    pub type_: String,
    #[serde(rename = "user")]
    ///The user ID, if type is set to "user"
    pub user: Option<String>,
}
impl std::fmt::Display for SecretServiceResourceScope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SepaDebitGeneratedFrom {
    #[serde(rename = "charge")]
    ///The ID of the Charge that generated this PaymentMethod, if any.
    pub charge: Option<serde_json::Value>,
    #[serde(rename = "setup_attempt")]
    ///The ID of the SetupAttempt that generated this PaymentMethod, if any.
    pub setup_attempt: Option<serde_json::Value>,
}
impl std::fmt::Display for SepaDebitGeneratedFrom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SetupAttempt {
    #[serde(rename = "application")]
    ///The value of [application](https://stripe.com/docs/api/setup_intents/object#setup_intent_object-application) on the SetupIntent at the time of this confirmation.
    pub application: Option<serde_json::Value>,
    #[serde(rename = "attach_to_self")]
    /**If present, the SetupIntent's payment method will be attached to the in-context Stripe Account.

It can only be used for this Stripe Account’s own money movement flows like InboundTransfer and OutboundTransfers. It cannot be set to true when setting up a PaymentMethod for a Customer, and defaults to false when attaching a PaymentMethod to a Customer.*/
    pub attach_to_self: Option<bool>,
    #[serde(rename = "created")]
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    #[serde(rename = "customer")]
    ///The value of [customer](https://stripe.com/docs/api/setup_intents/object#setup_intent_object-customer) on the SetupIntent at the time of this confirmation.
    pub customer: Option<serde_json::Value>,
    #[serde(rename = "flow_directions")]
    /**Indicates the directions of money movement for which this payment method is intended to be used.

Include `inbound` if you intend to use the payment method as the origin to pull funds from. Include `outbound` if you intend to use the payment method as the destination to send funds to. You can include both if you intend to use the payment method for both purposes.*/
    pub flow_directions: Option<Vec<String>>,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "livemode")]
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "on_behalf_of")]
    ///The value of [on_behalf_of](https://stripe.com/docs/api/setup_intents/object#setup_intent_object-on_behalf_of) on the SetupIntent at the time of this confirmation.
    pub on_behalf_of: Option<serde_json::Value>,
    #[serde(rename = "payment_method")]
    ///ID of the payment method used with this SetupAttempt.
    pub payment_method: serde_json::Value,
    #[serde(rename = "payment_method_details")]
    ///
    pub payment_method_details: SetupAttemptPaymentMethodDetails,
    #[serde(rename = "setup_error")]
    ///The error encountered during this attempt to confirm the SetupIntent, if any.
    pub setup_error: Option<serde_json::Value>,
    #[serde(rename = "setup_intent")]
    ///ID of the SetupIntent that this attempt belongs to.
    pub setup_intent: serde_json::Value,
    #[serde(rename = "status")]
    ///Status of this SetupAttempt, one of `requires_confirmation`, `requires_action`, `processing`, `succeeded`, `failed`, or `abandoned`.
    pub status: String,
    #[serde(rename = "usage")]
    ///The value of [usage](https://stripe.com/docs/api/setup_intents/object#setup_intent_object-usage) on the SetupIntent at the time of this confirmation, one of `off_session` or `on_session`.
    pub usage: String,
}
impl std::fmt::Display for SetupAttempt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SetupAttemptPaymentMethodDetails {
    #[serde(rename = "acss_debit")]
    ///
    pub acss_debit: Option<SetupAttemptPaymentMethodDetailsAcssDebit>,
    #[serde(rename = "au_becs_debit")]
    ///
    pub au_becs_debit: Option<SetupAttemptPaymentMethodDetailsAuBecsDebit>,
    #[serde(rename = "bacs_debit")]
    ///
    pub bacs_debit: Option<SetupAttemptPaymentMethodDetailsBacsDebit>,
    #[serde(rename = "bancontact")]
    ///
    pub bancontact: Option<SetupAttemptPaymentMethodDetailsBancontact>,
    #[serde(rename = "blik")]
    ///
    pub blik: Option<SetupAttemptPaymentMethodDetailsBlik>,
    #[serde(rename = "boleto")]
    ///
    pub boleto: Option<SetupAttemptPaymentMethodDetailsBoleto>,
    #[serde(rename = "card")]
    ///
    pub card: Option<SetupAttemptPaymentMethodDetailsCard>,
    #[serde(rename = "card_present")]
    ///
    pub card_present: Option<SetupAttemptPaymentMethodDetailsCardPresent>,
    #[serde(rename = "ideal")]
    ///
    pub ideal: Option<SetupAttemptPaymentMethodDetailsIdeal>,
    #[serde(rename = "link")]
    ///
    pub link: Option<SetupAttemptPaymentMethodDetailsLink>,
    #[serde(rename = "sepa_debit")]
    ///
    pub sepa_debit: Option<SetupAttemptPaymentMethodDetailsSepaDebit>,
    #[serde(rename = "sofort")]
    ///
    pub sofort: Option<SetupAttemptPaymentMethodDetailsSofort>,
    #[serde(rename = "type")]
    ///The type of the payment method used in the SetupIntent (e.g., `card`). An additional hash is included on `payment_method_details` with a name matching this value. It contains confirmation-specific information for the payment method.
    pub type_: String,
    #[serde(rename = "us_bank_account")]
    ///
    pub us_bank_account: Option<SetupAttemptPaymentMethodDetailsUsBankAccount>,
}
impl std::fmt::Display for SetupAttemptPaymentMethodDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SetupAttemptPaymentMethodDetailsAcssDebit {}
impl std::fmt::Display for SetupAttemptPaymentMethodDetailsAcssDebit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SetupAttemptPaymentMethodDetailsAuBecsDebit {}
impl std::fmt::Display for SetupAttemptPaymentMethodDetailsAuBecsDebit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SetupAttemptPaymentMethodDetailsBacsDebit {}
impl std::fmt::Display for SetupAttemptPaymentMethodDetailsBacsDebit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SetupAttemptPaymentMethodDetailsBancontact {
    #[serde(rename = "bank_code")]
    ///Bank code of bank associated with the bank account.
    pub bank_code: Option<String>,
    #[serde(rename = "bank_name")]
    ///Name of the bank associated with the bank account.
    pub bank_name: Option<String>,
    #[serde(rename = "bic")]
    ///Bank Identifier Code of the bank associated with the bank account.
    pub bic: Option<String>,
    #[serde(rename = "generated_sepa_debit")]
    ///The ID of the SEPA Direct Debit PaymentMethod which was generated by this SetupAttempt.
    pub generated_sepa_debit: Option<serde_json::Value>,
    #[serde(rename = "generated_sepa_debit_mandate")]
    ///The mandate for the SEPA Direct Debit PaymentMethod which was generated by this SetupAttempt.
    pub generated_sepa_debit_mandate: Option<serde_json::Value>,
    #[serde(rename = "iban_last4")]
    ///Last four characters of the IBAN.
    pub iban_last4: Option<String>,
    #[serde(rename = "preferred_language")]
    /**Preferred language of the Bancontact authorization page that the customer is redirected to.
Can be one of `en`, `de`, `fr`, or `nl`*/
    pub preferred_language: Option<String>,
    #[serde(rename = "verified_name")]
    /**Owner's verified full name. Values are verified or provided by Bancontact directly
(if supported) at the time of authorization or settlement. They cannot be set or mutated.*/
    pub verified_name: Option<String>,
}
impl std::fmt::Display for SetupAttemptPaymentMethodDetailsBancontact {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SetupAttemptPaymentMethodDetailsBlik {}
impl std::fmt::Display for SetupAttemptPaymentMethodDetailsBlik {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SetupAttemptPaymentMethodDetailsBoleto {}
impl std::fmt::Display for SetupAttemptPaymentMethodDetailsBoleto {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SetupAttemptPaymentMethodDetailsCard {
    #[serde(rename = "three_d_secure")]
    ///Populated if this authorization used 3D Secure authentication.
    pub three_d_secure: Option<serde_json::Value>,
}
impl std::fmt::Display for SetupAttemptPaymentMethodDetailsCard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SetupAttemptPaymentMethodDetailsCardPresent {
    #[serde(rename = "generated_card")]
    ///The ID of the Card PaymentMethod which was generated by this SetupAttempt.
    pub generated_card: Option<serde_json::Value>,
}
impl std::fmt::Display for SetupAttemptPaymentMethodDetailsCardPresent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SetupAttemptPaymentMethodDetailsIdeal {
    #[serde(rename = "bank")]
    ///The customer's bank. Can be one of `abn_amro`, `asn_bank`, `bunq`, `handelsbanken`, `ing`, `knab`, `moneyou`, `rabobank`, `regiobank`, `revolut`, `sns_bank`, `triodos_bank`, or `van_lanschot`.
    pub bank: Option<String>,
    #[serde(rename = "bic")]
    ///The Bank Identifier Code of the customer's bank.
    pub bic: Option<String>,
    #[serde(rename = "generated_sepa_debit")]
    ///The ID of the SEPA Direct Debit PaymentMethod which was generated by this SetupAttempt.
    pub generated_sepa_debit: Option<serde_json::Value>,
    #[serde(rename = "generated_sepa_debit_mandate")]
    ///The mandate for the SEPA Direct Debit PaymentMethod which was generated by this SetupAttempt.
    pub generated_sepa_debit_mandate: Option<serde_json::Value>,
    #[serde(rename = "iban_last4")]
    ///Last four characters of the IBAN.
    pub iban_last4: Option<String>,
    #[serde(rename = "verified_name")]
    /**Owner's verified full name. Values are verified or provided by iDEAL directly
(if supported) at the time of authorization or settlement. They cannot be set or mutated.*/
    pub verified_name: Option<String>,
}
impl std::fmt::Display for SetupAttemptPaymentMethodDetailsIdeal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SetupAttemptPaymentMethodDetailsLink {}
impl std::fmt::Display for SetupAttemptPaymentMethodDetailsLink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SetupAttemptPaymentMethodDetailsSepaDebit {}
impl std::fmt::Display for SetupAttemptPaymentMethodDetailsSepaDebit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SetupAttemptPaymentMethodDetailsSofort {
    #[serde(rename = "bank_code")]
    ///Bank code of bank associated with the bank account.
    pub bank_code: Option<String>,
    #[serde(rename = "bank_name")]
    ///Name of the bank associated with the bank account.
    pub bank_name: Option<String>,
    #[serde(rename = "bic")]
    ///Bank Identifier Code of the bank associated with the bank account.
    pub bic: Option<String>,
    #[serde(rename = "generated_sepa_debit")]
    ///The ID of the SEPA Direct Debit PaymentMethod which was generated by this SetupAttempt.
    pub generated_sepa_debit: Option<serde_json::Value>,
    #[serde(rename = "generated_sepa_debit_mandate")]
    ///The mandate for the SEPA Direct Debit PaymentMethod which was generated by this SetupAttempt.
    pub generated_sepa_debit_mandate: Option<serde_json::Value>,
    #[serde(rename = "iban_last4")]
    ///Last four characters of the IBAN.
    pub iban_last4: Option<String>,
    #[serde(rename = "preferred_language")]
    /**Preferred language of the Sofort authorization page that the customer is redirected to.
Can be one of `en`, `de`, `fr`, or `nl`*/
    pub preferred_language: Option<String>,
    #[serde(rename = "verified_name")]
    /**Owner's verified full name. Values are verified or provided by Sofort directly
(if supported) at the time of authorization or settlement. They cannot be set or mutated.*/
    pub verified_name: Option<String>,
}
impl std::fmt::Display for SetupAttemptPaymentMethodDetailsSofort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SetupAttemptPaymentMethodDetailsUsBankAccount {}
impl std::fmt::Display for SetupAttemptPaymentMethodDetailsUsBankAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SetupIntent {
    #[serde(rename = "application")]
    ///ID of the Connect application that created the SetupIntent.
    pub application: Option<serde_json::Value>,
    #[serde(rename = "attach_to_self")]
    /**If present, the SetupIntent's payment method will be attached to the in-context Stripe Account.

It can only be used for this Stripe Account’s own money movement flows like InboundTransfer and OutboundTransfers. It cannot be set to true when setting up a PaymentMethod for a Customer, and defaults to false when attaching a PaymentMethod to a Customer.*/
    pub attach_to_self: Option<bool>,
    #[serde(rename = "cancellation_reason")]
    ///Reason for cancellation of this SetupIntent, one of `abandoned`, `requested_by_customer`, or `duplicate`.
    pub cancellation_reason: Option<String>,
    #[serde(rename = "client_secret")]
    /**The client secret of this SetupIntent. Used for client-side retrieval using a publishable key.

The client secret can be used to complete payment setup from your frontend. It should not be stored, logged, or exposed to anyone other than the customer. Make sure that you have TLS enabled on any page that includes the client secret.*/
    pub client_secret: Option<String>,
    #[serde(rename = "created")]
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    #[serde(rename = "customer")]
    /**ID of the Customer this SetupIntent belongs to, if one exists.

If present, the SetupIntent's payment method will be attached to the Customer on successful setup. Payment methods attached to other Customers cannot be used with this SetupIntent.*/
    pub customer: Option<serde_json::Value>,
    #[serde(rename = "description")]
    ///An arbitrary string attached to the object. Often useful for displaying to users.
    pub description: Option<String>,
    #[serde(rename = "flow_directions")]
    /**Indicates the directions of money movement for which this payment method is intended to be used.

Include `inbound` if you intend to use the payment method as the origin to pull funds from. Include `outbound` if you intend to use the payment method as the destination to send funds to. You can include both if you intend to use the payment method for both purposes.*/
    pub flow_directions: Option<Vec<String>>,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "last_setup_error")]
    ///The error encountered in the previous SetupIntent confirmation.
    pub last_setup_error: Option<serde_json::Value>,
    #[serde(rename = "latest_attempt")]
    ///The most recent SetupAttempt for this SetupIntent.
    pub latest_attempt: Option<serde_json::Value>,
    #[serde(rename = "livemode")]
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    #[serde(rename = "mandate")]
    ///ID of the multi use Mandate generated by the SetupIntent.
    pub mandate: Option<serde_json::Value>,
    #[serde(rename = "metadata")]
    ///Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<serde_json::Value>,
    #[serde(rename = "next_action")]
    ///If present, this property tells you what actions you need to take in order for your customer to continue payment setup.
    pub next_action: Option<serde_json::Value>,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "on_behalf_of")]
    ///The account (if any) for which the setup is intended.
    pub on_behalf_of: Option<serde_json::Value>,
    #[serde(rename = "payment_method")]
    ///ID of the payment method used with this SetupIntent.
    pub payment_method: Option<serde_json::Value>,
    #[serde(rename = "payment_method_options")]
    ///Payment-method-specific configuration for this SetupIntent.
    pub payment_method_options: Option<serde_json::Value>,
    #[serde(rename = "payment_method_types")]
    ///The list of payment method types (e.g. card) that this SetupIntent is allowed to set up.
    pub payment_method_types: Vec<String>,
    #[serde(rename = "single_use_mandate")]
    ///ID of the single_use Mandate generated by the SetupIntent.
    pub single_use_mandate: Option<serde_json::Value>,
    #[serde(rename = "status")]
    ///[Status](https://stripe.com/docs/payments/intents#intent-statuses) of this SetupIntent, one of `requires_payment_method`, `requires_confirmation`, `requires_action`, `processing`, `canceled`, or `succeeded`.
    pub status: String,
    #[serde(rename = "usage")]
    /**Indicates how the payment method is intended to be used in the future.

Use `on_session` if you intend to only reuse the payment method when the customer is in your checkout flow. Use `off_session` if your customer may or may not be in your checkout flow. If not provided, this value defaults to `off_session`.*/
    pub usage: String,
}
impl std::fmt::Display for SetupIntent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SetupIntentNextAction {
    #[serde(rename = "redirect_to_url")]
    ///
    pub redirect_to_url: Option<SetupIntentNextActionRedirectToUrl>,
    #[serde(rename = "type")]
    ///Type of the next action to perform, one of `redirect_to_url`, `use_stripe_sdk`, `alipay_handle_redirect`, `oxxo_display_details`, or `verify_with_microdeposits`.
    pub type_: String,
    #[serde(rename = "use_stripe_sdk")]
    ///When confirming a SetupIntent with Stripe.js, Stripe.js depends on the contents of this dictionary to invoke authentication flows. The shape of the contents is subject to change and is only intended to be used by Stripe.js.
    pub use_stripe_sdk: Option<serde_json::Value>,
    #[serde(rename = "verify_with_microdeposits")]
    ///
    pub verify_with_microdeposits: Option<SetupIntentNextActionVerifyWithMicrodeposits>,
}
impl std::fmt::Display for SetupIntentNextAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SetupIntentNextActionRedirectToUrl {
    #[serde(rename = "return_url")]
    ///If the customer does not exit their browser while authenticating, they will be redirected to this specified URL after completion.
    pub return_url: Option<String>,
    #[serde(rename = "url")]
    ///The URL you must redirect your customer to in order to authenticate.
    pub url: Option<String>,
}
impl std::fmt::Display for SetupIntentNextActionRedirectToUrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SetupIntentNextActionVerifyWithMicrodeposits {
    #[serde(rename = "arrival_date")]
    ///The timestamp when the microdeposits are expected to land.
    pub arrival_date: i64,
    #[serde(rename = "hosted_verification_url")]
    ///The URL for the hosted verification page, which allows customers to verify their bank account.
    pub hosted_verification_url: String,
    #[serde(rename = "microdeposit_type")]
    ///The type of the microdeposit sent to the customer. Used to distinguish between different verification methods.
    pub microdeposit_type: Option<String>,
}
impl std::fmt::Display for SetupIntentNextActionVerifyWithMicrodeposits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SetupIntentPaymentMethodOptions {
    #[serde(rename = "acss_debit")]
    pub acss_debit: Option<serde_json::Value>,
    #[serde(rename = "blik")]
    pub blik: Option<serde_json::Value>,
    #[serde(rename = "card")]
    ///
    pub card: Option<SetupIntentPaymentMethodOptionsCard>,
    #[serde(rename = "link")]
    pub link: Option<serde_json::Value>,
    #[serde(rename = "sepa_debit")]
    pub sepa_debit: Option<serde_json::Value>,
    #[serde(rename = "us_bank_account")]
    pub us_bank_account: Option<serde_json::Value>,
}
impl std::fmt::Display for SetupIntentPaymentMethodOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SetupIntentPaymentMethodOptionsAcssDebit {
    #[serde(rename = "currency")]
    ///Currency supported by the bank account
    pub currency: Option<String>,
    #[serde(rename = "mandate_options")]
    ///
    pub mandate_options: Option<SetupIntentPaymentMethodOptionsMandateOptionsAcssDebit>,
    #[serde(rename = "verification_method")]
    ///Bank account verification method.
    pub verification_method: Option<String>,
}
impl std::fmt::Display for SetupIntentPaymentMethodOptionsAcssDebit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SetupIntentPaymentMethodOptionsBlik {
    #[serde(rename = "mandate_options")]
    ///
    pub mandate_options: Option<SetupIntentPaymentMethodOptionsMandateOptionsBlik>,
}
impl std::fmt::Display for SetupIntentPaymentMethodOptionsBlik {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SetupIntentPaymentMethodOptionsCard {
    #[serde(rename = "mandate_options")]
    ///Configuration options for setting up an eMandate for cards issued in India.
    pub mandate_options: Option<serde_json::Value>,
    #[serde(rename = "network")]
    ///Selected network to process this SetupIntent on. Depends on the available networks of the card attached to the setup intent. Can be only set confirm-time.
    pub network: Option<String>,
    #[serde(rename = "request_three_d_secure")]
    ///We strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based on risk level and [other requirements](https://stripe.com/docs/strong-customer-authentication). However, if you wish to request 3D Secure based on logic from your own fraud engine, provide this option. Permitted values include: `automatic` or `any`. If not provided, defaults to `automatic`. Read our guide on [manually requesting 3D Secure](https://stripe.com/docs/payments/3d-secure#manual-three-ds) for more information on how this configuration interacts with Radar and our SCA Engine.
    pub request_three_d_secure: Option<String>,
}
impl std::fmt::Display for SetupIntentPaymentMethodOptionsCard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SetupIntentPaymentMethodOptionsCardMandateOptions {
    #[serde(rename = "amount")]
    ///Amount to be charged for future payments.
    pub amount: i64,
    #[serde(rename = "amount_type")]
    ///One of `fixed` or `maximum`. If `fixed`, the `amount` param refers to the exact amount to be charged in future payments. If `maximum`, the amount charged can be up to the value passed for the `amount` param.
    pub amount_type: String,
    #[serde(rename = "currency")]
    ///Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: String,
    #[serde(rename = "description")]
    ///A description of the mandate or subscription that is meant to be displayed to the customer.
    pub description: Option<String>,
    #[serde(rename = "end_date")]
    ///End date of the mandate or subscription. If not provided, the mandate will be active until canceled. If provided, end date should be after start date.
    pub end_date: Option<i64>,
    #[serde(rename = "interval")]
    ///Specifies payment frequency. One of `day`, `week`, `month`, `year`, or `sporadic`.
    pub interval: String,
    #[serde(rename = "interval_count")]
    ///The number of intervals between payments. For example, `interval=month` and `interval_count=3` indicates one payment every three months. Maximum of one year interval allowed (1 year, 12 months, or 52 weeks). This parameter is optional when `interval=sporadic`.
    pub interval_count: Option<i64>,
    #[serde(rename = "reference")]
    ///Unique identifier for the mandate or subscription.
    pub reference: String,
    #[serde(rename = "start_date")]
    ///Start date of the mandate or subscription. Start date should not be lesser than yesterday.
    pub start_date: i64,
    #[serde(rename = "supported_types")]
    ///Specifies the type of mandates supported. Possible values are `india`.
    pub supported_types: Option<Vec<String>>,
}
impl std::fmt::Display for SetupIntentPaymentMethodOptionsCardMandateOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SetupIntentPaymentMethodOptionsLink {
    #[serde(rename = "persistent_token")]
    ///Token used for persistent Link logins.
    pub persistent_token: Option<String>,
}
impl std::fmt::Display for SetupIntentPaymentMethodOptionsLink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SetupIntentPaymentMethodOptionsMandateOptionsAcssDebit {
    #[serde(rename = "custom_mandate_url")]
    ///A URL for custom mandate text
    pub custom_mandate_url: Option<String>,
    #[serde(rename = "default_for")]
    ///List of Stripe products where this mandate can be selected automatically.
    pub default_for: Option<Vec<String>>,
    #[serde(rename = "interval_description")]
    ///Description of the interval. Only required if the 'payment_schedule' parameter is 'interval' or 'combined'.
    pub interval_description: Option<String>,
    #[serde(rename = "payment_schedule")]
    ///Payment schedule for the mandate.
    pub payment_schedule: Option<String>,
    #[serde(rename = "transaction_type")]
    ///Transaction type of the mandate.
    pub transaction_type: Option<String>,
}
impl std::fmt::Display for SetupIntentPaymentMethodOptionsMandateOptionsAcssDebit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SetupIntentPaymentMethodOptionsMandateOptionsBlik {
    #[serde(rename = "expires_after")]
    ///Date at which the mandate expires.
    pub expires_after: Option<i64>,
    #[serde(rename = "off_session")]
    ///
    pub off_session: Option<MandateOptionsOffSessionDetailsBlik>,
    #[serde(rename = "type")]
    ///Type of the mandate.
    pub type_: Option<String>,
}
impl std::fmt::Display for SetupIntentPaymentMethodOptionsMandateOptionsBlik {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SetupIntentPaymentMethodOptionsMandateOptionsSepaDebit {}
impl std::fmt::Display for SetupIntentPaymentMethodOptionsMandateOptionsSepaDebit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SetupIntentPaymentMethodOptionsSepaDebit {
    #[serde(rename = "mandate_options")]
    ///
    pub mandate_options: Option<SetupIntentPaymentMethodOptionsMandateOptionsSepaDebit>,
}
impl std::fmt::Display for SetupIntentPaymentMethodOptionsSepaDebit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SetupIntentPaymentMethodOptionsUsBankAccount {
    #[serde(rename = "financial_connections")]
    ///
    pub financial_connections: Option<LinkedAccountOptionsUsBankAccount>,
    #[serde(rename = "verification_method")]
    ///Bank account verification method.
    pub verification_method: Option<String>,
}
impl std::fmt::Display for SetupIntentPaymentMethodOptionsUsBankAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SetupIntentTypeSpecificPaymentMethodOptionsClient {
    #[serde(rename = "verification_method")]
    ///Bank account verification method.
    pub verification_method: Option<String>,
}
impl std::fmt::Display for SetupIntentTypeSpecificPaymentMethodOptionsClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Shipping {
    #[serde(rename = "address")]
    ///
    pub address: Option<Address>,
    #[serde(rename = "carrier")]
    ///The delivery service that shipped a physical product, such as Fedex, UPS, USPS, etc.
    pub carrier: Option<String>,
    #[serde(rename = "name")]
    ///Recipient name.
    pub name: Option<String>,
    #[serde(rename = "phone")]
    ///Recipient phone (including extension).
    pub phone: Option<String>,
    #[serde(rename = "tracking_number")]
    ///The tracking number for a physical product, obtained from the delivery service. If multiple tracking numbers were generated for this purchase, please separate them with commas.
    pub tracking_number: Option<String>,
}
impl std::fmt::Display for Shipping {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ShippingRate {
    #[serde(rename = "active")]
    ///Whether the shipping rate can be used for new purchases. Defaults to `true`.
    pub active: bool,
    #[serde(rename = "created")]
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    #[serde(rename = "delivery_estimate")]
    ///The estimated range for how long shipping will take, meant to be displayable to the customer. This will appear on CheckoutSessions.
    pub delivery_estimate: Option<serde_json::Value>,
    #[serde(rename = "display_name")]
    ///The name of the shipping rate, meant to be displayable to the customer. This will appear on CheckoutSessions.
    pub display_name: Option<String>,
    #[serde(rename = "fixed_amount")]
    ///
    pub fixed_amount: Option<ShippingRateFixedAmount>,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "livemode")]
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    #[serde(rename = "metadata")]
    ///Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format.
    pub metadata: serde_json::Value,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "tax_behavior")]
    ///Specifies whether the rate is considered inclusive of taxes or exclusive of taxes. One of `inclusive`, `exclusive`, or `unspecified`.
    pub tax_behavior: Option<String>,
    #[serde(rename = "tax_code")]
    ///A [tax code](https://stripe.com/docs/tax/tax-categories) ID. The Shipping tax code is `txcd_92010001`.
    pub tax_code: Option<serde_json::Value>,
    #[serde(rename = "type")]
    ///The type of calculation to use on the shipping rate. Can only be `fixed_amount` for now.
    pub type_: String,
}
impl std::fmt::Display for ShippingRate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ShippingRateCurrencyOption {
    #[serde(rename = "amount")]
    ///A non-negative integer in cents representing how much to charge.
    pub amount: i64,
    #[serde(rename = "tax_behavior")]
    ///Specifies whether the rate is considered inclusive of taxes or exclusive of taxes. One of `inclusive`, `exclusive`, or `unspecified`.
    pub tax_behavior: String,
}
impl std::fmt::Display for ShippingRateCurrencyOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ShippingRateDeliveryEstimate {
    #[serde(rename = "maximum")]
    ///The upper bound of the estimated range. If empty, represents no upper bound i.e., infinite.
    pub maximum: Option<serde_json::Value>,
    #[serde(rename = "minimum")]
    ///The lower bound of the estimated range. If empty, represents no lower bound.
    pub minimum: Option<serde_json::Value>,
}
impl std::fmt::Display for ShippingRateDeliveryEstimate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ShippingRateDeliveryEstimateBound {
    #[serde(rename = "unit")]
    ///A unit of time.
    pub unit: String,
    #[serde(rename = "value")]
    ///Must be greater than 0.
    pub value: i64,
}
impl std::fmt::Display for ShippingRateDeliveryEstimateBound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ShippingRateFixedAmount {
    #[serde(rename = "amount")]
    ///A non-negative integer in cents representing how much to charge.
    pub amount: i64,
    #[serde(rename = "currency")]
    ///Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: String,
    #[serde(rename = "currency_options")]
    ///Shipping rates defined in each available currency option. Each key must be a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html) and a [supported currency](https://stripe.com/docs/currencies).
    pub currency_options: Option<serde_json::Value>,
}
impl std::fmt::Display for ShippingRateFixedAmount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SigmaScheduledQueryRunError {
    #[serde(rename = "message")]
    ///Information about the run failure.
    pub message: String,
}
impl std::fmt::Display for SigmaScheduledQueryRunError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Sku {
    #[serde(rename = "active")]
    ///Whether the SKU is available for purchase.
    pub active: bool,
    #[serde(rename = "attributes")]
    ///A dictionary of attributes and values for the attributes defined by the product. If, for example, a product's attributes are `["size", "gender"]`, a valid SKU has the following dictionary of attributes: `{"size": "Medium", "gender": "Unisex"}`.
    pub attributes: serde_json::Value,
    #[serde(rename = "created")]
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    #[serde(rename = "currency")]
    ///Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: String,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "image")]
    ///The URL of an image for this SKU, meant to be displayable to the customer.
    pub image: Option<String>,
    #[serde(rename = "inventory")]
    ///
    pub inventory: SkuInventory,
    #[serde(rename = "livemode")]
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    #[serde(rename = "metadata")]
    ///Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format.
    pub metadata: serde_json::Value,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "package_dimensions")]
    ///The dimensions of this SKU for shipping purposes.
    pub package_dimensions: Option<serde_json::Value>,
    #[serde(rename = "price")]
    ///The cost of the item as a positive integer in the smallest currency unit (that is, 100 cents to charge $1.00, or 100 to charge ¥100, Japanese Yen being a zero-decimal currency).
    pub price: i64,
    #[serde(rename = "product")]
    ///The ID of the product this SKU is associated with. The product must be currently active.
    pub product: serde_json::Value,
    #[serde(rename = "updated")]
    ///Time at which the object was last updated. Measured in seconds since the Unix epoch.
    pub updated: i64,
}
impl std::fmt::Display for Sku {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SkuInventory {
    #[serde(rename = "quantity")]
    ///The count of inventory available. Will be present if and only if `type` is `finite`.
    pub quantity: Option<i64>,
    #[serde(rename = "type")]
    ///Inventory type. Possible values are `finite`, `bucket` (not quantified), and `infinite`.
    pub type_: String,
    #[serde(rename = "value")]
    ///An indicator of the inventory available. Possible values are `in_stock`, `limited`, and `out_of_stock`. Will be present if and only if `type` is `bucket`.
    pub value: Option<String>,
}
impl std::fmt::Display for SkuInventory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Source {
    #[serde(rename = "ach_credit_transfer")]
    pub ach_credit_transfer: Option<SourceTypeAchCreditTransfer>,
    #[serde(rename = "ach_debit")]
    pub ach_debit: Option<SourceTypeAchDebit>,
    #[serde(rename = "acss_debit")]
    pub acss_debit: Option<SourceTypeAcssDebit>,
    #[serde(rename = "alipay")]
    pub alipay: Option<SourceTypeAlipay>,
    #[serde(rename = "amount")]
    ///A positive integer in the smallest currency unit (that is, 100 cents for $1.00, or 1 for ¥1, Japanese Yen being a zero-decimal currency) representing the total amount associated with the source. This is the amount for which the source will be chargeable once ready. Required for `single_use` sources.
    pub amount: Option<i64>,
    #[serde(rename = "au_becs_debit")]
    pub au_becs_debit: Option<SourceTypeAuBecsDebit>,
    #[serde(rename = "bancontact")]
    pub bancontact: Option<SourceTypeBancontact>,
    #[serde(rename = "card")]
    pub card: Option<SourceTypeCard>,
    #[serde(rename = "card_present")]
    pub card_present: Option<SourceTypeCardPresent>,
    #[serde(rename = "client_secret")]
    ///The client secret of the source. Used for client-side retrieval using a publishable key.
    pub client_secret: String,
    #[serde(rename = "code_verification")]
    ///
    pub code_verification: Option<SourceCodeVerificationFlow>,
    #[serde(rename = "created")]
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    #[serde(rename = "currency")]
    ///Three-letter [ISO code for the currency](https://stripe.com/docs/currencies) associated with the source. This is the currency for which the source will be chargeable once ready. Required for `single_use` sources.
    pub currency: Option<String>,
    #[serde(rename = "customer")]
    ///The ID of the customer to which this source is attached. This will not be present when the source has not been attached to a customer.
    pub customer: Option<String>,
    #[serde(rename = "eps")]
    pub eps: Option<SourceTypeEps>,
    #[serde(rename = "flow")]
    ///The authentication `flow` of the source. `flow` is one of `redirect`, `receiver`, `code_verification`, `none`.
    pub flow: String,
    #[serde(rename = "giropay")]
    pub giropay: Option<SourceTypeGiropay>,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "ideal")]
    pub ideal: Option<SourceTypeIdeal>,
    #[serde(rename = "klarna")]
    pub klarna: Option<SourceTypeKlarna>,
    #[serde(rename = "livemode")]
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    #[serde(rename = "metadata")]
    ///Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<serde_json::Value>,
    #[serde(rename = "multibanco")]
    pub multibanco: Option<SourceTypeMultibanco>,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "owner")]
    ///Information about the owner of the payment instrument that may be used or required by particular source types.
    pub owner: Option<serde_json::Value>,
    #[serde(rename = "p24")]
    pub p24: Option<SourceTypeP24>,
    #[serde(rename = "receiver")]
    ///
    pub receiver: Option<SourceReceiverFlow>,
    #[serde(rename = "redirect")]
    ///
    pub redirect: Option<SourceRedirectFlow>,
    #[serde(rename = "sepa_debit")]
    pub sepa_debit: Option<SourceTypeSepaDebit>,
    #[serde(rename = "sofort")]
    pub sofort: Option<SourceTypeSofort>,
    #[serde(rename = "source_order")]
    ///
    pub source_order: Option<SourceOrder>,
    #[serde(rename = "statement_descriptor")]
    ///Extra information about a source. This will appear on your customer's statement every time you charge the source.
    pub statement_descriptor: Option<String>,
    #[serde(rename = "status")]
    ///The status of the source, one of `canceled`, `chargeable`, `consumed`, `failed`, or `pending`. Only `chargeable` sources can be used to create a charge.
    pub status: String,
    #[serde(rename = "three_d_secure")]
    pub three_d_secure: Option<SourceTypeThreeDSecure>,
    #[serde(rename = "type")]
    ///The `type` of the source. The `type` is a payment method, one of `ach_credit_transfer`, `ach_debit`, `alipay`, `bancontact`, `card`, `card_present`, `eps`, `giropay`, `ideal`, `multibanco`, `klarna`, `p24`, `sepa_debit`, `sofort`, `three_d_secure`, or `wechat`. An additional hash is included on the source with a name matching this value. It contains additional information specific to the [payment method](https://stripe.com/docs/sources) used.
    pub type_: String,
    #[serde(rename = "usage")]
    ///Either `reusable` or `single_use`. Whether this source should be reusable or not. Some source types may or may not be reusable by construction, while others may leave the option at creation. If an incompatible value is passed, an error will be returned.
    pub usage: Option<String>,
    #[serde(rename = "wechat")]
    pub wechat: Option<SourceTypeWechat>,
}
impl std::fmt::Display for Source {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SourceCodeVerificationFlow {
    #[serde(rename = "attempts_remaining")]
    ///The number of attempts remaining to authenticate the source object with a verification code.
    pub attempts_remaining: i64,
    #[serde(rename = "status")]
    ///The status of the code verification, either `pending` (awaiting verification, `attempts_remaining` should be greater than 0), `succeeded` (successful verification) or `failed` (failed verification, cannot be verified anymore as `attempts_remaining` should be 0).
    pub status: String,
}
impl std::fmt::Display for SourceCodeVerificationFlow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SourceMandateNotification {
    #[serde(rename = "acss_debit")]
    ///
    pub acss_debit: Option<SourceMandateNotificationAcssDebitData>,
    #[serde(rename = "amount")]
    ///A positive integer in the smallest currency unit (that is, 100 cents for $1.00, or 1 for ¥1, Japanese Yen being a zero-decimal currency) representing the amount associated with the mandate notification. The amount is expressed in the currency of the underlying source. Required if the notification type is `debit_initiated`.
    pub amount: Option<i64>,
    #[serde(rename = "bacs_debit")]
    ///
    pub bacs_debit: Option<SourceMandateNotificationBacsDebitData>,
    #[serde(rename = "created")]
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "livemode")]
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "reason")]
    ///The reason of the mandate notification. Valid reasons are `mandate_confirmed` or `debit_initiated`.
    pub reason: String,
    #[serde(rename = "sepa_debit")]
    ///
    pub sepa_debit: Option<SourceMandateNotificationSepaDebitData>,
    #[serde(rename = "source")]
    /**`Source` objects allow you to accept a variety of payment methods. They
represent a customer's payment instrument, and can be used with the Stripe API
just like a `Card` object: once chargeable, they can be charged, or can be
attached to customers.

Related guides: [Sources API](https://stripe.com/docs/sources) and [Sources & Customers](https://stripe.com/docs/sources/customers).*/
    pub source: Source,
    #[serde(rename = "status")]
    ///The status of the mandate notification. Valid statuses are `pending` or `submitted`.
    pub status: String,
    #[serde(rename = "type")]
    ///The type of source this mandate notification is attached to. Should be the source type identifier code for the payment method, such as `three_d_secure`.
    pub type_: String,
}
impl std::fmt::Display for SourceMandateNotification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SourceMandateNotificationAcssDebitData {
    #[serde(rename = "statement_descriptor")]
    ///The statement descriptor associate with the debit.
    pub statement_descriptor: Option<String>,
}
impl std::fmt::Display for SourceMandateNotificationAcssDebitData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SourceMandateNotificationBacsDebitData {
    #[serde(rename = "last4")]
    ///Last 4 digits of the account number associated with the debit.
    pub last4: Option<String>,
}
impl std::fmt::Display for SourceMandateNotificationBacsDebitData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SourceMandateNotificationSepaDebitData {
    #[serde(rename = "creditor_identifier")]
    ///SEPA creditor ID.
    pub creditor_identifier: Option<String>,
    #[serde(rename = "last4")]
    ///Last 4 digits of the account number associated with the debit.
    pub last4: Option<String>,
    #[serde(rename = "mandate_reference")]
    ///Mandate reference associated with the debit.
    pub mandate_reference: Option<String>,
}
impl std::fmt::Display for SourceMandateNotificationSepaDebitData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SourceOrder {
    #[serde(rename = "amount")]
    ///A positive integer in the smallest currency unit (that is, 100 cents for $1.00, or 1 for ¥1, Japanese Yen being a zero-decimal currency) representing the total amount for the order.
    pub amount: i64,
    #[serde(rename = "currency")]
    ///Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: String,
    #[serde(rename = "email")]
    ///The email address of the customer placing the order.
    pub email: Option<String>,
    #[serde(rename = "items")]
    ///List of items constituting the order.
    pub items: Option<Vec<SourceOrderItem>>,
    #[serde(rename = "shipping")]
    ///
    pub shipping: Option<Shipping>,
}
impl std::fmt::Display for SourceOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SourceOrderItem {
    #[serde(rename = "amount")]
    ///The amount (price) for this order item.
    pub amount: Option<i64>,
    #[serde(rename = "currency")]
    ///This currency of this order item. Required when `amount` is present.
    pub currency: Option<String>,
    #[serde(rename = "description")]
    ///Human-readable description for this order item.
    pub description: Option<String>,
    #[serde(rename = "parent")]
    ///The ID of the associated object for this line item. Expandable if not null (e.g., expandable to a SKU).
    pub parent: Option<String>,
    #[serde(rename = "quantity")]
    ///The quantity of this order item. When type is `sku`, this is the number of instances of the SKU to be ordered.
    pub quantity: Option<i64>,
    #[serde(rename = "type")]
    ///The type of this order item. Must be `sku`, `tax`, or `shipping`.
    pub type_: Option<String>,
}
impl std::fmt::Display for SourceOrderItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SourceOwner {
    #[serde(rename = "address")]
    ///Owner's address.
    pub address: Option<serde_json::Value>,
    #[serde(rename = "email")]
    ///Owner's email address.
    pub email: Option<String>,
    #[serde(rename = "name")]
    ///Owner's full name.
    pub name: Option<String>,
    #[serde(rename = "phone")]
    ///Owner's phone number (including extension).
    pub phone: Option<String>,
    #[serde(rename = "verified_address")]
    ///Verified owner's address. Verified values are verified or provided by the payment method directly (and if supported) at the time of authorization or settlement. They cannot be set or mutated.
    pub verified_address: Option<serde_json::Value>,
    #[serde(rename = "verified_email")]
    ///Verified owner's email address. Verified values are verified or provided by the payment method directly (and if supported) at the time of authorization or settlement. They cannot be set or mutated.
    pub verified_email: Option<String>,
    #[serde(rename = "verified_name")]
    ///Verified owner's full name. Verified values are verified or provided by the payment method directly (and if supported) at the time of authorization or settlement. They cannot be set or mutated.
    pub verified_name: Option<String>,
    #[serde(rename = "verified_phone")]
    ///Verified owner's phone number (including extension). Verified values are verified or provided by the payment method directly (and if supported) at the time of authorization or settlement. They cannot be set or mutated.
    pub verified_phone: Option<String>,
}
impl std::fmt::Display for SourceOwner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SourceReceiverFlow {
    #[serde(rename = "address")]
    ///The address of the receiver source. This is the value that should be communicated to the customer to send their funds to.
    pub address: Option<String>,
    #[serde(rename = "amount_charged")]
    ///The total amount that was moved to your balance. This is almost always equal to the amount charged. In rare cases when customers deposit excess funds and we are unable to refund those, those funds get moved to your balance and show up in amount_charged as well. The amount charged is expressed in the source's currency.
    pub amount_charged: i64,
    #[serde(rename = "amount_received")]
    ///The total amount received by the receiver source. `amount_received = amount_returned + amount_charged` should be true for consumed sources unless customers deposit excess funds. The amount received is expressed in the source's currency.
    pub amount_received: i64,
    #[serde(rename = "amount_returned")]
    ///The total amount that was returned to the customer. The amount returned is expressed in the source's currency.
    pub amount_returned: i64,
    #[serde(rename = "refund_attributes_method")]
    ///Type of refund attribute method, one of `email`, `manual`, or `none`.
    pub refund_attributes_method: String,
    #[serde(rename = "refund_attributes_status")]
    ///Type of refund attribute status, one of `missing`, `requested`, or `available`.
    pub refund_attributes_status: String,
}
impl std::fmt::Display for SourceReceiverFlow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SourceRedirectFlow {
    #[serde(rename = "failure_reason")]
    ///The failure reason for the redirect, either `user_abort` (the customer aborted or dropped out of the redirect flow), `declined` (the authentication failed or the transaction was declined), or `processing_error` (the redirect failed due to a technical error). Present only if the redirect status is `failed`.
    pub failure_reason: Option<String>,
    #[serde(rename = "return_url")]
    ///The URL you provide to redirect the customer to after they authenticated their payment.
    pub return_url: String,
    #[serde(rename = "status")]
    ///The status of the redirect, either `pending` (ready to be used by your customer to authenticate the transaction), `succeeded` (succesful authentication, cannot be reused) or `not_required` (redirect should not be used) or `failed` (failed authentication, cannot be reused).
    pub status: String,
    #[serde(rename = "url")]
    ///The URL provided to you to redirect a customer to as part of a `redirect` authentication flow.
    pub url: String,
}
impl std::fmt::Display for SourceRedirectFlow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SourceTransaction {
    #[serde(rename = "ach_credit_transfer")]
    ///
    pub ach_credit_transfer: Option<SourceTransactionAchCreditTransferData>,
    #[serde(rename = "amount")]
    ///A positive integer in the smallest currency unit (that is, 100 cents for $1.00, or 1 for ¥1, Japanese Yen being a zero-decimal currency) representing the amount your customer has pushed to the receiver.
    pub amount: i64,
    #[serde(rename = "chf_credit_transfer")]
    ///
    pub chf_credit_transfer: Option<SourceTransactionChfCreditTransferData>,
    #[serde(rename = "created")]
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    #[serde(rename = "currency")]
    ///Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: String,
    #[serde(rename = "gbp_credit_transfer")]
    ///
    pub gbp_credit_transfer: Option<SourceTransactionGbpCreditTransferData>,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "livemode")]
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "paper_check")]
    ///
    pub paper_check: Option<SourceTransactionPaperCheckData>,
    #[serde(rename = "sepa_credit_transfer")]
    ///
    pub sepa_credit_transfer: Option<SourceTransactionSepaCreditTransferData>,
    #[serde(rename = "source")]
    ///The ID of the source this transaction is attached to.
    pub source: String,
    #[serde(rename = "status")]
    ///The status of the transaction, one of `succeeded`, `pending`, or `failed`.
    pub status: String,
    #[serde(rename = "type")]
    ///The type of source this transaction is attached to.
    pub type_: String,
}
impl std::fmt::Display for SourceTransaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SourceTransactionAchCreditTransferData {
    #[serde(rename = "customer_data")]
    ///Customer data associated with the transfer.
    pub customer_data: Option<String>,
    #[serde(rename = "fingerprint")]
    ///Bank account fingerprint associated with the transfer.
    pub fingerprint: Option<String>,
    #[serde(rename = "last4")]
    ///Last 4 digits of the account number associated with the transfer.
    pub last4: Option<String>,
    #[serde(rename = "routing_number")]
    ///Routing number associated with the transfer.
    pub routing_number: Option<String>,
}
impl std::fmt::Display for SourceTransactionAchCreditTransferData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SourceTransactionChfCreditTransferData {
    #[serde(rename = "reference")]
    ///Reference associated with the transfer.
    pub reference: Option<String>,
    #[serde(rename = "sender_address_country")]
    ///Sender's country address.
    pub sender_address_country: Option<String>,
    #[serde(rename = "sender_address_line1")]
    ///Sender's line 1 address.
    pub sender_address_line1: Option<String>,
    #[serde(rename = "sender_iban")]
    ///Sender's bank account IBAN.
    pub sender_iban: Option<String>,
    #[serde(rename = "sender_name")]
    ///Sender's name.
    pub sender_name: Option<String>,
}
impl std::fmt::Display for SourceTransactionChfCreditTransferData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SourceTransactionGbpCreditTransferData {
    #[serde(rename = "fingerprint")]
    ///Bank account fingerprint associated with the Stripe owned bank account receiving the transfer.
    pub fingerprint: Option<String>,
    #[serde(rename = "funding_method")]
    ///The credit transfer rails the sender used to push this transfer. The possible rails are: Faster Payments, BACS, CHAPS, and wire transfers. Currently only Faster Payments is supported.
    pub funding_method: Option<String>,
    #[serde(rename = "last4")]
    ///Last 4 digits of sender account number associated with the transfer.
    pub last4: Option<String>,
    #[serde(rename = "reference")]
    ///Sender entered arbitrary information about the transfer.
    pub reference: Option<String>,
    #[serde(rename = "sender_account_number")]
    ///Sender account number associated with the transfer.
    pub sender_account_number: Option<String>,
    #[serde(rename = "sender_name")]
    ///Sender name associated with the transfer.
    pub sender_name: Option<String>,
    #[serde(rename = "sender_sort_code")]
    ///Sender sort code associated with the transfer.
    pub sender_sort_code: Option<String>,
}
impl std::fmt::Display for SourceTransactionGbpCreditTransferData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SourceTransactionPaperCheckData {
    #[serde(rename = "available_at")]
    ///Time at which the deposited funds will be available for use. Measured in seconds since the Unix epoch.
    pub available_at: Option<String>,
    #[serde(rename = "invoices")]
    ///Comma-separated list of invoice IDs associated with the paper check.
    pub invoices: Option<String>,
}
impl std::fmt::Display for SourceTransactionPaperCheckData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SourceTransactionSepaCreditTransferData {
    #[serde(rename = "reference")]
    ///Reference associated with the transfer.
    pub reference: Option<String>,
    #[serde(rename = "sender_iban")]
    ///Sender's bank account IBAN.
    pub sender_iban: Option<String>,
    #[serde(rename = "sender_name")]
    ///Sender's name.
    pub sender_name: Option<String>,
}
impl std::fmt::Display for SourceTransactionSepaCreditTransferData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SourceTypeAchCreditTransfer {
    #[serde(rename = "account_number")]
    pub account_number: Option<String>,
    #[serde(rename = "bank_name")]
    pub bank_name: Option<String>,
    #[serde(rename = "fingerprint")]
    pub fingerprint: Option<String>,
    #[serde(rename = "refund_account_holder_name")]
    pub refund_account_holder_name: Option<String>,
    #[serde(rename = "refund_account_holder_type")]
    pub refund_account_holder_type: Option<String>,
    #[serde(rename = "refund_routing_number")]
    pub refund_routing_number: Option<String>,
    #[serde(rename = "routing_number")]
    pub routing_number: Option<String>,
    #[serde(rename = "swift_code")]
    pub swift_code: Option<String>,
}
impl std::fmt::Display for SourceTypeAchCreditTransfer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SourceTypeAchDebit {
    #[serde(rename = "bank_name")]
    pub bank_name: Option<String>,
    #[serde(rename = "country")]
    pub country: Option<String>,
    #[serde(rename = "fingerprint")]
    pub fingerprint: Option<String>,
    #[serde(rename = "last4")]
    pub last4: Option<String>,
    #[serde(rename = "routing_number")]
    pub routing_number: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
}
impl std::fmt::Display for SourceTypeAchDebit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SourceTypeAcssDebit {
    #[serde(rename = "bank_address_city")]
    pub bank_address_city: Option<String>,
    #[serde(rename = "bank_address_line_1")]
    pub bank_address_line1: Option<String>,
    #[serde(rename = "bank_address_line_2")]
    pub bank_address_line2: Option<String>,
    #[serde(rename = "bank_address_postal_code")]
    pub bank_address_postal_code: Option<String>,
    #[serde(rename = "bank_name")]
    pub bank_name: Option<String>,
    #[serde(rename = "category")]
    pub category: Option<String>,
    #[serde(rename = "country")]
    pub country: Option<String>,
    #[serde(rename = "fingerprint")]
    pub fingerprint: Option<String>,
    #[serde(rename = "last4")]
    pub last4: Option<String>,
    #[serde(rename = "routing_number")]
    pub routing_number: Option<String>,
}
impl std::fmt::Display for SourceTypeAcssDebit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SourceTypeAlipay {
    #[serde(rename = "data_string")]
    pub data_string: Option<String>,
    #[serde(rename = "native_url")]
    pub native_url: Option<String>,
    #[serde(rename = "statement_descriptor")]
    pub statement_descriptor: Option<String>,
}
impl std::fmt::Display for SourceTypeAlipay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SourceTypeAuBecsDebit {
    #[serde(rename = "bsb_number")]
    pub bsb_number: Option<String>,
    #[serde(rename = "fingerprint")]
    pub fingerprint: Option<String>,
    #[serde(rename = "last4")]
    pub last4: Option<String>,
}
impl std::fmt::Display for SourceTypeAuBecsDebit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SourceTypeBancontact {
    #[serde(rename = "bank_code")]
    pub bank_code: Option<String>,
    #[serde(rename = "bank_name")]
    pub bank_name: Option<String>,
    #[serde(rename = "bic")]
    pub bic: Option<String>,
    #[serde(rename = "iban_last4")]
    pub iban_last4: Option<String>,
    #[serde(rename = "preferred_language")]
    pub preferred_language: Option<String>,
    #[serde(rename = "statement_descriptor")]
    pub statement_descriptor: Option<String>,
}
impl std::fmt::Display for SourceTypeBancontact {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SourceTypeCard {
    #[serde(rename = "address_line1_check")]
    pub address_line1_check: Option<String>,
    #[serde(rename = "address_zip_check")]
    pub address_zip_check: Option<String>,
    #[serde(rename = "brand")]
    pub brand: Option<String>,
    #[serde(rename = "country")]
    pub country: Option<String>,
    #[serde(rename = "cvc_check")]
    pub cvc_check: Option<String>,
    #[serde(rename = "dynamic_last4")]
    pub dynamic_last4: Option<String>,
    #[serde(rename = "exp_month")]
    pub exp_month: Option<i64>,
    #[serde(rename = "exp_year")]
    pub exp_year: Option<i64>,
    #[serde(rename = "fingerprint")]
    pub fingerprint: Option<String>,
    #[serde(rename = "funding")]
    pub funding: Option<String>,
    #[serde(rename = "last4")]
    pub last4: Option<String>,
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[serde(rename = "three_d_secure")]
    pub three_d_secure: Option<String>,
    #[serde(rename = "tokenization_method")]
    pub tokenization_method: Option<String>,
}
impl std::fmt::Display for SourceTypeCard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SourceTypeCardPresent {
    #[serde(rename = "application_cryptogram")]
    pub application_cryptogram: Option<String>,
    #[serde(rename = "application_preferred_name")]
    pub application_preferred_name: Option<String>,
    #[serde(rename = "authorization_code")]
    pub authorization_code: Option<String>,
    #[serde(rename = "authorization_response_code")]
    pub authorization_response_code: Option<String>,
    #[serde(rename = "brand")]
    pub brand: Option<String>,
    #[serde(rename = "country")]
    pub country: Option<String>,
    #[serde(rename = "cvm_type")]
    pub cvm_type: Option<String>,
    #[serde(rename = "data_type")]
    pub data_type: Option<String>,
    #[serde(rename = "dedicated_file_name")]
    pub dedicated_file_name: Option<String>,
    #[serde(rename = "emv_auth_data")]
    pub emv_auth_data: Option<String>,
    #[serde(rename = "evidence_customer_signature")]
    pub evidence_customer_signature: Option<String>,
    #[serde(rename = "evidence_transaction_certificate")]
    pub evidence_transaction_certificate: Option<String>,
    #[serde(rename = "exp_month")]
    pub exp_month: Option<i64>,
    #[serde(rename = "exp_year")]
    pub exp_year: Option<i64>,
    #[serde(rename = "fingerprint")]
    pub fingerprint: Option<String>,
    #[serde(rename = "funding")]
    pub funding: Option<String>,
    #[serde(rename = "last4")]
    pub last4: Option<String>,
    #[serde(rename = "pos_device_id")]
    pub pos_device_id: Option<String>,
    #[serde(rename = "pos_entry_mode")]
    pub pos_entry_mode: Option<String>,
    #[serde(rename = "read_method")]
    pub read_method: Option<String>,
    #[serde(rename = "reader")]
    pub reader: Option<String>,
    #[serde(rename = "terminal_verification_results")]
    pub terminal_verification_results: Option<String>,
    #[serde(rename = "transaction_status_information")]
    pub transaction_status_information: Option<String>,
}
impl std::fmt::Display for SourceTypeCardPresent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SourceTypeEps {
    #[serde(rename = "reference")]
    pub reference: Option<String>,
    #[serde(rename = "statement_descriptor")]
    pub statement_descriptor: Option<String>,
}
impl std::fmt::Display for SourceTypeEps {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SourceTypeGiropay {
    #[serde(rename = "bank_code")]
    pub bank_code: Option<String>,
    #[serde(rename = "bank_name")]
    pub bank_name: Option<String>,
    #[serde(rename = "bic")]
    pub bic: Option<String>,
    #[serde(rename = "statement_descriptor")]
    pub statement_descriptor: Option<String>,
}
impl std::fmt::Display for SourceTypeGiropay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SourceTypeIdeal {
    #[serde(rename = "bank")]
    pub bank: Option<String>,
    #[serde(rename = "bic")]
    pub bic: Option<String>,
    #[serde(rename = "iban_last4")]
    pub iban_last4: Option<String>,
    #[serde(rename = "statement_descriptor")]
    pub statement_descriptor: Option<String>,
}
impl std::fmt::Display for SourceTypeIdeal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SourceTypeKlarna {
    #[serde(rename = "background_image_url")]
    pub background_image_url: Option<String>,
    #[serde(rename = "client_token")]
    pub client_token: Option<String>,
    #[serde(rename = "first_name")]
    pub first_name: Option<String>,
    #[serde(rename = "last_name")]
    pub last_name: Option<String>,
    #[serde(rename = "locale")]
    pub locale: Option<String>,
    #[serde(rename = "logo_url")]
    pub logo_url: Option<String>,
    #[serde(rename = "page_title")]
    pub page_title: Option<String>,
    #[serde(rename = "pay_later_asset_urls_descriptive")]
    pub pay_later_asset_urls_descriptive: Option<String>,
    #[serde(rename = "pay_later_asset_urls_standard")]
    pub pay_later_asset_urls_standard: Option<String>,
    #[serde(rename = "pay_later_name")]
    pub pay_later_name: Option<String>,
    #[serde(rename = "pay_later_redirect_url")]
    pub pay_later_redirect_url: Option<String>,
    #[serde(rename = "pay_now_asset_urls_descriptive")]
    pub pay_now_asset_urls_descriptive: Option<String>,
    #[serde(rename = "pay_now_asset_urls_standard")]
    pub pay_now_asset_urls_standard: Option<String>,
    #[serde(rename = "pay_now_name")]
    pub pay_now_name: Option<String>,
    #[serde(rename = "pay_now_redirect_url")]
    pub pay_now_redirect_url: Option<String>,
    #[serde(rename = "pay_over_time_asset_urls_descriptive")]
    pub pay_over_time_asset_urls_descriptive: Option<String>,
    #[serde(rename = "pay_over_time_asset_urls_standard")]
    pub pay_over_time_asset_urls_standard: Option<String>,
    #[serde(rename = "pay_over_time_name")]
    pub pay_over_time_name: Option<String>,
    #[serde(rename = "pay_over_time_redirect_url")]
    pub pay_over_time_redirect_url: Option<String>,
    #[serde(rename = "payment_method_categories")]
    pub payment_method_categories: Option<String>,
    #[serde(rename = "purchase_country")]
    pub purchase_country: Option<String>,
    #[serde(rename = "purchase_type")]
    pub purchase_type: Option<String>,
    #[serde(rename = "redirect_url")]
    pub redirect_url: Option<String>,
    #[serde(rename = "shipping_delay")]
    pub shipping_delay: Option<i64>,
    #[serde(rename = "shipping_first_name")]
    pub shipping_first_name: Option<String>,
    #[serde(rename = "shipping_last_name")]
    pub shipping_last_name: Option<String>,
}
impl std::fmt::Display for SourceTypeKlarna {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SourceTypeMultibanco {
    #[serde(rename = "entity")]
    pub entity: Option<String>,
    #[serde(rename = "reference")]
    pub reference: Option<String>,
    #[serde(rename = "refund_account_holder_address_city")]
    pub refund_account_holder_address_city: Option<String>,
    #[serde(rename = "refund_account_holder_address_country")]
    pub refund_account_holder_address_country: Option<String>,
    #[serde(rename = "refund_account_holder_address_line1")]
    pub refund_account_holder_address_line1: Option<String>,
    #[serde(rename = "refund_account_holder_address_line2")]
    pub refund_account_holder_address_line2: Option<String>,
    #[serde(rename = "refund_account_holder_address_postal_code")]
    pub refund_account_holder_address_postal_code: Option<String>,
    #[serde(rename = "refund_account_holder_address_state")]
    pub refund_account_holder_address_state: Option<String>,
    #[serde(rename = "refund_account_holder_name")]
    pub refund_account_holder_name: Option<String>,
    #[serde(rename = "refund_iban")]
    pub refund_iban: Option<String>,
}
impl std::fmt::Display for SourceTypeMultibanco {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SourceTypeP24 {
    #[serde(rename = "reference")]
    pub reference: Option<String>,
}
impl std::fmt::Display for SourceTypeP24 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SourceTypeSepaDebit {
    #[serde(rename = "bank_code")]
    pub bank_code: Option<String>,
    #[serde(rename = "branch_code")]
    pub branch_code: Option<String>,
    #[serde(rename = "country")]
    pub country: Option<String>,
    #[serde(rename = "fingerprint")]
    pub fingerprint: Option<String>,
    #[serde(rename = "last4")]
    pub last4: Option<String>,
    #[serde(rename = "mandate_reference")]
    pub mandate_reference: Option<String>,
    #[serde(rename = "mandate_url")]
    pub mandate_url: Option<String>,
}
impl std::fmt::Display for SourceTypeSepaDebit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SourceTypeSofort {
    #[serde(rename = "bank_code")]
    pub bank_code: Option<String>,
    #[serde(rename = "bank_name")]
    pub bank_name: Option<String>,
    #[serde(rename = "bic")]
    pub bic: Option<String>,
    #[serde(rename = "country")]
    pub country: Option<String>,
    #[serde(rename = "iban_last4")]
    pub iban_last4: Option<String>,
    #[serde(rename = "preferred_language")]
    pub preferred_language: Option<String>,
    #[serde(rename = "statement_descriptor")]
    pub statement_descriptor: Option<String>,
}
impl std::fmt::Display for SourceTypeSofort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SourceTypeThreeDSecure {
    #[serde(rename = "address_line1_check")]
    pub address_line1_check: Option<String>,
    #[serde(rename = "address_zip_check")]
    pub address_zip_check: Option<String>,
    #[serde(rename = "authenticated")]
    pub authenticated: Option<bool>,
    #[serde(rename = "brand")]
    pub brand: Option<String>,
    #[serde(rename = "card")]
    pub card: Option<String>,
    #[serde(rename = "country")]
    pub country: Option<String>,
    #[serde(rename = "customer")]
    pub customer: Option<String>,
    #[serde(rename = "cvc_check")]
    pub cvc_check: Option<String>,
    #[serde(rename = "dynamic_last4")]
    pub dynamic_last4: Option<String>,
    #[serde(rename = "exp_month")]
    pub exp_month: Option<i64>,
    #[serde(rename = "exp_year")]
    pub exp_year: Option<i64>,
    #[serde(rename = "fingerprint")]
    pub fingerprint: Option<String>,
    #[serde(rename = "funding")]
    pub funding: Option<String>,
    #[serde(rename = "last4")]
    pub last4: Option<String>,
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[serde(rename = "three_d_secure")]
    pub three_d_secure: Option<String>,
    #[serde(rename = "tokenization_method")]
    pub tokenization_method: Option<String>,
}
impl std::fmt::Display for SourceTypeThreeDSecure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SourceTypeWechat {
    #[serde(rename = "prepay_id")]
    pub prepay_id: Option<String>,
    #[serde(rename = "qr_code_url")]
    pub qr_code_url: Option<String>,
    #[serde(rename = "statement_descriptor")]
    pub statement_descriptor: Option<String>,
}
impl std::fmt::Display for SourceTypeWechat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Subscription {
    #[serde(rename = "application")]
    ///ID of the Connect Application that created the subscription.
    pub application: Option<serde_json::Value>,
    #[serde(rename = "application_fee_percent")]
    ///A non-negative decimal between 0 and 100, with at most two decimal places. This represents the percentage of the subscription invoice subtotal that will be transferred to the application owner's Stripe account.
    pub application_fee_percent: Option<f64>,
    #[serde(rename = "automatic_tax")]
    ///
    pub automatic_tax: SubscriptionAutomaticTax,
    #[serde(rename = "billing_cycle_anchor")]
    ///Determines the date of the first full invoice, and, for plans with `month` or `year` intervals, the day of the month for subsequent invoices. The timestamp is in UTC format.
    pub billing_cycle_anchor: i64,
    #[serde(rename = "billing_thresholds")]
    ///Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period
    pub billing_thresholds: Option<serde_json::Value>,
    #[serde(rename = "cancel_at")]
    ///A date in the future at which the subscription will automatically get canceled
    pub cancel_at: Option<i64>,
    #[serde(rename = "cancel_at_period_end")]
    ///If the subscription has been canceled with the `at_period_end` flag set to `true`, `cancel_at_period_end` on the subscription will be true. You can use this attribute to determine whether a subscription that has a status of active is scheduled to be canceled at the end of the current period.
    pub cancel_at_period_end: bool,
    #[serde(rename = "canceled_at")]
    ///If the subscription has been canceled, the date of that cancellation. If the subscription was canceled with `cancel_at_period_end`, `canceled_at` will reflect the time of the most recent update request, not the end of the subscription period when the subscription is automatically moved to a canceled state.
    pub canceled_at: Option<i64>,
    #[serde(rename = "collection_method")]
    ///Either `charge_automatically`, or `send_invoice`. When charging automatically, Stripe will attempt to pay this subscription at the end of the cycle using the default source attached to the customer. When sending an invoice, Stripe will email your customer an invoice with payment instructions.
    pub collection_method: String,
    #[serde(rename = "created")]
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    #[serde(rename = "currency")]
    ///Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Option<String>,
    #[serde(rename = "current_period_end")]
    ///End of the current period that the subscription has been invoiced for. At the end of this period, a new invoice will be created.
    pub current_period_end: i64,
    #[serde(rename = "current_period_start")]
    ///Start of the current period that the subscription has been invoiced for.
    pub current_period_start: i64,
    #[serde(rename = "customer")]
    ///ID of the customer who owns the subscription.
    pub customer: serde_json::Value,
    #[serde(rename = "days_until_due")]
    ///Number of days a customer has to pay invoices generated by this subscription. This value will be `null` for subscriptions where `collection_method=charge_automatically`.
    pub days_until_due: Option<i64>,
    #[serde(rename = "default_payment_method")]
    ///ID of the default payment method for the subscription. It must belong to the customer associated with the subscription. This takes precedence over `default_source`. If neither are set, invoices will use the customer's [invoice_settings.default_payment_method](https://stripe.com/docs/api/customers/object#customer_object-invoice_settings-default_payment_method) or [default_source](https://stripe.com/docs/api/customers/object#customer_object-default_source).
    pub default_payment_method: Option<serde_json::Value>,
    #[serde(rename = "default_source")]
    ///ID of the default payment source for the subscription. It must belong to the customer associated with the subscription and be in a chargeable state. If `default_payment_method` is also set, `default_payment_method` will take precedence. If neither are set, invoices will use the customer's [invoice_settings.default_payment_method](https://stripe.com/docs/api/customers/object#customer_object-invoice_settings-default_payment_method) or [default_source](https://stripe.com/docs/api/customers/object#customer_object-default_source).
    pub default_source: Option<serde_json::Value>,
    #[serde(rename = "default_tax_rates")]
    ///The tax rates that will apply to any subscription item that does not have `tax_rates` set. Invoices created will have their `default_tax_rates` populated from the subscription.
    pub default_tax_rates: Option<Vec<TaxRate>>,
    #[serde(rename = "description")]
    ///The subscription's description, meant to be displayable to the customer. Use this field to optionally store an explanation of the subscription for rendering in Stripe surfaces.
    pub description: Option<String>,
    #[serde(rename = "discount")]
    ///Describes the current discount applied to this subscription, if there is one. When billing, a discount applied to a subscription overrides a discount applied on a customer-wide basis.
    pub discount: Option<serde_json::Value>,
    #[serde(rename = "ended_at")]
    ///If the subscription has ended, the date the subscription ended.
    pub ended_at: Option<i64>,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "items")]
    ///List of subscription items, each with an attached price.
    pub items: serde_json::Value,
    #[serde(rename = "latest_invoice")]
    ///The most recent invoice this subscription has generated.
    pub latest_invoice: Option<serde_json::Value>,
    #[serde(rename = "livemode")]
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    #[serde(rename = "metadata")]
    ///Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format.
    pub metadata: serde_json::Value,
    #[serde(rename = "next_pending_invoice_item_invoice")]
    ///Specifies the approximate timestamp on which any pending invoice items will be billed according to the schedule provided at `pending_invoice_item_interval`.
    pub next_pending_invoice_item_invoice: Option<i64>,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "pause_collection")]
    ///If specified, payment collection for this subscription will be paused.
    pub pause_collection: Option<serde_json::Value>,
    #[serde(rename = "payment_settings")]
    ///Payment settings passed on to invoices created by the subscription.
    pub payment_settings: Option<serde_json::Value>,
    #[serde(rename = "pending_invoice_item_interval")]
    ///Specifies an interval for how often to bill for any pending invoice items. It is analogous to calling [Create an invoice](https://stripe.com/docs/api#create_invoice) for the given subscription at the specified interval.
    pub pending_invoice_item_interval: Option<serde_json::Value>,
    #[serde(rename = "pending_setup_intent")]
    ///You can use this [SetupIntent](https://stripe.com/docs/api/setup_intents) to collect user authentication when creating a subscription without immediate payment or updating a subscription's payment method, allowing you to optimize for off-session payments. Learn more in the [SCA Migration Guide](https://stripe.com/docs/billing/migration/strong-customer-authentication#scenario-2).
    pub pending_setup_intent: Option<serde_json::Value>,
    #[serde(rename = "pending_update")]
    ///If specified, [pending updates](https://stripe.com/docs/billing/subscriptions/pending-updates) that will be applied to the subscription once the `latest_invoice` has been paid.
    pub pending_update: Option<serde_json::Value>,
    #[serde(rename = "schedule")]
    ///The schedule attached to the subscription
    pub schedule: Option<serde_json::Value>,
    #[serde(rename = "start_date")]
    ///Date when the subscription was first created. The date might differ from the `created` date due to backdating.
    pub start_date: i64,
    #[serde(rename = "status")]
    /**Possible values are `incomplete`, `incomplete_expired`, `trialing`, `active`, `past_due`, `canceled`, or `unpaid`.

For `collection_method=charge_automatically` a subscription moves into `incomplete` if the initial payment attempt fails. A subscription in this state can only have metadata and default_source updated. Once the first invoice is paid, the subscription moves into an `active` state. If the first invoice is not paid within 23 hours, the subscription transitions to `incomplete_expired`. This is a terminal state, the open invoice will be voided and no further invoices will be generated.

A subscription that is currently in a trial period is `trialing` and moves to `active` when the trial period is over.

If subscription `collection_method=charge_automatically` it becomes `past_due` when payment to renew it fails and `canceled` or `unpaid` (depending on your subscriptions settings) when Stripe has exhausted all payment retry attempts.

If subscription `collection_method=send_invoice` it becomes `past_due` when its invoice is not paid by the due date, and `canceled` or `unpaid` if it is still not paid by an additional deadline after that. Note that when a subscription has a status of `unpaid`, no subsequent invoices will be attempted (invoices will be created, but then immediately automatically closed). After receiving updated payment information from a customer, you may choose to reopen and pay their closed invoices.*/
    pub status: String,
    #[serde(rename = "test_clock")]
    ///ID of the test clock this subscription belongs to.
    pub test_clock: Option<serde_json::Value>,
    #[serde(rename = "transfer_data")]
    ///The account (if any) the subscription's payments will be attributed to for tax reporting, and where funds from each payment will be transferred to for each of the subscription's invoices.
    pub transfer_data: Option<serde_json::Value>,
    #[serde(rename = "trial_end")]
    ///If the subscription has a trial, the end of that trial.
    pub trial_end: Option<i64>,
    #[serde(rename = "trial_start")]
    ///If the subscription has a trial, the beginning of that trial.
    pub trial_start: Option<i64>,
}
impl std::fmt::Display for Subscription {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SubscriptionAutomaticTax {
    #[serde(rename = "enabled")]
    ///Whether Stripe automatically computes tax on this subscription.
    pub enabled: bool,
}
impl std::fmt::Display for SubscriptionAutomaticTax {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SubscriptionBillingThresholds {
    #[serde(rename = "amount_gte")]
    ///Monetary threshold that triggers the subscription to create an invoice
    pub amount_gte: Option<i64>,
    #[serde(rename = "reset_billing_cycle_anchor")]
    ///Indicates if the `billing_cycle_anchor` should be reset when a threshold is reached. If true, `billing_cycle_anchor` will be updated to the date/time the threshold was last reached; otherwise, the value will remain unchanged. This value may not be `true` if the subscription contains items with plans that have `aggregate_usage=last_ever`.
    pub reset_billing_cycle_anchor: Option<bool>,
}
impl std::fmt::Display for SubscriptionBillingThresholds {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SubscriptionItem {
    #[serde(rename = "billing_thresholds")]
    ///Define thresholds at which an invoice will be sent, and the related subscription advanced to a new billing period
    pub billing_thresholds: Option<serde_json::Value>,
    #[serde(rename = "created")]
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "metadata")]
    ///Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format.
    pub metadata: serde_json::Value,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "price")]
    /**Prices define the unit cost, currency, and (optional) billing cycle for both recurring and one-time purchases of products.
[Products](https://stripe.com/docs/api#products) help you track inventory or provisioning, and prices help you track payment terms. Different physical goods or levels of service should be represented by products, and pricing options should be represented by prices. This approach lets you change prices without having to change your provisioning scheme.

For example, you might have a single "gold" product that has prices for $10/month, $100/year, and €9 once.

Related guides: [Set up a subscription](https://stripe.com/docs/billing/subscriptions/set-up-subscription), [create an invoice](https://stripe.com/docs/billing/invoices/create), and more about [products and prices](https://stripe.com/docs/products-prices/overview).*/
    pub price: Price,
    #[serde(rename = "quantity")]
    ///The [quantity](https://stripe.com/docs/subscriptions/quantities) of the plan to which the customer should be subscribed.
    pub quantity: Option<i64>,
    #[serde(rename = "subscription")]
    ///The `subscription` this `subscription_item` belongs to.
    pub subscription: String,
    #[serde(rename = "tax_rates")]
    ///The tax rates which apply to this `subscription_item`. When set, the `default_tax_rates` on the subscription do not apply to this `subscription_item`.
    pub tax_rates: Option<Vec<TaxRate>>,
}
impl std::fmt::Display for SubscriptionItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SubscriptionItemBillingThresholds {
    #[serde(rename = "usage_gte")]
    ///Usage threshold that triggers the subscription to create an invoice
    pub usage_gte: Option<i64>,
}
impl std::fmt::Display for SubscriptionItemBillingThresholds {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SubscriptionPaymentMethodOptionsCard {
    #[serde(rename = "mandate_options")]
    ///
    pub mandate_options: Option<InvoiceMandateOptionsCard>,
    #[serde(rename = "request_three_d_secure")]
    ///We strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based on risk level and [other requirements](https://stripe.com/docs/strong-customer-authentication). However, if you wish to request 3D Secure based on logic from your own fraud engine, provide this option. Read our guide on [manually requesting 3D Secure](https://stripe.com/docs/payments/3d-secure#manual-three-ds) for more information on how this configuration interacts with Radar and our SCA Engine.
    pub request_three_d_secure: Option<String>,
}
impl std::fmt::Display for SubscriptionPaymentMethodOptionsCard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SubscriptionPendingInvoiceItemInterval {
    #[serde(rename = "interval")]
    ///Specifies invoicing frequency. Either `day`, `week`, `month` or `year`.
    pub interval: String,
    #[serde(rename = "interval_count")]
    ///The number of intervals between invoices. For example, `interval=month` and `interval_count=3` bills every 3 months. Maximum of one year interval allowed (1 year, 12 months, or 52 weeks).
    pub interval_count: i64,
}
impl std::fmt::Display for SubscriptionPendingInvoiceItemInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SubscriptionSchedule {
    #[serde(rename = "application")]
    ///ID of the Connect Application that created the schedule.
    pub application: Option<serde_json::Value>,
    #[serde(rename = "canceled_at")]
    ///Time at which the subscription schedule was canceled. Measured in seconds since the Unix epoch.
    pub canceled_at: Option<i64>,
    #[serde(rename = "completed_at")]
    ///Time at which the subscription schedule was completed. Measured in seconds since the Unix epoch.
    pub completed_at: Option<i64>,
    #[serde(rename = "created")]
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    #[serde(rename = "current_phase")]
    ///Object representing the start and end dates for the current phase of the subscription schedule, if it is `active`.
    pub current_phase: Option<serde_json::Value>,
    #[serde(rename = "customer")]
    ///ID of the customer who owns the subscription schedule.
    pub customer: serde_json::Value,
    #[serde(rename = "default_settings")]
    ///
    pub default_settings: SubscriptionSchedulesResourceDefaultSettings,
    #[serde(rename = "end_behavior")]
    ///Behavior of the subscription schedule and underlying subscription when it ends. Possible values are `release` and `cancel`.
    pub end_behavior: String,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "livemode")]
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    #[serde(rename = "metadata")]
    ///Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<serde_json::Value>,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "phases")]
    ///Configuration for the subscription schedule's phases.
    pub phases: Vec<SubscriptionSchedulePhaseConfiguration>,
    #[serde(rename = "released_at")]
    ///Time at which the subscription schedule was released. Measured in seconds since the Unix epoch.
    pub released_at: Option<i64>,
    #[serde(rename = "released_subscription")]
    ///ID of the subscription once managed by the subscription schedule (if it is released).
    pub released_subscription: Option<String>,
    #[serde(rename = "status")]
    ///The present status of the subscription schedule. Possible values are `not_started`, `active`, `completed`, `released`, and `canceled`. You can read more about the different states in our [behavior guide](https://stripe.com/docs/billing/subscriptions/subscription-schedules).
    pub status: String,
    #[serde(rename = "subscription")]
    ///ID of the subscription managed by the subscription schedule.
    pub subscription: Option<serde_json::Value>,
    #[serde(rename = "test_clock")]
    ///ID of the test clock this subscription schedule belongs to.
    pub test_clock: Option<serde_json::Value>,
}
impl std::fmt::Display for SubscriptionSchedule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SubscriptionScheduleAddInvoiceItem {
    #[serde(rename = "price")]
    ///ID of the price used to generate the invoice item.
    pub price: serde_json::Value,
    #[serde(rename = "quantity")]
    ///The quantity of the invoice item.
    pub quantity: Option<i64>,
    #[serde(rename = "tax_rates")]
    ///The tax rates which apply to the item. When set, the `default_tax_rates` do not apply to this item.
    pub tax_rates: Option<Vec<TaxRate>>,
}
impl std::fmt::Display for SubscriptionScheduleAddInvoiceItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SubscriptionScheduleConfigurationItem {
    #[serde(rename = "billing_thresholds")]
    ///Define thresholds at which an invoice will be sent, and the related subscription advanced to a new billing period
    pub billing_thresholds: Option<serde_json::Value>,
    #[serde(rename = "price")]
    ///ID of the price to which the customer should be subscribed.
    pub price: serde_json::Value,
    #[serde(rename = "quantity")]
    ///Quantity of the plan to which the customer should be subscribed.
    pub quantity: Option<i64>,
    #[serde(rename = "tax_rates")]
    ///The tax rates which apply to this `phase_item`. When set, the `default_tax_rates` on the phase do not apply to this `phase_item`.
    pub tax_rates: Option<Vec<TaxRate>>,
}
impl std::fmt::Display for SubscriptionScheduleConfigurationItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SubscriptionScheduleCurrentPhase {
    #[serde(rename = "end_date")]
    ///The end of this phase of the subscription schedule.
    pub end_date: i64,
    #[serde(rename = "start_date")]
    ///The start of this phase of the subscription schedule.
    pub start_date: i64,
}
impl std::fmt::Display for SubscriptionScheduleCurrentPhase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SubscriptionSchedulePhaseConfiguration {
    #[serde(rename = "add_invoice_items")]
    ///A list of prices and quantities that will generate invoice items appended to the next invoice for this phase.
    pub add_invoice_items: Vec<SubscriptionScheduleAddInvoiceItem>,
    #[serde(rename = "application_fee_percent")]
    ///A non-negative decimal between 0 and 100, with at most two decimal places. This represents the percentage of the subscription invoice subtotal that will be transferred to the application owner's Stripe account during this phase of the schedule.
    pub application_fee_percent: Option<f64>,
    #[serde(rename = "automatic_tax")]
    ///
    pub automatic_tax: Option<SchedulesPhaseAutomaticTax>,
    #[serde(rename = "billing_cycle_anchor")]
    ///Possible values are `phase_start` or `automatic`. If `phase_start` then billing cycle anchor of the subscription is set to the start of the phase when entering the phase. If `automatic` then the billing cycle anchor is automatically modified as needed when entering the phase. For more information, see the billing cycle [documentation](https://stripe.com/docs/billing/subscriptions/billing-cycle).
    pub billing_cycle_anchor: Option<String>,
    #[serde(rename = "billing_thresholds")]
    ///Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period
    pub billing_thresholds: Option<serde_json::Value>,
    #[serde(rename = "collection_method")]
    ///Either `charge_automatically`, or `send_invoice`. When charging automatically, Stripe will attempt to pay the underlying subscription at the end of each billing cycle using the default source attached to the customer. When sending an invoice, Stripe will email your customer an invoice with payment instructions.
    pub collection_method: Option<String>,
    #[serde(rename = "coupon")]
    ///ID of the coupon to use during this phase of the subscription schedule.
    pub coupon: Option<serde_json::Value>,
    #[serde(rename = "currency")]
    ///Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Option<String>,
    #[serde(rename = "default_payment_method")]
    ///ID of the default payment method for the subscription schedule. It must belong to the customer associated with the subscription schedule. If not set, invoices will use the default payment method in the customer's invoice settings.
    pub default_payment_method: Option<serde_json::Value>,
    #[serde(rename = "default_tax_rates")]
    ///The default tax rates to apply to the subscription during this phase of the subscription schedule.
    pub default_tax_rates: Option<Vec<TaxRate>>,
    #[serde(rename = "end_date")]
    ///The end of this phase of the subscription schedule.
    pub end_date: i64,
    #[serde(rename = "invoice_settings")]
    ///The invoice settings applicable during this phase.
    pub invoice_settings: Option<serde_json::Value>,
    #[serde(rename = "items")]
    ///Subscription items to configure the subscription to during this phase of the subscription schedule.
    pub items: Vec<SubscriptionScheduleConfigurationItem>,
    #[serde(rename = "metadata")]
    ///Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to a phase. Metadata on a schedule's phase will update the underlying subscription's `metadata` when the phase is entered. Updating the underlying subscription's `metadata` directly will not affect the current phase's `metadata`.
    pub metadata: Option<serde_json::Value>,
    #[serde(rename = "proration_behavior")]
    ///If the subscription schedule will prorate when transitioning to this phase. Possible values are `create_prorations` and `none`.
    pub proration_behavior: String,
    #[serde(rename = "start_date")]
    ///The start of this phase of the subscription schedule.
    pub start_date: i64,
    #[serde(rename = "transfer_data")]
    ///The account (if any) the associated subscription's payments will be attributed to for tax reporting, and where funds from each payment will be transferred to for each of the subscription's invoices.
    pub transfer_data: Option<serde_json::Value>,
    #[serde(rename = "trial_end")]
    ///When the trial ends within the phase.
    pub trial_end: Option<i64>,
}
impl std::fmt::Display for SubscriptionSchedulePhaseConfiguration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SubscriptionSchedulesResourceDefaultSettings {
    #[serde(rename = "application_fee_percent")]
    ///A non-negative decimal between 0 and 100, with at most two decimal places. This represents the percentage of the subscription invoice subtotal that will be transferred to the application owner's Stripe account during this phase of the schedule.
    pub application_fee_percent: Option<f64>,
    #[serde(rename = "automatic_tax")]
    ///
    pub automatic_tax: Option<SubscriptionSchedulesResourceDefaultSettingsAutomaticTax>,
    #[serde(rename = "billing_cycle_anchor")]
    ///Possible values are `phase_start` or `automatic`. If `phase_start` then billing cycle anchor of the subscription is set to the start of the phase when entering the phase. If `automatic` then the billing cycle anchor is automatically modified as needed when entering the phase. For more information, see the billing cycle [documentation](https://stripe.com/docs/billing/subscriptions/billing-cycle).
    pub billing_cycle_anchor: String,
    #[serde(rename = "billing_thresholds")]
    ///Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period
    pub billing_thresholds: Option<serde_json::Value>,
    #[serde(rename = "collection_method")]
    ///Either `charge_automatically`, or `send_invoice`. When charging automatically, Stripe will attempt to pay the underlying subscription at the end of each billing cycle using the default source attached to the customer. When sending an invoice, Stripe will email your customer an invoice with payment instructions.
    pub collection_method: Option<String>,
    #[serde(rename = "default_payment_method")]
    ///ID of the default payment method for the subscription schedule. If not set, invoices will use the default payment method in the customer's invoice settings.
    pub default_payment_method: Option<serde_json::Value>,
    #[serde(rename = "invoice_settings")]
    ///The subscription schedule's default invoice settings.
    pub invoice_settings: Option<serde_json::Value>,
    #[serde(rename = "transfer_data")]
    ///The account (if any) the associated subscription's payments will be attributed to for tax reporting, and where funds from each payment will be transferred to for each of the subscription's invoices.
    pub transfer_data: Option<serde_json::Value>,
}
impl std::fmt::Display for SubscriptionSchedulesResourceDefaultSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SubscriptionSchedulesResourceDefaultSettingsAutomaticTax {
    #[serde(rename = "enabled")]
    ///Whether Stripe automatically computes tax on invoices created during this phase.
    pub enabled: bool,
}
impl std::fmt::Display for SubscriptionSchedulesResourceDefaultSettingsAutomaticTax {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SubscriptionTransferData {
    #[serde(rename = "amount_percent")]
    ///A non-negative decimal between 0 and 100, with at most two decimal places. This represents the percentage of the subscription invoice subtotal that will be transferred to the destination account. By default, the entire amount is transferred to the destination.
    pub amount_percent: Option<f64>,
    #[serde(rename = "destination")]
    ///The account where funds from the payment will be transferred to upon payment success.
    pub destination: serde_json::Value,
}
impl std::fmt::Display for SubscriptionTransferData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SubscriptionsResourcePauseCollection {
    #[serde(rename = "behavior")]
    ///The payment collection behavior for this subscription while paused. One of `keep_as_draft`, `mark_uncollectible`, or `void`.
    pub behavior: String,
    #[serde(rename = "resumes_at")]
    ///The time after which the subscription will resume collecting payments.
    pub resumes_at: Option<i64>,
}
impl std::fmt::Display for SubscriptionsResourcePauseCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SubscriptionsResourcePaymentMethodOptions {
    #[serde(rename = "acss_debit")]
    ///This sub-hash contains details about the Canadian pre-authorized debit payment method options to pass to invoices created by the subscription.
    pub acss_debit: Option<serde_json::Value>,
    #[serde(rename = "bancontact")]
    ///This sub-hash contains details about the Bancontact payment method options to pass to invoices created by the subscription.
    pub bancontact: Option<serde_json::Value>,
    #[serde(rename = "card")]
    ///This sub-hash contains details about the Card payment method options to pass to invoices created by the subscription.
    pub card: Option<serde_json::Value>,
    #[serde(rename = "customer_balance")]
    ///This sub-hash contains details about the Bank transfer payment method options to pass to invoices created by the subscription.
    pub customer_balance: Option<serde_json::Value>,
    #[serde(rename = "konbini")]
    ///This sub-hash contains details about the Konbini payment method options to pass to invoices created by the subscription.
    pub konbini: Option<serde_json::Value>,
    #[serde(rename = "us_bank_account")]
    ///This sub-hash contains details about the ACH direct debit payment method options to pass to invoices created by the subscription.
    pub us_bank_account: Option<serde_json::Value>,
}
impl std::fmt::Display for SubscriptionsResourcePaymentMethodOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SubscriptionsResourcePaymentSettings {
    #[serde(rename = "payment_method_options")]
    ///Payment-method-specific configuration to provide to invoices created by the subscription.
    pub payment_method_options: Option<serde_json::Value>,
    #[serde(rename = "payment_method_types")]
    ///The list of payment method types to provide to every invoice created by the subscription. If not set, Stripe attempts to automatically determine the types to use by looking at the invoice’s default payment method, the subscription’s default payment method, the customer’s default payment method, and your [invoice template settings](https://dashboard.stripe.com/settings/billing/invoice).
    pub payment_method_types: Option<Vec<String>>,
    #[serde(rename = "save_default_payment_method")]
    ///Either `off`, or `on_subscription`. With `on_subscription` Stripe updates `subscription.default_payment_method` when a subscription payment succeeds.
    pub save_default_payment_method: Option<String>,
}
impl std::fmt::Display for SubscriptionsResourcePaymentSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SubscriptionsResourcePendingUpdate {
    #[serde(rename = "billing_cycle_anchor")]
    ///If the update is applied, determines the date of the first full invoice, and, for plans with `month` or `year` intervals, the day of the month for subsequent invoices. The timestamp is in UTC format.
    pub billing_cycle_anchor: Option<i64>,
    #[serde(rename = "expires_at")]
    ///The point after which the changes reflected by this update will be discarded and no longer applied.
    pub expires_at: i64,
    #[serde(rename = "subscription_items")]
    ///List of subscription items, each with an attached plan, that will be set if the update is applied.
    pub subscription_items: Option<Vec<SubscriptionItem>>,
    #[serde(rename = "trial_end")]
    ///Unix timestamp representing the end of the trial period the customer will get before being charged for the first time, if the update is applied.
    pub trial_end: Option<i64>,
    #[serde(rename = "trial_from_plan")]
    ///Indicates if a plan's `trial_period_days` should be applied to the subscription. Setting `trial_end` per subscription is preferred, and this defaults to `false`. Setting this flag to `true` together with `trial_end` is not allowed. See [Using trial periods on subscriptions](https://stripe.com/docs/billing/subscriptions/trials) to learn more.
    pub trial_from_plan: Option<bool>,
}
impl std::fmt::Display for SubscriptionsResourcePendingUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TaxCode {
    #[serde(rename = "description")]
    ///A detailed description of which types of products the tax code represents.
    pub description: String,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "name")]
    ///A short name for the tax code.
    pub name: String,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
}
impl std::fmt::Display for TaxCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TaxDeductedAtSource {
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "period_end")]
    ///The end of the invoicing period. This TDS applies to Stripe fees collected during this invoicing period.
    pub period_end: i64,
    #[serde(rename = "period_start")]
    ///The start of the invoicing period. This TDS applies to Stripe fees collected during this invoicing period.
    pub period_start: i64,
    #[serde(rename = "tax_deduction_account_number")]
    ///The TAN that was supplied to Stripe when TDS was assessed
    pub tax_deduction_account_number: String,
}
impl std::fmt::Display for TaxDeductedAtSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TaxId {
    #[serde(rename = "country")]
    ///Two-letter ISO code representing the country of the tax ID.
    pub country: Option<String>,
    #[serde(rename = "created")]
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    #[serde(rename = "customer")]
    ///ID of the customer.
    pub customer: Option<serde_json::Value>,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "livemode")]
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "type")]
    ///Type of the tax ID, one of `ae_trn`, `au_abn`, `au_arn`, `bg_uic`, `br_cnpj`, `br_cpf`, `ca_bn`, `ca_gst_hst`, `ca_pst_bc`, `ca_pst_mb`, `ca_pst_sk`, `ca_qst`, `ch_vat`, `cl_tin`, `es_cif`, `eu_oss_vat`, `eu_vat`, `gb_vat`, `ge_vat`, `hk_br`, `hu_tin`, `id_npwp`, `il_vat`, `in_gst`, `is_vat`, `jp_cn`, `jp_rn`, `kr_brn`, `li_uid`, `mx_rfc`, `my_frp`, `my_itn`, `my_sst`, `no_vat`, `nz_gst`, `ru_inn`, `ru_kpp`, `sa_vat`, `sg_gst`, `sg_uen`, `si_tin`, `th_vat`, `tw_vat`, `ua_vat`, `us_ein`, or `za_vat`. Note that some legacy tax IDs have type `unknown`
    pub type_: String,
    #[serde(rename = "value")]
    ///Value of the tax ID.
    pub value: String,
    #[serde(rename = "verification")]
    ///Tax ID verification information.
    pub verification: Option<serde_json::Value>,
}
impl std::fmt::Display for TaxId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TaxIdVerification {
    #[serde(rename = "status")]
    ///Verification status, one of `pending`, `verified`, `unverified`, or `unavailable`.
    pub status: String,
    #[serde(rename = "verified_address")]
    ///Verified address.
    pub verified_address: Option<String>,
    #[serde(rename = "verified_name")]
    ///Verified name.
    pub verified_name: Option<String>,
}
impl std::fmt::Display for TaxIdVerification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TaxRate {
    #[serde(rename = "active")]
    ///Defaults to `true`. When set to `false`, this tax rate cannot be used with new applications or Checkout Sessions, but will still work for subscriptions and invoices that already have it set.
    pub active: bool,
    #[serde(rename = "country")]
    ///Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    pub country: Option<String>,
    #[serde(rename = "created")]
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    #[serde(rename = "description")]
    ///An arbitrary string attached to the tax rate for your internal use only. It will not be visible to your customers.
    pub description: Option<String>,
    #[serde(rename = "display_name")]
    ///The display name of the tax rates as it will appear to your customer on their receipt email, PDF, and the hosted invoice page.
    pub display_name: String,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "inclusive")]
    ///This specifies if the tax rate is inclusive or exclusive.
    pub inclusive: bool,
    #[serde(rename = "jurisdiction")]
    ///The jurisdiction for the tax rate. You can use this label field for tax reporting purposes. It also appears on your customer’s invoice.
    pub jurisdiction: Option<String>,
    #[serde(rename = "livemode")]
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    #[serde(rename = "metadata")]
    ///Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<serde_json::Value>,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "percentage")]
    ///This represents the tax rate percent out of 100.
    pub percentage: f64,
    #[serde(rename = "state")]
    ///[ISO 3166-2 subdivision code](https://en.wikipedia.org/wiki/ISO_3166-2:US), without country prefix. For example, "NY" for New York, United States.
    pub state: Option<String>,
    #[serde(rename = "tax_type")]
    ///The high-level tax type, such as `vat` or `sales_tax`.
    pub tax_type: Option<String>,
}
impl std::fmt::Display for TaxRate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TerminalConfiguration {
    #[serde(rename = "bbpos_wisepos_e")]
    ///
    pub bbpos_wisepos_e: Option<
        TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfig,
    >,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "is_account_default")]
    ///Whether this Configuration is the default for your account
    pub is_account_default: Option<bool>,
    #[serde(rename = "livemode")]
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "tipping")]
    ///
    pub tipping: Option<TerminalConfigurationConfigurationResourceTipping>,
    #[serde(rename = "verifone_p400")]
    ///
    pub verifone_p400: Option<
        TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfig,
    >,
}
impl std::fmt::Display for TerminalConfiguration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TerminalConnectionToken {
    #[serde(rename = "location")]
    ///The id of the location that this connection token is scoped to. Note that location scoping only applies to internet-connected readers. For more details, see [the docs on scoping connection tokens](https://stripe.com/docs/terminal/fleet/locations#connection-tokens).
    pub location: Option<String>,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "secret")]
    ///Your application should pass this token to the Stripe Terminal SDK.
    pub secret: String,
}
impl std::fmt::Display for TerminalConnectionToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TerminalLocation {
    #[serde(rename = "address")]
    ///
    pub address: Address,
    #[serde(rename = "configuration_overrides")]
    ///The ID of a configuration that will be used to customize all readers in this location.
    pub configuration_overrides: Option<String>,
    #[serde(rename = "display_name")]
    ///The display name of the location.
    pub display_name: String,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "livemode")]
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    #[serde(rename = "metadata")]
    ///Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format.
    pub metadata: serde_json::Value,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
}
impl std::fmt::Display for TerminalLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TerminalReader {
    #[serde(rename = "action")]
    ///The most recent action performed by the reader.
    pub action: Option<serde_json::Value>,
    #[serde(rename = "device_sw_version")]
    ///The current software version of the reader.
    pub device_sw_version: Option<String>,
    #[serde(rename = "device_type")]
    ///Type of reader, one of `bbpos_wisepad3`, `stripe_m2`, `bbpos_chipper2x`, `bbpos_wisepos_e`, `verifone_P400`, or `simulated_wisepos_e`.
    pub device_type: String,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "ip_address")]
    ///The local IP address of the reader.
    pub ip_address: Option<String>,
    #[serde(rename = "label")]
    ///Custom label given to the reader for easier identification.
    pub label: String,
    #[serde(rename = "livemode")]
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    #[serde(rename = "location")]
    ///The location identifier of the reader.
    pub location: Option<serde_json::Value>,
    #[serde(rename = "metadata")]
    ///Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format.
    pub metadata: serde_json::Value,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "serial_number")]
    ///Serial number of the reader.
    pub serial_number: String,
    #[serde(rename = "status")]
    ///The networking status of the reader.
    pub status: Option<String>,
}
impl std::fmt::Display for TerminalReader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TerminalConfigurationConfigurationResourceCurrencySpecificConfig {
    #[serde(rename = "fixed_amounts")]
    ///Fixed amounts displayed when collecting a tip
    pub fixed_amounts: Option<Vec<i64>>,
    #[serde(rename = "percentages")]
    ///Percentages displayed when collecting a tip
    pub percentages: Option<Vec<i64>>,
    #[serde(rename = "smart_tip_threshold")]
    ///Below this amount, fixed amounts will be displayed; above it, percentages will be displayed
    pub smart_tip_threshold: Option<i64>,
}
impl std::fmt::Display
for TerminalConfigurationConfigurationResourceCurrencySpecificConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfig {
    #[serde(rename = "splashscreen")]
    ///A File ID representing an image you would like displayed on the reader.
    pub splashscreen: Option<serde_json::Value>,
}
impl std::fmt::Display
for TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TerminalConfigurationConfigurationResourceTipping {
    #[serde(rename = "aud")]
    ///
    pub aud: Option<TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    #[serde(rename = "cad")]
    ///
    pub cad: Option<TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    #[serde(rename = "chf")]
    ///
    pub chf: Option<TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    #[serde(rename = "czk")]
    ///
    pub czk: Option<TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    #[serde(rename = "dkk")]
    ///
    pub dkk: Option<TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    #[serde(rename = "eur")]
    ///
    pub eur: Option<TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    #[serde(rename = "gbp")]
    ///
    pub gbp: Option<TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    #[serde(rename = "hkd")]
    ///
    pub hkd: Option<TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    #[serde(rename = "myr")]
    ///
    pub myr: Option<TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    #[serde(rename = "nok")]
    ///
    pub nok: Option<TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    #[serde(rename = "nzd")]
    ///
    pub nzd: Option<TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    #[serde(rename = "sek")]
    ///
    pub sek: Option<TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    #[serde(rename = "sgd")]
    ///
    pub sgd: Option<TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    #[serde(rename = "usd")]
    ///
    pub usd: Option<TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
}
impl std::fmt::Display for TerminalConfigurationConfigurationResourceTipping {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TerminalReaderReaderResourceCart {
    #[serde(rename = "currency")]
    ///Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: String,
    #[serde(rename = "line_items")]
    ///List of line items in the cart.
    pub line_items: Vec<TerminalReaderReaderResourceLineItem>,
    #[serde(rename = "tax")]
    ///Tax amount for the entire cart. A positive integer in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    pub tax: Option<i64>,
    #[serde(rename = "total")]
    ///Total amount for the entire cart, including tax. A positive integer in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    pub total: i64,
}
impl std::fmt::Display for TerminalReaderReaderResourceCart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TerminalReaderReaderResourceLineItem {
    #[serde(rename = "amount")]
    ///The amount of the line item. A positive integer in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    pub amount: i64,
    #[serde(rename = "description")]
    ///Description of the line item.
    pub description: String,
    #[serde(rename = "quantity")]
    ///The quantity of the line item.
    pub quantity: i64,
}
impl std::fmt::Display for TerminalReaderReaderResourceLineItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TerminalReaderReaderResourceProcessConfig {
    #[serde(rename = "skip_tipping")]
    ///Override showing a tipping selection screen on this transaction.
    pub skip_tipping: Option<bool>,
}
impl std::fmt::Display for TerminalReaderReaderResourceProcessConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TerminalReaderReaderResourceProcessPaymentIntentAction {
    #[serde(rename = "payment_intent")]
    ///Most recent PaymentIntent processed by the reader.
    pub payment_intent: serde_json::Value,
    #[serde(rename = "process_config")]
    ///Represents a per-transaction override of a reader configuration
    pub process_config: Option<TerminalReaderReaderResourceProcessConfig>,
}
impl std::fmt::Display for TerminalReaderReaderResourceProcessPaymentIntentAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TerminalReaderReaderResourceProcessSetupIntentAction {
    #[serde(rename = "generated_card")]
    pub generated_card: Option<String>,
    #[serde(rename = "setup_intent")]
    ///Most recent SetupIntent processed by the reader.
    pub setup_intent: serde_json::Value,
}
impl std::fmt::Display for TerminalReaderReaderResourceProcessSetupIntentAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TerminalReaderReaderResourceReaderAction {
    #[serde(rename = "failure_code")]
    ///Failure code, only set if status is `failed`.
    pub failure_code: Option<String>,
    #[serde(rename = "failure_message")]
    ///Detailed failure message, only set if status is `failed`.
    pub failure_message: Option<String>,
    #[serde(rename = "process_payment_intent")]
    ///Represents a reader action to process a payment intent
    pub process_payment_intent: Option<
        TerminalReaderReaderResourceProcessPaymentIntentAction,
    >,
    #[serde(rename = "process_setup_intent")]
    ///Represents a reader action to process a setup intent
    pub process_setup_intent: Option<
        TerminalReaderReaderResourceProcessSetupIntentAction,
    >,
    #[serde(rename = "set_reader_display")]
    ///Represents a reader action to set the reader display
    pub set_reader_display: Option<TerminalReaderReaderResourceSetReaderDisplayAction>,
    #[serde(rename = "status")]
    ///Status of the action performed by the reader.
    pub status: String,
    #[serde(rename = "type")]
    ///Type of action performed by the reader.
    pub type_: String,
}
impl std::fmt::Display for TerminalReaderReaderResourceReaderAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TerminalReaderReaderResourceSetReaderDisplayAction {
    #[serde(rename = "cart")]
    ///Cart object to be displayed by the reader.
    pub cart: Option<serde_json::Value>,
    #[serde(rename = "type")]
    ///Type of information to be displayed by the reader.
    pub type_: String,
}
impl std::fmt::Display for TerminalReaderReaderResourceSetReaderDisplayAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TestHelpersTestClock {
    #[serde(rename = "created")]
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    #[serde(rename = "deletes_after")]
    ///Time at which this clock is scheduled to auto delete.
    pub deletes_after: i64,
    #[serde(rename = "frozen_time")]
    ///Time at which all objects belonging to this clock are frozen.
    pub frozen_time: i64,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "livemode")]
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    #[serde(rename = "name")]
    ///The custom name supplied at creation.
    pub name: Option<String>,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "status")]
    ///The status of the Test Clock.
    pub status: String,
}
impl std::fmt::Display for TestHelpersTestClock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ThreeDSecureDetails {
    #[serde(rename = "authentication_flow")]
    /**For authenticated transactions: how the customer was authenticated by
the issuing bank.*/
    pub authentication_flow: Option<String>,
    #[serde(rename = "result")]
    ///Indicates the outcome of 3D Secure authentication.
    pub result: Option<String>,
    #[serde(rename = "result_reason")]
    /**Additional information about why 3D Secure succeeded or failed based
on the `result`.*/
    pub result_reason: Option<String>,
    #[serde(rename = "version")]
    ///The version of 3D Secure that was used.
    pub version: Option<String>,
}
impl std::fmt::Display for ThreeDSecureDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ThreeDSecureUsage {
    #[serde(rename = "supported")]
    ///Whether 3D Secure is supported on this card.
    pub supported: bool,
}
impl std::fmt::Display for ThreeDSecureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    #[serde(rename = "bank_account")]
    /**These bank accounts are payment methods on `Customer` objects.

On the other hand [External Accounts](https://stripe.com/docs/api#external_accounts) are transfer
destinations on `Account` objects for [Custom accounts](https://stripe.com/docs/connect/custom-accounts).
They can be bank accounts or debit cards as well, and are documented in the links above.

Related guide: [Bank Debits and Transfers](https://stripe.com/docs/payments/bank-debits-transfers).*/
    pub bank_account: Option<BankAccount>,
    #[serde(rename = "card")]
    /**You can store multiple cards on a customer in order to charge the customer
later. You can also store multiple debit cards on a recipient in order to
transfer to those cards later.

Related guide: [Card Payments with Sources](https://stripe.com/docs/sources/cards).*/
    pub card: Option<Card>,
    #[serde(rename = "client_ip")]
    ///IP address of the client that generated the token.
    pub client_ip: Option<String>,
    #[serde(rename = "created")]
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "livemode")]
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "type")]
    ///Type of the token: `account`, `bank_account`, `card`, or `pii`.
    pub type_: String,
    #[serde(rename = "used")]
    ///Whether this token has already been used (tokens can be used only once).
    pub used: bool,
}
impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Topup {
    #[serde(rename = "amount")]
    ///Amount transferred.
    pub amount: i64,
    #[serde(rename = "balance_transaction")]
    ///ID of the balance transaction that describes the impact of this top-up on your account balance. May not be specified depending on status of top-up.
    pub balance_transaction: Option<serde_json::Value>,
    #[serde(rename = "created")]
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    #[serde(rename = "currency")]
    ///Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: String,
    #[serde(rename = "description")]
    ///An arbitrary string attached to the object. Often useful for displaying to users.
    pub description: Option<String>,
    #[serde(rename = "expected_availability_date")]
    ///Date the funds are expected to arrive in your Stripe account for payouts. This factors in delays like weekends or bank holidays. May not be specified depending on status of top-up.
    pub expected_availability_date: Option<i64>,
    #[serde(rename = "failure_code")]
    ///Error code explaining reason for top-up failure if available (see [the errors section](https://stripe.com/docs/api#errors) for a list of codes).
    pub failure_code: Option<String>,
    #[serde(rename = "failure_message")]
    ///Message to user further explaining reason for top-up failure if available.
    pub failure_message: Option<String>,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "livemode")]
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    #[serde(rename = "metadata")]
    ///Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format.
    pub metadata: serde_json::Value,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "source")]
    /**`Source` objects allow you to accept a variety of payment methods. They
represent a customer's payment instrument, and can be used with the Stripe API
just like a `Card` object: once chargeable, they can be charged, or can be
attached to customers.

Related guides: [Sources API](https://stripe.com/docs/sources) and [Sources & Customers](https://stripe.com/docs/sources/customers).*/
    pub source: Source,
    #[serde(rename = "statement_descriptor")]
    ///Extra information about a top-up. This will appear on your source's bank statement. It must contain at least one letter.
    pub statement_descriptor: Option<String>,
    #[serde(rename = "status")]
    ///The status of the top-up is either `canceled`, `failed`, `pending`, `reversed`, or `succeeded`.
    pub status: String,
    #[serde(rename = "transfer_group")]
    ///A string that identifies this top-up as part of a group.
    pub transfer_group: Option<String>,
}
impl std::fmt::Display for Topup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Transfer {
    #[serde(rename = "amount")]
    ///Amount in %s to be transferred.
    pub amount: i64,
    #[serde(rename = "amount_reversed")]
    ///Amount in %s reversed (can be less than the amount attribute on the transfer if a partial reversal was issued).
    pub amount_reversed: i64,
    #[serde(rename = "balance_transaction")]
    ///Balance transaction that describes the impact of this transfer on your account balance.
    pub balance_transaction: Option<serde_json::Value>,
    #[serde(rename = "created")]
    ///Time that this record of the transfer was first created.
    pub created: i64,
    #[serde(rename = "currency")]
    ///Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: String,
    #[serde(rename = "description")]
    ///An arbitrary string attached to the object. Often useful for displaying to users.
    pub description: Option<String>,
    #[serde(rename = "destination")]
    ///ID of the Stripe account the transfer was sent to.
    pub destination: Option<serde_json::Value>,
    #[serde(rename = "destination_payment")]
    ///If the destination is a Stripe account, this will be the ID of the payment that the destination account received for the transfer.
    pub destination_payment: Option<serde_json::Value>,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "livemode")]
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    #[serde(rename = "metadata")]
    ///Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format.
    pub metadata: serde_json::Value,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "reversals")]
    ///A list of reversals that have been applied to the transfer.
    pub reversals: serde_json::Value,
    #[serde(rename = "reversed")]
    ///Whether the transfer has been fully reversed. If the transfer is only partially reversed, this attribute will still be false.
    pub reversed: bool,
    #[serde(rename = "source_transaction")]
    ///ID of the charge or payment that was used to fund the transfer. If null, the transfer was funded from the available balance.
    pub source_transaction: Option<serde_json::Value>,
    #[serde(rename = "source_type")]
    ///The source balance this transfer came from. One of `card`, `fpx`, or `bank_account`.
    pub source_type: Option<String>,
    #[serde(rename = "transfer_group")]
    ///A string that identifies this transaction as part of a group. See the [Connect documentation](https://stripe.com/docs/connect/charges-transfers#transfer-options) for details.
    pub transfer_group: Option<String>,
}
impl std::fmt::Display for Transfer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferData {
    #[serde(rename = "amount")]
    ///Amount intended to be collected by this PaymentIntent. A positive integer representing how much to charge in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal) (e.g., 100 cents to charge $1.00 or 100 to charge ¥100, a zero-decimal currency). The minimum amount is $0.50 US or [equivalent in charge currency](https://stripe.com/docs/currencies#minimum-and-maximum-charge-amounts). The amount value supports up to eight digits (e.g., a value of 99999999 for a USD charge of $999,999.99).
    pub amount: Option<i64>,
    #[serde(rename = "destination")]
    /**The account (if any) the payment will be attributed to for tax
reporting, and where funds from the payment will be transferred to upon
payment success.*/
    pub destination: serde_json::Value,
}
impl std::fmt::Display for TransferData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferReversal {
    #[serde(rename = "amount")]
    ///Amount, in %s.
    pub amount: i64,
    #[serde(rename = "balance_transaction")]
    ///Balance transaction that describes the impact on your account balance.
    pub balance_transaction: Option<serde_json::Value>,
    #[serde(rename = "created")]
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    #[serde(rename = "currency")]
    ///Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: String,
    #[serde(rename = "destination_payment_refund")]
    ///Linked payment refund for the transfer reversal.
    pub destination_payment_refund: Option<serde_json::Value>,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "metadata")]
    ///Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<serde_json::Value>,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "source_refund")]
    ///ID of the refund responsible for the transfer reversal.
    pub source_refund: Option<serde_json::Value>,
    #[serde(rename = "transfer")]
    ///ID of the transfer that was reversed.
    pub transfer: serde_json::Value,
}
impl std::fmt::Display for TransferReversal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferSchedule {
    #[serde(rename = "delay_days")]
    ///The number of days charges for the account will be held before being paid out.
    pub delay_days: i64,
    #[serde(rename = "interval")]
    ///How frequently funds will be paid out. One of `manual` (payouts only created via API call), `daily`, `weekly`, or `monthly`.
    pub interval: String,
    #[serde(rename = "monthly_anchor")]
    ///The day of the month funds will be paid out. Only shown if `interval` is monthly. Payouts scheduled between the 29th and 31st of the month are sent on the last day of shorter months.
    pub monthly_anchor: Option<i64>,
    #[serde(rename = "weekly_anchor")]
    ///The day of the week funds will be paid out, of the style 'monday', 'tuesday', etc. Only shown if `interval` is weekly.
    pub weekly_anchor: Option<String>,
}
impl std::fmt::Display for TransferSchedule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransformQuantity {
    #[serde(rename = "divide_by")]
    ///Divide usage by this number.
    pub divide_by: i64,
    #[serde(rename = "round")]
    ///After division, either round the result `up` or `down`.
    pub round: String,
}
impl std::fmt::Display for TransformQuantity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransformUsage {
    #[serde(rename = "divide_by")]
    ///Divide usage by this number.
    pub divide_by: i64,
    #[serde(rename = "round")]
    ///After division, either round the result `up` or `down`.
    pub round: String,
}
impl std::fmt::Display for TransformUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TreasuryCreditReversal {
    #[serde(rename = "amount")]
    ///Amount (in cents) transferred.
    pub amount: i64,
    #[serde(rename = "currency")]
    ///Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: String,
    #[serde(rename = "financial_account")]
    ///The FinancialAccount to reverse funds from.
    pub financial_account: String,
    #[serde(rename = "hosted_regulatory_receipt_url")]
    ///A [hosted transaction receipt](https://stripe.com/docs/treasury/moving-money/regulatory-receipts) URL that is provided when money movement is considered regulated under Stripe's money transmission licenses.
    pub hosted_regulatory_receipt_url: Option<String>,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "livemode")]
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    #[serde(rename = "metadata")]
    ///Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format.
    pub metadata: serde_json::Value,
    #[serde(rename = "network")]
    ///The rails used to reverse the funds.
    pub network: String,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "received_credit")]
    ///The ReceivedCredit being reversed.
    pub received_credit: String,
    #[serde(rename = "status")]
    ///Status of the CreditReversal
    pub status: String,
    #[serde(rename = "status_transitions")]
    ///
    pub status_transitions: TreasuryReceivedCreditsResourceStatusTransitions,
    #[serde(rename = "transaction")]
    ///The Transaction associated with this object.
    pub transaction: Option<serde_json::Value>,
}
impl std::fmt::Display for TreasuryCreditReversal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TreasuryDebitReversal {
    #[serde(rename = "amount")]
    ///Amount (in cents) transferred.
    pub amount: i64,
    #[serde(rename = "currency")]
    ///Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: String,
    #[serde(rename = "financial_account")]
    ///The FinancialAccount to reverse funds from.
    pub financial_account: Option<String>,
    #[serde(rename = "hosted_regulatory_receipt_url")]
    ///A [hosted transaction receipt](https://stripe.com/docs/treasury/moving-money/regulatory-receipts) URL that is provided when money movement is considered regulated under Stripe's money transmission licenses.
    pub hosted_regulatory_receipt_url: Option<String>,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "linked_flows")]
    ///Other flows linked to a DebitReversal.
    pub linked_flows: Option<serde_json::Value>,
    #[serde(rename = "livemode")]
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    #[serde(rename = "metadata")]
    ///Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format.
    pub metadata: serde_json::Value,
    #[serde(rename = "network")]
    ///The rails used to reverse the funds.
    pub network: String,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "received_debit")]
    ///The ReceivedDebit being reversed.
    pub received_debit: String,
    #[serde(rename = "status")]
    ///Status of the DebitReversal
    pub status: String,
    #[serde(rename = "status_transitions")]
    ///
    pub status_transitions: TreasuryReceivedDebitsResourceStatusTransitions,
    #[serde(rename = "transaction")]
    ///The Transaction associated with this object.
    pub transaction: Option<serde_json::Value>,
}
impl std::fmt::Display for TreasuryDebitReversal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TreasuryFinancialAccount {
    #[serde(rename = "active_features")]
    ///The array of paths to active Features in the Features hash.
    pub active_features: Vec<String>,
    #[serde(rename = "balance")]
    ///Balance information for the FinancialAccount
    pub balance: TreasuryFinancialAccountsResourceBalance,
    #[serde(rename = "country")]
    ///Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    pub country: String,
    #[serde(rename = "created")]
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    #[serde(rename = "features")]
    /**Encodes whether a FinancialAccount has access to a particular Feature, with a `status` enum and associated `status_details`.
Stripe or the platform can control Features via the requested field.*/
    pub features: Option<TreasuryFinancialAccountFeatures>,
    #[serde(rename = "financial_addresses")]
    ///The set of credentials that resolve to a FinancialAccount.
    pub financial_addresses: Vec<TreasuryFinancialAccountsResourceFinancialAddress>,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "livemode")]
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    #[serde(rename = "metadata")]
    ///Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<serde_json::Value>,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "pending_features")]
    ///The array of paths to pending Features in the Features hash.
    pub pending_features: Vec<String>,
    #[serde(rename = "platform_restrictions")]
    ///The set of functionalities that the platform can restrict on the FinancialAccount.
    pub platform_restrictions: Option<serde_json::Value>,
    #[serde(rename = "restricted_features")]
    ///The array of paths to restricted Features in the Features hash.
    pub restricted_features: Vec<String>,
    #[serde(rename = "status")]
    ///The enum specifying what state the account is in.
    pub status: String,
    #[serde(rename = "status_details")]
    ///
    pub status_details: TreasuryFinancialAccountsResourceStatusDetails,
    #[serde(rename = "supported_currencies")]
    ///The currencies the FinancialAccount can hold a balance in. Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    pub supported_currencies: Vec<String>,
}
impl std::fmt::Display for TreasuryFinancialAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TreasuryFinancialAccountFeatures {
    #[serde(rename = "card_issuing")]
    ///Toggle settings for enabling/disabling a feature
    pub card_issuing: Option<TreasuryFinancialAccountsResourceToggleSettings>,
    #[serde(rename = "deposit_insurance")]
    ///Toggle settings for enabling/disabling a feature
    pub deposit_insurance: Option<TreasuryFinancialAccountsResourceToggleSettings>,
    #[serde(rename = "financial_addresses")]
    ///Settings related to Financial Addresses features on a Financial Account
    pub financial_addresses: Option<
        TreasuryFinancialAccountsResourceFinancialAddressesFeatures,
    >,
    #[serde(rename = "inbound_transfers")]
    ///InboundTransfers contains inbound transfers features for a FinancialAccount.
    pub inbound_transfers: Option<TreasuryFinancialAccountsResourceInboundTransfers>,
    #[serde(rename = "intra_stripe_flows")]
    ///Toggle settings for enabling/disabling a feature
    pub intra_stripe_flows: Option<TreasuryFinancialAccountsResourceToggleSettings>,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "outbound_payments")]
    ///Settings related to Outbound Payments features on a Financial Account
    pub outbound_payments: Option<TreasuryFinancialAccountsResourceOutboundPayments>,
    #[serde(rename = "outbound_transfers")]
    ///OutboundTransfers contains outbound transfers features for a FinancialAccount.
    pub outbound_transfers: Option<TreasuryFinancialAccountsResourceOutboundTransfers>,
}
impl std::fmt::Display for TreasuryFinancialAccountFeatures {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TreasuryInboundTransfer {
    #[serde(rename = "amount")]
    ///Amount (in cents) transferred.
    pub amount: i64,
    #[serde(rename = "cancelable")]
    ///Returns `true` if the InboundTransfer is able to be canceled.
    pub cancelable: bool,
    #[serde(rename = "created")]
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    #[serde(rename = "currency")]
    ///Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: String,
    #[serde(rename = "description")]
    ///An arbitrary string attached to the object. Often useful for displaying to users.
    pub description: Option<String>,
    #[serde(rename = "failure_details")]
    ///Details about this InboundTransfer's failure. Only set when status is `failed`.
    pub failure_details: Option<serde_json::Value>,
    #[serde(rename = "financial_account")]
    ///The FinancialAccount that received the funds.
    pub financial_account: String,
    #[serde(rename = "hosted_regulatory_receipt_url")]
    ///A [hosted transaction receipt](https://stripe.com/docs/treasury/moving-money/regulatory-receipts) URL that is provided when money movement is considered regulated under Stripe's money transmission licenses.
    pub hosted_regulatory_receipt_url: Option<String>,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "linked_flows")]
    ///
    pub linked_flows: TreasuryInboundTransfersResourceInboundTransferResourceLinkedFlows,
    #[serde(rename = "livemode")]
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    #[serde(rename = "metadata")]
    ///Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format.
    pub metadata: serde_json::Value,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "origin_payment_method")]
    ///The origin payment method to be debited for an InboundTransfer.
    pub origin_payment_method: String,
    #[serde(rename = "origin_payment_method_details")]
    ///Details about the PaymentMethod for an InboundTransfer.
    pub origin_payment_method_details: Option<serde_json::Value>,
    #[serde(rename = "returned")]
    ///Returns `true` if the funds for an InboundTransfer were returned after the InboundTransfer went to the `succeeded` state.
    pub returned: Option<bool>,
    #[serde(rename = "statement_descriptor")]
    ///Statement descriptor shown when funds are debited from the source. Not all payment networks support `statement_descriptor`.
    pub statement_descriptor: String,
    #[serde(rename = "status")]
    ///Status of the InboundTransfer: `processing`, `succeeded`, `failed`, and `canceled`. An InboundTransfer is `processing` if it is created and pending. The status changes to `succeeded` once the funds have been "confirmed" and a `transaction` is created and posted. The status changes to `failed` if the transfer fails.
    pub status: String,
    #[serde(rename = "status_transitions")]
    ///
    pub status_transitions: TreasuryInboundTransfersResourceInboundTransferResourceStatusTransitions,
    #[serde(rename = "transaction")]
    ///The Transaction associated with this object.
    pub transaction: Option<serde_json::Value>,
}
impl std::fmt::Display for TreasuryInboundTransfer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TreasuryOutboundPayment {
    #[serde(rename = "amount")]
    ///Amount (in cents) transferred.
    pub amount: i64,
    #[serde(rename = "cancelable")]
    ///Returns `true` if the object can be canceled, and `false` otherwise.
    pub cancelable: bool,
    #[serde(rename = "created")]
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    #[serde(rename = "currency")]
    ///Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: String,
    #[serde(rename = "customer")]
    ///ID of the [customer](https://stripe.com/docs/api/customers) to whom an OutboundPayment is sent.
    pub customer: Option<String>,
    #[serde(rename = "description")]
    ///An arbitrary string attached to the object. Often useful for displaying to users.
    pub description: Option<String>,
    #[serde(rename = "destination_payment_method")]
    ///The PaymentMethod via which an OutboundPayment is sent. This field can be empty if the OutboundPayment was created using `destination_payment_method_data`.
    pub destination_payment_method: Option<String>,
    #[serde(rename = "destination_payment_method_details")]
    ///Details about the PaymentMethod for an OutboundPayment.
    pub destination_payment_method_details: Option<serde_json::Value>,
    #[serde(rename = "end_user_details")]
    ///Details about the end user.
    pub end_user_details: Option<serde_json::Value>,
    #[serde(rename = "expected_arrival_date")]
    ///The date when funds are expected to arrive in the destination account.
    pub expected_arrival_date: i64,
    #[serde(rename = "financial_account")]
    ///The FinancialAccount that funds were pulled from.
    pub financial_account: String,
    #[serde(rename = "hosted_regulatory_receipt_url")]
    ///A [hosted transaction receipt](https://stripe.com/docs/treasury/moving-money/regulatory-receipts) URL that is provided when money movement is considered regulated under Stripe's money transmission licenses.
    pub hosted_regulatory_receipt_url: Option<String>,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "livemode")]
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    #[serde(rename = "metadata")]
    ///Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format.
    pub metadata: serde_json::Value,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "returned_details")]
    ///Details about a returned OutboundPayment. Only set when the status is `returned`.
    pub returned_details: Option<serde_json::Value>,
    #[serde(rename = "statement_descriptor")]
    ///The description that appears on the receiving end for an OutboundPayment (for example, bank statement for external bank transfer).
    pub statement_descriptor: String,
    #[serde(rename = "status")]
    ///Current status of the OutboundPayment: `processing`, `failed`, `posted`, `returned`, `canceled`. An OutboundPayment is `processing` if it has been created and is pending. The status changes to `posted` once the OutboundPayment has been "confirmed" and funds have left the account, or to `failed` or `canceled`. If an OutboundPayment fails to arrive at its destination, its status will change to `returned`.
    pub status: String,
    #[serde(rename = "status_transitions")]
    ///
    pub status_transitions: TreasuryOutboundPaymentsResourceOutboundPaymentResourceStatusTransitions,
    #[serde(rename = "transaction")]
    ///The Transaction associated with this object.
    pub transaction: serde_json::Value,
}
impl std::fmt::Display for TreasuryOutboundPayment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TreasuryOutboundTransfer {
    #[serde(rename = "amount")]
    ///Amount (in cents) transferred.
    pub amount: i64,
    #[serde(rename = "cancelable")]
    ///Returns `true` if the object can be canceled, and `false` otherwise.
    pub cancelable: bool,
    #[serde(rename = "created")]
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    #[serde(rename = "currency")]
    ///Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: String,
    #[serde(rename = "description")]
    ///An arbitrary string attached to the object. Often useful for displaying to users.
    pub description: Option<String>,
    #[serde(rename = "destination_payment_method")]
    ///The PaymentMethod used as the payment instrument for an OutboundTransfer.
    pub destination_payment_method: String,
    #[serde(rename = "destination_payment_method_details")]
    ///
    pub destination_payment_method_details: OutboundTransfersPaymentMethodDetails,
    #[serde(rename = "expected_arrival_date")]
    ///The date when funds are expected to arrive in the destination account.
    pub expected_arrival_date: i64,
    #[serde(rename = "financial_account")]
    ///The FinancialAccount that funds were pulled from.
    pub financial_account: String,
    #[serde(rename = "hosted_regulatory_receipt_url")]
    ///A [hosted transaction receipt](https://stripe.com/docs/treasury/moving-money/regulatory-receipts) URL that is provided when money movement is considered regulated under Stripe's money transmission licenses.
    pub hosted_regulatory_receipt_url: Option<String>,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "livemode")]
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    #[serde(rename = "metadata")]
    ///Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format.
    pub metadata: serde_json::Value,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "returned_details")]
    ///Details about a returned OutboundTransfer. Only set when the status is `returned`.
    pub returned_details: Option<serde_json::Value>,
    #[serde(rename = "statement_descriptor")]
    ///Information about the OutboundTransfer to be sent to the recipient account.
    pub statement_descriptor: String,
    #[serde(rename = "status")]
    ///Current status of the OutboundTransfer: `processing`, `failed`, `canceled`, `posted`, `returned`. An OutboundTransfer is `processing` if it has been created and is pending. The status changes to `posted` once the OutboundTransfer has been "confirmed" and funds have left the account, or to `failed` or `canceled`. If an OutboundTransfer fails to arrive at its destination, its status will change to `returned`.
    pub status: String,
    #[serde(rename = "status_transitions")]
    ///
    pub status_transitions: TreasuryOutboundTransfersResourceStatusTransitions,
    #[serde(rename = "transaction")]
    ///The Transaction associated with this object.
    pub transaction: serde_json::Value,
}
impl std::fmt::Display for TreasuryOutboundTransfer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TreasuryReceivedCredit {
    #[serde(rename = "amount")]
    ///Amount (in cents) transferred.
    pub amount: i64,
    #[serde(rename = "created")]
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    #[serde(rename = "currency")]
    ///Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: String,
    #[serde(rename = "description")]
    ///An arbitrary string attached to the object. Often useful for displaying to users.
    pub description: String,
    #[serde(rename = "failure_code")]
    ///Reason for the failure. A ReceivedCredit might fail because the receiving FinancialAccount is closed or frozen.
    pub failure_code: Option<String>,
    #[serde(rename = "financial_account")]
    ///The FinancialAccount that received the funds.
    pub financial_account: Option<String>,
    #[serde(rename = "hosted_regulatory_receipt_url")]
    ///A [hosted transaction receipt](https://stripe.com/docs/treasury/moving-money/regulatory-receipts) URL that is provided when money movement is considered regulated under Stripe's money transmission licenses.
    pub hosted_regulatory_receipt_url: Option<String>,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "initiating_payment_method_details")]
    ///
    pub initiating_payment_method_details: TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetails,
    #[serde(rename = "linked_flows")]
    ///
    pub linked_flows: TreasuryReceivedCreditsResourceLinkedFlows,
    #[serde(rename = "livemode")]
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    #[serde(rename = "network")]
    ///The rails used to send the funds.
    pub network: String,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "reversal_details")]
    ///Details describing when a ReceivedCredit may be reversed.
    pub reversal_details: Option<serde_json::Value>,
    #[serde(rename = "status")]
    ///Status of the ReceivedCredit. ReceivedCredits are created either `succeeded` (approved) or `failed` (declined). If a ReceivedCredit is declined, the failure reason can be found in the `failure_code` field.
    pub status: String,
    #[serde(rename = "transaction")]
    ///The Transaction associated with this object.
    pub transaction: Option<serde_json::Value>,
}
impl std::fmt::Display for TreasuryReceivedCredit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TreasuryReceivedDebit {
    #[serde(rename = "amount")]
    ///Amount (in cents) transferred.
    pub amount: i64,
    #[serde(rename = "created")]
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    #[serde(rename = "currency")]
    ///Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: String,
    #[serde(rename = "description")]
    ///An arbitrary string attached to the object. Often useful for displaying to users.
    pub description: String,
    #[serde(rename = "failure_code")]
    ///Reason for the failure. A ReceivedDebit might fail because the FinancialAccount doesn't have sufficient funds, is closed, or is frozen.
    pub failure_code: Option<String>,
    #[serde(rename = "financial_account")]
    ///The FinancialAccount that funds were pulled from.
    pub financial_account: Option<String>,
    #[serde(rename = "hosted_regulatory_receipt_url")]
    ///A [hosted transaction receipt](https://stripe.com/docs/treasury/moving-money/regulatory-receipts) URL that is provided when money movement is considered regulated under Stripe's money transmission licenses.
    pub hosted_regulatory_receipt_url: Option<String>,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "initiating_payment_method_details")]
    ///
    pub initiating_payment_method_details: Option<
        TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetails,
    >,
    #[serde(rename = "linked_flows")]
    ///
    pub linked_flows: TreasuryReceivedDebitsResourceLinkedFlows,
    #[serde(rename = "livemode")]
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    #[serde(rename = "network")]
    ///The network used for the ReceivedDebit.
    pub network: String,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "reversal_details")]
    ///Details describing when a ReceivedDebit might be reversed.
    pub reversal_details: Option<serde_json::Value>,
    #[serde(rename = "status")]
    ///Status of the ReceivedDebit. ReceivedDebits are created with a status of either `succeeded` (approved) or `failed` (declined). The failure reason can be found under the `failure_code`.
    pub status: String,
    #[serde(rename = "transaction")]
    ///The Transaction associated with this object.
    pub transaction: Option<serde_json::Value>,
}
impl std::fmt::Display for TreasuryReceivedDebit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TreasuryTransaction {
    #[serde(rename = "amount")]
    ///Amount (in cents) transferred.
    pub amount: i64,
    #[serde(rename = "balance_impact")]
    ///Change to a FinancialAccount's balance
    pub balance_impact: TreasuryTransactionsResourceBalanceImpact,
    #[serde(rename = "created")]
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    #[serde(rename = "currency")]
    ///Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: String,
    #[serde(rename = "description")]
    ///An arbitrary string attached to the object. Often useful for displaying to users.
    pub description: String,
    #[serde(rename = "entries")]
    ///A list of TransactionEntries that are part of this Transaction. This cannot be expanded in any list endpoints.
    pub entries: Option<serde_json::Value>,
    #[serde(rename = "financial_account")]
    ///The FinancialAccount associated with this object.
    pub financial_account: String,
    #[serde(rename = "flow")]
    ///ID of the flow that created the Transaction.
    pub flow: Option<String>,
    #[serde(rename = "flow_details")]
    ///Details of the flow that created the Transaction.
    pub flow_details: Option<serde_json::Value>,
    #[serde(rename = "flow_type")]
    ///Type of the flow that created the Transaction.
    pub flow_type: String,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "livemode")]
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "status")]
    ///Status of the Transaction.
    pub status: String,
    #[serde(rename = "status_transitions")]
    ///
    pub status_transitions: TreasuryTransactionsResourceAbstractTransactionResourceStatusTransitions,
}
impl std::fmt::Display for TreasuryTransaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TreasuryTransactionEntry {
    #[serde(rename = "balance_impact")]
    ///Change to a FinancialAccount's balance
    pub balance_impact: TreasuryTransactionsResourceBalanceImpact,
    #[serde(rename = "created")]
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    #[serde(rename = "currency")]
    ///Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: String,
    #[serde(rename = "effective_at")]
    ///When the TransactionEntry will impact the FinancialAccount's balance.
    pub effective_at: i64,
    #[serde(rename = "financial_account")]
    ///The FinancialAccount associated with this object.
    pub financial_account: String,
    #[serde(rename = "flow")]
    ///Token of the flow associated with the TransactionEntry.
    pub flow: Option<String>,
    #[serde(rename = "flow_details")]
    ///Details of the flow associated with the TransactionEntry.
    pub flow_details: Option<serde_json::Value>,
    #[serde(rename = "flow_type")]
    ///Type of the flow associated with the TransactionEntry.
    pub flow_type: String,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "livemode")]
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "transaction")]
    ///The Transaction associated with this object.
    pub transaction: serde_json::Value,
    #[serde(rename = "type")]
    ///The specific money movement that generated the TransactionEntry.
    pub type_: String,
}
impl std::fmt::Display for TreasuryTransactionEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TreasuryFinancialAccountsResourceAbaRecord {
    #[serde(rename = "account_holder_name")]
    ///The name of the person or business that owns the bank account.
    pub account_holder_name: String,
    #[serde(rename = "account_number")]
    ///The account number.
    pub account_number: Option<String>,
    #[serde(rename = "account_number_last4")]
    ///The last four characters of the account number.
    pub account_number_last4: String,
    #[serde(rename = "bank_name")]
    ///Name of the bank.
    pub bank_name: String,
    #[serde(rename = "routing_number")]
    ///Routing number for the account.
    pub routing_number: String,
}
impl std::fmt::Display for TreasuryFinancialAccountsResourceAbaRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TreasuryFinancialAccountsResourceBalance {
    #[serde(rename = "cash")]
    ///Funds the user can spend right now.
    pub cash: serde_json::Value,
    #[serde(rename = "inbound_pending")]
    ///Funds not spendable yet, but will become available at a later time.
    pub inbound_pending: serde_json::Value,
    #[serde(rename = "outbound_pending")]
    ///Funds in the account, but not spendable because they are being held for pending outbound flows.
    pub outbound_pending: serde_json::Value,
}
impl std::fmt::Display for TreasuryFinancialAccountsResourceBalance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TreasuryFinancialAccountsResourceClosedStatusDetails {
    #[serde(rename = "reasons")]
    ///The array that contains reasons for a FinancialAccount closure.
    pub reasons: Vec<String>,
}
impl std::fmt::Display for TreasuryFinancialAccountsResourceClosedStatusDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TreasuryFinancialAccountsResourceFinancialAddress {
    #[serde(rename = "aba")]
    ///ABA Records contain U.S. bank account details per the ABA format.
    pub aba: Option<TreasuryFinancialAccountsResourceAbaRecord>,
    #[serde(rename = "supported_networks")]
    ///The list of networks that the address supports
    pub supported_networks: Option<Vec<String>>,
    #[serde(rename = "type")]
    ///The type of financial address
    pub type_: String,
}
impl std::fmt::Display for TreasuryFinancialAccountsResourceFinancialAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TreasuryFinancialAccountsResourceFinancialAddressesFeatures {
    #[serde(rename = "aba")]
    ///Toggle settings for enabling/disabling a feature
    pub aba: Option<TreasuryFinancialAccountsResourceToggleSettings>,
}
impl std::fmt::Display for TreasuryFinancialAccountsResourceFinancialAddressesFeatures {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TreasuryFinancialAccountsResourceInboundTransfers {
    #[serde(rename = "ach")]
    ///Toggle settings for enabling/disabling a feature
    pub ach: Option<TreasuryFinancialAccountsResourceToggleSettings>,
}
impl std::fmt::Display for TreasuryFinancialAccountsResourceInboundTransfers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TreasuryFinancialAccountsResourceOutboundPayments {
    #[serde(rename = "ach")]
    ///Toggle settings for enabling/disabling a feature
    pub ach: Option<TreasuryFinancialAccountsResourceToggleSettings>,
    #[serde(rename = "us_domestic_wire")]
    ///Toggle settings for enabling/disabling a feature
    pub us_domestic_wire: Option<TreasuryFinancialAccountsResourceToggleSettings>,
}
impl std::fmt::Display for TreasuryFinancialAccountsResourceOutboundPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TreasuryFinancialAccountsResourceOutboundTransfers {
    #[serde(rename = "ach")]
    ///Toggle settings for enabling/disabling a feature
    pub ach: Option<TreasuryFinancialAccountsResourceToggleSettings>,
    #[serde(rename = "us_domestic_wire")]
    ///Toggle settings for enabling/disabling a feature
    pub us_domestic_wire: Option<TreasuryFinancialAccountsResourceToggleSettings>,
}
impl std::fmt::Display for TreasuryFinancialAccountsResourceOutboundTransfers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TreasuryFinancialAccountsResourcePlatformRestrictions {
    #[serde(rename = "inbound_flows")]
    ///Restricts all inbound money movement.
    pub inbound_flows: Option<String>,
    #[serde(rename = "outbound_flows")]
    ///Restricts all outbound money movement.
    pub outbound_flows: Option<String>,
}
impl std::fmt::Display for TreasuryFinancialAccountsResourcePlatformRestrictions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TreasuryFinancialAccountsResourceStatusDetails {
    #[serde(rename = "closed")]
    ///Details related to the closure of this FinancialAccount
    pub closed: Option<serde_json::Value>,
}
impl std::fmt::Display for TreasuryFinancialAccountsResourceStatusDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TreasuryFinancialAccountsResourceToggleSettings {
    #[serde(rename = "requested")]
    ///Whether the FinancialAccount should have the Feature.
    pub requested: bool,
    #[serde(rename = "status")]
    ///Whether the Feature is operational.
    pub status: String,
    #[serde(rename = "status_details")]
    ///Additional details; includes at least one entry when the status is not `active`.
    pub status_details: Vec<
        TreasuryFinancialAccountsResourceTogglesSettingStatusDetails,
    >,
}
impl std::fmt::Display for TreasuryFinancialAccountsResourceToggleSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TreasuryFinancialAccountsResourceTogglesSettingStatusDetails {
    #[serde(rename = "code")]
    ///Represents the reason why the status is `pending` or `restricted`.
    pub code: String,
    #[serde(rename = "resolution")]
    ///Represents what the user should do, if anything, to activate the Feature.
    pub resolution: Option<String>,
    #[serde(rename = "restriction")]
    ///The `platform_restrictions` that are restricting this Feature.
    pub restriction: Option<String>,
}
impl std::fmt::Display for TreasuryFinancialAccountsResourceTogglesSettingStatusDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TreasuryInboundTransfersResourceFailureDetails {
    #[serde(rename = "code")]
    ///Reason for the failure.
    pub code: String,
}
impl std::fmt::Display for TreasuryInboundTransfersResourceFailureDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TreasuryInboundTransfersResourceInboundTransferResourceLinkedFlows {
    #[serde(rename = "received_debit")]
    ///If funds for this flow were returned after the flow went to the `succeeded` state, this field contains a reference to the ReceivedDebit return.
    pub received_debit: Option<String>,
}
impl std::fmt::Display
for TreasuryInboundTransfersResourceInboundTransferResourceLinkedFlows {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TreasuryInboundTransfersResourceInboundTransferResourceStatusTransitions {
    #[serde(rename = "canceled_at")]
    ///Timestamp describing when an InboundTransfer changed status to `canceled`.
    pub canceled_at: Option<i64>,
    #[serde(rename = "failed_at")]
    ///Timestamp describing when an InboundTransfer changed status to `failed`.
    pub failed_at: Option<i64>,
    #[serde(rename = "succeeded_at")]
    ///Timestamp describing when an InboundTransfer changed status to `succeeded`.
    pub succeeded_at: Option<i64>,
}
impl std::fmt::Display
for TreasuryInboundTransfersResourceInboundTransferResourceStatusTransitions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TreasuryOutboundPaymentsResourceOutboundPaymentResourceEndUserDetails {
    #[serde(rename = "ip_address")]
    ///IP address of the user initiating the OutboundPayment. Set if `present` is set to `true`. IP address collection is required for risk and compliance reasons. This will be used to help determine if the OutboundPayment is authorized or should be blocked.
    pub ip_address: Option<String>,
    #[serde(rename = "present")]
    ///`true`` if the OutboundPayment creation request is being made on behalf of an end user by a platform. Otherwise, `false`.
    pub present: bool,
}
impl std::fmt::Display
for TreasuryOutboundPaymentsResourceOutboundPaymentResourceEndUserDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TreasuryOutboundPaymentsResourceOutboundPaymentResourceStatusTransitions {
    #[serde(rename = "canceled_at")]
    ///Timestamp describing when an OutboundPayment changed status to `canceled`.
    pub canceled_at: Option<i64>,
    #[serde(rename = "failed_at")]
    ///Timestamp describing when an OutboundPayment changed status to `failed`.
    pub failed_at: Option<i64>,
    #[serde(rename = "posted_at")]
    ///Timestamp describing when an OutboundPayment changed status to `posted`.
    pub posted_at: Option<i64>,
    #[serde(rename = "returned_at")]
    ///Timestamp describing when an OutboundPayment changed status to `returned`.
    pub returned_at: Option<i64>,
}
impl std::fmt::Display
for TreasuryOutboundPaymentsResourceOutboundPaymentResourceStatusTransitions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TreasuryOutboundPaymentsResourceReturnedStatus {
    #[serde(rename = "code")]
    ///Reason for the return.
    pub code: String,
    #[serde(rename = "transaction")]
    ///The Transaction associated with this object.
    pub transaction: serde_json::Value,
}
impl std::fmt::Display for TreasuryOutboundPaymentsResourceReturnedStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TreasuryOutboundTransfersResourceReturnedDetails {
    #[serde(rename = "code")]
    ///Reason for the return.
    pub code: String,
    #[serde(rename = "transaction")]
    ///The Transaction associated with this object.
    pub transaction: serde_json::Value,
}
impl std::fmt::Display for TreasuryOutboundTransfersResourceReturnedDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TreasuryOutboundTransfersResourceStatusTransitions {
    #[serde(rename = "canceled_at")]
    ///Timestamp describing when an OutboundTransfer changed status to `canceled`
    pub canceled_at: Option<i64>,
    #[serde(rename = "failed_at")]
    ///Timestamp describing when an OutboundTransfer changed status to `failed`
    pub failed_at: Option<i64>,
    #[serde(rename = "posted_at")]
    ///Timestamp describing when an OutboundTransfer changed status to `posted`
    pub posted_at: Option<i64>,
    #[serde(rename = "returned_at")]
    ///Timestamp describing when an OutboundTransfer changed status to `returned`
    pub returned_at: Option<i64>,
}
impl std::fmt::Display for TreasuryOutboundTransfersResourceStatusTransitions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TreasuryReceivedCreditsResourceLinkedFlows {
    #[serde(rename = "credit_reversal")]
    ///The CreditReversal created as a result of this ReceivedCredit being reversed.
    pub credit_reversal: Option<String>,
    #[serde(rename = "issuing_authorization")]
    ///Set if the ReceivedCredit was created due to an [Issuing Authorization](https://stripe.com/docs/api#issuing_authorizations) object.
    pub issuing_authorization: Option<String>,
    #[serde(rename = "issuing_transaction")]
    ///Set if the ReceivedCredit is also viewable as an [Issuing transaction](https://stripe.com/docs/api#issuing_transactions) object.
    pub issuing_transaction: Option<String>,
    #[serde(rename = "source_flow")]
    ///ID of the source flow. Set if `network` is `stripe` and the source flow is visible to the user. Examples of source flows include OutboundPayments, payouts, or CreditReversals.
    pub source_flow: Option<String>,
    #[serde(rename = "source_flow_details")]
    ///The expandable object of the source flow.
    pub source_flow_details: Option<serde_json::Value>,
    #[serde(rename = "source_flow_type")]
    ///The type of flow that originated the ReceivedCredit (for example, `outbound_payment`).
    pub source_flow_type: Option<String>,
}
impl std::fmt::Display for TreasuryReceivedCreditsResourceLinkedFlows {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TreasuryReceivedCreditsResourceReversalDetails {
    #[serde(rename = "deadline")]
    ///Time before which a ReceivedCredit can be reversed.
    pub deadline: Option<i64>,
    #[serde(rename = "restricted_reason")]
    ///Set if a ReceivedCredit cannot be reversed.
    pub restricted_reason: Option<String>,
}
impl std::fmt::Display for TreasuryReceivedCreditsResourceReversalDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TreasuryReceivedCreditsResourceSourceFlowsDetails {
    #[serde(rename = "credit_reversal")]
    ///You can reverse some [ReceivedCredits](https://stripe.com/docs/api#received_credits) depending on their network and source flow. Reversing a ReceivedCredit leads to the creation of a new object known as a CreditReversal.
    pub credit_reversal: Option<TreasuryCreditReversal>,
    #[serde(rename = "outbound_payment")]
    /**Use OutboundPayments to send funds to another party's external bank account or [FinancialAccount](https://stripe.com/docs/api#financial_accounts). To send money to an account belonging to the same user, use an [OutboundTransfer](https://stripe.com/docs/api#outbound_transfers).

Simulate OutboundPayment state changes with the `/v1/test_helpers/treasury/outbound_payments` endpoints. These methods can only be called on test mode objects.*/
    pub outbound_payment: Option<TreasuryOutboundPayment>,
    #[serde(rename = "payout")]
    /**A `Payout` object is created when you receive funds from Stripe, or when you
initiate a payout to either a bank account or debit card of a [connected
Stripe account](/docs/connect/bank-debit-card-payouts). You can retrieve individual payouts,
as well as list all payouts. Payouts are made on [varying
schedules](/docs/connect/manage-payout-schedule), depending on your country and
industry.

Related guide: [Receiving Payouts](https://stripe.com/docs/payouts).*/
    pub payout: Option<Payout>,
    #[serde(rename = "type")]
    ///The type of the source flow that originated the ReceivedCredit.
    pub type_: String,
}
impl std::fmt::Display for TreasuryReceivedCreditsResourceSourceFlowsDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TreasuryReceivedCreditsResourceStatusTransitions {
    #[serde(rename = "posted_at")]
    ///Timestamp describing when the CreditReversal changed status to `posted`
    pub posted_at: Option<i64>,
}
impl std::fmt::Display for TreasuryReceivedCreditsResourceStatusTransitions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TreasuryReceivedDebitsResourceDebitReversalLinkedFlows {
    #[serde(rename = "issuing_dispute")]
    ///Set if there is an Issuing dispute associated with the DebitReversal.
    pub issuing_dispute: Option<String>,
}
impl std::fmt::Display for TreasuryReceivedDebitsResourceDebitReversalLinkedFlows {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TreasuryReceivedDebitsResourceLinkedFlows {
    #[serde(rename = "debit_reversal")]
    ///The DebitReversal created as a result of this ReceivedDebit being reversed.
    pub debit_reversal: Option<String>,
    #[serde(rename = "inbound_transfer")]
    ///Set if the ReceivedDebit is associated with an InboundTransfer's return of funds.
    pub inbound_transfer: Option<String>,
    #[serde(rename = "issuing_authorization")]
    ///Set if the ReceivedDebit was created due to an [Issuing Authorization](https://stripe.com/docs/api#issuing_authorizations) object.
    pub issuing_authorization: Option<String>,
    #[serde(rename = "issuing_transaction")]
    ///Set if the ReceivedDebit is also viewable as an [Issuing Dispute](https://stripe.com/docs/api#issuing_disputes) object.
    pub issuing_transaction: Option<String>,
}
impl std::fmt::Display for TreasuryReceivedDebitsResourceLinkedFlows {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TreasuryReceivedDebitsResourceReversalDetails {
    #[serde(rename = "deadline")]
    ///Time before which a ReceivedDebit can be reversed.
    pub deadline: Option<i64>,
    #[serde(rename = "restricted_reason")]
    ///Set if a ReceivedDebit can't be reversed.
    pub restricted_reason: Option<String>,
}
impl std::fmt::Display for TreasuryReceivedDebitsResourceReversalDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TreasuryReceivedDebitsResourceStatusTransitions {
    #[serde(rename = "completed_at")]
    ///Timestamp describing when the DebitReversal changed status to `completed`.
    pub completed_at: Option<i64>,
}
impl std::fmt::Display for TreasuryReceivedDebitsResourceStatusTransitions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TreasurySharedResourceBillingDetails {
    #[serde(rename = "address")]
    ///
    pub address: Address,
    #[serde(rename = "email")]
    ///Email address.
    pub email: Option<String>,
    #[serde(rename = "name")]
    ///Full name.
    pub name: Option<String>,
}
impl std::fmt::Display for TreasurySharedResourceBillingDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetails {
    #[serde(rename = "balance")]
    ///Set when `type` is `balance`.
    pub balance: Option<String>,
    #[serde(rename = "billing_details")]
    ///
    pub billing_details: TreasurySharedResourceBillingDetails,
    #[serde(rename = "financial_account")]
    ///
    pub financial_account: Option<ReceivedPaymentMethodDetailsFinancialAccount>,
    #[serde(rename = "issuing_card")]
    ///Set when `type` is `issuing_card`. This is an [Issuing Card](https://stripe.com/docs/api#issuing_cards) ID.
    pub issuing_card: Option<String>,
    #[serde(rename = "type")]
    ///Polymorphic type matching the originating money movement's source. This can be an external account, a Stripe balance, or a FinancialAccount.
    pub type_: String,
    #[serde(rename = "us_bank_account")]
    ///
    pub us_bank_account: Option<
        TreasurySharedResourceInitiatingPaymentMethodDetailsUsBankAccount,
    >,
}
impl std::fmt::Display
for TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TreasurySharedResourceInitiatingPaymentMethodDetailsUsBankAccount {
    #[serde(rename = "bank_name")]
    ///Bank name.
    pub bank_name: Option<String>,
    #[serde(rename = "last4")]
    ///The last four digits of the bank account number.
    pub last4: Option<String>,
    #[serde(rename = "routing_number")]
    ///The routing number for the bank account.
    pub routing_number: Option<String>,
}
impl std::fmt::Display
for TreasurySharedResourceInitiatingPaymentMethodDetailsUsBankAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TreasuryTransactionsResourceAbstractTransactionResourceStatusTransitions {
    #[serde(rename = "posted_at")]
    ///Timestamp describing when the Transaction changed status to `posted`.
    pub posted_at: Option<i64>,
    #[serde(rename = "void_at")]
    ///Timestamp describing when the Transaction changed status to `void`.
    pub void_at: Option<i64>,
}
impl std::fmt::Display
for TreasuryTransactionsResourceAbstractTransactionResourceStatusTransitions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TreasuryTransactionsResourceBalanceImpact {
    #[serde(rename = "cash")]
    ///The change made to funds the user can spend right now.
    pub cash: i64,
    #[serde(rename = "inbound_pending")]
    ///The change made to funds that are not spendable yet, but will become available at a later time.
    pub inbound_pending: i64,
    #[serde(rename = "outbound_pending")]
    ///The change made to funds in the account, but not spendable because they are being held for pending outbound flows.
    pub outbound_pending: i64,
}
impl std::fmt::Display for TreasuryTransactionsResourceBalanceImpact {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TreasuryTransactionsResourceFlowDetails {
    #[serde(rename = "credit_reversal")]
    ///You can reverse some [ReceivedCredits](https://stripe.com/docs/api#received_credits) depending on their network and source flow. Reversing a ReceivedCredit leads to the creation of a new object known as a CreditReversal.
    pub credit_reversal: Option<TreasuryCreditReversal>,
    #[serde(rename = "debit_reversal")]
    ///You can reverse some [ReceivedDebits](https://stripe.com/docs/api#received_debits) depending on their network and source flow. Reversing a ReceivedDebit leads to the creation of a new object known as a DebitReversal.
    pub debit_reversal: Option<TreasuryDebitReversal>,
    #[serde(rename = "inbound_transfer")]
    ///Use [InboundTransfers](https://stripe.com/docs/treasury/moving-money/financial-accounts/into/inbound-transfers) to add funds to your [FinancialAccount](https://stripe.com/docs/api#financial_accounts) via a PaymentMethod that is owned by you. The funds will be transferred via an ACH debit.
    pub inbound_transfer: Option<TreasuryInboundTransfer>,
    #[serde(rename = "issuing_authorization")]
    /**When an [issued card](https://stripe.com/docs/issuing) is used to make a purchase, an Issuing `Authorization`
object is created. [Authorizations](https://stripe.com/docs/issuing/purchases/authorizations) must be approved for the
purchase to be completed successfully.

Related guide: [Issued Card Authorizations](https://stripe.com/docs/issuing/purchases/authorizations).*/
    pub issuing_authorization: Option<IssuingAuthorization>,
    #[serde(rename = "outbound_payment")]
    /**Use OutboundPayments to send funds to another party's external bank account or [FinancialAccount](https://stripe.com/docs/api#financial_accounts). To send money to an account belonging to the same user, use an [OutboundTransfer](https://stripe.com/docs/api#outbound_transfers).

Simulate OutboundPayment state changes with the `/v1/test_helpers/treasury/outbound_payments` endpoints. These methods can only be called on test mode objects.*/
    pub outbound_payment: Option<TreasuryOutboundPayment>,
    #[serde(rename = "outbound_transfer")]
    /**Use OutboundTransfers to transfer funds from a [FinancialAccount](https://stripe.com/docs/api#financial_accounts) to a PaymentMethod belonging to the same entity. To send funds to a different party, use [OutboundPayments](https://stripe.com/docs/api#outbound_payments) instead. You can send funds over ACH rails or through a domestic wire transfer to a user's own external bank account.

Simulate OutboundTransfer state changes with the `/v1/test_helpers/treasury/outbound_transfers` endpoints. These methods can only be called on test mode objects.*/
    pub outbound_transfer: Option<TreasuryOutboundTransfer>,
    #[serde(rename = "received_credit")]
    ///ReceivedCredits represent funds sent to a [FinancialAccount](https://stripe.com/docs/api#financial_accounts) (for example, via ACH or wire). These money movements are not initiated from the FinancialAccount.
    pub received_credit: Option<TreasuryReceivedCredit>,
    #[serde(rename = "received_debit")]
    ///ReceivedDebits represent funds pulled from a [FinancialAccount](https://stripe.com/docs/api#financial_accounts). These are not initiated from the FinancialAccount.
    pub received_debit: Option<TreasuryReceivedDebit>,
    #[serde(rename = "type")]
    ///Type of the flow that created the Transaction. Set to the same value as `flow_type`.
    pub type_: String,
}
impl std::fmt::Display for TreasuryTransactionsResourceFlowDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct UsBankAccountNetworks {
    #[serde(rename = "preferred")]
    ///The preferred network.
    pub preferred: Option<String>,
    #[serde(rename = "supported")]
    ///All supported networks.
    pub supported: Vec<String>,
}
impl std::fmt::Display for UsBankAccountNetworks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct UsageRecord {
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "livemode")]
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "quantity")]
    ///The usage quantity for the specified date.
    pub quantity: i64,
    #[serde(rename = "subscription_item")]
    ///The ID of the subscription item this usage record contains data for.
    pub subscription_item: String,
    #[serde(rename = "timestamp")]
    ///The timestamp when this usage occurred.
    pub timestamp: i64,
}
impl std::fmt::Display for UsageRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct UsageRecordSummary {
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "invoice")]
    ///The invoice in which this usage period has been billed for.
    pub invoice: Option<String>,
    #[serde(rename = "livemode")]
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "period")]
    ///
    pub period: Period,
    #[serde(rename = "subscription_item")]
    ///The ID of the subscription item this summary is describing.
    pub subscription_item: String,
    #[serde(rename = "total_usage")]
    ///The total usage within this usage period.
    pub total_usage: i64,
}
impl std::fmt::Display for UsageRecordSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct VerificationSessionRedaction {
    #[serde(rename = "status")]
    ///Indicates whether this object and its related objects have been redacted or not.
    pub status: String,
}
impl std::fmt::Display for VerificationSessionRedaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WebhookEndpoint {
    #[serde(rename = "api_version")]
    ///The API version events are rendered as for this webhook endpoint.
    pub api_version: Option<String>,
    #[serde(rename = "application")]
    ///The ID of the associated Connect application.
    pub application: Option<String>,
    #[serde(rename = "created")]
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    #[serde(rename = "description")]
    ///An optional description of what the webhook is used for.
    pub description: Option<String>,
    #[serde(rename = "enabled_events")]
    ///The list of events to enable for this endpoint. `['*']` indicates that all events are enabled, except those that require explicit selection.
    pub enabled_events: Vec<String>,
    #[serde(rename = "id")]
    ///Unique identifier for the object.
    pub id: String,
    #[serde(rename = "livemode")]
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    #[serde(rename = "metadata")]
    ///Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format.
    pub metadata: serde_json::Value,
    #[serde(rename = "object")]
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    #[serde(rename = "secret")]
    ///The endpoint's secret, used to generate [webhook signatures](https://stripe.com/docs/webhooks/signatures). Only returned at creation.
    pub secret: Option<String>,
    #[serde(rename = "status")]
    ///The status of the webhook. It can be `enabled` or `disabled`.
    pub status: String,
    #[serde(rename = "url")]
    ///The URL of the webhook endpoint.
    pub url: String,
}
impl std::fmt::Display for WebhookEndpoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
