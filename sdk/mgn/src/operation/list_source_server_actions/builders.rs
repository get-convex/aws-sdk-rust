// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_source_server_actions::_list_source_server_actions_output::ListSourceServerActionsOutputBuilder;

pub use crate::operation::list_source_server_actions::_list_source_server_actions_input::ListSourceServerActionsInputBuilder;

impl ListSourceServerActionsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_source_server_actions::ListSourceServerActionsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_source_server_actions::ListSourceServerActionsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_source_server_actions();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListSourceServerActions`.
///
/// <p>List source server post migration custom actions.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListSourceServerActionsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_source_server_actions::builders::ListSourceServerActionsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_source_server_actions::ListSourceServerActionsOutput,
        crate::operation::list_source_server_actions::ListSourceServerActionsError,
    > for ListSourceServerActionsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_source_server_actions::ListSourceServerActionsOutput,
            crate::operation::list_source_server_actions::ListSourceServerActionsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListSourceServerActionsFluentBuilder {
    /// Creates a new `ListSourceServerActions`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListSourceServerActions as a reference.
    pub fn as_input(&self) -> &crate::operation::list_source_server_actions::builders::ListSourceServerActionsInputBuilder {
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
        crate::operation::list_source_server_actions::ListSourceServerActionsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_source_server_actions::ListSourceServerActionsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_source_server_actions::ListSourceServerActions::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_source_server_actions::ListSourceServerActions::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::list_source_server_actions::ListSourceServerActionsOutput,
        crate::operation::list_source_server_actions::ListSourceServerActionsError,
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
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::list_source_server_actions::paginator::ListSourceServerActionsPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::list_source_server_actions::paginator::ListSourceServerActionsPaginator {
        crate::operation::list_source_server_actions::paginator::ListSourceServerActionsPaginator::new(self.handle, self.inner)
    }
    /// <p>Source server ID.</p>
    pub fn source_server_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.source_server_id(input.into());
        self
    }
    /// <p>Source server ID.</p>
    pub fn set_source_server_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_source_server_id(input);
        self
    }
    /// <p>Source server ID.</p>
    pub fn get_source_server_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_source_server_id()
    }
    /// <p>Filters to apply when listing source server post migration custom actions.</p>
    pub fn filters(mut self, input: crate::types::SourceServerActionsRequestFilters) -> Self {
        self.inner = self.inner.filters(input);
        self
    }
    /// <p>Filters to apply when listing source server post migration custom actions.</p>
    pub fn set_filters(mut self, input: ::std::option::Option<crate::types::SourceServerActionsRequestFilters>) -> Self {
        self.inner = self.inner.set_filters(input);
        self
    }
    /// <p>Filters to apply when listing source server post migration custom actions.</p>
    pub fn get_filters(&self) -> &::std::option::Option<crate::types::SourceServerActionsRequestFilters> {
        self.inner.get_filters()
    }
    /// <p>Maximum amount of items to return when listing source server post migration custom actions.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>Maximum amount of items to return when listing source server post migration custom actions.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>Maximum amount of items to return when listing source server post migration custom actions.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
    /// <p>Next token to use when listing source server post migration custom actions.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>Next token to use when listing source server post migration custom actions.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>Next token to use when listing source server post migration custom actions.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
    /// <p>Account ID to return when listing source server post migration custom actions.</p>
    pub fn account_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.account_id(input.into());
        self
    }
    /// <p>Account ID to return when listing source server post migration custom actions.</p>
    pub fn set_account_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_account_id(input);
        self
    }
    /// <p>Account ID to return when listing source server post migration custom actions.</p>
    pub fn get_account_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_account_id()
    }
}
