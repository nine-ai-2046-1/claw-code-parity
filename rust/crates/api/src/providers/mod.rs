use std::future::Future;
use std::pin::Pin;

use crate::error::ApiError;
use crate::types::{MessageRequest, MessageResponse};

pub mod anthropic;
pub mod openai_compat;

pub type ProviderFuture<'a, T> = Pin<Box<dyn Future<Output = Result<T, ApiError>> + Send + 'a>>;

pub trait Provider {
    type Stream;

    fn send_message<'a>(
        &'a self,
        request: &'a MessageRequest,
    ) -> ProviderFuture<'a, MessageResponse>;

    fn stream_message<'a>(
        &'a self,
        request: &'a MessageRequest,
    ) -> ProviderFuture<'a, Self::Stream>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProviderKind {
    Anthropic,
    Xai,
    OpenAi,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProviderMetadata {
    pub provider: ProviderKind,
    pub auth_env: &'static str,
    pub base_url_env: &'static str,
    pub default_base_url: &'static str,
}

const MODEL_REGISTRY: &[(&str, ProviderMetadata)] = &[
    (
        "opus",
        ProviderMetadata {
            provider: ProviderKind::Anthropic,
            auth_env: "ANTHROPIC_API_KEY",
            base_url_env: "ANTHROPIC_BASE_URL",
            default_base_url: anthropic::DEFAULT_BASE_URL,
        },
    ),
    (
        "sonnet",
        ProviderMetadata {
            provider: ProviderKind::Anthropic,
            auth_env: "ANTHROPIC_API_KEY",
            base_url_env: "ANTHROPIC_BASE_URL",
            default_base_url: anthropic::DEFAULT_BASE_URL,
        },
    ),
    (
        "haiku",
        ProviderMetadata {
            provider: ProviderKind::Anthropic,
            auth_env: "ANTHROPIC_API_KEY",
            base_url_env: "ANTHROPIC_BASE_URL",
            default_base_url: anthropic::DEFAULT_BASE_URL,
        },
    ),
    (
        "grok",
        ProviderMetadata {
            provider: ProviderKind::Xai,
            auth_env: "XAI_API_KEY",
            base_url_env: "XAI_BASE_URL",
            default_base_url: openai_compat::DEFAULT_XAI_BASE_URL,
        },
    ),
    (
        "grok-3",
        ProviderMetadata {
            provider: ProviderKind::Xai,
            auth_env: "XAI_API_KEY",
            base_url_env: "XAI_BASE_URL",
            default_base_url: openai_compat::DEFAULT_XAI_BASE_URL,
        },
    ),
    (
        "grok-mini",
        ProviderMetadata {
            provider: ProviderKind::Xai,
            auth_env: "XAI_API_KEY",
            base_url_env: "XAI_BASE_URL",
            default_base_url: openai_compat::DEFAULT_XAI_BASE_URL,
        },
    ),
    (
        "grok-3-mini",
        ProviderMetadata {
            provider: ProviderKind::Xai,
            auth_env: "XAI_API_KEY",
            base_url_env: "XAI_BASE_URL",
            default_base_url: openai_compat::DEFAULT_XAI_BASE_URL,
        },
    ),
    (
        "grok-2",
        ProviderMetadata {
            provider: ProviderKind::Xai,
            auth_env: "XAI_API_KEY",
            base_url_env: "XAI_BASE_URL",
            default_base_url: openai_compat::DEFAULT_XAI_BASE_URL,
        },
    ),
];

#[must_use]
pub fn resolve_model_alias(model: &str) -> String {
    let trimmed = model.trim();
    let lower = trimmed.to_ascii_lowercase();
    MODEL_REGISTRY
        .iter()
        .find_map(|(alias, metadata)| {
            (*alias == lower).then_some(match metadata.provider {
                ProviderKind::Anthropic => match *alias {
                    "opus" => "claude-opus-4-6",
                    "sonnet" => "claude-sonnet-4-6",
                    "haiku" => "claude-haiku-4-5-20251213",
                    _ => trimmed,
                },
                ProviderKind::Xai => match *alias {
                    "grok" | "grok-3" => "grok-3",
                    "grok-mini" | "grok-3-mini" => "grok-3-mini",
                    "grok-2" => "grok-2",
                    _ => trimmed,
                },
                ProviderKind::OpenAi => trimmed,
            })
        })
        .map_or_else(|| trimmed.to_string(), ToOwned::to_owned)
}

