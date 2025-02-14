// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::generate_change_set::_generate_change_set_output::GenerateChangeSetOutputBuilder;

pub use crate::operation::generate_change_set::_generate_change_set_input::GenerateChangeSetInputBuilder;

impl GenerateChangeSetInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::generate_change_set::GenerateChangeSetOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::generate_change_set::GenerateChangeSetError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.generate_change_set();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GenerateChangeSet`.
///
/// <p>Generates a target change set for a currently launched stack and writes it to an Amazon S3 object in the customer’s Amazon S3 bucket.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GenerateChangeSetFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::generate_change_set::builders::GenerateChangeSetInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::generate_change_set::GenerateChangeSetOutput,
        crate::operation::generate_change_set::GenerateChangeSetError,
    > for GenerateChangeSetFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::generate_change_set::GenerateChangeSetOutput,
            crate::operation::generate_change_set::GenerateChangeSetError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GenerateChangeSetFluentBuilder {
    /// Creates a new `GenerateChangeSet`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GenerateChangeSet as a reference.
    pub fn as_input(&self) -> &crate::operation::generate_change_set::builders::GenerateChangeSetInputBuilder {
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
        crate::operation::generate_change_set::GenerateChangeSetOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::generate_change_set::GenerateChangeSetError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::generate_change_set::GenerateChangeSet::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::generate_change_set::GenerateChangeSet::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::generate_change_set::GenerateChangeSetOutput,
        crate::operation::generate_change_set::GenerateChangeSetError,
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
    /// <p>The ID of the application associated with the change set.</p>
    pub fn app_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.app_id(input.into());
        self
    }
    /// <p>The ID of the application associated with the change set.</p>
    pub fn set_app_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_app_id(input);
        self
    }
    /// <p>The ID of the application associated with the change set.</p>
    pub fn get_app_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_app_id()
    }
    /// <p>The format for the change set.</p>
    pub fn changeset_format(mut self, input: crate::types::OutputFormat) -> Self {
        self.inner = self.inner.changeset_format(input);
        self
    }
    /// <p>The format for the change set.</p>
    pub fn set_changeset_format(mut self, input: ::std::option::Option<crate::types::OutputFormat>) -> Self {
        self.inner = self.inner.set_changeset_format(input);
        self
    }
    /// <p>The format for the change set.</p>
    pub fn get_changeset_format(&self) -> &::std::option::Option<crate::types::OutputFormat> {
        self.inner.get_changeset_format()
    }
}
