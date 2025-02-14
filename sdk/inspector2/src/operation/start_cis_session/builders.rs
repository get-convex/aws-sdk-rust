// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::start_cis_session::_start_cis_session_output::StartCisSessionOutputBuilder;

pub use crate::operation::start_cis_session::_start_cis_session_input::StartCisSessionInputBuilder;

impl StartCisSessionInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::start_cis_session::StartCisSessionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::start_cis_session::StartCisSessionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.start_cis_session();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `StartCisSession`.
///
/// <p>Starts a CIS session. This API is used by the Amazon Inspector SSM plugin to communicate with the Amazon Inspector service. The Amazon Inspector SSM plugin calls this API to start a CIS scan session for the scan ID supplied by the service.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct StartCisSessionFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::start_cis_session::builders::StartCisSessionInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::start_cis_session::StartCisSessionOutput,
        crate::operation::start_cis_session::StartCisSessionError,
    > for StartCisSessionFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::start_cis_session::StartCisSessionOutput,
            crate::operation::start_cis_session::StartCisSessionError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl StartCisSessionFluentBuilder {
    /// Creates a new `StartCisSession`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the StartCisSession as a reference.
    pub fn as_input(&self) -> &crate::operation::start_cis_session::builders::StartCisSessionInputBuilder {
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
        crate::operation::start_cis_session::StartCisSessionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::start_cis_session::StartCisSessionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::start_cis_session::StartCisSession::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::start_cis_session::StartCisSession::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::start_cis_session::StartCisSessionOutput,
        crate::operation::start_cis_session::StartCisSessionError,
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
    /// <p>A unique identifier for the scan job.</p>
    pub fn scan_job_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.scan_job_id(input.into());
        self
    }
    /// <p>A unique identifier for the scan job.</p>
    pub fn set_scan_job_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_scan_job_id(input);
        self
    }
    /// <p>A unique identifier for the scan job.</p>
    pub fn get_scan_job_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_scan_job_id()
    }
    /// <p>The start CIS session message.</p>
    pub fn message(mut self, input: crate::types::StartCisSessionMessage) -> Self {
        self.inner = self.inner.message(input);
        self
    }
    /// <p>The start CIS session message.</p>
    pub fn set_message(mut self, input: ::std::option::Option<crate::types::StartCisSessionMessage>) -> Self {
        self.inner = self.inner.set_message(input);
        self
    }
    /// <p>The start CIS session message.</p>
    pub fn get_message(&self) -> &::std::option::Option<crate::types::StartCisSessionMessage> {
        self.inner.get_message()
    }
}
