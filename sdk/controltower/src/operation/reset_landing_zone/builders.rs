// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::reset_landing_zone::_reset_landing_zone_output::ResetLandingZoneOutputBuilder;

pub use crate::operation::reset_landing_zone::_reset_landing_zone_input::ResetLandingZoneInputBuilder;

impl ResetLandingZoneInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::reset_landing_zone::ResetLandingZoneOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::reset_landing_zone::ResetLandingZoneError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.reset_landing_zone();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ResetLandingZone`.
///
/// <p>This API call resets a landing zone. It starts an asynchronous operation that resets the landing zone to the parameters specified in its original configuration.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ResetLandingZoneFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::reset_landing_zone::builders::ResetLandingZoneInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::reset_landing_zone::ResetLandingZoneOutput,
        crate::operation::reset_landing_zone::ResetLandingZoneError,
    > for ResetLandingZoneFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::reset_landing_zone::ResetLandingZoneOutput,
            crate::operation::reset_landing_zone::ResetLandingZoneError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ResetLandingZoneFluentBuilder {
    /// Creates a new `ResetLandingZone`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ResetLandingZone as a reference.
    pub fn as_input(&self) -> &crate::operation::reset_landing_zone::builders::ResetLandingZoneInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::reset_landing_zone::ResetLandingZoneOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::reset_landing_zone::ResetLandingZoneError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::reset_landing_zone::ResetLandingZone::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::reset_landing_zone::ResetLandingZone::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::reset_landing_zone::ResetLandingZoneOutput,
        crate::operation::reset_landing_zone::ResetLandingZoneError,
        Self,
    > {
        crate::client::customize::CustomizableOperation::new(self)
    }
    pub(crate) fn config_override(mut self, config_override: impl Into<crate::config::Builder>) -> Self {
        self.set_config_override(Some(config_override.into()));
        self
    }

    pub(crate) fn set_config_override(&mut self, config_override: Option<crate::config::Builder>) -> &mut Self {
        self.config_override = config_override;
        self
    }
    /// <p>The unique identifier of the landing zone.</p>
    pub fn landing_zone_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.landing_zone_identifier(input.into());
        self
    }
    /// <p>The unique identifier of the landing zone.</p>
    pub fn set_landing_zone_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_landing_zone_identifier(input);
        self
    }
    /// <p>The unique identifier of the landing zone.</p>
    pub fn get_landing_zone_identifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_landing_zone_identifier()
    }
}
