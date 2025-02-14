// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetAgentAlias`](crate::operation::get_agent_alias::builders::GetAgentAliasFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`agent_id(impl Into<String>)`](crate::operation::get_agent_alias::builders::GetAgentAliasFluentBuilder::agent_id) / [`set_agent_id(Option<String>)`](crate::operation::get_agent_alias::builders::GetAgentAliasFluentBuilder::set_agent_id):<br>required: **true**<br>Id generated at the server side when an Agent is created<br>
    ///   - [`agent_alias_id(impl Into<String>)`](crate::operation::get_agent_alias::builders::GetAgentAliasFluentBuilder::agent_alias_id) / [`set_agent_alias_id(Option<String>)`](crate::operation::get_agent_alias::builders::GetAgentAliasFluentBuilder::set_agent_alias_id):<br>required: **true**<br>Id generated at the server side when an Agent Alias is created<br>
    /// - On success, responds with [`GetAgentAliasOutput`](crate::operation::get_agent_alias::GetAgentAliasOutput) with field(s):
    ///   - [`agent_alias(Option<AgentAlias>)`](crate::operation::get_agent_alias::GetAgentAliasOutput::agent_alias): Contains the information of an agent alias
    /// - On failure, responds with [`SdkError<GetAgentAliasError>`](crate::operation::get_agent_alias::GetAgentAliasError)
    pub fn get_agent_alias(&self) -> crate::operation::get_agent_alias::builders::GetAgentAliasFluentBuilder {
        crate::operation::get_agent_alias::builders::GetAgentAliasFluentBuilder::new(self.handle.clone())
    }
}
