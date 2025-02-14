// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_keys::_list_keys_output::ListKeysOutputBuilder;

pub use crate::operation::list_keys::_list_keys_input::ListKeysInputBuilder;

impl ListKeysInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_keys::ListKeysOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_keys::ListKeysError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_keys();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListKeys`.
///
/// <p>Returns a list of key value pairs.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListKeysFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_keys::builders::ListKeysInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl crate::client::customize::internal::CustomizableSend<crate::operation::list_keys::ListKeysOutput, crate::operation::list_keys::ListKeysError>
    for ListKeysFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<crate::operation::list_keys::ListKeysOutput, crate::operation::list_keys::ListKeysError>,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListKeysFluentBuilder {
    /// Creates a new `ListKeys`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListKeys as a reference.
    pub fn as_input(&self) -> &crate::operation::list_keys::builders::ListKeysInputBuilder {
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
        crate::operation::list_keys::ListKeysOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_keys::ListKeysError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_keys::ListKeys::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_keys::ListKeys::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<crate::operation::list_keys::ListKeysOutput, crate::operation::list_keys::ListKeysError, Self>
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
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::list_keys::paginator::ListKeysPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::list_keys::paginator::ListKeysPaginator {
        crate::operation::list_keys::paginator::ListKeysPaginator::new(self.handle, self.inner)
    }
    /// <p>The Amazon Resource Name (ARN) of the Key Value Store.</p>
    pub fn kvs_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.kvs_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the Key Value Store.</p>
    pub fn set_kvs_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_kvs_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the Key Value Store.</p>
    pub fn get_kvs_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_kvs_arn()
    }
    /// <p>If nextToken is returned in the response, there are more results available. Make the next call using the returned token to retrieve the next page.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>If nextToken is returned in the response, there are more results available. Make the next call using the returned token to retrieve the next page.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>If nextToken is returned in the response, there are more results available. Make the next call using the returned token to retrieve the next page.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
    /// <p>Maximum number of results that are returned per call. The default is 10 and maximum allowed page is 50.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>Maximum number of results that are returned per call. The default is 10 and maximum allowed page is 50.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>Maximum number of results that are returned per call. The default is 10 and maximum allowed page is 50.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
}
