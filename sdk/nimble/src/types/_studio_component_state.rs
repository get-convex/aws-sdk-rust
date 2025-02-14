// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// When writing a match expression against `StudioComponentState`, it is important to ensure
/// your code is forward-compatible. That is, if a match arm handles a case for a
/// feature that is supported by the service but has not been represented as an enum
/// variant in a current version of SDK, your code should continue to work when you
/// upgrade SDK to a future version in which the enum does include a variant for that
/// feature.
///
/// Here is an example of how you can make a match expression forward-compatible:
///
/// ```text
/// # let studiocomponentstate = unimplemented!();
/// match studiocomponentstate {
///     StudioComponentState::CreateFailed => { /* ... */ },
///     StudioComponentState::CreateInProgress => { /* ... */ },
///     StudioComponentState::Deleted => { /* ... */ },
///     StudioComponentState::DeleteFailed => { /* ... */ },
///     StudioComponentState::DeleteInProgress => { /* ... */ },
///     StudioComponentState::Ready => { /* ... */ },
///     StudioComponentState::UpdateFailed => { /* ... */ },
///     StudioComponentState::UpdateInProgress => { /* ... */ },
///     other @ _ if other.as_str() == "NewFeature" => { /* handles a case for `NewFeature` */ },
///     _ => { /* ... */ },
/// }
/// ```
/// The above code demonstrates that when `studiocomponentstate` represents
/// `NewFeature`, the execution path will lead to the second last match arm,
/// even though the enum does not contain a variant `StudioComponentState::NewFeature`
/// in the current version of SDK. The reason is that the variable `other`,
/// created by the `@` operator, is bound to
/// `StudioComponentState::Unknown(UnknownVariantValue("NewFeature".to_owned()))`
/// and calling `as_str` on it yields `"NewFeature"`.
/// This match expression is forward-compatible when executed with a newer
/// version of SDK where the variant `StudioComponentState::NewFeature` is defined.
/// Specifically, when `studiocomponentstate` represents `NewFeature`,
/// the execution path will hit the second last match arm as before by virtue of
/// calling `as_str` on `StudioComponentState::NewFeature` also yielding `"NewFeature"`.
///
/// Explicitly matching on the `Unknown` variant should
/// be avoided for two reasons:
/// - The inner data `UnknownVariantValue` is opaque, and no further information can be extracted.
/// - It might inadvertently shadow other intended match arms.
/// <p>The current state of the studio component resource.</p>
/// <p>While a studio component is being created, modified, or deleted, its state will be
/// <code>CREATE_IN_PROGRESS</code>, <code>UPDATE_IN_PROGRESS</code>, or
/// <code>DELETE_IN_PROGRESS</code>.</p>
/// <p>These are called <i>transition states</i>.</p>
/// <p>No modifications may be made to the studio component while it is in a transition
/// state.</p>
/// <p>If creation of the resource fails, the state will change to
/// <code>CREATE_FAILED</code>. The resource <code>StatusCode</code> and
/// <code>StatusMessage</code> will provide more information of why creation failed. The
/// resource in this state will automatically be deleted from your account after a period of
/// time.</p>
/// <p>If updating the resource fails, the state will change to <code>UPDATE_FAILED</code>.
/// The resource <code>StatusCode</code> and <code>StatusMessage</code> will provide more
/// information of why the update failed. The resource will be returned to the state it was
/// in when the update request was invoked.</p>
/// <p>If deleting the resource fails, the state will change to <code>DELETE_FAILED</code>.
/// The resource <code>StatusCode</code> and <code>StatusMessage</code> will provide more
/// information of why the update failed. The resource will be returned to the state it was
/// in when the update request was invoked. After the resource is deleted successfully, it
/// will change to the <code>DELETED</code> state. The resource will no longer count against
/// service quotas and cannot be used or acted upon any futher. It will be removed from your
/// account after a period of time.</p>
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::Eq, ::std::cmp::Ord, ::std::cmp::PartialEq, ::std::cmp::PartialOrd, ::std::fmt::Debug, ::std::hash::Hash,
)]
pub enum StudioComponentState {
    #[allow(missing_docs)] // documentation missing in model
    CreateFailed,
    #[allow(missing_docs)] // documentation missing in model
    CreateInProgress,
    #[allow(missing_docs)] // documentation missing in model
    Deleted,
    #[allow(missing_docs)] // documentation missing in model
    DeleteFailed,
    #[allow(missing_docs)] // documentation missing in model
    DeleteInProgress,
    #[allow(missing_docs)] // documentation missing in model
    Ready,
    #[allow(missing_docs)] // documentation missing in model
    UpdateFailed,
    #[allow(missing_docs)] // documentation missing in model
    UpdateInProgress,
    /// `Unknown` contains new variants that have been added since this code was generated.
    #[deprecated(note = "Don't directly match on `Unknown`. See the docs on this enum for the correct way to handle unknown variants.")]
    Unknown(crate::primitives::sealed_enum_unknown::UnknownVariantValue),
}
impl ::std::convert::From<&str> for StudioComponentState {
    fn from(s: &str) -> Self {
        match s {
            "CREATE_FAILED" => StudioComponentState::CreateFailed,
            "CREATE_IN_PROGRESS" => StudioComponentState::CreateInProgress,
            "DELETED" => StudioComponentState::Deleted,
            "DELETE_FAILED" => StudioComponentState::DeleteFailed,
            "DELETE_IN_PROGRESS" => StudioComponentState::DeleteInProgress,
            "READY" => StudioComponentState::Ready,
            "UPDATE_FAILED" => StudioComponentState::UpdateFailed,
            "UPDATE_IN_PROGRESS" => StudioComponentState::UpdateInProgress,
            other => StudioComponentState::Unknown(crate::primitives::sealed_enum_unknown::UnknownVariantValue(other.to_owned())),
        }
    }
}
impl ::std::str::FromStr for StudioComponentState {
    type Err = ::std::convert::Infallible;

