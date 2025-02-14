// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_resource_groups::_describe_resource_groups_output::DescribeResourceGroupsOutputBuilder;

pub use crate::operation::describe_resource_groups::_describe_resource_groups_input::DescribeResourceGroupsInputBuilder;

impl DescribeResourceGroupsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::describe_resource_groups::DescribeResourceGroupsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_resource_groups::DescribeResourceGroupsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.describe_resource_groups();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DescribeResourceGroups`.
///
/// <p>Describes the resource groups that are specified by the ARNs of the resource groups.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeResourceGroupsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_resource_groups::builders::DescribeResourceGroupsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::describe_resource_groups::DescribeResourceGroupsOutput,
        crate::operation::describe_resource_groups::DescribeResourceGroupsError,
    > for DescribeResourceGroupsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::describe_resource_groups::DescribeResourceGroupsOutput,
            crate::operation::describe_resource_groups::DescribeResourceGroupsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DescribeResourceGroupsFluentBuilder {
    /// Creates a new `DescribeResourceGroups`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DescribeResourceGroups as a reference.
    pub fn as_input(&self) -> &crate::operation::describe_resource_groups::builders::DescribeResourceGroupsInputBuilder {
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
        crate::operation::describe_resource_groups::DescribeResourceGroupsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_resource_groups::DescribeResourceGroupsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::describe_resource_groups::DescribeResourceGroups::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::describe_resource_groups::DescribeResourceGroups::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::describe_resource_groups::DescribeResourceGroupsOutput,
        crate::operation::describe_resource_groups::DescribeResourceGroupsError,
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
    /// Appends an item to `resourceGroupArns`.
    ///
    /// To override the contents of this collection use [`set_resource_group_arns`](Self::set_resource_group_arns).
    ///
    /// <p>The ARN that specifies the resource group that you want to describe.</p>
    pub fn resource_group_arns(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.resource_group_arns(input.into());
        self
    }
    /// <p>The ARN that specifies the resource group that you want to describe.</p>
    pub fn set_resource_group_arns(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_resource_group_arns(input);
        self
    }
    /// <p>The ARN that specifies the resource group that you want to describe.</p>
    pub fn get_resource_group_arns(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_resource_group_arns()
    }
}
