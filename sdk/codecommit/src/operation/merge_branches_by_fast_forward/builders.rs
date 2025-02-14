// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::merge_branches_by_fast_forward::_merge_branches_by_fast_forward_output::MergeBranchesByFastForwardOutputBuilder;

pub use crate::operation::merge_branches_by_fast_forward::_merge_branches_by_fast_forward_input::MergeBranchesByFastForwardInputBuilder;

impl MergeBranchesByFastForwardInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::merge_branches_by_fast_forward::MergeBranchesByFastForwardOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::merge_branches_by_fast_forward::MergeBranchesByFastForwardError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.merge_branches_by_fast_forward();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `MergeBranchesByFastForward`.
///
/// <p>Merges two branches using the fast-forward merge strategy.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct MergeBranchesByFastForwardFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::merge_branches_by_fast_forward::builders::MergeBranchesByFastForwardInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::merge_branches_by_fast_forward::MergeBranchesByFastForwardOutput,
        crate::operation::merge_branches_by_fast_forward::MergeBranchesByFastForwardError,
    > for MergeBranchesByFastForwardFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::merge_branches_by_fast_forward::MergeBranchesByFastForwardOutput,
            crate::operation::merge_branches_by_fast_forward::MergeBranchesByFastForwardError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl MergeBranchesByFastForwardFluentBuilder {
    /// Creates a new `MergeBranchesByFastForward`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the MergeBranchesByFastForward as a reference.
    pub fn as_input(&self) -> &crate::operation::merge_branches_by_fast_forward::builders::MergeBranchesByFastForwardInputBuilder {
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
        crate::operation::merge_branches_by_fast_forward::MergeBranchesByFastForwardOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::merge_branches_by_fast_forward::MergeBranchesByFastForwardError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::merge_branches_by_fast_forward::MergeBranchesByFastForward::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::merge_branches_by_fast_forward::MergeBranchesByFastForward::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::merge_branches_by_fast_forward::MergeBranchesByFastForwardOutput,
        crate::operation::merge_branches_by_fast_forward::MergeBranchesByFastForwardError,
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
    /// <p>The name of the repository where you want to merge two branches.</p>
    pub fn repository_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.repository_name(input.into());
        self
    }
    /// <p>The name of the repository where you want to merge two branches.</p>
    pub fn set_repository_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_repository_name(input);
        self
    }
    /// <p>The name of the repository where you want to merge two branches.</p>
    pub fn get_repository_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_repository_name()
    }
    /// <p>The branch, tag, HEAD, or other fully qualified reference used to identify a commit (for example, a branch name or a full commit ID).</p>
    pub fn source_commit_specifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.source_commit_specifier(input.into());
        self
    }
    /// <p>The branch, tag, HEAD, or other fully qualified reference used to identify a commit (for example, a branch name or a full commit ID).</p>
    pub fn set_source_commit_specifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_source_commit_specifier(input);
        self
    }
    /// <p>The branch, tag, HEAD, or other fully qualified reference used to identify a commit (for example, a branch name or a full commit ID).</p>
    pub fn get_source_commit_specifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_source_commit_specifier()
    }
    /// <p>The branch, tag, HEAD, or other fully qualified reference used to identify a commit (for example, a branch name or a full commit ID).</p>
    pub fn destination_commit_specifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.destination_commit_specifier(input.into());
        self
    }
    /// <p>The branch, tag, HEAD, or other fully qualified reference used to identify a commit (for example, a branch name or a full commit ID).</p>
    pub fn set_destination_commit_specifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_destination_commit_specifier(input);
        self
    }
    /// <p>The branch, tag, HEAD, or other fully qualified reference used to identify a commit (for example, a branch name or a full commit ID).</p>
    pub fn get_destination_commit_specifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_destination_commit_specifier()
    }
    /// <p>The branch where the merge is applied.</p>
    pub fn target_branch(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.target_branch(input.into());
        self
    }
    /// <p>The branch where the merge is applied.</p>
    pub fn set_target_branch(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_target_branch(input);
        self
    }
    /// <p>The branch where the merge is applied.</p>
    pub fn get_target_branch(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_target_branch()
    }
}
