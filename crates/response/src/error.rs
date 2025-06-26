#[derive(Debug, Clone, Default)]
#[derive(thiserror::Error)]
pub enum Error {
    /// General request failed error, including message parsing failed.
    #[error("Bad Request")]
    BadRequest,
    /// Invalid format
    #[error("Invalid Field Format {0}")]
    InvalidFieldFormat(String),
    /// Missing or invalid format on mandatory field
    #[error("Invalid Mandatory Field {0}")]
    InvalidMandatoryField(String),
    /// General unauthorized error (No Interface Def, API is Invalid, Oauth
    /// Failed, Verify Client Secret Fail, Client Forbidden Access API, Unknown
    /// Client, Key not Found)
    #[error("Unauthorized. {0}")]
    Unathorized(String),
    /// Token found in request is invalid (Access Token Not Exist, Access Token
    /// Expiry)
    #[error("Invalid Token (B2B)")]
    InvalidTokenB2B,
    /// Token found in request is invalid (Access Token Not Exist, Access Token
    /// Expiry)
    #[error("Invalid Customer Token")]
    InvalidCustomerToken,
    /// Token not found in the system. This occurs on any API that requires
    /// token as input parameter
    #[error("Token Not Found (B2B)")]
    TokenNotFoundB2B,
    /// Token not found in the system. This occurs on any API that requires
    /// token as input parameter
    #[error("Customer Token Not Found")]
    CustomerTokenNotFound,
    /// Transaction expired
    #[error("Transaction Expired")]
    TransactionExpired,
    /// This merchant is not allowed to call Direct Debit APIs
    #[error("Feature Not Allowed {0}")]
    FeatureNotAllowed(String),
    /// Exceeds Transaction Amount Limit
    #[error("Exceeds Transaction Amount Limit")]
    ExceedsTransactionAmountLimit,
    /// Suspected Fraud
    #[error("Suspected Fraud")]
    SuspectedFraud,
    /// Too many request, Exceeds Transaction Frequency Limit
    #[error("Activity Count Limit Exceeded")]
    ActivityCountLimitExceeded,
    /// Account or User status is abnormal
    #[error("Do Not Honor")]
    DoNotHonor,
    /// Cut off In Progress
    #[error("Feature Not Allowed At This Time. {0}")]
    FeatureNotAllowedAtThisTime(String),
    /// The payment card is blocked
    #[error("Card Blocked")]
    CardBlocked,
    /// The payment card is expired
    #[error("Card Expired")]
    CardExpired,
    /// The account is dormant
    #[error("Dormant Account")]
    DormantAccount,
    /// Need to set token limit
    #[error("Need To Set Token Limit")]
    NeedToSetTokenLimit,
    /// OTP has been blocked
    #[error("OTP Blocked")]
    OTPBlocked,
    /// OTP has been expired
    #[error("OTP Lifetime Expired")]
    OTPLifetimeExpired,
    /// Initiates request OTP to the issuer
    #[error("OTP Sent To Cardholder")]
    OTPSentToCardholder,
    /// Insufficient Funds
    #[error("Insufficient Funds")]
    InsufficientFunds,
    /// Transaction Not Permitted
    #[error("Transaction Not Permitted. {0}")]
    TransactionNotPermitted(String),
    /// Suspend Transaction
    #[error("Suspend Transaction")]
    SuspendTransaction,
    /// Purchase amount exceeds the token limit set prior
    #[error("Token Limit Exceeded")]
    TokenLimitExceeded,
    /// Indicates inactive account
    #[error("Inactive Card/Account/Customer")]
    InactiveCardOrAccountOrCustomer,
    /// Merchant is suspended from calling any APIs
    #[error("Merchant Blacklisted")]
    MerchantBlacklisted,
    /// Merchant aggregated purchase amount on that day exceeds the agreed limit
    #[error("Merchant Limit Exceed")]
    MerchantLimitExceed,
    /// Set limit not allowed on particular token
    #[error("Set Limit Not Allowed")]
    SetLimitNotAllowed,
    /// The token limit desired by the merchant is not within the agreed range
    /// between the merchant and the Issuer
    #[error("Token Limit Invalid")]
    TokenLimitInvalid,
    /// Account aggregated purchase amount on that day exceeds the agreed limit
    #[error("Account Limit Exceed")]
    AccountLimitExceed,
    /// Invalid transaction status
    #[error("Invalid Transaction Status")]
    InvalidTransactionStatus,
    /// Transaction not found
    #[error("Transaction Not Found")]
    TransactionNotFound,
    /// Invalid Routing
    #[error("Invalid Routing")]
    InvalidRouting,
    /// Bank not supported by switch
    #[error("Bank Not Supported By Switch")]
    BankNotSupportedBySwitch,
    /// Transaction is cancelled by customer
    #[error("Transaction Cancelled")]
    TransactionCancelled,
    /// Merchant is not registered for Card Registration services
    #[error("Merchant Is Not Registered For Card Registration Services")]
    MerchantNotRegisteredForCardRegistrationServices,
    /// Need to request OTP
    #[error("Need To Request OTP")]
    NeedToRequestOTP,
    /// The journeyId cannot be found in the system
    #[error("Journey Not Found")]
    JourneyNotFound,
    /// Merchant does not exist or status abnormal
    #[error("Invalid Merchant")]
    InvalidMerchant,
    /// No issuer
    #[error("No Issuer")]
    NoIssuer,
    /// Invalid API transition within a journey
    #[error("Invalid API Transition")]
    InvalidAPITransition,
    /// Card information may be invalid, or the card account may be blacklisted,
    /// or Virtual Account number maybe invalid.
    #[error("Invalid Card/Account/Customer {0}/Virtual Account")]
    InvalidCardOrAccountOrCustomerOrVirtualAccount(String),
    /// The bill or Virtual account is blocked/ suspended/not found.
    #[error("Invalid Bill/Virtual Account {0}")]
    InvalidBillOrVirtualAccountWithReason(String),
    /// The amount doesn't match with what supposed to
    #[error("Invalid Amount")]
    InvalidAmount,
    /// The bill has been paid
    #[error("Paid Bill")]
    PaidBill,
    /// OTP is incorrect
    #[error("Invalid OTP")]
    InvalidOTP,
    /// Partner number can't be found
    #[error("Partner Not Found")]
    PartnerNotFound,
    /// Terminal does not exist in the system
    #[error("Invalid Terminal")]
    InvalidTerminal,
    /// Inconsistent request parameter found for the same partner reference
    /// number/transaction id It can be considered as failed in transfer
    /// debit, but it should be considered as success in transfer credit.
    /// Considered as success:
    /// - Transfer credit = (i) Intrabank transfer; (ii) Interbank transfer;
    ///   (iii) RTGS transfer; (iv) SKNBI transfer;
    /// - Virtual account = (i) Payment VA; (ii) Payment to VA;
    /// - Transfer debit = (i) Refund payment; (ii) Void;
    /// Considered as failed:
    /// - Transfer credit = (i) Transfer to OTC;
    /// - Transfer debit = (i) Direct debit payment; (ii) QR CPM payment; (iii)
    ///   Auth payment; (iv) Capture;
    #[error("Inconsistent Request")]
    InconsistentRequest,
    /// The bill is expired.
    #[error("Invalid Bill/Virtual Account")]
    InvalidBillOrVirtualAccount,
    /// Requested function is not supported
    #[error("Requested Function Is Not Supported")]
    RequestedFunctionIsNotSupported,
    /// Requested operation to cancel/refund transaction Is not allowed at this
    /// time.
    #[error("Requested Opearation Is Not Allowed")]
    RequestedOperationIsNotAllowed,
    /// Cannot use same X-EXTERNAL-ID in same day
    #[error("Conflict")]
    Conflict,
    /// Transaction has previously been processed indicates the same
    /// partnerReferenceNo already success
    #[error("Duplicate partnerReferenceNo")]
    DuplicatePartnerReferenceNo,
    /// Maximum transaction limit exceeded
    #[error("Too Many Requests")]
    TooManyRequests,
    /// General Error
    #[error("General Error")]
    #[default]
    GeneralError,
    /// Unknown Internal Server Failure, Please retry the process again
    #[error("Internal Server Error")]
    InternalServerError,
    /// Backend system failure, etc
    #[error("External Server Error")]
    ExternalServerError,
    /// Timeout from the issuer
    #[error("Timeout")]
    Timeout,
}

