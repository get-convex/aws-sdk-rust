// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// History event for an alias for an Agent.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AgentAliasHistoryEvent {
    /// Routing configuration for an Agent alias.
    pub routing_configuration: ::std::option::Option<::std::vec::Vec<crate::types::AgentAliasRoutingConfigurationListItem>>,
    /// Time Stamp.
    pub end_date: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// Time Stamp.
    pub start_date: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl AgentAliasHistoryEvent {
    /// Routing configuration for an Agent alias.
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.routing_configuration.is_none()`.
    pub fn routing_configuration(&self) -> &[crate::types::AgentAliasRoutingConfigurationListItem] {
        self.routing_configuration.as_deref().unwrap_or_default()
    }
    /// Time Stamp.
    pub fn end_date(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.end_date.as_ref()
    }
    /// Time Stamp.
    pub fn start_date(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.start_date.as_ref()
    }
}
impl AgentAliasHistoryEvent {
    /// Creates a new builder-style object to manufacture [`AgentAliasHistoryEvent`](crate::types::AgentAliasHistoryEvent).
    pub fn builder() -> crate::types::builders::AgentAliasHistoryEventBuilder {
        crate::types::builders::AgentAliasHistoryEventBuilder::default()
    }
}

/// A builder for [`AgentAliasHistoryEvent`](crate::types::AgentAliasHistoryEvent).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct AgentAliasHistoryEventBuilder {
    pub(crate) routing_configuration: ::std::option::Option<::std::vec::Vec<crate::types::AgentAliasRoutingConfigurationListItem>>,
    pub(crate) end_date: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) start_date: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl AgentAliasHistoryEventBuilder {
    /// Appends an item to `routing_configuration`.
    ///
    /// To override the contents of this collection use [`set_routing_configuration`](Self::set_routing_configuration).
    ///
    /// Routing configuration for an Agent alias.
    pub fn routing_configuration(mut self, input: crate::types::AgentAliasRoutingConfigurationListItem) -> Self {
        let mut v = self.routing_configuration.unwrap_or_default();
        v.push(input);
        self.routing_configuration = ::std::option::Option::Some(v);
        self
    }
    /// Routing configuration for an Agent alias.
    pub fn set_routing_configuration(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::AgentAliasRoutingConfigurationListItem>>,
    ) -> Self {
        self.routing_configuration = input;
        self
    }
    /// Routing configuration for an Agent alias.
    pub fn get_routing_configuration(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::AgentAliasRoutingConfigurationListItem>> {
        &self.routing_configuration
    }
    /// Time Stamp.
    pub fn end_date(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.end_date = ::std::option::Option::Some(input);
        self
    }
    /// Time Stamp.
    pub fn set_end_date(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.end_date = input;
        self
    }
    /// Time Stamp.
    pub fn get_end_date(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.end_date
    }
    /// Time Stamp.
    pub fn start_date(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.start_date = ::std::option::Option::Some(input);
        self
    }
    /// Time Stamp.
    pub fn set_start_date(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.start_date = input;
        self
    }
    /// Time Stamp.
    pub fn get_start_date(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.start_date
    }
    /// Consumes the builder and constructs a [`AgentAliasHistoryEvent`](crate::types::AgentAliasHistoryEvent).
    pub fn build(self) -> crate::types::AgentAliasHistoryEvent {
        crate::types::AgentAliasHistoryEvent {
            routing_configuration: self.routing_configuration,
            end_date: self.end_date,
            start_date: self.start_date,
        }
    }
}
