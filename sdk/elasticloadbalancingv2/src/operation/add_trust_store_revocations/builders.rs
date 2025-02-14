// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::add_trust_store_revocations::_add_trust_store_revocations_output::AddTrustStoreRevocationsOutputBuilder;

pub use crate::operation::add_trust_store_revocations::_add_trust_store_revocations_input::AddTrustStoreRevocationsInputBuilder;

impl AddTrustStoreRevocationsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::add_trust_store_revocations::AddTrustStoreRevocationsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::add_trust_store_revocations::AddTrustStoreRevocationsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.add_trust_store_revocations();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `AddTrustStoreRevocations`.
///
/// <p>Adds the specified revocation file to the specified trust store.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct AddTrustStoreRevocationsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::add_trust_store_revocations::builders::AddTrustStoreRevocationsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::add_trust_store_revocations::AddTrustStoreRevocationsOutput,
        crate::operation::add_trust_store_revocations::AddTrustStoreRevocationsError,
    > for AddTrustStoreRevocationsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::add_trust_store_revocations::AddTrustStoreRevocationsOutput,
            crate::operation::add_trust_store_revocations::AddTrustStoreRevocationsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl AddTrustStoreRevocationsFluentBuilder {
    /// Creates a new `AddTrustStoreRevocations`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the AddTrustStoreRevocations as a reference.
    pub fn as_input(&self) -> &crate::operation::add_trust_store_revocations::builders::AddTrustStoreRevocationsInputBuilder {
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
        crate::operation::add_trust_store_revocations::AddTrustStoreRevocationsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::add_trust_store_revocations::AddTrustStoreRevocationsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::add_trust_store_revocations::AddTrustStoreRevocations::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::add_trust_store_revocations::AddTrustStoreRevocations::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::add_trust_store_revocations::AddTrustStoreRevocationsOutput,
        crate::operation::add_trust_store_revocations::AddTrustStoreRevocationsError,
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
    /// <p>The Amazon Resource Name (ARN) of the trust store.</p>
    pub fn trust_store_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.trust_store_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the trust store.</p>
    pub fn set_trust_store_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_trust_store_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the trust store.</p>
    pub fn get_trust_store_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_trust_store_arn()
    }
    /// Appends an item to `RevocationContents`.
    ///
    /// To override the contents of this collection use [`set_revocation_contents`](Self::set_revocation_contents).
    ///
    /// <p>The revocation file to add.</p>
    pub fn revocation_contents(mut self, input: crate::types::RevocationContent) -> Self {
        self.inner = self.inner.revocation_contents(input);
        self
    }
    /// <p>The revocation file to add.</p>
    pub fn set_revocation_contents(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::RevocationContent>>) -> Self {
        self.inner = self.inner.set_revocation_contents(input);
        self
    }
    /// <p>The revocation file to add.</p>
    pub fn get_revocation_contents(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::RevocationContent>> {
        self.inner.get_revocation_contents()
    }
}
