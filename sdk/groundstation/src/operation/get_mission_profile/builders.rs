// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_mission_profile::_get_mission_profile_output::GetMissionProfileOutputBuilder;

pub use crate::operation::get_mission_profile::_get_mission_profile_input::GetMissionProfileInputBuilder;

impl GetMissionProfileInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_mission_profile::GetMissionProfileOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_mission_profile::GetMissionProfileError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_mission_profile();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetMissionProfile`.
///
/// <p>Returns a mission profile.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetMissionProfileFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_mission_profile::builders::GetMissionProfileInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::get_mission_profile::GetMissionProfileOutput,
        crate::operation::get_mission_profile::GetMissionProfileError,
    > for GetMissionProfileFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::get_mission_profile::GetMissionProfileOutput,
            crate::operation::get_mission_profile::GetMissionProfileError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetMissionProfileFluentBuilder {
    /// Creates a new `GetMissionProfile`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetMissionProfile as a reference.
    pub fn as_input(&self) -> &crate::operation::get_mission_profile::builders::GetMissionProfileInputBuilder {
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
        crate::operation::get_mission_profile::GetMissionProfileOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_mission_profile::GetMissionProfileError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_mission_profile::GetMissionProfile::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_mission_profile::GetMissionProfile::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::get_mission_profile::GetMissionProfileOutput,
        crate::operation::get_mission_profile::GetMissionProfileError,
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
    /// <p>UUID of a mission profile.</p>
    pub fn mission_profile_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.mission_profile_id(input.into());
        self
    }
    /// <p>UUID of a mission profile.</p>
    pub fn set_mission_profile_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_mission_profile_id(input);
        self
    }
    /// <p>UUID of a mission profile.</p>
    pub fn get_mission_profile_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_mission_profile_id()
    }
}
