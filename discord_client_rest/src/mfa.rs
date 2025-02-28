use serde::Deserialize;

#[derive(Debug)]
pub struct MfaRequiredError {
    pub verification_request: MfaVerificationRequest,
}

impl std::fmt::Display for MfaRequiredError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MFA required: {:?}", self.verification_request)
    }
}

impl std::error::Error for MfaRequiredError {}

#[derive(Debug, Deserialize, Clone)]
pub struct MfaVerificationRequest {
    pub ticket: String,
    pub methods: Vec<MfaMethod>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MfaMethod {
    #[serde(deserialize_with = "deserialize_mfa_type")]
    pub r#type: MfaType,
    pub challenge: Option<String>,
    pub backup_codes_allowed: Option<bool>,
}

#[derive(Debug, Clone)]
pub enum MfaType {
    Totp(),
    Sms(),
    Backup(),
    Webauthn(),
    Password(),
    Unknown(String),
}
impl std::fmt::Display for MfaType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MfaType::Totp() => write!(f, "totp"),
            MfaType::Sms() => write!(f, "sms"),
            MfaType::Backup() => write!(f, "backup",),
            MfaType::Webauthn() => write!(f, "webauthn",),
            MfaType::Password() => write!(f, "password",),
            MfaType::Unknown(s) => write!(f, "{}", s),
        }
    }
}

fn deserialize_mfa_type<'de, D>(deserializer: D) -> Result<MfaType, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let r#type: String = String::deserialize(deserializer)?;
    Ok(match r#type.as_str() {
        "totp" => MfaType::Totp(),
        "sms" => MfaType::Sms(),
        "backup" => MfaType::Backup(),
        "webauthn" => MfaType::Webauthn(),
        "password" => MfaType::Password(),
        _ => MfaType::Unknown(r#type),
    })
}