impl Error {
    pub fn get_category(&self) -> crate::ResponseCategory {
        match self {
            Self::BadRequest => crate::ResponseCategory::System,
            Self::InvalidFieldFormat(_) => crate::ResponseCategory::Message,
            Self::InvalidMandatoryField(_) => crate::ResponseCategory::Message,
            Self::Unathorized(_) => crate::ResponseCategory::System,
            Self::InvalidTokenB2B => crate::ResponseCategory::System,
            Self::InvalidCustomerToken => crate::ResponseCategory::System,
            Self::TokenNotFoundB2B => crate::ResponseCategory::System,
            Self::CustomerTokenNotFound => crate::ResponseCategory::System,
            Self::TransactionExpired => crate::ResponseCategory::Business,
            Self::FeatureNotAllowed(_) => crate::ResponseCategory::System,
            Self::ExceedsTransactionAmountLimit => crate::ResponseCategory::Business,
            Self::SuspectedFraud => crate::ResponseCategory::Business,
            Self::ActivityCountLimitExceeded => crate::ResponseCategory::Business,
            Self::DoNotHonor => crate::ResponseCategory::Business,
            Self::FeatureNotAllowedAtThisTime(_) => crate::ResponseCategory::System,
            Self::CardBlocked => crate::ResponseCategory::Business,
            Self::CardExpired => crate::ResponseCategory::Business,
            Self::DormantAccount => crate::ResponseCategory::Business,
            Self::NeedToSetTokenLimit => crate::ResponseCategory::Business,
            Self::OTPBlocked => crate::ResponseCategory::System,
            Self::OTPLifetimeExpired => crate::ResponseCategory::System,
            Self::OTPSentToCardholder => crate::ResponseCategory::System,
            Self::InsufficientFunds => crate::ResponseCategory::Business,
            Self::TransactionNotPermitted(_) => crate::ResponseCategory::Business,
            Self::SuspendTransaction => crate::ResponseCategory::Business,
            Self::TokenLimitExceeded => crate::ResponseCategory::Business,
            Self::InactiveCardOrAccountOrCustomer => crate::ResponseCategory::Business,
            Self::MerchantBlacklisted => crate::ResponseCategory::Business,
            Self::MerchantLimitExceed => crate::ResponseCategory::Business,
            Self::SetLimitNotAllowed => crate::ResponseCategory::Business,
            Self::TokenLimitInvalid => crate::ResponseCategory::Business,
            Self::AccountLimitExceed => crate::ResponseCategory::Business,
            Self::InvalidTransactionStatus => crate::ResponseCategory::Business,
            Self::TransactionNotFound => crate::ResponseCategory::Business,
            Self::InvalidRouting => crate::ResponseCategory::System,
            Self::BankNotSupportedBySwitch => crate::ResponseCategory::System,
            Self::TransactionCancelled => crate::ResponseCategory::Business,
            Self::MerchantNotRegisteredForCardRegistrationServices => crate::ResponseCategory::Business,
            Self::NeedToRequestOTP => crate::ResponseCategory::System,
            Self::JourneyNotFound => crate::ResponseCategory::System,
            Self::InvalidMerchant => crate::ResponseCategory::Business,
            Self::NoIssuer => crate::ResponseCategory::Business,
            Self::InvalidAPITransition => crate::ResponseCategory::System,
            Self::InvalidCardOrAccountOrCustomerOrVirtualAccount(_) => crate::ResponseCategory::Business,
            Self::InvalidBillOrVirtualAccountWithReason(_) => crate::ResponseCategory::Business,
            Self::InvalidAmount => crate::ResponseCategory::Business,
            Self::PaidBill => crate::ResponseCategory::Business,
            Self::InvalidOTP => crate::ResponseCategory::System,
            Self::PartnerNotFound => crate::ResponseCategory::Business,
            Self::InvalidTerminal => crate::ResponseCategory::Business,
            Self::InconsistentRequest => crate::ResponseCategory::Business,
            Self::InvalidBillOrVirtualAccount => crate::ResponseCategory::Business,
            Self::RequestedFunctionIsNotSupported => crate::ResponseCategory::System,
            Self::RequestedOperationIsNotAllowed => crate::ResponseCategory::Business,
            Self::Conflict => crate::ResponseCategory::System,
            Self::DuplicatePartnerReferenceNo => crate::ResponseCategory::System,
            Self::TooManyRequests => crate::ResponseCategory::System,
            Self::GeneralError => crate::ResponseCategory::System,
            Self::InternalServerError => crate::ResponseCategory::System,
            Self::ExternalServerError => crate::ResponseCategory::System,
            Self::Timeout => crate::ResponseCategory::System,
        }
    }

