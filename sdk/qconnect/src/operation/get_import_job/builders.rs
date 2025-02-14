// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_import_job::_get_import_job_output::GetImportJobOutputBuilder;

pub use crate::operation::get_import_job::_get_import_job_input::GetImportJobInputBuilder;

impl GetImportJobInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_import_job::GetImportJobOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_import_job::GetImportJobError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_import_job();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetImportJob`.
///
/// <p>Retrieves the started import job.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetImportJobFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_import_job::builders::GetImportJobInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::get_import_job::GetImportJobOutput,
        crate::operation::get_import_job::GetImportJobError,
    > for GetImportJobFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::get_import_job::GetImportJobOutput,
            crate::operation::get_import_job::GetImportJobError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetImportJobFluentBuilder {
    /// Creates a new `GetImportJob`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetImportJob as a reference.
    pub fn as_input(&self) -> &crate::operation::get_import_job::builders::GetImportJobInputBuilder {
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
        crate::operation::get_import_job::GetImportJobOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_import_job::GetImportJobError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_import_job::GetImportJob::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_import_job::GetImportJob::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::get_import_job::GetImportJobOutput,
        crate::operation::get_import_job::GetImportJobError,
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
    /// <p>The identifier of the import job to retrieve.</p>
    pub fn import_job_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.import_job_id(input.into());
        self
    }
    /// <p>The identifier of the import job to retrieve.</p>
    pub fn set_import_job_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_import_job_id(input);
        self
    }
    /// <p>The identifier of the import job to retrieve.</p>
    pub fn get_import_job_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_import_job_id()
    }
    /// <p>The identifier of the knowledge base that the import job belongs to.</p>
    pub fn knowledge_base_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.knowledge_base_id(input.into());
        self
    }
    /// <p>The identifier of the knowledge base that the import job belongs to.</p>
    pub fn set_knowledge_base_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_knowledge_base_id(input);
        self
    }
    /// <p>The identifier of the knowledge base that the import job belongs to.</p>
    pub fn get_knowledge_base_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_knowledge_base_id()
    }
}
