// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_pipeline_declaration<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> Result<Option<crate::types::PipelineDeclaration>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::PipelineDeclarationBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "name" => {
                            builder = builder.set_name(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "roleArn" => {
                            builder = builder.set_role_arn(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "artifactStore" => {
                            builder = builder.set_artifact_store(crate::protocol_serde::shape_artifact_store::de_artifact_store(tokens)?);
                        }
                        "artifactStores" => {
                            builder = builder.set_artifact_stores(crate::protocol_serde::shape_artifact_store_map::de_artifact_store_map(tokens)?);
                        }
                        "stages" => {
                            builder = builder.set_stages(
                                crate::protocol_serde::shape_pipeline_stage_declaration_list::de_pipeline_stage_declaration_list(tokens)?,
                            );
                        }
                        "version" => {
                            builder = builder.set_version(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i32::try_from)
                                    .transpose()?,
                            );
                        }
                        "pipelineType" => {
                            builder = builder.set_pipeline_type(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::PipelineType::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "triggers" => {
                            builder = builder.set_triggers(
                                crate::protocol_serde::shape_pipeline_trigger_declaration_list::de_pipeline_trigger_declaration_list(tokens)?,
                            );
                        }
                        "variables" => {
                            builder = builder.set_variables(
                                crate::protocol_serde::shape_pipeline_variable_declaration_list::de_pipeline_variable_declaration_list(tokens)?,
                            );
                        }
                        _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
                    },
                    other => {
                        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
                            "expected object key or end object, found: {:?}",
                            other
                        )))
                    }
                }
            }
            Ok(Some(crate::serde_util::pipeline_declaration_correct_errors(builder).build().map_err(
                |err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err),
            )?))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}

pub fn ser_pipeline_declaration(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::PipelineDeclaration,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object.key("name").string(input.name.as_str());
    }
    {
        object.key("roleArn").string(input.role_arn.as_str());
    }
    if let Some(var_1) = &input.artifact_store {
        #[allow(unused_mut)]
        let mut object_2 = object.key("artifactStore").start_object();
        crate::protocol_serde::shape_artifact_store::ser_artifact_store(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.artifact_stores {
        #[allow(unused_mut)]
        let mut object_4 = object.key("artifactStores").start_object();
        for (key_5, value_6) in var_3 {
            {
                #[allow(unused_mut)]
                let mut object_7 = object_4.key(key_5.as_str()).start_object();
                crate::protocol_serde::shape_artifact_store::ser_artifact_store(&mut object_7, value_6)?;
                object_7.finish();
            }
        }
        object_4.finish();
    }
    {
        let mut array_8 = object.key("stages").start_array();
        for item_9 in &input.stages {
            {
                #[allow(unused_mut)]
                let mut object_10 = array_8.value().start_object();
                crate::protocol_serde::shape_stage_declaration::ser_stage_declaration(&mut object_10, item_9)?;
                object_10.finish();
            }
        }
        array_8.finish();
    }
    if let Some(var_11) = &input.version {
        object.key("version").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_11).into()),
        );
    }
    if let Some(var_12) = &input.pipeline_type {
        object.key("pipelineType").string(var_12.as_str());
    }
    if let Some(var_13) = &input.triggers {
        let mut array_14 = object.key("triggers").start_array();
        for item_15 in var_13 {
            {
                #[allow(unused_mut)]
                let mut object_16 = array_14.value().start_object();
                crate::protocol_serde::shape_pipeline_trigger_declaration::ser_pipeline_trigger_declaration(&mut object_16, item_15)?;
                object_16.finish();
            }
        }
        array_14.finish();
    }
    if let Some(var_17) = &input.variables {
        let mut array_18 = object.key("variables").start_array();
        for item_19 in var_17 {
            {
                #[allow(unused_mut)]
                let mut object_20 = array_18.value().start_object();
                crate::protocol_serde::shape_pipeline_variable_declaration::ser_pipeline_variable_declaration(&mut object_20, item_19)?;
                object_20.finish();
            }
        }
        array_18.finish();
    }
    Ok(())
}
