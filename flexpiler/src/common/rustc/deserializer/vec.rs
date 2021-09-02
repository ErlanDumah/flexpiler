use crate::common::rustc::block as block;
use crate::error;
use crate::error::Trait;
use crate::parser;
use crate::reader;


pub struct Vec;


impl<DataType> crate::identity::Trait for std::vec::Vec<DataType>
    where DataType: crate::identity::Trait {
    fn definition() -> String {
        String::from(format!("Vec<{}>", DataType::definition()))
    }
}


impl<DataType> crate::deserializer::Trait<
    std::vec::Vec<DataType>,
    crate::common::rustc::deserializer::Context,
    crate::common::rustc::error::Source
> for Vec
where DataType: crate::Deserialization<crate::common::rustc::Format>
                + crate::identity::Trait
{
    fn deserialize<ReaderType>(reader_mut_ref: &mut ReaderType)
        -> crate::deserializer::Result<std::vec::Vec<DataType>, crate::common::rustc::deserializer::Context, error::Error<crate::common::rustc::error::Source>>
    where ReaderType: reader::Trait {
        use crate::deserializer::context::Trait as DeserializerContextTrait;
        use crate::error::Trait as ErrorTrait;
        use crate::error::propagation::Trait as PropagationTrait;
        use crate::parser::Parse;

        // Vec::new()
        // vec![ #(element,)* ]
        // [#(element,)*]
        match block::ContextDenominator::parse(reader_mut_ref) {
            Err(parser_error) => {
                let error = error::Error::gen(parser_error)
                    .propagate(<Vec as crate::deserializer::context::Trait<std::vec::Vec<DataType>, crate::common::rustc::Format>>::context_general());
                return crate::deserializer::Result::Err(error);
            }
            Ok(result) => {
                let context: crate::common::rustc::deserializer::Context = result.finish.into();
                match &context {
                    crate::common::rustc::deserializer::Context::ListStart => {},
                    _ => {
                        let unexpected_context = crate::common::rustc::error::UnexpectedContext {
                            context_found: context,
                            context_expected: crate::error::ExpectedEntries::from(vec![
                                crate::common::rustc::deserializer::Context::ListStart
                            ]),
                        };
                        let error = error::Error::gen(unexpected_context)
                            .propagate(<Vec as crate::deserializer::context::Trait<std::vec::Vec<DataType>, crate::common::rustc::Format>>::context_general());
                        return crate::deserializer::Result::Err(error);
                    }
                }
            }
        }

        let mut data_vec = std::vec::Vec::new();
        let mut context = crate::common::rustc::deserializer::Context::Freestanding;
        loop {
            let data_result = DataType::Deserializer::deserialize(reader_mut_ref);
            context = match data_result {
                crate::deserializer::Result::DataFound{ data, context } => {
                    data_vec.push(data);
                    context
                },
                crate::deserializer::Result::NoDataFound { context } => {
                    context
                },
                crate::deserializer::Result::Err(error) => {
                    let error = error.propagate(<Vec as crate::deserializer::context::Trait<std::vec::Vec<DataType>, crate::common::rustc::Format>>::context_general());
                    return crate::deserializer::Result::Err(error);
                },
            };

            if context == crate::common::rustc::deserializer::Context::Freestanding {
                let context_denominator_result = match block::ContextDenominator::parse(reader_mut_ref) {
                    Ok(result) => result,
                    Err(parser_error) => {
                        let error = crate::Error::gen(parser_error)
                            .propagate(<Vec as crate::deserializer::context::Trait<std::vec::Vec<DataType>, crate::common::rustc::Format>>::context_general());
                        return crate::deserializer::Result::Err(error);
                    }
                };
                context = context_denominator_result.finish.into();
            }

            match context {
                crate::common::rustc::deserializer::Context::ListEnd => {
                    break;
                },
                crate::common::rustc::deserializer::Context::Separator => {
                    continue;
                },
                _ => {
                    let unexpected_context = crate::common::rustc::error::UnexpectedContext {
                        context_found: context,
                        context_expected: crate::error::ExpectedEntries::from(vec![
                            crate::common::rustc::deserializer::Context::ListEnd,
                            crate::common::rustc::deserializer::Context::Separator,
                        ]),
                    };

                    let error = crate::Error::gen(unexpected_context)
                        .propagate(<Vec as crate::deserializer::context::Trait<std::vec::Vec<DataType>, crate::common::rustc::Format>>::context_general());
                    return crate::deserializer::Result::Err(error);
                },
            }
        }

        return crate::deserializer::Result::DataFound{
            data: data_vec,
            context,
        };
    }
}


impl<DataType> crate::Deserialization<crate::common::rustc::Format> for std::vec::Vec<DataType>
where DataType: crate::Deserialization<crate::common::rustc::Format>
                + crate::identity::Trait
{
    type Deserializer = Vec;
}