    fn from_str(s: &str) -> ::std::result::Result<Self, <Self as ::std::str::FromStr>::Err> {
        ::std::result::Result::Ok(StudioComponentState::from(s))
    }
}
impl StudioComponentState {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            StudioComponentState::CreateFailed => "CREATE_FAILED",
            StudioComponentState::CreateInProgress => "CREATE_IN_PROGRESS",
            StudioComponentState::Deleted => "DELETED",
            StudioComponentState::DeleteFailed => "DELETE_FAILED",
            StudioComponentState::DeleteInProgress => "DELETE_IN_PROGRESS",
            StudioComponentState::Ready => "READY",
            StudioComponentState::UpdateFailed => "UPDATE_FAILED",
            StudioComponentState::UpdateInProgress => "UPDATE_IN_PROGRESS",
            StudioComponentState::Unknown(value) => value.as_str(),
        }
    }
    /// Returns all the `&str` representations of the enum members.
    pub const fn values() -> &'static [&'static str] {
        &[
            "CREATE_FAILED",
            "CREATE_IN_PROGRESS",
            "DELETED",
            "DELETE_FAILED",
            "DELETE_IN_PROGRESS",
            "READY",
            "UPDATE_FAILED",
            "UPDATE_IN_PROGRESS",
        ]
    }
}
impl ::std::convert::AsRef<str> for StudioComponentState {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl StudioComponentState {
    /// Parses the enum value while disallowing unknown variants.
    ///
    /// Unknown variants will result in an error.
    pub fn try_parse(value: &str) -> ::std::result::Result<Self, crate::error::UnknownVariantError> {
        match Self::from(value) {
            #[allow(deprecated)]
            Self::Unknown(_) => ::std::result::Result::Err(crate::error::UnknownVariantError::new(value)),
            known => Ok(known),
        }
    }
}
