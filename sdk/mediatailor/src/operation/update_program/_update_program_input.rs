// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpdateProgramInput {
    /// <p>The ad break configuration settings.</p>
    pub ad_breaks: ::std::option::Option<::std::vec::Vec<crate::types::AdBreak>>,
    /// <p>The name of the channel for this Program.</p>
    pub channel_name: ::std::option::Option<::std::string::String>,
    /// <p>The name of the Program.</p>
    pub program_name: ::std::option::Option<::std::string::String>,
    /// <p>The schedule configuration settings.</p>
    pub schedule_configuration: ::std::option::Option<crate::types::UpdateProgramScheduleConfiguration>,
}
impl UpdateProgramInput {
    /// <p>The ad break configuration settings.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.ad_breaks.is_none()`.
    pub fn ad_breaks(&self) -> &[crate::types::AdBreak] {
        self.ad_breaks.as_deref().unwrap_or_default()
    }
    /// <p>The name of the channel for this Program.</p>
    pub fn channel_name(&self) -> ::std::option::Option<&str> {
        self.channel_name.as_deref()
    }
    /// <p>The name of the Program.</p>
    pub fn program_name(&self) -> ::std::option::Option<&str> {
        self.program_name.as_deref()
    }
    /// <p>The schedule configuration settings.</p>
    pub fn schedule_configuration(&self) -> ::std::option::Option<&crate::types::UpdateProgramScheduleConfiguration> {
        self.schedule_configuration.as_ref()
    }
}
impl UpdateProgramInput {
    /// Creates a new builder-style object to manufacture [`UpdateProgramInput`](crate::operation::update_program::UpdateProgramInput).
    pub fn builder() -> crate::operation::update_program::builders::UpdateProgramInputBuilder {
        crate::operation::update_program::builders::UpdateProgramInputBuilder::default()
    }
}

/// A builder for [`UpdateProgramInput`](crate::operation::update_program::UpdateProgramInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct UpdateProgramInputBuilder {
    pub(crate) ad_breaks: ::std::option::Option<::std::vec::Vec<crate::types::AdBreak>>,
    pub(crate) channel_name: ::std::option::Option<::std::string::String>,
    pub(crate) program_name: ::std::option::Option<::std::string::String>,
    pub(crate) schedule_configuration: ::std::option::Option<crate::types::UpdateProgramScheduleConfiguration>,
}
impl UpdateProgramInputBuilder {
    /// Appends an item to `ad_breaks`.
    ///
    /// To override the contents of this collection use [`set_ad_breaks`](Self::set_ad_breaks).
    ///
    /// <p>The ad break configuration settings.</p>
    pub fn ad_breaks(mut self, input: crate::types::AdBreak) -> Self {
        let mut v = self.ad_breaks.unwrap_or_default();
        v.push(input);
        self.ad_breaks = ::std::option::Option::Some(v);
        self
    }
    /// <p>The ad break configuration settings.</p>
    pub fn set_ad_breaks(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::AdBreak>>) -> Self {
        self.ad_breaks = input;
        self
    }
    /// <p>The ad break configuration settings.</p>
    pub fn get_ad_breaks(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::AdBreak>> {
        &self.ad_breaks
    }
    /// <p>The name of the channel for this Program.</p>
    /// This field is required.
    pub fn channel_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.channel_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the channel for this Program.</p>
    pub fn set_channel_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.channel_name = input;
        self
    }
    /// <p>The name of the channel for this Program.</p>
    pub fn get_channel_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.channel_name
    }
    /// <p>The name of the Program.</p>
    /// This field is required.
    pub fn program_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.program_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the Program.</p>
    pub fn set_program_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.program_name = input;
        self
    }
    /// <p>The name of the Program.</p>
    pub fn get_program_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.program_name
    }
    /// <p>The schedule configuration settings.</p>
    /// This field is required.
    pub fn schedule_configuration(mut self, input: crate::types::UpdateProgramScheduleConfiguration) -> Self {
        self.schedule_configuration = ::std::option::Option::Some(input);
        self
    }
    /// <p>The schedule configuration settings.</p>
    pub fn set_schedule_configuration(mut self, input: ::std::option::Option<crate::types::UpdateProgramScheduleConfiguration>) -> Self {
        self.schedule_configuration = input;
        self
    }
    /// <p>The schedule configuration settings.</p>
    pub fn get_schedule_configuration(&self) -> &::std::option::Option<crate::types::UpdateProgramScheduleConfiguration> {
        &self.schedule_configuration
    }
    /// Consumes the builder and constructs a [`UpdateProgramInput`](crate::operation::update_program::UpdateProgramInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::update_program::UpdateProgramInput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::operation::update_program::UpdateProgramInput {
            ad_breaks: self.ad_breaks,
            channel_name: self.channel_name,
            program_name: self.program_name,
            schedule_configuration: self.schedule_configuration,
        })
    }
}
