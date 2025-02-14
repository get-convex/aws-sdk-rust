// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_flow_logs_integration_template::_get_flow_logs_integration_template_output::GetFlowLogsIntegrationTemplateOutputBuilder;

pub use crate::operation::get_flow_logs_integration_template::_get_flow_logs_integration_template_input::GetFlowLogsIntegrationTemplateInputBuilder;

impl GetFlowLogsIntegrationTemplateInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_flow_logs_integration_template::GetFlowLogsIntegrationTemplateOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_flow_logs_integration_template::GetFlowLogsIntegrationTemplateError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_flow_logs_integration_template();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetFlowLogsIntegrationTemplate`.
///
/// <p>Generates a CloudFormation template that streamlines and automates the integration of VPC flow logs with Amazon Athena. This make it easier for you to query and gain insights from VPC flow logs data. Based on the information that you provide, we configure resources in the template to do the following:</p>
/// <ul>
/// <li>
/// <p>Create a table in Athena that maps fields to a custom log format</p></li>
/// <li>
/// <p>Create a Lambda function that updates the table with new partitions on a daily, weekly, or monthly basis</p></li>
/// <li>
/// <p>Create a table partitioned between two timestamps in the past</p></li>
/// <li>
/// <p>Create a set of named queries in Athena that you can use to get started quickly</p></li>
/// </ul><note>
/// <p><code>GetFlowLogsIntegrationTemplate</code> does not support integration between Amazon Web Services Transit Gateway Flow Logs and Amazon Athena.</p>
/// </note>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetFlowLogsIntegrationTemplateFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_flow_logs_integration_template::builders::GetFlowLogsIntegrationTemplateInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::get_flow_logs_integration_template::GetFlowLogsIntegrationTemplateOutput,
        crate::operation::get_flow_logs_integration_template::GetFlowLogsIntegrationTemplateError,
    > for GetFlowLogsIntegrationTemplateFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::get_flow_logs_integration_template::GetFlowLogsIntegrationTemplateOutput,
            crate::operation::get_flow_logs_integration_template::GetFlowLogsIntegrationTemplateError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetFlowLogsIntegrationTemplateFluentBuilder {
    /// Creates a new `GetFlowLogsIntegrationTemplate`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetFlowLogsIntegrationTemplate as a reference.
    pub fn as_input(&self) -> &crate::operation::get_flow_logs_integration_template::builders::GetFlowLogsIntegrationTemplateInputBuilder {
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
        crate::operation::get_flow_logs_integration_template::GetFlowLogsIntegrationTemplateOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_flow_logs_integration_template::GetFlowLogsIntegrationTemplateError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_flow_logs_integration_template::GetFlowLogsIntegrationTemplate::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_flow_logs_integration_template::GetFlowLogsIntegrationTemplate::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::get_flow_logs_integration_template::GetFlowLogsIntegrationTemplateOutput,
        crate::operation::get_flow_logs_integration_template::GetFlowLogsIntegrationTemplateError,
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
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.inner = self.inner.dry_run(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_dry_run(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn get_dry_run(&self) -> &::std::option::Option<bool> {
        self.inner.get_dry_run()
    }
    /// <p>The ID of the flow log.</p>
    pub fn flow_log_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.flow_log_id(input.into());
        self
    }
    /// <p>The ID of the flow log.</p>
    pub fn set_flow_log_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_flow_log_id(input);
        self
    }
    /// <p>The ID of the flow log.</p>
    pub fn get_flow_log_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_flow_log_id()
    }
    /// <p>To store the CloudFormation template in Amazon S3, specify the location in Amazon S3.</p>
    pub fn config_delivery_s3_destination_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.config_delivery_s3_destination_arn(input.into());
        self
    }
    /// <p>To store the CloudFormation template in Amazon S3, specify the location in Amazon S3.</p>
    pub fn set_config_delivery_s3_destination_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_config_delivery_s3_destination_arn(input);
        self
    }
    /// <p>To store the CloudFormation template in Amazon S3, specify the location in Amazon S3.</p>
    pub fn get_config_delivery_s3_destination_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_config_delivery_s3_destination_arn()
    }
    /// <p>Information about the service integration.</p>
    pub fn integrate_services(mut self, input: crate::types::IntegrateServices) -> Self {
        self.inner = self.inner.integrate_services(input);
        self
    }
    /// <p>Information about the service integration.</p>
    pub fn set_integrate_services(mut self, input: ::std::option::Option<crate::types::IntegrateServices>) -> Self {
        self.inner = self.inner.set_integrate_services(input);
        self
    }
    /// <p>Information about the service integration.</p>
    pub fn get_integrate_services(&self) -> &::std::option::Option<crate::types::IntegrateServices> {
        self.inner.get_integrate_services()
    }
}