#[must_use]
pub fn metadata_for_model(model: &str) -> Option<ProviderMetadata> {
    let canonical = resolve_model_alias(model);
    if canonical.starts_with("claude") {
        return Some(ProviderMetadata {
            provider: ProviderKind::Anthropic,
            auth_env: "ANTHROPIC_API_KEY",
            base_url_env: "ANTHROPIC_BASE_URL",
            default_base_url: anthropic::DEFAULT_BASE_URL,
        });
    }
    if canonical.starts_with("grok") {
        return Some(ProviderMetadata {
            provider: ProviderKind::Xai,
            auth_env: "XAI_API_KEY",
            base_url_env: "XAI_BASE_URL",
            default_base_url: openai_compat::DEFAULT_XAI_BASE_URL,
        });
    }
    None
}

#[must_use]
pub fn detect_provider_kind(model: &str) -> ProviderKind {
    detect_provider_kind_with_override(model, None).unwrap_or(ProviderKind::Anthropic)
}

/// Map a provider name string to a `ProviderKind`.
/// Returns `None` for unrecognised names.
pub fn provider_kind_from_str(s: &str) -> Option<ProviderKind> {
    match s.trim().to_ascii_lowercase().as_str() {
        "anthropic" | "claude" => Some(ProviderKind::Anthropic),
        "openai" | "openai-compat" | "poe" | "groq" | "azure" | "openrouter" | "gemini" => {
            Some(ProviderKind::OpenAi)
        }
        "xai" | "grok" => Some(ProviderKind::Xai),
        _ => None,
    }
}

/// Detect provider with optional explicit override (from `--provider` flag).
/// Priority: explicit override > model registry > credentials > fallback Anthropic.
/// Returns `Err` if `explicit_provider` is set but unrecognised.
pub fn detect_provider_kind_with_override(
    model: &str,
    explicit_provider: Option<&str>,
) -> Result<ProviderKind, String> {
    // 1. Explicit override (CLI flag — highest priority)
    if let Some(provider_str) = explicit_provider {
        return provider_kind_from_str(provider_str).ok_or_else(|| {
            format!(
                "unrecognised provider '{}'. Supported: anthropic, claude, openai, xai, poe, groq, azure, openrouter, gemini",
                provider_str
            )
        });
    }

    // 2. CLAW_PROVIDER env var (warn but don't error on unrecognised value)
    if let Ok(env_provider) = std::env::var("CLAW_PROVIDER") {
        if let Some(kind) = provider_kind_from_str(&env_provider) {
            return Ok(kind);
        }
        eprintln!(
            "warning: unrecognised CLAW_PROVIDER '{}', ignoring",
            env_provider
        );
    }

    // 3. Model name registry
    if let Some(metadata) = metadata_for_model(model) {
        return Ok(metadata.provider);
    }

    // 4. Available credentials
    if anthropic::has_auth_from_env_or_saved().unwrap_or(false) {
        return Ok(ProviderKind::Anthropic);
    }
    if openai_compat::has_api_key("OPENAI_API_KEY") {
        return Ok(ProviderKind::OpenAi);
    }
    if openai_compat::has_api_key("XAI_API_KEY") {
        return Ok(ProviderKind::Xai);
    }

    // 5. Fallback
    Ok(ProviderKind::Anthropic)
}

