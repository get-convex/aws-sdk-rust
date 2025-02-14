// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::put_account_vdm_attributes::_put_account_vdm_attributes_output::PutAccountVdmAttributesOutputBuilder;

pub use crate::operation::put_account_vdm_attributes::_put_account_vdm_attributes_input::PutAccountVdmAttributesInputBuilder;

impl PutAccountVdmAttributesInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::put_account_vdm_attributes::PutAccountVdmAttributesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::put_account_vdm_attributes::PutAccountVdmAttributesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.put_account_vdm_attributes();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `PutAccountVdmAttributes`.
///
/// <p>Update your Amazon SES account VDM attributes.</p>
/// <p>You can execute this operation no more than once per second.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct PutAccountVdmAttributesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::put_account_vdm_attributes::builders::PutAccountVdmAttributesInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::put_account_vdm_attributes::PutAccountVdmAttributesOutput,
        crate::operation::put_account_vdm_attributes::PutAccountVdmAttributesError,
    > for PutAccountVdmAttributesFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::put_account_vdm_attributes::PutAccountVdmAttributesOutput,
            crate::operation::put_account_vdm_attributes::PutAccountVdmAttributesError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl PutAccountVdmAttributesFluentBuilder {
    /// Creates a new `PutAccountVdmAttributes`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the PutAccountVdmAttributes as a reference.
    pub fn as_input(&self) -> &crate::operation::put_account_vdm_attributes::builders::PutAccountVdmAttributesInputBuilder {
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
        crate::operation::put_account_vdm_attributes::PutAccountVdmAttributesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::put_account_vdm_attributes::PutAccountVdmAttributesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::put_account_vdm_attributes::PutAccountVdmAttributes::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::put_account_vdm_attributes::PutAccountVdmAttributes::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::put_account_vdm_attributes::PutAccountVdmAttributesOutput,
        crate::operation::put_account_vdm_attributes::PutAccountVdmAttributesError,
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
    /// <p>The VDM attributes that you wish to apply to your Amazon SES account.</p>
    pub fn vdm_attributes(mut self, input: crate::types::VdmAttributes) -> Self {
        self.inner = self.inner.vdm_attributes(input);
        self
    }
    /// <p>The VDM attributes that you wish to apply to your Amazon SES account.</p>
    pub fn set_vdm_attributes(mut self, input: ::std::option::Option<crate::types::VdmAttributes>) -> Self {
        self.inner = self.inner.set_vdm_attributes(input);
        self
    }
    /// <p>The VDM attributes that you wish to apply to your Amazon SES account.</p>
    pub fn get_vdm_attributes(&self) -> &::std::option::Option<crate::types::VdmAttributes> {
        self.inner.get_vdm_attributes()
    }
}