    pub fn get_http_status_code(&self) -> http::StatusCode {
        match self {
            // 400 Bad Request
            Self::BadRequest | Self::InvalidFieldFormat(_) | Self::InvalidMandatoryField(_) => {
                http::StatusCode::BAD_REQUEST
            }

            // 401 Unauthorized
            Self::Unathorized(_)
            | Self::InvalidTokenB2B
            | Self::InvalidCustomerToken
            | Self::TokenNotFoundB2B
            | Self::CustomerTokenNotFound => http::StatusCode::UNAUTHORIZED,

            // 403 Forbidden
            Self::TransactionExpired
            | Self::FeatureNotAllowed(_)
            | Self::ExceedsTransactionAmountLimit
            | Self::SuspectedFraud
            | Self::ActivityCountLimitExceeded
            | Self::DoNotHonor
            | Self::FeatureNotAllowedAtThisTime(_)
            | Self::CardBlocked
            | Self::CardExpired
            | Self::DormantAccount
            | Self::NeedToSetTokenLimit
            | Self::OTPBlocked
            | Self::OTPLifetimeExpired
            | Self::OTPSentToCardholder
            | Self::InsufficientFunds
            | Self::TransactionNotPermitted(_)
            | Self::SuspendTransaction
            | Self::TokenLimitExceeded
            | Self::InactiveCardOrAccountOrCustomer
            | Self::MerchantBlacklisted
            | Self::MerchantLimitExceed
            | Self::SetLimitNotAllowed
            | Self::TokenLimitInvalid
            | Self::AccountLimitExceed => http::StatusCode::FORBIDDEN,

            // 404 Not Found
            Self::InvalidOTP
            | Self::InvalidTransactionStatus
            | Self::TransactionCancelled
            | Self::MerchantNotRegisteredForCardRegistrationServices
            | Self::PaidBill
            | Self::PartnerNotFound
            | Self::JourneyNotFound
            | Self::InvalidMerchant
            | Self::NoIssuer
            | Self::TransactionNotFound
            | Self::InconsistentRequest
            | Self::InvalidAmount
            | Self::InvalidAPITransition
            | Self::InvalidRouting
            | Self::BankNotSupportedBySwitch
            | Self::InvalidTerminal
            | Self::InvalidCardOrAccountOrCustomerOrVirtualAccount(_)
            | Self::InvalidBillOrVirtualAccountWithReason(_)
            | Self::InvalidBillOrVirtualAccount
            | Self::NeedToRequestOTP => http::StatusCode::NOT_FOUND,

            // 405 Method Not Allowed
            Self::RequestedFunctionIsNotSupported | Self::RequestedOperationIsNotAllowed => {
                http::StatusCode::METHOD_NOT_ALLOWED
            }

            // 409 Conflict
            Self::Conflict | Self::DuplicatePartnerReferenceNo => http::StatusCode::CONFLICT,

            // 429 Too Many Requests
            Self::TooManyRequests => http::StatusCode::TOO_MANY_REQUESTS,

            // 500 Internal Server Error
            Self::GeneralError | Self::InternalServerError | Self::ExternalServerError => {
                http::StatusCode::INTERNAL_SERVER_ERROR
            }

            // 504 Gateway Timeout
            Self::Timeout => http::StatusCode::GATEWAY_TIMEOUT,
        }
    }