#[must_use]
pub fn max_tokens_for_model(model: &str) -> u32 {
    // Allow user override via env var
    if let Ok(val) = std::env::var("CLAW_MAX_TOKENS") {
        if let Ok(n) = val.trim().parse::<u32>() {
            return n;
        }
        eprintln!("warning: invalid CLAW_MAX_TOKENS value '{}', ignoring", val);
    }

    let canonical = resolve_model_alias(model).to_ascii_lowercase();

    // Anthropic
    if canonical.contains("opus") {
        return 32_000;
    }
    if canonical.starts_with("claude") {
        return 64_000;
    }

    // Groq-hosted open models (conservative default to stay within limits)
    if canonical.contains("llama")
        || canonical.contains("gemma")
        || canonical.contains("qwen")
        || canonical.contains("mixtral")
        || canonical.contains("mistral")
    {
        return 8_000;
    }

    // xAI Grok
    if canonical.starts_with("grok") {
        return 32_000;
    }

    // OpenAI / Gemini / others
    4_096
}

#[cfg(test)]
mod tests {
    use super::{
        detect_provider_kind, detect_provider_kind_with_override, max_tokens_for_model,
        provider_kind_from_str, resolve_model_alias, ProviderKind,
    };

    #[test]
    fn resolves_grok_aliases() {
        assert_eq!(resolve_model_alias("grok"), "grok-3");
        assert_eq!(resolve_model_alias("grok-mini"), "grok-3-mini");
        assert_eq!(resolve_model_alias("grok-2"), "grok-2");
    }

    #[test]
    fn detects_provider_from_model_name_first() {
        assert_eq!(detect_provider_kind("grok"), ProviderKind::Xai);
        assert_eq!(
            detect_provider_kind("claude-sonnet-4-6"),
            ProviderKind::Anthropic
        );
    }

    #[test]
    fn keeps_existing_max_token_heuristic() {
        assert_eq!(max_tokens_for_model("opus"), 32_000);
        assert_eq!(max_tokens_for_model("claude-sonnet-4-6"), 64_000);
        assert_eq!(max_tokens_for_model("grok-3"), 32_000);
        assert_eq!(max_tokens_for_model("llama-3.1-8b-instant"), 8_000);
        assert_eq!(max_tokens_for_model("gemma-7b-it"), 8_000);
        assert_eq!(max_tokens_for_model("gpt-4o"), 4_096);
        assert_eq!(max_tokens_for_model("gemini-2.5-flash"), 4_096);
    }

    #[test]
    fn provider_kind_from_str_maps_known_providers() {
        assert_eq!(provider_kind_from_str("anthropic"), Some(ProviderKind::Anthropic));
        assert_eq!(provider_kind_from_str("claude"), Some(ProviderKind::Anthropic));
        assert_eq!(provider_kind_from_str("openai"), Some(ProviderKind::OpenAi));
        assert_eq!(provider_kind_from_str("gemini"), Some(ProviderKind::OpenAi));
        assert_eq!(provider_kind_from_str("groq"), Some(ProviderKind::OpenAi));
        assert_eq!(provider_kind_from_str("xai"), Some(ProviderKind::Xai));
        assert_eq!(provider_kind_from_str("grok"), Some(ProviderKind::Xai));
        assert_eq!(provider_kind_from_str("unknown"), None);
    }

    #[test]
    fn provider_kind_from_str_is_case_insensitive() {
        assert_eq!(provider_kind_from_str("OPENAI"), Some(ProviderKind::OpenAi));
        assert_eq!(provider_kind_from_str("Gemini"), Some(ProviderKind::OpenAi));
    }

    #[test]
    fn detect_with_override_respects_explicit_provider() {
        assert_eq!(
            detect_provider_kind_with_override("claude-sonnet-4-6", Some("openai")),
            Ok(ProviderKind::OpenAi)
        );
        assert_eq!(
            detect_provider_kind_with_override("grok-3", Some("anthropic")),
            Ok(ProviderKind::Anthropic)
        );
    }

    #[test]
    fn detect_with_override_errors_on_unknown_provider() {
        assert!(detect_provider_kind_with_override("claude-sonnet-4-6", Some("unknown")).is_err());
    }

    #[test]
    fn detect_with_no_override_falls_back_to_model_registry() {
        assert_eq!(
            detect_provider_kind_with_override("grok-3", None),
            Ok(ProviderKind::Xai)
        );
    }
}
