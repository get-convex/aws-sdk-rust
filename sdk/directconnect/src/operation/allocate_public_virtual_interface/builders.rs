// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::allocate_public_virtual_interface::_allocate_public_virtual_interface_output::AllocatePublicVirtualInterfaceOutputBuilder;

pub use crate::operation::allocate_public_virtual_interface::_allocate_public_virtual_interface_input::AllocatePublicVirtualInterfaceInputBuilder;

impl AllocatePublicVirtualInterfaceInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::allocate_public_virtual_interface::AllocatePublicVirtualInterfaceOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::allocate_public_virtual_interface::AllocatePublicVirtualInterfaceError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.allocate_public_virtual_interface();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `AllocatePublicVirtualInterface`.
///
/// <p>Provisions a public virtual interface to be owned by the specified Amazon Web Services account.</p>
/// <p>The owner of a connection calls this function to provision a public virtual interface to be owned by the specified Amazon Web Services account.</p>
/// <p>Virtual interfaces created using this function must be confirmed by the owner using <code>ConfirmPublicVirtualInterface</code>. Until this step has been completed, the virtual interface is in the <code>confirming</code> state and is not available to handle traffic.</p>
/// <p>When creating an IPv6 public virtual interface, omit the Amazon address and customer address. IPv6 addresses are automatically assigned from the Amazon pool of IPv6 addresses; you cannot specify custom IPv6 addresses.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct AllocatePublicVirtualInterfaceFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::allocate_public_virtual_interface::builders::AllocatePublicVirtualInterfaceInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::allocate_public_virtual_interface::AllocatePublicVirtualInterfaceOutput,
        crate::operation::allocate_public_virtual_interface::AllocatePublicVirtualInterfaceError,
    > for AllocatePublicVirtualInterfaceFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::allocate_public_virtual_interface::AllocatePublicVirtualInterfaceOutput,
            crate::operation::allocate_public_virtual_interface::AllocatePublicVirtualInterfaceError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl AllocatePublicVirtualInterfaceFluentBuilder {
    /// Creates a new `AllocatePublicVirtualInterface`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the AllocatePublicVirtualInterface as a reference.
    pub fn as_input(&self) -> &crate::operation::allocate_public_virtual_interface::builders::AllocatePublicVirtualInterfaceInputBuilder {
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
        crate::operation::allocate_public_virtual_interface::AllocatePublicVirtualInterfaceOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::allocate_public_virtual_interface::AllocatePublicVirtualInterfaceError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::allocate_public_virtual_interface::AllocatePublicVirtualInterface::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::allocate_public_virtual_interface::AllocatePublicVirtualInterface::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::allocate_public_virtual_interface::AllocatePublicVirtualInterfaceOutput,
        crate::operation::allocate_public_virtual_interface::AllocatePublicVirtualInterfaceError,
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
    /// <p>The ID of the connection on which the public virtual interface is provisioned.</p>
    pub fn connection_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.connection_id(input.into());
        self
    }
    /// <p>The ID of the connection on which the public virtual interface is provisioned.</p>
    pub fn set_connection_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_connection_id(input);
        self
    }
    /// <p>The ID of the connection on which the public virtual interface is provisioned.</p>
    pub fn get_connection_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_connection_id()
    }
    /// <p>The ID of the Amazon Web Services account that owns the public virtual interface.</p>
    pub fn owner_account(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.owner_account(input.into());
        self
    }
    /// <p>The ID of the Amazon Web Services account that owns the public virtual interface.</p>
    pub fn set_owner_account(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_owner_account(input);
        self
    }
    /// <p>The ID of the Amazon Web Services account that owns the public virtual interface.</p>
    pub fn get_owner_account(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_owner_account()
    }
    /// <p>Information about the public virtual interface.</p>
    pub fn new_public_virtual_interface_allocation(mut self, input: crate::types::NewPublicVirtualInterfaceAllocation) -> Self {
        self.inner = self.inner.new_public_virtual_interface_allocation(input);
        self
    }
    /// <p>Information about the public virtual interface.</p>
    pub fn set_new_public_virtual_interface_allocation(
        mut self,
        input: ::std::option::Option<crate::types::NewPublicVirtualInterfaceAllocation>,
    ) -> Self {
        self.inner = self.inner.set_new_public_virtual_interface_allocation(input);
        self
    }
    /// <p>Information about the public virtual interface.</p>
    pub fn get_new_public_virtual_interface_allocation(&self) -> &::std::option::Option<crate::types::NewPublicVirtualInterfaceAllocation> {
        self.inner.get_new_public_virtual_interface_allocation()
    }
}
