use kernel_domain::{EnglishNamespace, StableVersion};

use crate::gateway_error::{GatewayError, GatewayErrorCode, GatewayResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GatewayOperationKind {
    Command,
    Query,
    Status,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GatewayApiVersion(StableVersion);

impl GatewayApiVersion {
    pub fn new(value: impl Into<String>) -> GatewayResult<Self> {
        let value = value.into();
        let trimmed = value.trim();
        if trimmed.is_empty()
            || trimmed.contains('/')
            || !trimmed.chars().all(|character| {
                character.is_ascii_alphanumeric() || matches!(character, '.' | '_' | '-')
            })
        {
            return Err(GatewayError::new(
                GatewayErrorCode::UnsupportedGatewayVersion,
                "gateway API version must be namespace-safe and transport-neutral",
            )?);
        }
        Ok(Self(
            StableVersion::new("gateway_api_version", trimmed.to_owned())
                .map_err(GatewayError::from_domain_rejection)?,
        ))
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GatewayOperationReference {
    operation_kind: GatewayOperationKind,
    operation_name: EnglishNamespace,
}

impl GatewayOperationReference {
    pub fn new(
        operation_kind: GatewayOperationKind,
        operation_name: impl Into<String>,
    ) -> GatewayResult<Self> {
        let operation_name = EnglishNamespace::new("gateway_operation_name", operation_name)
            .map_err(GatewayError::from_domain_rejection)?;
        if !operation_name.as_str().contains('.') {
            return Err(GatewayError::new(
                GatewayErrorCode::UnsupportedOperation,
                "gateway operation reference requires a namespaced logical operation",
            )?);
        }
        Ok(Self {
            operation_kind,
            operation_name,
        })
    }

    pub fn operation_kind(&self) -> GatewayOperationKind {
        self.operation_kind
    }

    pub fn operation_name(&self) -> &str {
        self.operation_name.as_str()
    }

    pub fn resource_segment(&self) -> &str {
        self.operation_name
            .as_str()
            .split('.')
            .next()
            .unwrap_or_default()
    }
}