    pub fn get_case_code(&self) -> u8 {
        match self {
            Self::BadRequest => 00,
            Self::InvalidFieldFormat(_) => 01,
            Self::InvalidMandatoryField(_) => 02,
            Self::Unathorized(_) => 00,
            Self::InvalidTokenB2B => 01,
            Self::InvalidCustomerToken => 02,
            Self::TokenNotFoundB2B => 03,
            Self::CustomerTokenNotFound => 04,
            Self::TransactionExpired => 00,
            Self::FeatureNotAllowed(_) => 01,
            Self::ExceedsTransactionAmountLimit => 02,
            Self::SuspectedFraud => 03,
            Self::ActivityCountLimitExceeded => 04,
            Self::DoNotHonor => 05,
            Self::FeatureNotAllowedAtThisTime(_) => 06,
            Self::CardBlocked => 07,
            Self::CardExpired => 08,
            Self::DormantAccount => 09,
            Self::NeedToSetTokenLimit => 10,
            Self::OTPBlocked => 11,
            Self::OTPLifetimeExpired => 12,
            Self::OTPSentToCardholder => 13,
            Self::InsufficientFunds => 14,
            Self::TransactionNotPermitted(_) => 15,
            Self::SuspendTransaction => 16,
            Self::TokenLimitExceeded => 17,
            Self::InactiveCardOrAccountOrCustomer => 18,
            Self::MerchantBlacklisted => 19,
            Self::MerchantLimitExceed => 20,
            Self::SetLimitNotAllowed => 21,
            Self::TokenLimitInvalid => 22,
            Self::AccountLimitExceed => 23,
            Self::InvalidTransactionStatus => 00,
            Self::TransactionNotFound => 01,
            Self::InvalidRouting => 02,
            Self::BankNotSupportedBySwitch => 03,
            Self::TransactionCancelled => 04,
            Self::MerchantNotRegisteredForCardRegistrationServices => 05,
            Self::NeedToRequestOTP => 06,
            Self::JourneyNotFound => 07,
            Self::InvalidMerchant => 08,
            Self::NoIssuer => 09,
            Self::InvalidAPITransition => 10,
            Self::InvalidCardOrAccountOrCustomerOrVirtualAccount(_) => 11,
            Self::InvalidBillOrVirtualAccountWithReason(_) => 12,
            Self::InvalidAmount => 13,
            Self::PaidBill => 14,
            Self::InvalidOTP => 15,
            Self::PartnerNotFound => 16,
            Self::InvalidTerminal => 17,
            Self::InconsistentRequest => 18,
            Self::InvalidBillOrVirtualAccount => 19,
            Self::RequestedFunctionIsNotSupported => 00,
            Self::RequestedOperationIsNotAllowed => 01,
            Self::Conflict => 00,
            Self::DuplicatePartnerReferenceNo => 01,
            Self::TooManyRequests => 00,
            Self::GeneralError => 00,
            Self::InternalServerError => 01,
            Self::ExternalServerError => 02,
            Self::Timeout => 00,
        }
    }

    pub fn get_code(&self, service_code: u8) -> u32 {
        let http_status_code = (self.get_http_status_code().as_u16() as u32) * 10_000;
        let case_code = self.get_case_code() as u32;
        let service_code = ((service_code % 100) as u32) * 100;

        http_status_code + service_code + case_code
    }
}
