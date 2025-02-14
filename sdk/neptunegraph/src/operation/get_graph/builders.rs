// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_graph::_get_graph_output::GetGraphOutputBuilder;

pub use crate::operation::get_graph::_get_graph_input::GetGraphInputBuilder;

impl GetGraphInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_graph::GetGraphOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_graph::GetGraphError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_graph();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetGraph`.
///
/// <p>Gets information about a specified graph.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetGraphFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_graph::builders::GetGraphInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl crate::client::customize::internal::CustomizableSend<crate::operation::get_graph::GetGraphOutput, crate::operation::get_graph::GetGraphError>
    for GetGraphFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<crate::operation::get_graph::GetGraphOutput, crate::operation::get_graph::GetGraphError>,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetGraphFluentBuilder {
    /// Creates a new `GetGraph`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetGraph as a reference.
    pub fn as_input(&self) -> &crate::operation::get_graph::builders::GetGraphInputBuilder {
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
        crate::operation::get_graph::GetGraphOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_graph::GetGraphError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_graph::GetGraph::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_graph::GetGraph::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<crate::operation::get_graph::GetGraphOutput, crate::operation::get_graph::GetGraphError, Self>
    {
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
    /// <p>The unique identifier of the Neptune Analytics graph.</p>
    pub fn graph_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.graph_identifier(input.into());
        self
    }
    /// <p>The unique identifier of the Neptune Analytics graph.</p>
    pub fn set_graph_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_graph_identifier(input);
        self
    }
    /// <p>The unique identifier of the Neptune Analytics graph.</p>
    pub fn get_graph_identifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_graph_identifier()
    }
}
