// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_processing_job::_describe_processing_job_output::DescribeProcessingJobOutputBuilder;

pub use crate::operation::describe_processing_job::_describe_processing_job_input::DescribeProcessingJobInputBuilder;

impl DescribeProcessingJobInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::describe_processing_job::DescribeProcessingJobOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_processing_job::DescribeProcessingJobError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.describe_processing_job();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DescribeProcessingJob`.
///
/// <p>Returns a description of a processing job.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeProcessingJobFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_processing_job::builders::DescribeProcessingJobInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::describe_processing_job::DescribeProcessingJobOutput,
        crate::operation::describe_processing_job::DescribeProcessingJobError,
    > for DescribeProcessingJobFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::describe_processing_job::DescribeProcessingJobOutput,
            crate::operation::describe_processing_job::DescribeProcessingJobError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DescribeProcessingJobFluentBuilder {
    /// Creates a new `DescribeProcessingJob`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DescribeProcessingJob as a reference.
    pub fn as_input(&self) -> &crate::operation::describe_processing_job::builders::DescribeProcessingJobInputBuilder {
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
        crate::operation::describe_processing_job::DescribeProcessingJobOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_processing_job::DescribeProcessingJobError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::describe_processing_job::DescribeProcessingJob::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::describe_processing_job::DescribeProcessingJob::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::describe_processing_job::DescribeProcessingJobOutput,
        crate::operation::describe_processing_job::DescribeProcessingJobError,
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
    /// <p>The name of the processing job. The name must be unique within an Amazon Web Services Region in the Amazon Web Services account.</p>
    pub fn processing_job_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.processing_job_name(input.into());
        self
    }
    /// <p>The name of the processing job. The name must be unique within an Amazon Web Services Region in the Amazon Web Services account.</p>
    pub fn set_processing_job_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_processing_job_name(input);
        self
    }
    /// <p>The name of the processing job. The name must be unique within an Amazon Web Services Region in the Amazon Web Services account.</p>
    pub fn get_processing_job_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_processing_job_name()
    }
}
