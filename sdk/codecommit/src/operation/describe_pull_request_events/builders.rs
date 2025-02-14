// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_pull_request_events::_describe_pull_request_events_output::DescribePullRequestEventsOutputBuilder;

pub use crate::operation::describe_pull_request_events::_describe_pull_request_events_input::DescribePullRequestEventsInputBuilder;

impl DescribePullRequestEventsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::describe_pull_request_events::DescribePullRequestEventsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_pull_request_events::DescribePullRequestEventsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.describe_pull_request_events();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DescribePullRequestEvents`.
///
/// <p>Returns information about one or more pull request events.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribePullRequestEventsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_pull_request_events::builders::DescribePullRequestEventsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::describe_pull_request_events::DescribePullRequestEventsOutput,
        crate::operation::describe_pull_request_events::DescribePullRequestEventsError,
    > for DescribePullRequestEventsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::describe_pull_request_events::DescribePullRequestEventsOutput,
            crate::operation::describe_pull_request_events::DescribePullRequestEventsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DescribePullRequestEventsFluentBuilder {
    /// Creates a new `DescribePullRequestEvents`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DescribePullRequestEvents as a reference.
    pub fn as_input(&self) -> &crate::operation::describe_pull_request_events::builders::DescribePullRequestEventsInputBuilder {
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
        crate::operation::describe_pull_request_events::DescribePullRequestEventsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_pull_request_events::DescribePullRequestEventsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::describe_pull_request_events::DescribePullRequestEvents::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::describe_pull_request_events::DescribePullRequestEvents::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::describe_pull_request_events::DescribePullRequestEventsOutput,
        crate::operation::describe_pull_request_events::DescribePullRequestEventsError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::describe_pull_request_events::paginator::DescribePullRequestEventsPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::describe_pull_request_events::paginator::DescribePullRequestEventsPaginator {
        crate::operation::describe_pull_request_events::paginator::DescribePullRequestEventsPaginator::new(self.handle, self.inner)
    }
    /// <p>The system-generated ID of the pull request. To get this ID, use <code>ListPullRequests</code>.</p>
    pub fn pull_request_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.pull_request_id(input.into());
        self
    }
    /// <p>The system-generated ID of the pull request. To get this ID, use <code>ListPullRequests</code>.</p>
    pub fn set_pull_request_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_pull_request_id(input);
        self
    }
    /// <p>The system-generated ID of the pull request. To get this ID, use <code>ListPullRequests</code>.</p>
    pub fn get_pull_request_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_pull_request_id()
    }
    /// <p>Optional. The pull request event type about which you want to return information.</p>
    pub fn pull_request_event_type(mut self, input: crate::types::PullRequestEventType) -> Self {
        self.inner = self.inner.pull_request_event_type(input);
        self
    }
    /// <p>Optional. The pull request event type about which you want to return information.</p>
    pub fn set_pull_request_event_type(mut self, input: ::std::option::Option<crate::types::PullRequestEventType>) -> Self {
        self.inner = self.inner.set_pull_request_event_type(input);
        self
    }
    /// <p>Optional. The pull request event type about which you want to return information.</p>
    pub fn get_pull_request_event_type(&self) -> &::std::option::Option<crate::types::PullRequestEventType> {
        self.inner.get_pull_request_event_type()
    }
    /// <p>The Amazon Resource Name (ARN) of the user whose actions resulted in the event. Examples include updating the pull request with more commits or changing the status of a pull request.</p>
    pub fn actor_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.actor_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the user whose actions resulted in the event. Examples include updating the pull request with more commits or changing the status of a pull request.</p>
    pub fn set_actor_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_actor_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the user whose actions resulted in the event. Examples include updating the pull request with more commits or changing the status of a pull request.</p>
    pub fn get_actor_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_actor_arn()
    }
    /// <p>An enumeration token that, when provided in a request, returns the next batch of the results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>An enumeration token that, when provided in a request, returns the next batch of the results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>An enumeration token that, when provided in a request, returns the next batch of the results.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
    /// <p>A non-zero, non-negative integer used to limit the number of returned results. The default is 100 events, which is also the maximum number of events that can be returned in a result.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>A non-zero, non-negative integer used to limit the number of returned results. The default is 100 events, which is also the maximum number of events that can be returned in a result.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>A non-zero, non-negative integer used to limit the number of returned results. The default is 100 events, which is also the maximum number of events that can be returned in a result.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
}
