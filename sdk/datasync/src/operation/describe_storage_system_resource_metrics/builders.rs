// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_storage_system_resource_metrics::_describe_storage_system_resource_metrics_output::DescribeStorageSystemResourceMetricsOutputBuilder;

pub use crate::operation::describe_storage_system_resource_metrics::_describe_storage_system_resource_metrics_input::DescribeStorageSystemResourceMetricsInputBuilder;

impl DescribeStorageSystemResourceMetricsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::describe_storage_system_resource_metrics::DescribeStorageSystemResourceMetricsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_storage_system_resource_metrics::DescribeStorageSystemResourceMetricsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.describe_storage_system_resource_metrics();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DescribeStorageSystemResourceMetrics`.
///
/// <p>Returns information, including performance data and capacity usage, which DataSync Discovery collects about a specific resource in your-premises storage system.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeStorageSystemResourceMetricsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_storage_system_resource_metrics::builders::DescribeStorageSystemResourceMetricsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::describe_storage_system_resource_metrics::DescribeStorageSystemResourceMetricsOutput,
        crate::operation::describe_storage_system_resource_metrics::DescribeStorageSystemResourceMetricsError,
    > for DescribeStorageSystemResourceMetricsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::describe_storage_system_resource_metrics::DescribeStorageSystemResourceMetricsOutput,
            crate::operation::describe_storage_system_resource_metrics::DescribeStorageSystemResourceMetricsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DescribeStorageSystemResourceMetricsFluentBuilder {
    /// Creates a new `DescribeStorageSystemResourceMetrics`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DescribeStorageSystemResourceMetrics as a reference.
    pub fn as_input(
        &self,
    ) -> &crate::operation::describe_storage_system_resource_metrics::builders::DescribeStorageSystemResourceMetricsInputBuilder {
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
        crate::operation::describe_storage_system_resource_metrics::DescribeStorageSystemResourceMetricsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_storage_system_resource_metrics::DescribeStorageSystemResourceMetricsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins =
            crate::operation::describe_storage_system_resource_metrics::DescribeStorageSystemResourceMetrics::operation_runtime_plugins(
                self.handle.runtime_plugins.clone(),
                &self.handle.conf,
                self.config_override,
            );
        crate::operation::describe_storage_system_resource_metrics::DescribeStorageSystemResourceMetrics::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::describe_storage_system_resource_metrics::DescribeStorageSystemResourceMetricsOutput,
        crate::operation::describe_storage_system_resource_metrics::DescribeStorageSystemResourceMetricsError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::describe_storage_system_resource_metrics::paginator::DescribeStorageSystemResourceMetricsPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(
        self,
    ) -> crate::operation::describe_storage_system_resource_metrics::paginator::DescribeStorageSystemResourceMetricsPaginator {
        crate::operation::describe_storage_system_resource_metrics::paginator::DescribeStorageSystemResourceMetricsPaginator::new(
            self.handle,
            self.inner,
        )
    }
    /// <p>Specifies the Amazon Resource Name (ARN) of the discovery job that collects information about your on-premises storage system.</p>
    pub fn discovery_job_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.discovery_job_arn(input.into());
        self
    }
    /// <p>Specifies the Amazon Resource Name (ARN) of the discovery job that collects information about your on-premises storage system.</p>
    pub fn set_discovery_job_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_discovery_job_arn(input);
        self
    }
    /// <p>Specifies the Amazon Resource Name (ARN) of the discovery job that collects information about your on-premises storage system.</p>
    pub fn get_discovery_job_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_discovery_job_arn()
    }
    /// <p>Specifies the kind of storage system resource that you want information about.</p>
    pub fn resource_type(mut self, input: crate::types::DiscoveryResourceType) -> Self {
        self.inner = self.inner.resource_type(input);
        self
    }
    /// <p>Specifies the kind of storage system resource that you want information about.</p>
    pub fn set_resource_type(mut self, input: ::std::option::Option<crate::types::DiscoveryResourceType>) -> Self {
        self.inner = self.inner.set_resource_type(input);
        self
    }
    /// <p>Specifies the kind of storage system resource that you want information about.</p>
    pub fn get_resource_type(&self) -> &::std::option::Option<crate::types::DiscoveryResourceType> {
        self.inner.get_resource_type()
    }
    /// <p>Specifies the universally unique identifier (UUID) of the storage system resource that you want information about.</p>
    pub fn resource_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.resource_id(input.into());
        self
    }
    /// <p>Specifies the universally unique identifier (UUID) of the storage system resource that you want information about.</p>
    pub fn set_resource_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_resource_id(input);
        self
    }
    /// <p>Specifies the universally unique identifier (UUID) of the storage system resource that you want information about.</p>
    pub fn get_resource_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_resource_id()
    }
    /// <p>Specifies a time within the total duration that the discovery job ran. To see information gathered during a certain time frame, use this parameter with <code>EndTime</code>.</p>
    pub fn start_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.start_time(input);
        self
    }
    /// <p>Specifies a time within the total duration that the discovery job ran. To see information gathered during a certain time frame, use this parameter with <code>EndTime</code>.</p>
    pub fn set_start_time(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.inner = self.inner.set_start_time(input);
        self
    }
    /// <p>Specifies a time within the total duration that the discovery job ran. To see information gathered during a certain time frame, use this parameter with <code>EndTime</code>.</p>
    pub fn get_start_time(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        self.inner.get_start_time()
    }
    /// <p>Specifies a time within the total duration that the discovery job ran. To see information gathered during a certain time frame, use this parameter with <code>StartTime</code>.</p>
    pub fn end_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.end_time(input);
        self
    }
    /// <p>Specifies a time within the total duration that the discovery job ran. To see information gathered during a certain time frame, use this parameter with <code>StartTime</code>.</p>
    pub fn set_end_time(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.inner = self.inner.set_end_time(input);
        self
    }
    /// <p>Specifies a time within the total duration that the discovery job ran. To see information gathered during a certain time frame, use this parameter with <code>StartTime</code>.</p>
    pub fn get_end_time(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        self.inner.get_end_time()
    }
    /// <p>Specifies how many results that you want in the response.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>Specifies how many results that you want in the response.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>Specifies how many results that you want in the response.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
    /// <p>Specifies an opaque string that indicates the position to begin the next list of results in the response.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>Specifies an opaque string that indicates the position to begin the next list of results in the response.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>Specifies an opaque string that indicates the position to begin the next list of results in the response.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
}
