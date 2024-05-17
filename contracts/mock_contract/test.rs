#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
mod custom_resp {
    #![allow(unused)]
    use cosmwasm_schema::{cw_serde, QueryResponses};
    use cosmwasm_std::{
        to_json_binary, Binary, CustomMsg, CustomQuery, Deps, DepsMut, Env, MessageInfo,
        Response, StdError, StdResult, Uint128,
    };
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[serde(deny_unknown_fields, crate = "::cosmwasm_schema::serde")]
    #[schemars(crate = "::cosmwasm_schema::schemars")]
    pub struct A;
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        use ::cosmwasm_schema::serde as _serde;
        #[automatically_derived]
        impl ::cosmwasm_schema::serde::Serialize for A {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> ::cosmwasm_schema::serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: ::cosmwasm_schema::serde::Serializer,
            {
                _serde::Serializer::serialize_unit_struct(__serializer, "A")
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        use ::cosmwasm_schema::serde as _serde;
        #[automatically_derived]
        impl<'de> ::cosmwasm_schema::serde::Deserialize<'de> for A {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> ::cosmwasm_schema::serde::__private::Result<Self, __D::Error>
            where
                __D: ::cosmwasm_schema::serde::Deserializer<'de>,
            {
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<A>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = A;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "unit struct A",
                        )
                    }
                    #[inline]
                    fn visit_unit<__E>(
                        self,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(A)
                    }
                }
                _serde::Deserializer::deserialize_unit_struct(
                    __deserializer,
                    "A",
                    __Visitor {
                        marker: _serde::__private::PhantomData::<A>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::clone::Clone for A {
        #[inline]
        fn clone(&self) -> A {
            A
        }
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::fmt::Debug for A {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "A")
        }
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::marker::StructuralPartialEq for A {}
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::cmp::PartialEq for A {
        #[inline]
        fn eq(&self, other: &A) -> bool {
            true
        }
    }
    const _: () = {
        use ::cosmwasm_schema::schemars as schemars;
        #[automatically_derived]
        #[allow(unused_braces)]
        impl schemars::JsonSchema for A {
            fn schema_name() -> std::string::String {
                "A".to_owned()
            }
            fn schema_id() -> std::borrow::Cow<'static, str> {
                std::borrow::Cow::Borrowed("mock_contract::custom_resp::A")
            }
            fn json_schema(
                gen: &mut schemars::gen::SchemaGenerator,
            ) -> schemars::schema::Schema {
                gen.subschema_for::<()>()
            }
        }
    };
    impl CustomMsg for A {}
    impl CustomQuery for A {}
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[serde(deny_unknown_fields, crate = "::cosmwasm_schema::serde")]
    #[schemars(crate = "::cosmwasm_schema::schemars")]
    pub struct InstantiateMsg {}
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        use ::cosmwasm_schema::serde as _serde;
        #[automatically_derived]
        impl ::cosmwasm_schema::serde::Serialize for InstantiateMsg {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> ::cosmwasm_schema::serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: ::cosmwasm_schema::serde::Serializer,
            {
                let __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "InstantiateMsg",
                    false as usize,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        use ::cosmwasm_schema::serde as _serde;
        #[automatically_derived]
        impl<'de> ::cosmwasm_schema::serde::Deserialize<'de> for InstantiateMsg {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> ::cosmwasm_schema::serde::__private::Result<Self, __D::Error>
            where
                __D: ::cosmwasm_schema::serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {}
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::invalid_value(
                                        _serde::de::Unexpected::Unsigned(__value),
                                        &"field index 0 <= i < 0",
                                    ),
                                )
                            }
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::unknown_field(__value, FIELDS),
                                )
                            }
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            _ => {
                                let __value = &_serde::__private::from_utf8_lossy(__value);
                                _serde::__private::Err(
                                    _serde::de::Error::unknown_field(__value, FIELDS),
                                )
                            }
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<InstantiateMsg>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = InstantiateMsg;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct InstantiateMsg",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        _: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        _serde::__private::Ok(InstantiateMsg {})
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        _serde::__private::Option::map(
                            _serde::de::MapAccess::next_key::<__Field>(&mut __map)?,
                            |__impossible| match __impossible {},
                        );
                        _serde::__private::Ok(InstantiateMsg {})
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &[];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "InstantiateMsg",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<InstantiateMsg>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::clone::Clone for InstantiateMsg {
        #[inline]
        fn clone(&self) -> InstantiateMsg {
            InstantiateMsg {}
        }
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::fmt::Debug for InstantiateMsg {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "InstantiateMsg")
        }
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::marker::StructuralPartialEq for InstantiateMsg {}
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::cmp::PartialEq for InstantiateMsg {
        #[inline]
        fn eq(&self, other: &InstantiateMsg) -> bool {
            true
        }
    }
    const _: () = {
        use ::cosmwasm_schema::schemars as schemars;
        #[automatically_derived]
        #[allow(unused_braces)]
        impl schemars::JsonSchema for InstantiateMsg {
            fn schema_name() -> std::string::String {
                "InstantiateMsg".to_owned()
            }
            fn schema_id() -> std::borrow::Cow<'static, str> {
                std::borrow::Cow::Borrowed("mock_contract::custom_resp::InstantiateMsg")
            }
            fn json_schema(
                gen: &mut schemars::gen::SchemaGenerator,
            ) -> schemars::schema::Schema {
                {
                    let mut schema_object = schemars::schema::SchemaObject {
                        instance_type: Some(
                            schemars::schema::InstanceType::Object.into(),
                        ),
                        ..Default::default()
                    };
                    let object_validation = schema_object.object();
                    object_validation
                        .additional_properties = Some(Box::new(false.into()));
                    schemars::schema::Schema::Object(schema_object)
                }
            }
        }
    };
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[serde(
        deny_unknown_fields,
        rename_all = "snake_case",
        crate = "::cosmwasm_schema::serde"
    )]
    #[schemars(crate = "::cosmwasm_schema::schemars")]
    pub enum ExecuteMsg {
        FirstMessage {},
    }
    #[cfg(not(target_arch = "wasm32"))]
    /// Automatically derived trait that allows you to call the variants of the message directly without the need to construct the struct yourself.
    pub trait ExecuteMsgFns<
        Chain: ::cw_orch::core::environment::TxHandler,
        CwOrchExecuteMsgType,
    >: ::cw_orch::core::contract::interface_traits::CwOrchExecute<
            Chain,
            ExecuteMsg = CwOrchExecuteMsgType,
        >
    where
        ExecuteMsg: Into<CwOrchExecuteMsgType>,
    {
        ///Automatically generated wrapper around ExecuteMsg::FirstMessage variant
        #[allow(clippy::too_many_arguments)]
        fn first_message(
            &self,
        ) -> Result<
            ::cw_orch::core::environment::TxResponse<Chain>,
            ::cw_orch::core::CwEnvError,
        > {
            let msg = ExecuteMsg::FirstMessage {};
            <Self as ::cw_orch::core::contract::interface_traits::CwOrchExecute<
                Chain,
            >>::execute(self, &msg.into(), None)
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[automatically_derived]
    impl<
        SupportedContract,
        Chain: ::cw_orch::core::environment::TxHandler,
        CwOrchExecuteMsgType,
    > ExecuteMsgFns<Chain, CwOrchExecuteMsgType> for SupportedContract
    where
        SupportedContract: ::cw_orch::core::contract::interface_traits::CwOrchExecute<
            Chain,
            ExecuteMsg = CwOrchExecuteMsgType,
        >,
        ExecuteMsg: Into<CwOrchExecuteMsgType>,
    {}
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        use ::cosmwasm_schema::serde as _serde;
        #[automatically_derived]
        impl ::cosmwasm_schema::serde::Serialize for ExecuteMsg {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> ::cosmwasm_schema::serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: ::cosmwasm_schema::serde::Serializer,
            {
                match *self {
                    ExecuteMsg::FirstMessage {} => {
                        let __serde_state = _serde::Serializer::serialize_struct_variant(
                            __serializer,
                            "ExecuteMsg",
                            0u32,
                            "first_message",
                            0,
                        )?;
                        _serde::ser::SerializeStructVariant::end(__serde_state)
                    }
                }
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        use ::cosmwasm_schema::serde as _serde;
        #[automatically_derived]
        impl<'de> ::cosmwasm_schema::serde::Deserialize<'de> for ExecuteMsg {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> ::cosmwasm_schema::serde::__private::Result<Self, __D::Error>
            where
                __D: ::cosmwasm_schema::serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "variant identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::invalid_value(
                                        _serde::de::Unexpected::Unsigned(__value),
                                        &"variant index 0 <= i < 1",
                                    ),
                                )
                            }
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "first_message" => _serde::__private::Ok(__Field::__field0),
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::unknown_variant(__value, VARIANTS),
                                )
                            }
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"first_message" => _serde::__private::Ok(__Field::__field0),
                            _ => {
                                let __value = &_serde::__private::from_utf8_lossy(__value);
                                _serde::__private::Err(
                                    _serde::de::Error::unknown_variant(__value, VARIANTS),
                                )
                            }
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<ExecuteMsg>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = ExecuteMsg;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "enum ExecuteMsg",
                        )
                    }
                    fn visit_enum<__A>(
                        self,
                        __data: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::EnumAccess<'de>,
                    {
                        match _serde::de::EnumAccess::variant(__data)? {
                            (__Field::__field0, __variant) => {
                                #[allow(non_camel_case_types)]
                                #[doc(hidden)]
                                enum __Field {}
                                #[doc(hidden)]
                                struct __FieldVisitor;
                                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                                    type Value = __Field;
                                    fn expecting(
                                        &self,
                                        __formatter: &mut _serde::__private::Formatter,
                                    ) -> _serde::__private::fmt::Result {
                                        _serde::__private::Formatter::write_str(
                                            __formatter,
                                            "field identifier",
                                        )
                                    }
                                    fn visit_u64<__E>(
                                        self,
                                        __value: u64,
                                    ) -> _serde::__private::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            _ => {
                                                _serde::__private::Err(
                                                    _serde::de::Error::invalid_value(
                                                        _serde::de::Unexpected::Unsigned(__value),
                                                        &"field index 0 <= i < 0",
                                                    ),
                                                )
                                            }
                                        }
                                    }
                                    fn visit_str<__E>(
                                        self,
                                        __value: &str,
                                    ) -> _serde::__private::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            _ => {
                                                _serde::__private::Err(
                                                    _serde::de::Error::unknown_field(__value, FIELDS),
                                                )
                                            }
                                        }
                                    }
                                    fn visit_bytes<__E>(
                                        self,
                                        __value: &[u8],
                                    ) -> _serde::__private::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            _ => {
                                                let __value = &_serde::__private::from_utf8_lossy(__value);
                                                _serde::__private::Err(
                                                    _serde::de::Error::unknown_field(__value, FIELDS),
                                                )
                                            }
                                        }
                                    }
                                }
                                impl<'de> _serde::Deserialize<'de> for __Field {
                                    #[inline]
                                    fn deserialize<__D>(
                                        __deserializer: __D,
                                    ) -> _serde::__private::Result<Self, __D::Error>
                                    where
                                        __D: _serde::Deserializer<'de>,
                                    {
                                        _serde::Deserializer::deserialize_identifier(
                                            __deserializer,
                                            __FieldVisitor,
                                        )
                                    }
                                }
                                #[doc(hidden)]
                                struct __Visitor<'de> {
                                    marker: _serde::__private::PhantomData<ExecuteMsg>,
                                    lifetime: _serde::__private::PhantomData<&'de ()>,
                                }
                                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                    type Value = ExecuteMsg;
                                    fn expecting(
                                        &self,
                                        __formatter: &mut _serde::__private::Formatter,
                                    ) -> _serde::__private::fmt::Result {
                                        _serde::__private::Formatter::write_str(
                                            __formatter,
                                            "struct variant ExecuteMsg::FirstMessage",
                                        )
                                    }
                                    #[inline]
                                    fn visit_seq<__A>(
                                        self,
                                        _: __A,
                                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                                    where
                                        __A: _serde::de::SeqAccess<'de>,
                                    {
                                        _serde::__private::Ok(ExecuteMsg::FirstMessage {})
                                    }
                                    #[inline]
                                    fn visit_map<__A>(
                                        self,
                                        mut __map: __A,
                                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                                    where
                                        __A: _serde::de::MapAccess<'de>,
                                    {
                                        _serde::__private::Option::map(
                                            _serde::de::MapAccess::next_key::<__Field>(&mut __map)?,
                                            |__impossible| match __impossible {},
                                        );
                                        _serde::__private::Ok(ExecuteMsg::FirstMessage {})
                                    }
                                }
                                #[doc(hidden)]
                                const FIELDS: &'static [&'static str] = &[];
                                _serde::de::VariantAccess::struct_variant(
                                    __variant,
                                    FIELDS,
                                    __Visitor {
                                        marker: _serde::__private::PhantomData::<ExecuteMsg>,
                                        lifetime: _serde::__private::PhantomData,
                                    },
                                )
                            }
                        }
                    }
                }
                #[doc(hidden)]
                const VARIANTS: &'static [&'static str] = &["first_message"];
                _serde::Deserializer::deserialize_enum(
                    __deserializer,
                    "ExecuteMsg",
                    VARIANTS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<ExecuteMsg>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::clone::Clone for ExecuteMsg {
        #[inline]
        fn clone(&self) -> ExecuteMsg {
            ExecuteMsg::FirstMessage {}
        }
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::fmt::Debug for ExecuteMsg {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "FirstMessage")
        }
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::marker::StructuralPartialEq for ExecuteMsg {}
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::cmp::PartialEq for ExecuteMsg {
        #[inline]
        fn eq(&self, other: &ExecuteMsg) -> bool {
            true
        }
    }
    const _: () = {
        use ::cosmwasm_schema::schemars as schemars;
        #[automatically_derived]
        #[allow(unused_braces)]
        impl schemars::JsonSchema for ExecuteMsg {
            fn schema_name() -> std::string::String {
                "ExecuteMsg".to_owned()
            }
            fn schema_id() -> std::borrow::Cow<'static, str> {
                std::borrow::Cow::Borrowed("mock_contract::custom_resp::ExecuteMsg")
            }
            fn json_schema(
                gen: &mut schemars::gen::SchemaGenerator,
            ) -> schemars::schema::Schema {
                schemars::schema::Schema::Object(schemars::schema::SchemaObject {
                    subschemas: Some(
                        Box::new(schemars::schema::SubschemaValidation {
                            one_of: Some(
                                <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new([
                                        schemars::_private::new_externally_tagged_enum(
                                            "first_message",
                                            {
                                                let mut schema_object = schemars::schema::SchemaObject {
                                                    instance_type: Some(
                                                        schemars::schema::InstanceType::Object.into(),
                                                    ),
                                                    ..Default::default()
                                                };
                                                let object_validation = schema_object.object();
                                                object_validation
                                                    .additional_properties = Some(Box::new(false.into()));
                                                schemars::schema::Schema::Object(schema_object)
                                            },
                                        ),
                                    ]),
                                ),
                            ),
                            ..Default::default()
                        }),
                    ),
                    ..Default::default()
                })
            }
        }
    };
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[serde(
        deny_unknown_fields,
        rename_all = "snake_case",
        crate = "::cosmwasm_schema::serde"
    )]
    #[schemars(crate = "::cosmwasm_schema::schemars")]
    pub enum QueryMsg {
        #[returns(String)]
        FirstQuery {},
        #[returns(String)]
        SecondQuery { t: String },
    }
    #[cfg(not(target_arch = "wasm32"))]
    /// Automatically derived trait that allows you to call the variants of the message directly without the need to construct the struct yourself.
    pub trait QueryMsgFns<
        Chain: ::cw_orch::core::environment::QueryHandler
            + ::cw_orch::core::environment::ChainState,
        CwOrchQueryMsgType,
    >: ::cw_orch::core::contract::interface_traits::CwOrchQuery<
            Chain,
            QueryMsg = CwOrchQueryMsgType,
        >
    where
        QueryMsg: Into<CwOrchQueryMsgType>,
    {
        ///Automatically generated wrapper around QueryMsg::FirstQuery variant
        #[allow(clippy::too_many_arguments)]
        fn first_query(&self) -> Result<String, ::cw_orch::core::CwEnvError> {
            let msg = QueryMsg::FirstQuery {};
            <Self as ::cw_orch::core::contract::interface_traits::CwOrchQuery<
                Chain,
            >>::query(self, &msg.into())
        }
        ///Automatically generated wrapper around QueryMsg::SecondQuery variant
        #[allow(clippy::too_many_arguments)]
        fn second_query(
            &self,
            t: impl Into<String>,
        ) -> Result<String, ::cw_orch::core::CwEnvError> {
            let msg = QueryMsg::SecondQuery {
                t: t.into(),
            };
            <Self as ::cw_orch::core::contract::interface_traits::CwOrchQuery<
                Chain,
            >>::query(self, &msg.into())
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[automatically_derived]
    impl<
        SupportedContract,
        Chain: ::cw_orch::core::environment::QueryHandler
            + ::cw_orch::core::environment::ChainState,
        CwOrchQueryMsgType,
    > QueryMsgFns<Chain, CwOrchQueryMsgType> for SupportedContract
    where
        SupportedContract: ::cw_orch::core::contract::interface_traits::CwOrchQuery<
            Chain,
            QueryMsg = CwOrchQueryMsgType,
        >,
        QueryMsg: Into<CwOrchQueryMsgType>,
    {}
    #[automatically_derived]
    #[cfg(not(target_arch = "wasm32"))]
    impl ::cosmwasm_schema::QueryResponses for QueryMsg {
        fn response_schemas_impl() -> ::std::collections::BTreeMap<
            String,
            ::cosmwasm_schema::schemars::schema::RootSchema,
        > {
            ::std::collections::BTreeMap::from([
                (
                    "first_query".to_string(),
                    ::cosmwasm_schema::schemars::gen::SchemaGenerator::new(
                            ::cosmwasm_schema::schemars::gen::SchemaSettings::draft07(),
                        )
                        .into_root_schema_for::<String>(),
                ),
                (
                    "second_query".to_string(),
                    ::cosmwasm_schema::schemars::gen::SchemaGenerator::new(
                            ::cosmwasm_schema::schemars::gen::SchemaSettings::draft07(),
                        )
                        .into_root_schema_for::<String>(),
                ),
            ])
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        use ::cosmwasm_schema::serde as _serde;
        #[automatically_derived]
        impl ::cosmwasm_schema::serde::Serialize for QueryMsg {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> ::cosmwasm_schema::serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: ::cosmwasm_schema::serde::Serializer,
            {
                match *self {
                    QueryMsg::FirstQuery {} => {
                        let __serde_state = _serde::Serializer::serialize_struct_variant(
                            __serializer,
                            "QueryMsg",
                            0u32,
                            "first_query",
                            0,
                        )?;
                        _serde::ser::SerializeStructVariant::end(__serde_state)
                    }
                    QueryMsg::SecondQuery { ref t } => {
                        let mut __serde_state = _serde::Serializer::serialize_struct_variant(
                            __serializer,
                            "QueryMsg",
                            1u32,
                            "second_query",
                            0 + 1,
                        )?;
                        _serde::ser::SerializeStructVariant::serialize_field(
                            &mut __serde_state,
                            "t",
                            t,
                        )?;
                        _serde::ser::SerializeStructVariant::end(__serde_state)
                    }
                }
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        use ::cosmwasm_schema::serde as _serde;
        #[automatically_derived]
        impl<'de> ::cosmwasm_schema::serde::Deserialize<'de> for QueryMsg {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> ::cosmwasm_schema::serde::__private::Result<Self, __D::Error>
            where
                __D: ::cosmwasm_schema::serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "variant identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::invalid_value(
                                        _serde::de::Unexpected::Unsigned(__value),
                                        &"variant index 0 <= i < 2",
                                    ),
                                )
                            }
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "first_query" => _serde::__private::Ok(__Field::__field0),
                            "second_query" => _serde::__private::Ok(__Field::__field1),
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::unknown_variant(__value, VARIANTS),
                                )
                            }
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"first_query" => _serde::__private::Ok(__Field::__field0),
                            b"second_query" => _serde::__private::Ok(__Field::__field1),
                            _ => {
                                let __value = &_serde::__private::from_utf8_lossy(__value);
                                _serde::__private::Err(
                                    _serde::de::Error::unknown_variant(__value, VARIANTS),
                                )
                            }
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<QueryMsg>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = QueryMsg;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "enum QueryMsg",
                        )
                    }
                    fn visit_enum<__A>(
                        self,
                        __data: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::EnumAccess<'de>,
                    {
                        match _serde::de::EnumAccess::variant(__data)? {
                            (__Field::__field0, __variant) => {
                                #[allow(non_camel_case_types)]
                                #[doc(hidden)]
                                enum __Field {}
                                #[doc(hidden)]
                                struct __FieldVisitor;
                                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                                    type Value = __Field;
                                    fn expecting(
                                        &self,
                                        __formatter: &mut _serde::__private::Formatter,
                                    ) -> _serde::__private::fmt::Result {
                                        _serde::__private::Formatter::write_str(
                                            __formatter,
                                            "field identifier",
                                        )
                                    }
                                    fn visit_u64<__E>(
                                        self,
                                        __value: u64,
                                    ) -> _serde::__private::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            _ => {
                                                _serde::__private::Err(
                                                    _serde::de::Error::invalid_value(
                                                        _serde::de::Unexpected::Unsigned(__value),
                                                        &"field index 0 <= i < 0",
                                                    ),
                                                )
                                            }
                                        }
                                    }
                                    fn visit_str<__E>(
                                        self,
                                        __value: &str,
                                    ) -> _serde::__private::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            _ => {
                                                _serde::__private::Err(
                                                    _serde::de::Error::unknown_field(__value, FIELDS),
                                                )
                                            }
                                        }
                                    }
                                    fn visit_bytes<__E>(
                                        self,
                                        __value: &[u8],
                                    ) -> _serde::__private::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            _ => {
                                                let __value = &_serde::__private::from_utf8_lossy(__value);
                                                _serde::__private::Err(
                                                    _serde::de::Error::unknown_field(__value, FIELDS),
                                                )
                                            }
                                        }
                                    }
                                }
                                impl<'de> _serde::Deserialize<'de> for __Field {
                                    #[inline]
                                    fn deserialize<__D>(
                                        __deserializer: __D,
                                    ) -> _serde::__private::Result<Self, __D::Error>
                                    where
                                        __D: _serde::Deserializer<'de>,
                                    {
                                        _serde::Deserializer::deserialize_identifier(
                                            __deserializer,
                                            __FieldVisitor,
                                        )
                                    }
                                }
                                #[doc(hidden)]
                                struct __Visitor<'de> {
                                    marker: _serde::__private::PhantomData<QueryMsg>,
                                    lifetime: _serde::__private::PhantomData<&'de ()>,
                                }
                                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                    type Value = QueryMsg;
                                    fn expecting(
                                        &self,
                                        __formatter: &mut _serde::__private::Formatter,
                                    ) -> _serde::__private::fmt::Result {
                                        _serde::__private::Formatter::write_str(
                                            __formatter,
                                            "struct variant QueryMsg::FirstQuery",
                                        )
                                    }
                                    #[inline]
                                    fn visit_seq<__A>(
                                        self,
                                        _: __A,
                                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                                    where
                                        __A: _serde::de::SeqAccess<'de>,
                                    {
                                        _serde::__private::Ok(QueryMsg::FirstQuery {})
                                    }
                                    #[inline]
                                    fn visit_map<__A>(
                                        self,
                                        mut __map: __A,
                                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                                    where
                                        __A: _serde::de::MapAccess<'de>,
                                    {
                                        _serde::__private::Option::map(
                                            _serde::de::MapAccess::next_key::<__Field>(&mut __map)?,
                                            |__impossible| match __impossible {},
                                        );
                                        _serde::__private::Ok(QueryMsg::FirstQuery {})
                                    }
                                }
                                #[doc(hidden)]
                                const FIELDS: &'static [&'static str] = &[];
                                _serde::de::VariantAccess::struct_variant(
                                    __variant,
                                    FIELDS,
                                    __Visitor {
                                        marker: _serde::__private::PhantomData::<QueryMsg>,
                                        lifetime: _serde::__private::PhantomData,
                                    },
                                )
                            }
                            (__Field::__field1, __variant) => {
                                #[allow(non_camel_case_types)]
                                #[doc(hidden)]
                                enum __Field {
                                    __field0,
                                }
                                #[doc(hidden)]
                                struct __FieldVisitor;
                                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                                    type Value = __Field;
                                    fn expecting(
                                        &self,
                                        __formatter: &mut _serde::__private::Formatter,
                                    ) -> _serde::__private::fmt::Result {
                                        _serde::__private::Formatter::write_str(
                                            __formatter,
                                            "field identifier",
                                        )
                                    }
                                    fn visit_u64<__E>(
                                        self,
                                        __value: u64,
                                    ) -> _serde::__private::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            0u64 => _serde::__private::Ok(__Field::__field0),
                                            _ => {
                                                _serde::__private::Err(
                                                    _serde::de::Error::invalid_value(
                                                        _serde::de::Unexpected::Unsigned(__value),
                                                        &"field index 0 <= i < 1",
                                                    ),
                                                )
                                            }
                                        }
                                    }
                                    fn visit_str<__E>(
                                        self,
                                        __value: &str,
                                    ) -> _serde::__private::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            "t" => _serde::__private::Ok(__Field::__field0),
                                            _ => {
                                                _serde::__private::Err(
                                                    _serde::de::Error::unknown_field(__value, FIELDS),
                                                )
                                            }
                                        }
                                    }
                                    fn visit_bytes<__E>(
                                        self,
                                        __value: &[u8],
                                    ) -> _serde::__private::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            b"t" => _serde::__private::Ok(__Field::__field0),
                                            _ => {
                                                let __value = &_serde::__private::from_utf8_lossy(__value);
                                                _serde::__private::Err(
                                                    _serde::de::Error::unknown_field(__value, FIELDS),
                                                )
                                            }
                                        }
                                    }
                                }
                                impl<'de> _serde::Deserialize<'de> for __Field {
                                    #[inline]
                                    fn deserialize<__D>(
                                        __deserializer: __D,
                                    ) -> _serde::__private::Result<Self, __D::Error>
                                    where
                                        __D: _serde::Deserializer<'de>,
                                    {
                                        _serde::Deserializer::deserialize_identifier(
                                            __deserializer,
                                            __FieldVisitor,
                                        )
                                    }
                                }
                                #[doc(hidden)]
                                struct __Visitor<'de> {
                                    marker: _serde::__private::PhantomData<QueryMsg>,
                                    lifetime: _serde::__private::PhantomData<&'de ()>,
                                }
                                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                    type Value = QueryMsg;
                                    fn expecting(
                                        &self,
                                        __formatter: &mut _serde::__private::Formatter,
                                    ) -> _serde::__private::fmt::Result {
                                        _serde::__private::Formatter::write_str(
                                            __formatter,
                                            "struct variant QueryMsg::SecondQuery",
                                        )
                                    }
                                    #[inline]
                                    fn visit_seq<__A>(
                                        self,
                                        mut __seq: __A,
                                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                                    where
                                        __A: _serde::de::SeqAccess<'de>,
                                    {
                                        let __field0 = match _serde::de::SeqAccess::next_element::<
                                            String,
                                        >(&mut __seq)? {
                                            _serde::__private::Some(__value) => __value,
                                            _serde::__private::None => {
                                                return _serde::__private::Err(
                                                    _serde::de::Error::invalid_length(
                                                        0usize,
                                                        &"struct variant QueryMsg::SecondQuery with 1 element",
                                                    ),
                                                );
                                            }
                                        };
                                        _serde::__private::Ok(QueryMsg::SecondQuery {
                                            t: __field0,
                                        })
                                    }
                                    #[inline]
                                    fn visit_map<__A>(
                                        self,
                                        mut __map: __A,
                                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                                    where
                                        __A: _serde::de::MapAccess<'de>,
                                    {
                                        let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                            __Field,
                                        >(&mut __map)? {
                                            match __key {
                                                __Field::__field0 => {
                                                    if _serde::__private::Option::is_some(&__field0) {
                                                        return _serde::__private::Err(
                                                            <__A::Error as _serde::de::Error>::duplicate_field("t"),
                                                        );
                                                    }
                                                    __field0 = _serde::__private::Some(
                                                        _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                                    );
                                                }
                                            }
                                        }
                                        let __field0 = match __field0 {
                                            _serde::__private::Some(__field0) => __field0,
                                            _serde::__private::None => {
                                                _serde::__private::de::missing_field("t")?
                                            }
                                        };
                                        _serde::__private::Ok(QueryMsg::SecondQuery {
                                            t: __field0,
                                        })
                                    }
                                }
                                #[doc(hidden)]
                                const FIELDS: &'static [&'static str] = &["t"];
                                _serde::de::VariantAccess::struct_variant(
                                    __variant,
                                    FIELDS,
                                    __Visitor {
                                        marker: _serde::__private::PhantomData::<QueryMsg>,
                                        lifetime: _serde::__private::PhantomData,
                                    },
                                )
                            }
                        }
                    }
                }
                #[doc(hidden)]
                const VARIANTS: &'static [&'static str] = &[
                    "first_query",
                    "second_query",
                ];
                _serde::Deserializer::deserialize_enum(
                    __deserializer,
                    "QueryMsg",
                    VARIANTS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<QueryMsg>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::clone::Clone for QueryMsg {
        #[inline]
        fn clone(&self) -> QueryMsg {
            match self {
                QueryMsg::FirstQuery {} => QueryMsg::FirstQuery {},
                QueryMsg::SecondQuery { t: __self_0 } => {
                    QueryMsg::SecondQuery {
                        t: ::core::clone::Clone::clone(__self_0),
                    }
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::fmt::Debug for QueryMsg {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                QueryMsg::FirstQuery {} => {
                    ::core::fmt::Formatter::write_str(f, "FirstQuery")
                }
                QueryMsg::SecondQuery { t: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "SecondQuery",
                        "t",
                        &__self_0,
                    )
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::marker::StructuralPartialEq for QueryMsg {}
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::cmp::PartialEq for QueryMsg {
        #[inline]
        fn eq(&self, other: &QueryMsg) -> bool {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            __self_tag == __arg1_tag
                && match (self, other) {
                    (
                        QueryMsg::SecondQuery { t: __self_0 },
                        QueryMsg::SecondQuery { t: __arg1_0 },
                    ) => *__self_0 == *__arg1_0,
                    _ => true,
                }
        }
    }
    const _: () = {
        use ::cosmwasm_schema::schemars as schemars;
        #[automatically_derived]
        #[allow(unused_braces)]
        impl schemars::JsonSchema for QueryMsg {
            fn schema_name() -> std::string::String {
                "QueryMsg".to_owned()
            }
            fn schema_id() -> std::borrow::Cow<'static, str> {
                std::borrow::Cow::Borrowed("mock_contract::custom_resp::QueryMsg")
            }
            fn json_schema(
                gen: &mut schemars::gen::SchemaGenerator,
            ) -> schemars::schema::Schema {
                schemars::schema::Schema::Object(schemars::schema::SchemaObject {
                    subschemas: Some(
                        Box::new(schemars::schema::SubschemaValidation {
                            one_of: Some(
                                <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new([
                                        schemars::_private::new_externally_tagged_enum(
                                            "first_query",
                                            {
                                                let mut schema_object = schemars::schema::SchemaObject {
                                                    instance_type: Some(
                                                        schemars::schema::InstanceType::Object.into(),
                                                    ),
                                                    ..Default::default()
                                                };
                                                let object_validation = schema_object.object();
                                                object_validation
                                                    .additional_properties = Some(Box::new(false.into()));
                                                schemars::schema::Schema::Object(schema_object)
                                            },
                                        ),
                                        schemars::_private::new_externally_tagged_enum(
                                            "second_query",
                                            {
                                                let mut schema_object = schemars::schema::SchemaObject {
                                                    instance_type: Some(
                                                        schemars::schema::InstanceType::Object.into(),
                                                    ),
                                                    ..Default::default()
                                                };
                                                let object_validation = schema_object.object();
                                                object_validation
                                                    .additional_properties = Some(Box::new(false.into()));
                                                {
                                                    schemars::_private::insert_object_property::<
                                                        String,
                                                    >(
                                                        object_validation,
                                                        "t",
                                                        false,
                                                        false,
                                                        gen.subschema_for::<String>(),
                                                    );
                                                }
                                                schemars::schema::Schema::Object(schema_object)
                                            },
                                        ),
                                    ]),
                                ),
                            ),
                            ..Default::default()
                        }),
                    ),
                    ..Default::default()
                })
            }
        }
    };
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[serde(deny_unknown_fields, crate = "::cosmwasm_schema::serde")]
    #[schemars(crate = "::cosmwasm_schema::schemars")]
    pub struct MigrateMsg {
        pub t: String,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        use ::cosmwasm_schema::serde as _serde;
        #[automatically_derived]
        impl ::cosmwasm_schema::serde::Serialize for MigrateMsg {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> ::cosmwasm_schema::serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: ::cosmwasm_schema::serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "MigrateMsg",
                    false as usize + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "t",
                    &self.t,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        use ::cosmwasm_schema::serde as _serde;
        #[automatically_derived]
        impl<'de> ::cosmwasm_schema::serde::Deserialize<'de> for MigrateMsg {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> ::cosmwasm_schema::serde::__private::Result<Self, __D::Error>
            where
                __D: ::cosmwasm_schema::serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::invalid_value(
                                        _serde::de::Unexpected::Unsigned(__value),
                                        &"field index 0 <= i < 1",
                                    ),
                                )
                            }
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "t" => _serde::__private::Ok(__Field::__field0),
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::unknown_field(__value, FIELDS),
                                )
                            }
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"t" => _serde::__private::Ok(__Field::__field0),
                            _ => {
                                let __value = &_serde::__private::from_utf8_lossy(__value);
                                _serde::__private::Err(
                                    _serde::de::Error::unknown_field(__value, FIELDS),
                                )
                            }
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<MigrateMsg>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = MigrateMsg;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct MigrateMsg",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct MigrateMsg with 1 element",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(MigrateMsg { t: __field0 })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("t"),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                    );
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("t")?
                            }
                        };
                        _serde::__private::Ok(MigrateMsg { t: __field0 })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["t"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "MigrateMsg",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<MigrateMsg>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::clone::Clone for MigrateMsg {
        #[inline]
        fn clone(&self) -> MigrateMsg {
            MigrateMsg {
                t: ::core::clone::Clone::clone(&self.t),
            }
        }
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::fmt::Debug for MigrateMsg {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "MigrateMsg",
                "t",
                &&self.t,
            )
        }
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::marker::StructuralPartialEq for MigrateMsg {}
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::cmp::PartialEq for MigrateMsg {
        #[inline]
        fn eq(&self, other: &MigrateMsg) -> bool {
            self.t == other.t
        }
    }
    const _: () = {
        use ::cosmwasm_schema::schemars as schemars;
        #[automatically_derived]
        #[allow(unused_braces)]
        impl schemars::JsonSchema for MigrateMsg {
            fn schema_name() -> std::string::String {
                "MigrateMsg".to_owned()
            }
            fn schema_id() -> std::borrow::Cow<'static, str> {
                std::borrow::Cow::Borrowed("mock_contract::custom_resp::MigrateMsg")
            }
            fn json_schema(
                gen: &mut schemars::gen::SchemaGenerator,
            ) -> schemars::schema::Schema {
                {
                    let mut schema_object = schemars::schema::SchemaObject {
                        instance_type: Some(
                            schemars::schema::InstanceType::Object.into(),
                        ),
                        ..Default::default()
                    };
                    let object_validation = schema_object.object();
                    object_validation
                        .additional_properties = Some(Box::new(false.into()));
                    {
                        schemars::_private::insert_object_property::<
                            String,
                        >(
                            object_validation,
                            "t",
                            false,
                            false,
                            gen.subschema_for::<String>(),
                        );
                    }
                    schemars::schema::Schema::Object(schema_object)
                }
            }
        }
    };
    pub fn instantiate(
        _deps: DepsMut,
        _env: Env,
        _info: MessageInfo,
        _msg: InstantiateMsg,
    ) -> StdResult<Response<A>> {
        Ok(Response::new().add_attribute("action", "instantiate"))
    }
    pub fn execute(
        _deps: DepsMut,
        _env: Env,
        _info: MessageInfo,
        msg: ExecuteMsg,
    ) -> StdResult<Response<A>> {
        match msg {
            ExecuteMsg::FirstMessage {} => {
                Ok(Response::new().add_attribute("action", "first message passed"))
            }
        }
    }
    pub fn query(_deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
        match msg {
            QueryMsg::FirstQuery {} => to_json_binary("first query passed"),
            QueryMsg::SecondQuery { .. } => {
                Err(StdError::generic_err("Query not available"))
            }
        }
    }
    pub fn migrate(
        _deps: DepsMut,
        _env: Env,
        msg: MigrateMsg,
    ) -> StdResult<Response<A>> {
        if msg.t.eq("success") {
            Ok(Response::new())
        } else {
            Err(
                StdError::generic_err(
                    "migrate endpoint reached but no test implementation",
                ),
            )
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    pub struct MockContract<Chain>(::cw_orch::core::contract::Contract<Chain>);
    #[automatically_derived]
    impl<Chain: ::core::clone::Clone> ::core::clone::Clone for MockContract<Chain> {
        #[inline]
        fn clone(&self) -> MockContract<Chain> {
            MockContract(::core::clone::Clone::clone(&self.0))
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    impl<Chain> MockContract<Chain> {
        pub fn new(contract_id: impl ToString, chain: Chain) -> Self {
            Self(::cw_orch::core::contract::Contract::new(contract_id, chain))
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    impl<
        Chain: ::cw_orch::core::environment::ChainState,
    > ::cw_orch::core::contract::interface_traits::ContractInstance<Chain>
    for MockContract<Chain> {
        fn as_instance(&self) -> &::cw_orch::core::contract::Contract<Chain> {
            &self.0
        }
        fn as_instance_mut(
            &mut self,
        ) -> &mut ::cw_orch::core::contract::Contract<Chain> {
            &mut self.0
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    impl<Chain> ::cw_orch::core::contract::interface_traits::InstantiableContract
    for MockContract<Chain> {
        type InstantiateMsg = InstantiateMsg;
    }
    #[cfg(not(target_arch = "wasm32"))]
    impl<Chain> ::cw_orch::core::contract::interface_traits::ExecutableContract
    for MockContract<Chain> {
        type ExecuteMsg = ExecuteMsg;
    }
    #[cfg(not(target_arch = "wasm32"))]
    impl<Chain> ::cw_orch::core::contract::interface_traits::QueryableContract
    for MockContract<Chain> {
        type QueryMsg = QueryMsg;
    }
    #[cfg(not(target_arch = "wasm32"))]
    impl<Chain> ::cw_orch::core::contract::interface_traits::MigratableContract
    for MockContract<Chain> {
        type MigrateMsg = MigrateMsg;
    }
}
mod msg_tests {
    use cosmwasm_schema::cw_serde;
    use cosmwasm_std::{
        Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult,
    };
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[serde(
        deny_unknown_fields,
        rename_all = "snake_case",
        crate = "::cosmwasm_schema::serde"
    )]
    #[schemars(crate = "::cosmwasm_schema::schemars")]
    #[disable_fields_sorting]
    pub enum ExecuteMsg {
        Test { b: u64, a: String },
    }
    #[cfg(not(target_arch = "wasm32"))]
    /// Automatically derived trait that allows you to call the variants of the message directly without the need to construct the struct yourself.
    pub trait ExecuteMsgFns<
        Chain: ::cw_orch::core::environment::TxHandler,
        CwOrchExecuteMsgType,
    >: ::cw_orch::core::contract::interface_traits::CwOrchExecute<
            Chain,
            ExecuteMsg = CwOrchExecuteMsgType,
        >
    where
        ExecuteMsg: Into<CwOrchExecuteMsgType>,
    {
        ///Automatically generated wrapper around ExecuteMsg::Test variant
        #[allow(clippy::too_many_arguments)]
        fn test(
            &self,
            b: impl Into<u64>,
            a: impl Into<String>,
        ) -> Result<
            ::cw_orch::core::environment::TxResponse<Chain>,
            ::cw_orch::core::CwEnvError,
        > {
            let msg = ExecuteMsg::Test {
                b: b.into(),
                a: a.into(),
            };
            <Self as ::cw_orch::core::contract::interface_traits::CwOrchExecute<
                Chain,
            >>::execute(self, &msg.into(), None)
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[automatically_derived]
    impl<
        SupportedContract,
        Chain: ::cw_orch::core::environment::TxHandler,
        CwOrchExecuteMsgType,
    > ExecuteMsgFns<Chain, CwOrchExecuteMsgType> for SupportedContract
    where
        SupportedContract: ::cw_orch::core::contract::interface_traits::CwOrchExecute<
            Chain,
            ExecuteMsg = CwOrchExecuteMsgType,
        >,
        ExecuteMsg: Into<CwOrchExecuteMsgType>,
    {}
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        use ::cosmwasm_schema::serde as _serde;
        #[automatically_derived]
        impl ::cosmwasm_schema::serde::Serialize for ExecuteMsg {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> ::cosmwasm_schema::serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: ::cosmwasm_schema::serde::Serializer,
            {
                match *self {
                    ExecuteMsg::Test { ref b, ref a } => {
                        let mut __serde_state = _serde::Serializer::serialize_struct_variant(
                            __serializer,
                            "ExecuteMsg",
                            0u32,
                            "test",
                            0 + 1 + 1,
                        )?;
                        _serde::ser::SerializeStructVariant::serialize_field(
                            &mut __serde_state,
                            "b",
                            b,
                        )?;
                        _serde::ser::SerializeStructVariant::serialize_field(
                            &mut __serde_state,
                            "a",
                            a,
                        )?;
                        _serde::ser::SerializeStructVariant::end(__serde_state)
                    }
                }
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        use ::cosmwasm_schema::serde as _serde;
        #[automatically_derived]
        impl<'de> ::cosmwasm_schema::serde::Deserialize<'de> for ExecuteMsg {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> ::cosmwasm_schema::serde::__private::Result<Self, __D::Error>
            where
                __D: ::cosmwasm_schema::serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "variant identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::invalid_value(
                                        _serde::de::Unexpected::Unsigned(__value),
                                        &"variant index 0 <= i < 1",
                                    ),
                                )
                            }
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "test" => _serde::__private::Ok(__Field::__field0),
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::unknown_variant(__value, VARIANTS),
                                )
                            }
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"test" => _serde::__private::Ok(__Field::__field0),
                            _ => {
                                let __value = &_serde::__private::from_utf8_lossy(__value);
                                _serde::__private::Err(
                                    _serde::de::Error::unknown_variant(__value, VARIANTS),
                                )
                            }
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<ExecuteMsg>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = ExecuteMsg;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "enum ExecuteMsg",
                        )
                    }
                    fn visit_enum<__A>(
                        self,
                        __data: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::EnumAccess<'de>,
                    {
                        match _serde::de::EnumAccess::variant(__data)? {
                            (__Field::__field0, __variant) => {
                                #[allow(non_camel_case_types)]
                                #[doc(hidden)]
                                enum __Field {
                                    __field0,
                                    __field1,
                                }
                                #[doc(hidden)]
                                struct __FieldVisitor;
                                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                                    type Value = __Field;
                                    fn expecting(
                                        &self,
                                        __formatter: &mut _serde::__private::Formatter,
                                    ) -> _serde::__private::fmt::Result {
                                        _serde::__private::Formatter::write_str(
                                            __formatter,
                                            "field identifier",
                                        )
                                    }
                                    fn visit_u64<__E>(
                                        self,
                                        __value: u64,
                                    ) -> _serde::__private::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            0u64 => _serde::__private::Ok(__Field::__field0),
                                            1u64 => _serde::__private::Ok(__Field::__field1),
                                            _ => {
                                                _serde::__private::Err(
                                                    _serde::de::Error::invalid_value(
                                                        _serde::de::Unexpected::Unsigned(__value),
                                                        &"field index 0 <= i < 2",
                                                    ),
                                                )
                                            }
                                        }
                                    }
                                    fn visit_str<__E>(
                                        self,
                                        __value: &str,
                                    ) -> _serde::__private::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            "b" => _serde::__private::Ok(__Field::__field0),
                                            "a" => _serde::__private::Ok(__Field::__field1),
                                            _ => {
                                                _serde::__private::Err(
                                                    _serde::de::Error::unknown_field(__value, FIELDS),
                                                )
                                            }
                                        }
                                    }
                                    fn visit_bytes<__E>(
                                        self,
                                        __value: &[u8],
                                    ) -> _serde::__private::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            b"b" => _serde::__private::Ok(__Field::__field0),
                                            b"a" => _serde::__private::Ok(__Field::__field1),
                                            _ => {
                                                let __value = &_serde::__private::from_utf8_lossy(__value);
                                                _serde::__private::Err(
                                                    _serde::de::Error::unknown_field(__value, FIELDS),
                                                )
                                            }
                                        }
                                    }
                                }
                                impl<'de> _serde::Deserialize<'de> for __Field {
                                    #[inline]
                                    fn deserialize<__D>(
                                        __deserializer: __D,
                                    ) -> _serde::__private::Result<Self, __D::Error>
                                    where
                                        __D: _serde::Deserializer<'de>,
                                    {
                                        _serde::Deserializer::deserialize_identifier(
                                            __deserializer,
                                            __FieldVisitor,
                                        )
                                    }
                                }
                                #[doc(hidden)]
                                struct __Visitor<'de> {
                                    marker: _serde::__private::PhantomData<ExecuteMsg>,
                                    lifetime: _serde::__private::PhantomData<&'de ()>,
                                }
                                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                    type Value = ExecuteMsg;
                                    fn expecting(
                                        &self,
                                        __formatter: &mut _serde::__private::Formatter,
                                    ) -> _serde::__private::fmt::Result {
                                        _serde::__private::Formatter::write_str(
                                            __formatter,
                                            "struct variant ExecuteMsg::Test",
                                        )
                                    }
                                    #[inline]
                                    fn visit_seq<__A>(
                                        self,
                                        mut __seq: __A,
                                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                                    where
                                        __A: _serde::de::SeqAccess<'de>,
                                    {
                                        let __field0 = match _serde::de::SeqAccess::next_element::<
                                            u64,
                                        >(&mut __seq)? {
                                            _serde::__private::Some(__value) => __value,
                                            _serde::__private::None => {
                                                return _serde::__private::Err(
                                                    _serde::de::Error::invalid_length(
                                                        0usize,
                                                        &"struct variant ExecuteMsg::Test with 2 elements",
                                                    ),
                                                );
                                            }
                                        };
                                        let __field1 = match _serde::de::SeqAccess::next_element::<
                                            String,
                                        >(&mut __seq)? {
                                            _serde::__private::Some(__value) => __value,
                                            _serde::__private::None => {
                                                return _serde::__private::Err(
                                                    _serde::de::Error::invalid_length(
                                                        1usize,
                                                        &"struct variant ExecuteMsg::Test with 2 elements",
                                                    ),
                                                );
                                            }
                                        };
                                        _serde::__private::Ok(ExecuteMsg::Test {
                                            b: __field0,
                                            a: __field1,
                                        })
                                    }
                                    #[inline]
                                    fn visit_map<__A>(
                                        self,
                                        mut __map: __A,
                                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                                    where
                                        __A: _serde::de::MapAccess<'de>,
                                    {
                                        let mut __field0: _serde::__private::Option<u64> = _serde::__private::None;
                                        let mut __field1: _serde::__private::Option<String> = _serde::__private::None;
                                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                            __Field,
                                        >(&mut __map)? {
                                            match __key {
                                                __Field::__field0 => {
                                                    if _serde::__private::Option::is_some(&__field0) {
                                                        return _serde::__private::Err(
                                                            <__A::Error as _serde::de::Error>::duplicate_field("b"),
                                                        );
                                                    }
                                                    __field0 = _serde::__private::Some(
                                                        _serde::de::MapAccess::next_value::<u64>(&mut __map)?,
                                                    );
                                                }
                                                __Field::__field1 => {
                                                    if _serde::__private::Option::is_some(&__field1) {
                                                        return _serde::__private::Err(
                                                            <__A::Error as _serde::de::Error>::duplicate_field("a"),
                                                        );
                                                    }
                                                    __field1 = _serde::__private::Some(
                                                        _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                                    );
                                                }
                                            }
                                        }
                                        let __field0 = match __field0 {
                                            _serde::__private::Some(__field0) => __field0,
                                            _serde::__private::None => {
                                                _serde::__private::de::missing_field("b")?
                                            }
                                        };
                                        let __field1 = match __field1 {
                                            _serde::__private::Some(__field1) => __field1,
                                            _serde::__private::None => {
                                                _serde::__private::de::missing_field("a")?
                                            }
                                        };
                                        _serde::__private::Ok(ExecuteMsg::Test {
                                            b: __field0,
                                            a: __field1,
                                        })
                                    }
                                }
                                #[doc(hidden)]
                                const FIELDS: &'static [&'static str] = &["b", "a"];
                                _serde::de::VariantAccess::struct_variant(
                                    __variant,
                                    FIELDS,
                                    __Visitor {
                                        marker: _serde::__private::PhantomData::<ExecuteMsg>,
                                        lifetime: _serde::__private::PhantomData,
                                    },
                                )
                            }
                        }
                    }
                }
                #[doc(hidden)]
                const VARIANTS: &'static [&'static str] = &["test"];
                _serde::Deserializer::deserialize_enum(
                    __deserializer,
                    "ExecuteMsg",
                    VARIANTS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<ExecuteMsg>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::clone::Clone for ExecuteMsg {
        #[inline]
        fn clone(&self) -> ExecuteMsg {
            match self {
                ExecuteMsg::Test { b: __self_0, a: __self_1 } => {
                    ExecuteMsg::Test {
                        b: ::core::clone::Clone::clone(__self_0),
                        a: ::core::clone::Clone::clone(__self_1),
                    }
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::fmt::Debug for ExecuteMsg {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                ExecuteMsg::Test { b: __self_0, a: __self_1 } => {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "Test",
                        "b",
                        __self_0,
                        "a",
                        &__self_1,
                    )
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::marker::StructuralPartialEq for ExecuteMsg {}
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::cmp::PartialEq for ExecuteMsg {
        #[inline]
        fn eq(&self, other: &ExecuteMsg) -> bool {
            match (self, other) {
                (
                    ExecuteMsg::Test { b: __self_0, a: __self_1 },
                    ExecuteMsg::Test { b: __arg1_0, a: __arg1_1 },
                ) => *__self_0 == *__arg1_0 && *__self_1 == *__arg1_1,
            }
        }
    }
    const _: () = {
        use ::cosmwasm_schema::schemars as schemars;
        #[automatically_derived]
        #[allow(unused_braces)]
        impl schemars::JsonSchema for ExecuteMsg {
            fn schema_name() -> std::string::String {
                "ExecuteMsg".to_owned()
            }
            fn schema_id() -> std::borrow::Cow<'static, str> {
                std::borrow::Cow::Borrowed("mock_contract::msg_tests::ExecuteMsg")
            }
            fn json_schema(
                gen: &mut schemars::gen::SchemaGenerator,
            ) -> schemars::schema::Schema {
                schemars::schema::Schema::Object(schemars::schema::SchemaObject {
                    subschemas: Some(
                        Box::new(schemars::schema::SubschemaValidation {
                            one_of: Some(
                                <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new([
                                        schemars::_private::new_externally_tagged_enum(
                                            "test",
                                            {
                                                let mut schema_object = schemars::schema::SchemaObject {
                                                    instance_type: Some(
                                                        schemars::schema::InstanceType::Object.into(),
                                                    ),
                                                    ..Default::default()
                                                };
                                                let object_validation = schema_object.object();
                                                object_validation
                                                    .additional_properties = Some(Box::new(false.into()));
                                                {
                                                    schemars::_private::insert_object_property::<
                                                        u64,
                                                    >(
                                                        object_validation,
                                                        "b",
                                                        false,
                                                        false,
                                                        gen.subschema_for::<u64>(),
                                                    );
                                                }
                                                {
                                                    schemars::_private::insert_object_property::<
                                                        String,
                                                    >(
                                                        object_validation,
                                                        "a",
                                                        false,
                                                        false,
                                                        gen.subschema_for::<String>(),
                                                    );
                                                }
                                                schemars::schema::Schema::Object(schema_object)
                                            },
                                        ),
                                    ]),
                                ),
                            ),
                            ..Default::default()
                        }),
                    ),
                    ..Default::default()
                })
            }
        }
    };
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[serde(
        deny_unknown_fields,
        rename_all = "snake_case",
        crate = "::cosmwasm_schema::serde"
    )]
    #[schemars(crate = "::cosmwasm_schema::schemars")]
    pub enum ExecuteMsgOrdered {
        Test { b: u64, a: String },
    }
    #[cfg(not(target_arch = "wasm32"))]
    /// Automatically derived trait that allows you to call the variants of the message directly without the need to construct the struct yourself.
    pub trait ExecuteMsgOrderedFns<
        Chain: ::cw_orch::core::environment::TxHandler,
        CwOrchExecuteMsgType,
    >: ::cw_orch::core::contract::interface_traits::CwOrchExecute<
            Chain,
            ExecuteMsg = CwOrchExecuteMsgType,
        >
    where
        ExecuteMsgOrdered: Into<CwOrchExecuteMsgType>,
    {
        ///Automatically generated wrapper around ExecuteMsgOrdered::Test variant
        #[allow(clippy::too_many_arguments)]
        fn test(
            &self,
            a: impl Into<String>,
            b: impl Into<u64>,
        ) -> Result<
            ::cw_orch::core::environment::TxResponse<Chain>,
            ::cw_orch::core::CwEnvError,
        > {
            let msg = ExecuteMsgOrdered::Test {
                a: a.into(),
                b: b.into(),
            };
            <Self as ::cw_orch::core::contract::interface_traits::CwOrchExecute<
                Chain,
            >>::execute(self, &msg.into(), None)
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[automatically_derived]
    impl<
        SupportedContract,
        Chain: ::cw_orch::core::environment::TxHandler,
        CwOrchExecuteMsgType,
    > ExecuteMsgOrderedFns<Chain, CwOrchExecuteMsgType> for SupportedContract
    where
        SupportedContract: ::cw_orch::core::contract::interface_traits::CwOrchExecute<
            Chain,
            ExecuteMsg = CwOrchExecuteMsgType,
        >,
        ExecuteMsgOrdered: Into<CwOrchExecuteMsgType>,
    {}
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        use ::cosmwasm_schema::serde as _serde;
        #[automatically_derived]
        impl ::cosmwasm_schema::serde::Serialize for ExecuteMsgOrdered {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> ::cosmwasm_schema::serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: ::cosmwasm_schema::serde::Serializer,
            {
                match *self {
                    ExecuteMsgOrdered::Test { ref b, ref a } => {
                        let mut __serde_state = _serde::Serializer::serialize_struct_variant(
                            __serializer,
                            "ExecuteMsgOrdered",
                            0u32,
                            "test",
                            0 + 1 + 1,
                        )?;
                        _serde::ser::SerializeStructVariant::serialize_field(
                            &mut __serde_state,
                            "b",
                            b,
                        )?;
                        _serde::ser::SerializeStructVariant::serialize_field(
                            &mut __serde_state,
                            "a",
                            a,
                        )?;
                        _serde::ser::SerializeStructVariant::end(__serde_state)
                    }
                }
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        use ::cosmwasm_schema::serde as _serde;
        #[automatically_derived]
        impl<'de> ::cosmwasm_schema::serde::Deserialize<'de> for ExecuteMsgOrdered {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> ::cosmwasm_schema::serde::__private::Result<Self, __D::Error>
            where
                __D: ::cosmwasm_schema::serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "variant identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::invalid_value(
                                        _serde::de::Unexpected::Unsigned(__value),
                                        &"variant index 0 <= i < 1",
                                    ),
                                )
                            }
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "test" => _serde::__private::Ok(__Field::__field0),
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::unknown_variant(__value, VARIANTS),
                                )
                            }
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"test" => _serde::__private::Ok(__Field::__field0),
                            _ => {
                                let __value = &_serde::__private::from_utf8_lossy(__value);
                                _serde::__private::Err(
                                    _serde::de::Error::unknown_variant(__value, VARIANTS),
                                )
                            }
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<ExecuteMsgOrdered>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = ExecuteMsgOrdered;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "enum ExecuteMsgOrdered",
                        )
                    }
                    fn visit_enum<__A>(
                        self,
                        __data: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::EnumAccess<'de>,
                    {
                        match _serde::de::EnumAccess::variant(__data)? {
                            (__Field::__field0, __variant) => {
                                #[allow(non_camel_case_types)]
                                #[doc(hidden)]
                                enum __Field {
                                    __field0,
                                    __field1,
                                }
                                #[doc(hidden)]
                                struct __FieldVisitor;
                                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                                    type Value = __Field;
                                    fn expecting(
                                        &self,
                                        __formatter: &mut _serde::__private::Formatter,
                                    ) -> _serde::__private::fmt::Result {
                                        _serde::__private::Formatter::write_str(
                                            __formatter,
                                            "field identifier",
                                        )
                                    }
                                    fn visit_u64<__E>(
                                        self,
                                        __value: u64,
                                    ) -> _serde::__private::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            0u64 => _serde::__private::Ok(__Field::__field0),
                                            1u64 => _serde::__private::Ok(__Field::__field1),
                                            _ => {
                                                _serde::__private::Err(
                                                    _serde::de::Error::invalid_value(
                                                        _serde::de::Unexpected::Unsigned(__value),
                                                        &"field index 0 <= i < 2",
                                                    ),
                                                )
                                            }
                                        }
                                    }
                                    fn visit_str<__E>(
                                        self,
                                        __value: &str,
                                    ) -> _serde::__private::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            "b" => _serde::__private::Ok(__Field::__field0),
                                            "a" => _serde::__private::Ok(__Field::__field1),
                                            _ => {
                                                _serde::__private::Err(
                                                    _serde::de::Error::unknown_field(__value, FIELDS),
                                                )
                                            }
                                        }
                                    }
                                    fn visit_bytes<__E>(
                                        self,
                                        __value: &[u8],
                                    ) -> _serde::__private::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            b"b" => _serde::__private::Ok(__Field::__field0),
                                            b"a" => _serde::__private::Ok(__Field::__field1),
                                            _ => {
                                                let __value = &_serde::__private::from_utf8_lossy(__value);
                                                _serde::__private::Err(
                                                    _serde::de::Error::unknown_field(__value, FIELDS),
                                                )
                                            }
                                        }
                                    }
                                }
                                impl<'de> _serde::Deserialize<'de> for __Field {
                                    #[inline]
                                    fn deserialize<__D>(
                                        __deserializer: __D,
                                    ) -> _serde::__private::Result<Self, __D::Error>
                                    where
                                        __D: _serde::Deserializer<'de>,
                                    {
                                        _serde::Deserializer::deserialize_identifier(
                                            __deserializer,
                                            __FieldVisitor,
                                        )
                                    }
                                }
                                #[doc(hidden)]
                                struct __Visitor<'de> {
                                    marker: _serde::__private::PhantomData<ExecuteMsgOrdered>,
                                    lifetime: _serde::__private::PhantomData<&'de ()>,
                                }
                                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                    type Value = ExecuteMsgOrdered;
                                    fn expecting(
                                        &self,
                                        __formatter: &mut _serde::__private::Formatter,
                                    ) -> _serde::__private::fmt::Result {
                                        _serde::__private::Formatter::write_str(
                                            __formatter,
                                            "struct variant ExecuteMsgOrdered::Test",
                                        )
                                    }
                                    #[inline]
                                    fn visit_seq<__A>(
                                        self,
                                        mut __seq: __A,
                                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                                    where
                                        __A: _serde::de::SeqAccess<'de>,
                                    {
                                        let __field0 = match _serde::de::SeqAccess::next_element::<
                                            u64,
                                        >(&mut __seq)? {
                                            _serde::__private::Some(__value) => __value,
                                            _serde::__private::None => {
                                                return _serde::__private::Err(
                                                    _serde::de::Error::invalid_length(
                                                        0usize,
                                                        &"struct variant ExecuteMsgOrdered::Test with 2 elements",
                                                    ),
                                                );
                                            }
                                        };
                                        let __field1 = match _serde::de::SeqAccess::next_element::<
                                            String,
                                        >(&mut __seq)? {
                                            _serde::__private::Some(__value) => __value,
                                            _serde::__private::None => {
                                                return _serde::__private::Err(
                                                    _serde::de::Error::invalid_length(
                                                        1usize,
                                                        &"struct variant ExecuteMsgOrdered::Test with 2 elements",
                                                    ),
                                                );
                                            }
                                        };
                                        _serde::__private::Ok(ExecuteMsgOrdered::Test {
                                            b: __field0,
                                            a: __field1,
                                        })
                                    }
                                    #[inline]
                                    fn visit_map<__A>(
                                        self,
                                        mut __map: __A,
                                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                                    where
                                        __A: _serde::de::MapAccess<'de>,
                                    {
                                        let mut __field0: _serde::__private::Option<u64> = _serde::__private::None;
                                        let mut __field1: _serde::__private::Option<String> = _serde::__private::None;
                                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                            __Field,
                                        >(&mut __map)? {
                                            match __key {
                                                __Field::__field0 => {
                                                    if _serde::__private::Option::is_some(&__field0) {
                                                        return _serde::__private::Err(
                                                            <__A::Error as _serde::de::Error>::duplicate_field("b"),
                                                        );
                                                    }
                                                    __field0 = _serde::__private::Some(
                                                        _serde::de::MapAccess::next_value::<u64>(&mut __map)?,
                                                    );
                                                }
                                                __Field::__field1 => {
                                                    if _serde::__private::Option::is_some(&__field1) {
                                                        return _serde::__private::Err(
                                                            <__A::Error as _serde::de::Error>::duplicate_field("a"),
                                                        );
                                                    }
                                                    __field1 = _serde::__private::Some(
                                                        _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                                    );
                                                }
                                            }
                                        }
                                        let __field0 = match __field0 {
                                            _serde::__private::Some(__field0) => __field0,
                                            _serde::__private::None => {
                                                _serde::__private::de::missing_field("b")?
                                            }
                                        };
                                        let __field1 = match __field1 {
                                            _serde::__private::Some(__field1) => __field1,
                                            _serde::__private::None => {
                                                _serde::__private::de::missing_field("a")?
                                            }
                                        };
                                        _serde::__private::Ok(ExecuteMsgOrdered::Test {
                                            b: __field0,
                                            a: __field1,
                                        })
                                    }
                                }
                                #[doc(hidden)]
                                const FIELDS: &'static [&'static str] = &["b", "a"];
                                _serde::de::VariantAccess::struct_variant(
                                    __variant,
                                    FIELDS,
                                    __Visitor {
                                        marker: _serde::__private::PhantomData::<ExecuteMsgOrdered>,
                                        lifetime: _serde::__private::PhantomData,
                                    },
                                )
                            }
                        }
                    }
                }
                #[doc(hidden)]
                const VARIANTS: &'static [&'static str] = &["test"];
                _serde::Deserializer::deserialize_enum(
                    __deserializer,
                    "ExecuteMsgOrdered",
                    VARIANTS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<ExecuteMsgOrdered>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::clone::Clone for ExecuteMsgOrdered {
        #[inline]
        fn clone(&self) -> ExecuteMsgOrdered {
            match self {
                ExecuteMsgOrdered::Test { b: __self_0, a: __self_1 } => {
                    ExecuteMsgOrdered::Test {
                        b: ::core::clone::Clone::clone(__self_0),
                        a: ::core::clone::Clone::clone(__self_1),
                    }
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::fmt::Debug for ExecuteMsgOrdered {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                ExecuteMsgOrdered::Test { b: __self_0, a: __self_1 } => {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "Test",
                        "b",
                        __self_0,
                        "a",
                        &__self_1,
                    )
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::marker::StructuralPartialEq for ExecuteMsgOrdered {}
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::cmp::PartialEq for ExecuteMsgOrdered {
        #[inline]
        fn eq(&self, other: &ExecuteMsgOrdered) -> bool {
            match (self, other) {
                (
                    ExecuteMsgOrdered::Test { b: __self_0, a: __self_1 },
                    ExecuteMsgOrdered::Test { b: __arg1_0, a: __arg1_1 },
                ) => *__self_0 == *__arg1_0 && *__self_1 == *__arg1_1,
            }
        }
    }
    const _: () = {
        use ::cosmwasm_schema::schemars as schemars;
        #[automatically_derived]
        #[allow(unused_braces)]
        impl schemars::JsonSchema for ExecuteMsgOrdered {
            fn schema_name() -> std::string::String {
                "ExecuteMsgOrdered".to_owned()
            }
            fn schema_id() -> std::borrow::Cow<'static, str> {
                std::borrow::Cow::Borrowed("mock_contract::msg_tests::ExecuteMsgOrdered")
            }
            fn json_schema(
                gen: &mut schemars::gen::SchemaGenerator,
            ) -> schemars::schema::Schema {
                schemars::schema::Schema::Object(schemars::schema::SchemaObject {
                    subschemas: Some(
                        Box::new(schemars::schema::SubschemaValidation {
                            one_of: Some(
                                <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new([
                                        schemars::_private::new_externally_tagged_enum(
                                            "test",
                                            {
                                                let mut schema_object = schemars::schema::SchemaObject {
                                                    instance_type: Some(
                                                        schemars::schema::InstanceType::Object.into(),
                                                    ),
                                                    ..Default::default()
                                                };
                                                let object_validation = schema_object.object();
                                                object_validation
                                                    .additional_properties = Some(Box::new(false.into()));
                                                {
                                                    schemars::_private::insert_object_property::<
                                                        u64,
                                                    >(
                                                        object_validation,
                                                        "b",
                                                        false,
                                                        false,
                                                        gen.subschema_for::<u64>(),
                                                    );
                                                }
                                                {
                                                    schemars::_private::insert_object_property::<
                                                        String,
                                                    >(
                                                        object_validation,
                                                        "a",
                                                        false,
                                                        false,
                                                        gen.subschema_for::<String>(),
                                                    );
                                                }
                                                schemars::schema::Schema::Object(schema_object)
                                            },
                                        ),
                                    ]),
                                ),
                            ),
                            ..Default::default()
                        }),
                    ),
                    ..Default::default()
                })
            }
        }
    };
    #[cfg(not(target_arch = "wasm32"))]
    pub struct TestContract<Chain>(::cw_orch::core::contract::Contract<Chain>);
    #[automatically_derived]
    impl<Chain: ::core::clone::Clone> ::core::clone::Clone for TestContract<Chain> {
        #[inline]
        fn clone(&self) -> TestContract<Chain> {
            TestContract(::core::clone::Clone::clone(&self.0))
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    impl<Chain> TestContract<Chain> {
        pub fn new(contract_id: impl ToString, chain: Chain) -> Self {
            Self(::cw_orch::core::contract::Contract::new(contract_id, chain))
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    impl<
        Chain: ::cw_orch::core::environment::ChainState,
    > ::cw_orch::core::contract::interface_traits::ContractInstance<Chain>
    for TestContract<Chain> {
        fn as_instance(&self) -> &::cw_orch::core::contract::Contract<Chain> {
            &self.0
        }
        fn as_instance_mut(
            &mut self,
        ) -> &mut ::cw_orch::core::contract::Contract<Chain> {
            &mut self.0
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    impl<Chain> ::cw_orch::core::contract::interface_traits::InstantiableContract
    for TestContract<Chain> {
        type InstantiateMsg = Empty;
    }
    #[cfg(not(target_arch = "wasm32"))]
    impl<Chain> ::cw_orch::core::contract::interface_traits::ExecutableContract
    for TestContract<Chain> {
        type ExecuteMsg = ExecuteMsg;
    }
    #[cfg(not(target_arch = "wasm32"))]
    impl<Chain> ::cw_orch::core::contract::interface_traits::QueryableContract
    for TestContract<Chain> {
        type QueryMsg = Empty;
    }
    #[cfg(not(target_arch = "wasm32"))]
    impl<Chain> ::cw_orch::core::contract::interface_traits::MigratableContract
    for TestContract<Chain> {
        type MigrateMsg = Empty;
    }
    #[cfg(not(target_arch = "wasm32"))]
    pub struct OrderedTestContract<Chain>(::cw_orch::core::contract::Contract<Chain>);
    #[automatically_derived]
    impl<Chain: ::core::clone::Clone> ::core::clone::Clone
    for OrderedTestContract<Chain> {
        #[inline]
        fn clone(&self) -> OrderedTestContract<Chain> {
            OrderedTestContract(::core::clone::Clone::clone(&self.0))
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    impl<Chain> OrderedTestContract<Chain> {
        pub fn new(contract_id: impl ToString, chain: Chain) -> Self {
            Self(::cw_orch::core::contract::Contract::new(contract_id, chain))
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    impl<
        Chain: ::cw_orch::core::environment::ChainState,
    > ::cw_orch::core::contract::interface_traits::ContractInstance<Chain>
    for OrderedTestContract<Chain> {
        fn as_instance(&self) -> &::cw_orch::core::contract::Contract<Chain> {
            &self.0
        }
        fn as_instance_mut(
            &mut self,
        ) -> &mut ::cw_orch::core::contract::Contract<Chain> {
            &mut self.0
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    impl<Chain> ::cw_orch::core::contract::interface_traits::InstantiableContract
    for OrderedTestContract<Chain> {
        type InstantiateMsg = Empty;
    }
    #[cfg(not(target_arch = "wasm32"))]
    impl<Chain> ::cw_orch::core::contract::interface_traits::ExecutableContract
    for OrderedTestContract<Chain> {
        type ExecuteMsg = ExecuteMsgOrdered;
    }
    #[cfg(not(target_arch = "wasm32"))]
    impl<Chain> ::cw_orch::core::contract::interface_traits::QueryableContract
    for OrderedTestContract<Chain> {
        type QueryMsg = Empty;
    }
    #[cfg(not(target_arch = "wasm32"))]
    impl<Chain> ::cw_orch::core::contract::interface_traits::MigratableContract
    for OrderedTestContract<Chain> {
        type MigrateMsg = Empty;
    }
    pub fn instantiate(
        _deps: DepsMut,
        _env: Env,
        _info: MessageInfo,
        _msg: Empty,
    ) -> StdResult<Response> {
        Ok(Response::new())
    }
    pub fn execute(
        _deps: DepsMut,
        _env: Env,
        _info: MessageInfo,
        _msg: ExecuteMsg,
    ) -> StdResult<Response> {
        Ok(Response::new())
    }
    pub fn execute_ordered(
        _deps: DepsMut,
        _env: Env,
        _info: MessageInfo,
        _msg: ExecuteMsgOrdered,
    ) -> StdResult<Response> {
        Ok(Response::new())
    }
    pub fn query(_deps: Deps, _env: Env, _msg: Empty) -> StdResult<Binary> {
        Ok(::alloc::vec::Vec::new().into())
    }
    #[cfg(not(target_arch = "wasm32"))]
    mod interface {
        use super::*;
        use cw_orch::prelude::*;
        impl<Chain> Uploadable for TestContract<Chain> {
            fn wrapper() -> <Mock as TxHandler>::ContractSource {
                Box::new(ContractWrapper::new_with_empty(execute, instantiate, query))
            }
        }
        impl<Chain> Uploadable for OrderedTestContract<Chain> {
            fn wrapper() -> <Mock as TxHandler>::ContractSource {
                Box::new(
                    ContractWrapper::new_with_empty(execute_ordered, instantiate, query),
                )
            }
        }
    }
}
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{
    to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError,
    StdResult, Uint128,
};
use serde::Serialize;
#[allow(clippy::derive_partial_eq_without_eq)]
#[serde(deny_unknown_fields, crate = "::cosmwasm_schema::serde")]
#[schemars(crate = "::cosmwasm_schema::schemars")]
pub struct InstantiateMsg {}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use ::cosmwasm_schema::serde as _serde;
    #[automatically_derived]
    impl ::cosmwasm_schema::serde::Serialize for InstantiateMsg {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> ::cosmwasm_schema::serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: ::cosmwasm_schema::serde::Serializer,
        {
            let __serde_state = _serde::Serializer::serialize_struct(
                __serializer,
                "InstantiateMsg",
                false as usize,
            )?;
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use ::cosmwasm_schema::serde as _serde;
    #[automatically_derived]
    impl<'de> ::cosmwasm_schema::serde::Deserialize<'de> for InstantiateMsg {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> ::cosmwasm_schema::serde::__private::Result<Self, __D::Error>
        where
            __D: ::cosmwasm_schema::serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {}
            #[doc(hidden)]
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "field identifier",
                    )
                }
                fn visit_u64<__E>(
                    self,
                    __value: u64,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Unsigned(__value),
                                    &"field index 0 <= i < 0",
                                ),
                            )
                        }
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::unknown_field(__value, FIELDS),
                            )
                        }
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        _ => {
                            let __value = &_serde::__private::from_utf8_lossy(__value);
                            _serde::__private::Err(
                                _serde::de::Error::unknown_field(__value, FIELDS),
                            )
                        }
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<InstantiateMsg>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = InstantiateMsg;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "struct InstantiateMsg",
                    )
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    _: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    _serde::__private::Ok(InstantiateMsg {})
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    _serde::__private::Option::map(
                        _serde::de::MapAccess::next_key::<__Field>(&mut __map)?,
                        |__impossible| match __impossible {},
                    );
                    _serde::__private::Ok(InstantiateMsg {})
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &[];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "InstantiateMsg",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<InstantiateMsg>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
#[automatically_derived]
#[allow(clippy::derive_partial_eq_without_eq)]
impl ::core::clone::Clone for InstantiateMsg {
    #[inline]
    fn clone(&self) -> InstantiateMsg {
        InstantiateMsg {}
    }
}
#[automatically_derived]
#[allow(clippy::derive_partial_eq_without_eq)]
impl ::core::fmt::Debug for InstantiateMsg {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(f, "InstantiateMsg")
    }
}
#[automatically_derived]
#[allow(clippy::derive_partial_eq_without_eq)]
impl ::core::marker::StructuralPartialEq for InstantiateMsg {}
#[automatically_derived]
#[allow(clippy::derive_partial_eq_without_eq)]
impl ::core::cmp::PartialEq for InstantiateMsg {
    #[inline]
    fn eq(&self, other: &InstantiateMsg) -> bool {
        true
    }
}
const _: () = {
    use ::cosmwasm_schema::schemars as schemars;
    #[automatically_derived]
    #[allow(unused_braces)]
    impl schemars::JsonSchema for InstantiateMsg {
        fn schema_name() -> std::string::String {
            "InstantiateMsg".to_owned()
        }
        fn schema_id() -> std::borrow::Cow<'static, str> {
            std::borrow::Cow::Borrowed("mock_contract::InstantiateMsg")
        }
        fn json_schema(
            gen: &mut schemars::gen::SchemaGenerator,
        ) -> schemars::schema::Schema {
            {
                let mut schema_object = schemars::schema::SchemaObject {
                    instance_type: Some(schemars::schema::InstanceType::Object.into()),
                    ..Default::default()
                };
                let object_validation = schema_object.object();
                object_validation.additional_properties = Some(Box::new(false.into()));
                schemars::schema::Schema::Object(schema_object)
            }
        }
    }
};
#[allow(clippy::derive_partial_eq_without_eq)]
#[serde(
    deny_unknown_fields,
    rename_all = "snake_case",
    crate = "::cosmwasm_schema::serde"
)]
#[schemars(crate = "::cosmwasm_schema::schemars")]
pub enum ExecuteMsg<T = String>
where
    T: Serialize,
{
    FirstMessage {},
    #[payable]
    SecondMessage {
        /// test doc-comment
        t: T,
    },
    /// test doc-comment
    ThirdMessage {
        /// test doc-comment
        t: T,
    },
    FourthMessage,
    #[payable]
    FifthMessage,
    SixthMessage(u64, String),
    #[payable]
    SeventhMessage(Uint128, String),
}
#[cfg(not(target_arch = "wasm32"))]
/// Automatically derived trait that allows you to call the variants of the message directly without the need to construct the struct yourself.
pub trait ExecuteMsgFns<
    Chain: ::cw_orch::core::environment::TxHandler,
    CwOrchExecuteMsgType,
    T,
>: ::cw_orch::core::contract::interface_traits::CwOrchExecute<
        Chain,
        ExecuteMsg = CwOrchExecuteMsgType,
    >
where
    T: Serialize,
    ExecuteMsg<T>: Into<CwOrchExecuteMsgType>,
{
    ///Automatically generated wrapper around ExecuteMsg::FirstMessage variant
    #[allow(clippy::too_many_arguments)]
    fn first_message(
        &self,
    ) -> Result<
        ::cw_orch::core::environment::TxResponse<Chain>,
        ::cw_orch::core::CwEnvError,
    > {
        let msg = ExecuteMsg::FirstMessage {};
        <Self as ::cw_orch::core::contract::interface_traits::CwOrchExecute<
            Chain,
        >>::execute(self, &msg.into(), None)
    }
    ///Automatically generated wrapper around ExecuteMsg::SecondMessage variant
    #[allow(clippy::too_many_arguments)]
    fn second_message(
        &self,
        t: impl Into<T>,
        coins: &[::cosmwasm_std::Coin],
    ) -> Result<
        ::cw_orch::core::environment::TxResponse<Chain>,
        ::cw_orch::core::CwEnvError,
    > {
        let msg = ExecuteMsg::SecondMessage {
            t: t.into(),
        };
        <Self as ::cw_orch::core::contract::interface_traits::CwOrchExecute<
            Chain,
        >>::execute(self, &msg.into(), Some(coins))
    }
    ///Automatically generated wrapper around ExecuteMsg::ThirdMessage variant
    #[allow(clippy::too_many_arguments)]
    fn third_message(
        &self,
        t: impl Into<T>,
    ) -> Result<
        ::cw_orch::core::environment::TxResponse<Chain>,
        ::cw_orch::core::CwEnvError,
    > {
        let msg = ExecuteMsg::ThirdMessage {
            t: t.into(),
        };
        <Self as ::cw_orch::core::contract::interface_traits::CwOrchExecute<
            Chain,
        >>::execute(self, &msg.into(), None)
    }
    ///Automatically generated wrapper around ExecuteMsg::FourthMessage variant
    fn fourth_message(
        &self,
    ) -> Result<
        ::cw_orch::core::environment::TxResponse<Chain>,
        ::cw_orch::core::CwEnvError,
    > {
        let msg = ExecuteMsg::FourthMessage;
        <Self as ::cw_orch::core::contract::interface_traits::CwOrchExecute<
            Chain,
        >>::execute(self, &msg.into(), None)
    }
    ///Automatically generated wrapper around ExecuteMsg::FifthMessage variant
    fn fifth_message(
        &self,
        coins: &[::cosmwasm_std::Coin],
    ) -> Result<
        ::cw_orch::core::environment::TxResponse<Chain>,
        ::cw_orch::core::CwEnvError,
    > {
        let msg = ExecuteMsg::FifthMessage;
        <Self as ::cw_orch::core::contract::interface_traits::CwOrchExecute<
            Chain,
        >>::execute(self, &msg.into(), Some(coins))
    }
    ///Automatically generated wrapper around ExecuteMsg::SixthMessage variant
    #[allow(clippy::too_many_arguments)]
    fn sixth_message(
        &self,
        arg0: impl Into<u64>,
        arg1: impl Into<String>,
    ) -> Result<
        ::cw_orch::core::environment::TxResponse<Chain>,
        ::cw_orch::core::CwEnvError,
    > {
        let msg = ExecuteMsg::SixthMessage(arg0.into(), arg1.into());
        <Self as ::cw_orch::core::contract::interface_traits::CwOrchExecute<
            Chain,
        >>::execute(self, &msg.into(), None)
    }
    ///Automatically generated wrapper around ExecuteMsg::SeventhMessage variant
    #[allow(clippy::too_many_arguments)]
    fn seventh_message(
        &self,
        arg0: impl Into<Uint128>,
        arg1: impl Into<String>,
        coins: &[::cosmwasm_std::Coin],
    ) -> Result<
        ::cw_orch::core::environment::TxResponse<Chain>,
        ::cw_orch::core::CwEnvError,
    > {
        let msg = ExecuteMsg::SeventhMessage(arg0.into(), arg1.into());
        <Self as ::cw_orch::core::contract::interface_traits::CwOrchExecute<
            Chain,
        >>::execute(self, &msg.into(), Some(coins))
    }
}
#[cfg(not(target_arch = "wasm32"))]
#[automatically_derived]
impl<
    SupportedContract,
    Chain: ::cw_orch::core::environment::TxHandler,
    CwOrchExecuteMsgType,
    T,
> ExecuteMsgFns<Chain, CwOrchExecuteMsgType, T> for SupportedContract
where
    T: Serialize,
    SupportedContract: ::cw_orch::core::contract::interface_traits::CwOrchExecute<
        Chain,
        ExecuteMsg = CwOrchExecuteMsgType,
    >,
    ExecuteMsg<T>: Into<CwOrchExecuteMsgType>,
{}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use ::cosmwasm_schema::serde as _serde;
    #[automatically_derived]
    impl<T> ::cosmwasm_schema::serde::Serialize for ExecuteMsg<T>
    where
        T: Serialize,
        T: _serde::Serialize,
    {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> ::cosmwasm_schema::serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: ::cosmwasm_schema::serde::Serializer,
        {
            match *self {
                ExecuteMsg::FirstMessage {} => {
                    let __serde_state = _serde::Serializer::serialize_struct_variant(
                        __serializer,
                        "ExecuteMsg",
                        0u32,
                        "first_message",
                        0,
                    )?;
                    _serde::ser::SerializeStructVariant::end(__serde_state)
                }
                ExecuteMsg::SecondMessage { ref t } => {
                    let mut __serde_state = _serde::Serializer::serialize_struct_variant(
                        __serializer,
                        "ExecuteMsg",
                        1u32,
                        "second_message",
                        0 + 1,
                    )?;
                    _serde::ser::SerializeStructVariant::serialize_field(
                        &mut __serde_state,
                        "t",
                        t,
                    )?;
                    _serde::ser::SerializeStructVariant::end(__serde_state)
                }
                ExecuteMsg::ThirdMessage { ref t } => {
                    let mut __serde_state = _serde::Serializer::serialize_struct_variant(
                        __serializer,
                        "ExecuteMsg",
                        2u32,
                        "third_message",
                        0 + 1,
                    )?;
                    _serde::ser::SerializeStructVariant::serialize_field(
                        &mut __serde_state,
                        "t",
                        t,
                    )?;
                    _serde::ser::SerializeStructVariant::end(__serde_state)
                }
                ExecuteMsg::FourthMessage => {
                    _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "ExecuteMsg",
                        3u32,
                        "fourth_message",
                    )
                }
                ExecuteMsg::FifthMessage => {
                    _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "ExecuteMsg",
                        4u32,
                        "fifth_message",
                    )
                }
                ExecuteMsg::SixthMessage(ref __field0, ref __field1) => {
                    let mut __serde_state = _serde::Serializer::serialize_tuple_variant(
                        __serializer,
                        "ExecuteMsg",
                        5u32,
                        "sixth_message",
                        0 + 1 + 1,
                    )?;
                    _serde::ser::SerializeTupleVariant::serialize_field(
                        &mut __serde_state,
                        __field0,
                    )?;
                    _serde::ser::SerializeTupleVariant::serialize_field(
                        &mut __serde_state,
                        __field1,
                    )?;
                    _serde::ser::SerializeTupleVariant::end(__serde_state)
                }
                ExecuteMsg::SeventhMessage(ref __field0, ref __field1) => {
                    let mut __serde_state = _serde::Serializer::serialize_tuple_variant(
                        __serializer,
                        "ExecuteMsg",
                        6u32,
                        "seventh_message",
                        0 + 1 + 1,
                    )?;
                    _serde::ser::SerializeTupleVariant::serialize_field(
                        &mut __serde_state,
                        __field0,
                    )?;
                    _serde::ser::SerializeTupleVariant::serialize_field(
                        &mut __serde_state,
                        __field1,
                    )?;
                    _serde::ser::SerializeTupleVariant::end(__serde_state)
                }
            }
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use ::cosmwasm_schema::serde as _serde;
    #[automatically_derived]
    impl<'de, T> ::cosmwasm_schema::serde::Deserialize<'de> for ExecuteMsg<T>
    where
        T: Serialize,
        T: _serde::Deserialize<'de>,
    {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> ::cosmwasm_schema::serde::__private::Result<Self, __D::Error>
        where
            __D: ::cosmwasm_schema::serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __field1,
                __field2,
                __field3,
                __field4,
                __field5,
                __field6,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "variant identifier",
                    )
                }
                fn visit_u64<__E>(
                    self,
                    __value: u64,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        2u64 => _serde::__private::Ok(__Field::__field2),
                        3u64 => _serde::__private::Ok(__Field::__field3),
                        4u64 => _serde::__private::Ok(__Field::__field4),
                        5u64 => _serde::__private::Ok(__Field::__field5),
                        6u64 => _serde::__private::Ok(__Field::__field6),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Unsigned(__value),
                                    &"variant index 0 <= i < 7",
                                ),
                            )
                        }
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "first_message" => _serde::__private::Ok(__Field::__field0),
                        "second_message" => _serde::__private::Ok(__Field::__field1),
                        "third_message" => _serde::__private::Ok(__Field::__field2),
                        "fourth_message" => _serde::__private::Ok(__Field::__field3),
                        "fifth_message" => _serde::__private::Ok(__Field::__field4),
                        "sixth_message" => _serde::__private::Ok(__Field::__field5),
                        "seventh_message" => _serde::__private::Ok(__Field::__field6),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::unknown_variant(__value, VARIANTS),
                            )
                        }
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"first_message" => _serde::__private::Ok(__Field::__field0),
                        b"second_message" => _serde::__private::Ok(__Field::__field1),
                        b"third_message" => _serde::__private::Ok(__Field::__field2),
                        b"fourth_message" => _serde::__private::Ok(__Field::__field3),
                        b"fifth_message" => _serde::__private::Ok(__Field::__field4),
                        b"sixth_message" => _serde::__private::Ok(__Field::__field5),
                        b"seventh_message" => _serde::__private::Ok(__Field::__field6),
                        _ => {
                            let __value = &_serde::__private::from_utf8_lossy(__value);
                            _serde::__private::Err(
                                _serde::de::Error::unknown_variant(__value, VARIANTS),
                            )
                        }
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de, T>
            where
                T: Serialize,
                T: _serde::Deserialize<'de>,
            {
                marker: _serde::__private::PhantomData<ExecuteMsg<T>>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de, T> _serde::de::Visitor<'de> for __Visitor<'de, T>
            where
                T: Serialize,
                T: _serde::Deserialize<'de>,
            {
                type Value = ExecuteMsg<T>;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "enum ExecuteMsg",
                    )
                }
                fn visit_enum<__A>(
                    self,
                    __data: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::EnumAccess<'de>,
                {
                    match _serde::de::EnumAccess::variant(__data)? {
                        (__Field::__field0, __variant) => {
                            #[allow(non_camel_case_types)]
                            #[doc(hidden)]
                            enum __Field {}
                            #[doc(hidden)]
                            struct __FieldVisitor;
                            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                                type Value = __Field;
                                fn expecting(
                                    &self,
                                    __formatter: &mut _serde::__private::Formatter,
                                ) -> _serde::__private::fmt::Result {
                                    _serde::__private::Formatter::write_str(
                                        __formatter,
                                        "field identifier",
                                    )
                                }
                                fn visit_u64<__E>(
                                    self,
                                    __value: u64,
                                ) -> _serde::__private::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        _ => {
                                            _serde::__private::Err(
                                                _serde::de::Error::invalid_value(
                                                    _serde::de::Unexpected::Unsigned(__value),
                                                    &"field index 0 <= i < 0",
                                                ),
                                            )
                                        }
                                    }
                                }
                                fn visit_str<__E>(
                                    self,
                                    __value: &str,
                                ) -> _serde::__private::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        _ => {
                                            _serde::__private::Err(
                                                _serde::de::Error::unknown_field(__value, FIELDS),
                                            )
                                        }
                                    }
                                }
                                fn visit_bytes<__E>(
                                    self,
                                    __value: &[u8],
                                ) -> _serde::__private::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        _ => {
                                            let __value = &_serde::__private::from_utf8_lossy(__value);
                                            _serde::__private::Err(
                                                _serde::de::Error::unknown_field(__value, FIELDS),
                                            )
                                        }
                                    }
                                }
                            }
                            impl<'de> _serde::Deserialize<'de> for __Field {
                                #[inline]
                                fn deserialize<__D>(
                                    __deserializer: __D,
                                ) -> _serde::__private::Result<Self, __D::Error>
                                where
                                    __D: _serde::Deserializer<'de>,
                                {
                                    _serde::Deserializer::deserialize_identifier(
                                        __deserializer,
                                        __FieldVisitor,
                                    )
                                }
                            }
                            #[doc(hidden)]
                            struct __Visitor<'de, T>
                            where
                                T: Serialize,
                                T: _serde::Deserialize<'de>,
                            {
                                marker: _serde::__private::PhantomData<ExecuteMsg<T>>,
                                lifetime: _serde::__private::PhantomData<&'de ()>,
                            }
                            impl<'de, T> _serde::de::Visitor<'de> for __Visitor<'de, T>
                            where
                                T: Serialize,
                                T: _serde::Deserialize<'de>,
                            {
                                type Value = ExecuteMsg<T>;
                                fn expecting(
                                    &self,
                                    __formatter: &mut _serde::__private::Formatter,
                                ) -> _serde::__private::fmt::Result {
                                    _serde::__private::Formatter::write_str(
                                        __formatter,
                                        "struct variant ExecuteMsg::FirstMessage",
                                    )
                                }
                                #[inline]
                                fn visit_seq<__A>(
                                    self,
                                    _: __A,
                                ) -> _serde::__private::Result<Self::Value, __A::Error>
                                where
                                    __A: _serde::de::SeqAccess<'de>,
                                {
                                    _serde::__private::Ok(ExecuteMsg::FirstMessage {})
                                }
                                #[inline]
                                fn visit_map<__A>(
                                    self,
                                    mut __map: __A,
                                ) -> _serde::__private::Result<Self::Value, __A::Error>
                                where
                                    __A: _serde::de::MapAccess<'de>,
                                {
                                    _serde::__private::Option::map(
                                        _serde::de::MapAccess::next_key::<__Field>(&mut __map)?,
                                        |__impossible| match __impossible {},
                                    );
                                    _serde::__private::Ok(ExecuteMsg::FirstMessage {})
                                }
                            }
                            #[doc(hidden)]
                            const FIELDS: &'static [&'static str] = &[];
                            _serde::de::VariantAccess::struct_variant(
                                __variant,
                                FIELDS,
                                __Visitor {
                                    marker: _serde::__private::PhantomData::<ExecuteMsg<T>>,
                                    lifetime: _serde::__private::PhantomData,
                                },
                            )
                        }
                        (__Field::__field1, __variant) => {
                            #[allow(non_camel_case_types)]
                            #[doc(hidden)]
                            enum __Field {
                                __field0,
                            }
                            #[doc(hidden)]
                            struct __FieldVisitor;
                            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                                type Value = __Field;
                                fn expecting(
                                    &self,
                                    __formatter: &mut _serde::__private::Formatter,
                                ) -> _serde::__private::fmt::Result {
                                    _serde::__private::Formatter::write_str(
                                        __formatter,
                                        "field identifier",
                                    )
                                }
                                fn visit_u64<__E>(
                                    self,
                                    __value: u64,
                                ) -> _serde::__private::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        0u64 => _serde::__private::Ok(__Field::__field0),
                                        _ => {
                                            _serde::__private::Err(
                                                _serde::de::Error::invalid_value(
                                                    _serde::de::Unexpected::Unsigned(__value),
                                                    &"field index 0 <= i < 1",
                                                ),
                                            )
                                        }
                                    }
                                }
                                fn visit_str<__E>(
                                    self,
                                    __value: &str,
                                ) -> _serde::__private::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        "t" => _serde::__private::Ok(__Field::__field0),
                                        _ => {
                                            _serde::__private::Err(
                                                _serde::de::Error::unknown_field(__value, FIELDS),
                                            )
                                        }
                                    }
                                }
                                fn visit_bytes<__E>(
                                    self,
                                    __value: &[u8],
                                ) -> _serde::__private::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        b"t" => _serde::__private::Ok(__Field::__field0),
                                        _ => {
                                            let __value = &_serde::__private::from_utf8_lossy(__value);
                                            _serde::__private::Err(
                                                _serde::de::Error::unknown_field(__value, FIELDS),
                                            )
                                        }
                                    }
                                }
                            }
                            impl<'de> _serde::Deserialize<'de> for __Field {
                                #[inline]
                                fn deserialize<__D>(
                                    __deserializer: __D,
                                ) -> _serde::__private::Result<Self, __D::Error>
                                where
                                    __D: _serde::Deserializer<'de>,
                                {
                                    _serde::Deserializer::deserialize_identifier(
                                        __deserializer,
                                        __FieldVisitor,
                                    )
                                }
                            }
                            #[doc(hidden)]
                            struct __Visitor<'de, T>
                            where
                                T: Serialize,
                                T: _serde::Deserialize<'de>,
                            {
                                marker: _serde::__private::PhantomData<ExecuteMsg<T>>,
                                lifetime: _serde::__private::PhantomData<&'de ()>,
                            }
                            impl<'de, T> _serde::de::Visitor<'de> for __Visitor<'de, T>
                            where
                                T: Serialize,
                                T: _serde::Deserialize<'de>,
                            {
                                type Value = ExecuteMsg<T>;
                                fn expecting(
                                    &self,
                                    __formatter: &mut _serde::__private::Formatter,
                                ) -> _serde::__private::fmt::Result {
                                    _serde::__private::Formatter::write_str(
                                        __formatter,
                                        "struct variant ExecuteMsg::SecondMessage",
                                    )
                                }
                                #[inline]
                                fn visit_seq<__A>(
                                    self,
                                    mut __seq: __A,
                                ) -> _serde::__private::Result<Self::Value, __A::Error>
                                where
                                    __A: _serde::de::SeqAccess<'de>,
                                {
                                    let __field0 = match _serde::de::SeqAccess::next_element::<
                                        T,
                                    >(&mut __seq)? {
                                        _serde::__private::Some(__value) => __value,
                                        _serde::__private::None => {
                                            return _serde::__private::Err(
                                                _serde::de::Error::invalid_length(
                                                    0usize,
                                                    &"struct variant ExecuteMsg::SecondMessage with 1 element",
                                                ),
                                            );
                                        }
                                    };
                                    _serde::__private::Ok(ExecuteMsg::SecondMessage {
                                        t: __field0,
                                    })
                                }
                                #[inline]
                                fn visit_map<__A>(
                                    self,
                                    mut __map: __A,
                                ) -> _serde::__private::Result<Self::Value, __A::Error>
                                where
                                    __A: _serde::de::MapAccess<'de>,
                                {
                                    let mut __field0: _serde::__private::Option<T> = _serde::__private::None;
                                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                        __Field,
                                    >(&mut __map)? {
                                        match __key {
                                            __Field::__field0 => {
                                                if _serde::__private::Option::is_some(&__field0) {
                                                    return _serde::__private::Err(
                                                        <__A::Error as _serde::de::Error>::duplicate_field("t"),
                                                    );
                                                }
                                                __field0 = _serde::__private::Some(
                                                    _serde::de::MapAccess::next_value::<T>(&mut __map)?,
                                                );
                                            }
                                        }
                                    }
                                    let __field0 = match __field0 {
                                        _serde::__private::Some(__field0) => __field0,
                                        _serde::__private::None => {
                                            _serde::__private::de::missing_field("t")?
                                        }
                                    };
                                    _serde::__private::Ok(ExecuteMsg::SecondMessage {
                                        t: __field0,
                                    })
                                }
                            }
                            #[doc(hidden)]
                            const FIELDS: &'static [&'static str] = &["t"];
                            _serde::de::VariantAccess::struct_variant(
                                __variant,
                                FIELDS,
                                __Visitor {
                                    marker: _serde::__private::PhantomData::<ExecuteMsg<T>>,
                                    lifetime: _serde::__private::PhantomData,
                                },
                            )
                        }
                        (__Field::__field2, __variant) => {
                            #[allow(non_camel_case_types)]
                            #[doc(hidden)]
                            enum __Field {
                                __field0,
                            }
                            #[doc(hidden)]
                            struct __FieldVisitor;
                            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                                type Value = __Field;
                                fn expecting(
                                    &self,
                                    __formatter: &mut _serde::__private::Formatter,
                                ) -> _serde::__private::fmt::Result {
                                    _serde::__private::Formatter::write_str(
                                        __formatter,
                                        "field identifier",
                                    )
                                }
                                fn visit_u64<__E>(
                                    self,
                                    __value: u64,
                                ) -> _serde::__private::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        0u64 => _serde::__private::Ok(__Field::__field0),
                                        _ => {
                                            _serde::__private::Err(
                                                _serde::de::Error::invalid_value(
                                                    _serde::de::Unexpected::Unsigned(__value),
                                                    &"field index 0 <= i < 1",
                                                ),
                                            )
                                        }
                                    }
                                }
                                fn visit_str<__E>(
                                    self,
                                    __value: &str,
                                ) -> _serde::__private::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        "t" => _serde::__private::Ok(__Field::__field0),
                                        _ => {
                                            _serde::__private::Err(
                                                _serde::de::Error::unknown_field(__value, FIELDS),
                                            )
                                        }
                                    }
                                }
                                fn visit_bytes<__E>(
                                    self,
                                    __value: &[u8],
                                ) -> _serde::__private::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        b"t" => _serde::__private::Ok(__Field::__field0),
                                        _ => {
                                            let __value = &_serde::__private::from_utf8_lossy(__value);
                                            _serde::__private::Err(
                                                _serde::de::Error::unknown_field(__value, FIELDS),
                                            )
                                        }
                                    }
                                }
                            }
                            impl<'de> _serde::Deserialize<'de> for __Field {
                                #[inline]
                                fn deserialize<__D>(
                                    __deserializer: __D,
                                ) -> _serde::__private::Result<Self, __D::Error>
                                where
                                    __D: _serde::Deserializer<'de>,
                                {
                                    _serde::Deserializer::deserialize_identifier(
                                        __deserializer,
                                        __FieldVisitor,
                                    )
                                }
                            }
                            #[doc(hidden)]
                            struct __Visitor<'de, T>
                            where
                                T: Serialize,
                                T: _serde::Deserialize<'de>,
                            {
                                marker: _serde::__private::PhantomData<ExecuteMsg<T>>,
                                lifetime: _serde::__private::PhantomData<&'de ()>,
                            }
                            impl<'de, T> _serde::de::Visitor<'de> for __Visitor<'de, T>
                            where
                                T: Serialize,
                                T: _serde::Deserialize<'de>,
                            {
                                type Value = ExecuteMsg<T>;
                                fn expecting(
                                    &self,
                                    __formatter: &mut _serde::__private::Formatter,
                                ) -> _serde::__private::fmt::Result {
                                    _serde::__private::Formatter::write_str(
                                        __formatter,
                                        "struct variant ExecuteMsg::ThirdMessage",
                                    )
                                }
                                #[inline]
                                fn visit_seq<__A>(
                                    self,
                                    mut __seq: __A,
                                ) -> _serde::__private::Result<Self::Value, __A::Error>
                                where
                                    __A: _serde::de::SeqAccess<'de>,
                                {
                                    let __field0 = match _serde::de::SeqAccess::next_element::<
                                        T,
                                    >(&mut __seq)? {
                                        _serde::__private::Some(__value) => __value,
                                        _serde::__private::None => {
                                            return _serde::__private::Err(
                                                _serde::de::Error::invalid_length(
                                                    0usize,
                                                    &"struct variant ExecuteMsg::ThirdMessage with 1 element",
                                                ),
                                            );
                                        }
                                    };
                                    _serde::__private::Ok(ExecuteMsg::ThirdMessage {
                                        t: __field0,
                                    })
                                }
                                #[inline]
                                fn visit_map<__A>(
                                    self,
                                    mut __map: __A,
                                ) -> _serde::__private::Result<Self::Value, __A::Error>
                                where
                                    __A: _serde::de::MapAccess<'de>,
                                {
                                    let mut __field0: _serde::__private::Option<T> = _serde::__private::None;
                                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                        __Field,
                                    >(&mut __map)? {
                                        match __key {
                                            __Field::__field0 => {
                                                if _serde::__private::Option::is_some(&__field0) {
                                                    return _serde::__private::Err(
                                                        <__A::Error as _serde::de::Error>::duplicate_field("t"),
                                                    );
                                                }
                                                __field0 = _serde::__private::Some(
                                                    _serde::de::MapAccess::next_value::<T>(&mut __map)?,
                                                );
                                            }
                                        }
                                    }
                                    let __field0 = match __field0 {
                                        _serde::__private::Some(__field0) => __field0,
                                        _serde::__private::None => {
                                            _serde::__private::de::missing_field("t")?
                                        }
                                    };
                                    _serde::__private::Ok(ExecuteMsg::ThirdMessage {
                                        t: __field0,
                                    })
                                }
                            }
                            #[doc(hidden)]
                            const FIELDS: &'static [&'static str] = &["t"];
                            _serde::de::VariantAccess::struct_variant(
                                __variant,
                                FIELDS,
                                __Visitor {
                                    marker: _serde::__private::PhantomData::<ExecuteMsg<T>>,
                                    lifetime: _serde::__private::PhantomData,
                                },
                            )
                        }
                        (__Field::__field3, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExecuteMsg::FourthMessage)
                        }
                        (__Field::__field4, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExecuteMsg::FifthMessage)
                        }
                        (__Field::__field5, __variant) => {
                            #[doc(hidden)]
                            struct __Visitor<'de, T>
                            where
                                T: Serialize,
                                T: _serde::Deserialize<'de>,
                            {
                                marker: _serde::__private::PhantomData<ExecuteMsg<T>>,
                                lifetime: _serde::__private::PhantomData<&'de ()>,
                            }
                            impl<'de, T> _serde::de::Visitor<'de> for __Visitor<'de, T>
                            where
                                T: Serialize,
                                T: _serde::Deserialize<'de>,
                            {
                                type Value = ExecuteMsg<T>;
                                fn expecting(
                                    &self,
                                    __formatter: &mut _serde::__private::Formatter,
                                ) -> _serde::__private::fmt::Result {
                                    _serde::__private::Formatter::write_str(
                                        __formatter,
                                        "tuple variant ExecuteMsg::SixthMessage",
                                    )
                                }
                                #[inline]
                                fn visit_seq<__A>(
                                    self,
                                    mut __seq: __A,
                                ) -> _serde::__private::Result<Self::Value, __A::Error>
                                where
                                    __A: _serde::de::SeqAccess<'de>,
                                {
                                    let __field0 = match _serde::de::SeqAccess::next_element::<
                                        u64,
                                    >(&mut __seq)? {
                                        _serde::__private::Some(__value) => __value,
                                        _serde::__private::None => {
                                            return _serde::__private::Err(
                                                _serde::de::Error::invalid_length(
                                                    0usize,
                                                    &"tuple variant ExecuteMsg::SixthMessage with 2 elements",
                                                ),
                                            );
                                        }
                                    };
                                    let __field1 = match _serde::de::SeqAccess::next_element::<
                                        String,
                                    >(&mut __seq)? {
                                        _serde::__private::Some(__value) => __value,
                                        _serde::__private::None => {
                                            return _serde::__private::Err(
                                                _serde::de::Error::invalid_length(
                                                    1usize,
                                                    &"tuple variant ExecuteMsg::SixthMessage with 2 elements",
                                                ),
                                            );
                                        }
                                    };
                                    _serde::__private::Ok(
                                        ExecuteMsg::SixthMessage(__field0, __field1),
                                    )
                                }
                            }
                            _serde::de::VariantAccess::tuple_variant(
                                __variant,
                                2usize,
                                __Visitor {
                                    marker: _serde::__private::PhantomData::<ExecuteMsg<T>>,
                                    lifetime: _serde::__private::PhantomData,
                                },
                            )
                        }
                        (__Field::__field6, __variant) => {
                            #[doc(hidden)]
                            struct __Visitor<'de, T>
                            where
                                T: Serialize,
                                T: _serde::Deserialize<'de>,
                            {
                                marker: _serde::__private::PhantomData<ExecuteMsg<T>>,
                                lifetime: _serde::__private::PhantomData<&'de ()>,
                            }
                            impl<'de, T> _serde::de::Visitor<'de> for __Visitor<'de, T>
                            where
                                T: Serialize,
                                T: _serde::Deserialize<'de>,
                            {
                                type Value = ExecuteMsg<T>;
                                fn expecting(
                                    &self,
                                    __formatter: &mut _serde::__private::Formatter,
                                ) -> _serde::__private::fmt::Result {
                                    _serde::__private::Formatter::write_str(
                                        __formatter,
                                        "tuple variant ExecuteMsg::SeventhMessage",
                                    )
                                }
                                #[inline]
                                fn visit_seq<__A>(
                                    self,
                                    mut __seq: __A,
                                ) -> _serde::__private::Result<Self::Value, __A::Error>
                                where
                                    __A: _serde::de::SeqAccess<'de>,
                                {
                                    let __field0 = match _serde::de::SeqAccess::next_element::<
                                        Uint128,
                                    >(&mut __seq)? {
                                        _serde::__private::Some(__value) => __value,
                                        _serde::__private::None => {
                                            return _serde::__private::Err(
                                                _serde::de::Error::invalid_length(
                                                    0usize,
                                                    &"tuple variant ExecuteMsg::SeventhMessage with 2 elements",
                                                ),
                                            );
                                        }
                                    };
                                    let __field1 = match _serde::de::SeqAccess::next_element::<
                                        String,
                                    >(&mut __seq)? {
                                        _serde::__private::Some(__value) => __value,
                                        _serde::__private::None => {
                                            return _serde::__private::Err(
                                                _serde::de::Error::invalid_length(
                                                    1usize,
                                                    &"tuple variant ExecuteMsg::SeventhMessage with 2 elements",
                                                ),
                                            );
                                        }
                                    };
                                    _serde::__private::Ok(
                                        ExecuteMsg::SeventhMessage(__field0, __field1),
                                    )
                                }
                            }
                            _serde::de::VariantAccess::tuple_variant(
                                __variant,
                                2usize,
                                __Visitor {
                                    marker: _serde::__private::PhantomData::<ExecuteMsg<T>>,
                                    lifetime: _serde::__private::PhantomData,
                                },
                            )
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &[
                "first_message",
                "second_message",
                "third_message",
                "fourth_message",
                "fifth_message",
                "sixth_message",
                "seventh_message",
            ];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "ExecuteMsg",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<ExecuteMsg<T>>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
#[automatically_derived]
#[allow(clippy::derive_partial_eq_without_eq)]
impl<T: ::core::clone::Clone> ::core::clone::Clone for ExecuteMsg<T>
where
    T: Serialize,
{
    #[inline]
    fn clone(&self) -> ExecuteMsg<T> {
        match self {
            ExecuteMsg::FirstMessage {} => ExecuteMsg::FirstMessage {},
            ExecuteMsg::SecondMessage { t: __self_0 } => {
                ExecuteMsg::SecondMessage {
                    t: ::core::clone::Clone::clone(__self_0),
                }
            }
            ExecuteMsg::ThirdMessage { t: __self_0 } => {
                ExecuteMsg::ThirdMessage {
                    t: ::core::clone::Clone::clone(__self_0),
                }
            }
            ExecuteMsg::FourthMessage => ExecuteMsg::FourthMessage,
            ExecuteMsg::FifthMessage => ExecuteMsg::FifthMessage,
            ExecuteMsg::SixthMessage(__self_0, __self_1) => {
                ExecuteMsg::SixthMessage(
                    ::core::clone::Clone::clone(__self_0),
                    ::core::clone::Clone::clone(__self_1),
                )
            }
            ExecuteMsg::SeventhMessage(__self_0, __self_1) => {
                ExecuteMsg::SeventhMessage(
                    ::core::clone::Clone::clone(__self_0),
                    ::core::clone::Clone::clone(__self_1),
                )
            }
        }
    }
}
#[automatically_derived]
#[allow(clippy::derive_partial_eq_without_eq)]
impl<T: ::core::fmt::Debug> ::core::fmt::Debug for ExecuteMsg<T>
where
    T: Serialize,
{
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            ExecuteMsg::FirstMessage {} => {
                ::core::fmt::Formatter::write_str(f, "FirstMessage")
            }
            ExecuteMsg::SecondMessage { t: __self_0 } => {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "SecondMessage",
                    "t",
                    &__self_0,
                )
            }
            ExecuteMsg::ThirdMessage { t: __self_0 } => {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "ThirdMessage",
                    "t",
                    &__self_0,
                )
            }
            ExecuteMsg::FourthMessage => {
                ::core::fmt::Formatter::write_str(f, "FourthMessage")
            }
            ExecuteMsg::FifthMessage => {
                ::core::fmt::Formatter::write_str(f, "FifthMessage")
            }
            ExecuteMsg::SixthMessage(__self_0, __self_1) => {
                ::core::fmt::Formatter::debug_tuple_field2_finish(
                    f,
                    "SixthMessage",
                    __self_0,
                    &__self_1,
                )
            }
            ExecuteMsg::SeventhMessage(__self_0, __self_1) => {
                ::core::fmt::Formatter::debug_tuple_field2_finish(
                    f,
                    "SeventhMessage",
                    __self_0,
                    &__self_1,
                )
            }
        }
    }
}
#[automatically_derived]
#[allow(clippy::derive_partial_eq_without_eq)]
impl<T> ::core::marker::StructuralPartialEq for ExecuteMsg<T>
where
    T: Serialize,
{}
#[automatically_derived]
#[allow(clippy::derive_partial_eq_without_eq)]
impl<T: ::core::cmp::PartialEq> ::core::cmp::PartialEq for ExecuteMsg<T>
where
    T: Serialize,
{
    #[inline]
    fn eq(&self, other: &ExecuteMsg<T>) -> bool {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        let __arg1_tag = ::core::intrinsics::discriminant_value(other);
        __self_tag == __arg1_tag
            && match (self, other) {
                (
                    ExecuteMsg::SecondMessage { t: __self_0 },
                    ExecuteMsg::SecondMessage { t: __arg1_0 },
                ) => *__self_0 == *__arg1_0,
                (
                    ExecuteMsg::ThirdMessage { t: __self_0 },
                    ExecuteMsg::ThirdMessage { t: __arg1_0 },
                ) => *__self_0 == *__arg1_0,
                (
                    ExecuteMsg::SixthMessage(__self_0, __self_1),
                    ExecuteMsg::SixthMessage(__arg1_0, __arg1_1),
                ) => *__self_0 == *__arg1_0 && *__self_1 == *__arg1_1,
                (
                    ExecuteMsg::SeventhMessage(__self_0, __self_1),
                    ExecuteMsg::SeventhMessage(__arg1_0, __arg1_1),
                ) => *__self_0 == *__arg1_0 && *__self_1 == *__arg1_1,
                _ => true,
            }
    }
}
const _: () = {
    use ::cosmwasm_schema::schemars as schemars;
    #[automatically_derived]
    #[allow(unused_braces)]
    impl<T: schemars::JsonSchema> schemars::JsonSchema for ExecuteMsg<T>
    where
        T: Serialize,
    {
        fn schema_name() -> std::string::String {
            {
                let res = ::alloc::fmt::format(
                    format_args!("ExecuteMsg_for_{0}", T::schema_name()),
                );
                res
            }
        }
        fn schema_id() -> std::borrow::Cow<'static, str> {
            std::borrow::Cow::Owned({
                let res = ::alloc::fmt::format(
                    format_args!("mock_contract::ExecuteMsg_for_{0}", T::schema_id()),
                );
                res
            })
        }
        fn json_schema(
            gen: &mut schemars::gen::SchemaGenerator,
        ) -> schemars::schema::Schema {
            schemars::schema::Schema::Object(schemars::schema::SchemaObject {
                subschemas: Some(
                    Box::new(schemars::schema::SubschemaValidation {
                        one_of: Some(
                            <[_]>::into_vec(
                                #[rustc_box]
                                ::alloc::boxed::Box::new([
                                    schemars::schema::Schema::Object(schemars::schema::SchemaObject {
                                        instance_type: Some(
                                            schemars::schema::InstanceType::String.into(),
                                        ),
                                        enum_values: Some(
                                            <[_]>::into_vec(
                                                #[rustc_box]
                                                ::alloc::boxed::Box::new([
                                                    "fourth_message".into(),
                                                    "fifth_message".into(),
                                                ]),
                                            ),
                                        ),
                                        ..Default::default()
                                    }),
                                    schemars::_private::new_externally_tagged_enum(
                                        "first_message",
                                        {
                                            let mut schema_object = schemars::schema::SchemaObject {
                                                instance_type: Some(
                                                    schemars::schema::InstanceType::Object.into(),
                                                ),
                                                ..Default::default()
                                            };
                                            let object_validation = schema_object.object();
                                            object_validation
                                                .additional_properties = Some(Box::new(false.into()));
                                            schemars::schema::Schema::Object(schema_object)
                                        },
                                    ),
                                    schemars::_private::new_externally_tagged_enum(
                                        "second_message",
                                        {
                                            let mut schema_object = schemars::schema::SchemaObject {
                                                instance_type: Some(
                                                    schemars::schema::InstanceType::Object.into(),
                                                ),
                                                ..Default::default()
                                            };
                                            let object_validation = schema_object.object();
                                            object_validation
                                                .additional_properties = Some(Box::new(false.into()));
                                            {
                                                schemars::_private::insert_object_property::<
                                                    T,
                                                >(
                                                    object_validation,
                                                    "t",
                                                    false,
                                                    false,
                                                    schemars::_private::metadata::add_description(
                                                        gen.subschema_for::<T>(),
                                                        "test doc-comment",
                                                    ),
                                                );
                                            }
                                            schemars::schema::Schema::Object(schema_object)
                                        },
                                    ),
                                    schemars::_private::metadata::add_description(
                                        schemars::_private::new_externally_tagged_enum(
                                            "third_message",
                                            {
                                                let mut schema_object = schemars::schema::SchemaObject {
                                                    instance_type: Some(
                                                        schemars::schema::InstanceType::Object.into(),
                                                    ),
                                                    ..Default::default()
                                                };
                                                let object_validation = schema_object.object();
                                                object_validation
                                                    .additional_properties = Some(Box::new(false.into()));
                                                {
                                                    schemars::_private::insert_object_property::<
                                                        T,
                                                    >(
                                                        object_validation,
                                                        "t",
                                                        false,
                                                        false,
                                                        schemars::_private::metadata::add_description(
                                                            gen.subschema_for::<T>(),
                                                            "test doc-comment",
                                                        ),
                                                    );
                                                }
                                                schemars::schema::Schema::Object(schema_object)
                                            },
                                        ),
                                        "test doc-comment",
                                    ),
                                    schemars::_private::new_externally_tagged_enum(
                                        "sixth_message",
                                        schemars::schema::Schema::Object(schemars::schema::SchemaObject {
                                            instance_type: Some(
                                                schemars::schema::InstanceType::Array.into(),
                                            ),
                                            array: Some(
                                                Box::new(schemars::schema::ArrayValidation {
                                                    items: Some(
                                                        <[_]>::into_vec(
                                                                #[rustc_box]
                                                                ::alloc::boxed::Box::new([
                                                                    gen.subschema_for::<u64>(),
                                                                    gen.subschema_for::<String>(),
                                                                ]),
                                                            )
                                                            .into(),
                                                    ),
                                                    max_items: Some(2u32),
                                                    min_items: Some(2u32),
                                                    ..Default::default()
                                                }),
                                            ),
                                            ..Default::default()
                                        }),
                                    ),
                                    schemars::_private::new_externally_tagged_enum(
                                        "seventh_message",
                                        schemars::schema::Schema::Object(schemars::schema::SchemaObject {
                                            instance_type: Some(
                                                schemars::schema::InstanceType::Array.into(),
                                            ),
                                            array: Some(
                                                Box::new(schemars::schema::ArrayValidation {
                                                    items: Some(
                                                        <[_]>::into_vec(
                                                                #[rustc_box]
                                                                ::alloc::boxed::Box::new([
                                                                    gen.subschema_for::<Uint128>(),
                                                                    gen.subschema_for::<String>(),
                                                                ]),
                                                            )
                                                            .into(),
                                                    ),
                                                    max_items: Some(2u32),
                                                    min_items: Some(2u32),
                                                    ..Default::default()
                                                }),
                                            ),
                                            ..Default::default()
                                        }),
                                    ),
                                ]),
                            ),
                        ),
                        ..Default::default()
                    }),
                ),
                ..Default::default()
            })
        }
    }
};
#[allow(clippy::derive_partial_eq_without_eq)]
#[serde(
    deny_unknown_fields,
    rename_all = "snake_case",
    crate = "::cosmwasm_schema::serde"
)]
#[schemars(crate = "::cosmwasm_schema::schemars")]
pub enum QueryMsg<T = String>
where
    T: Serialize,
{
    #[returns(String)]
    /// test-doc-comment
    FirstQuery {},
    #[returns(String)]
    SecondQuery {
        /// test doc-comment
        t: T,
    },
    #[returns(String)]
    ThirdQuery {
        /// test doc-comment
        t: T,
    },
    #[returns(u64)]
    FourthQuery(u64, String),
}
#[cfg(not(target_arch = "wasm32"))]
/// Automatically derived trait that allows you to call the variants of the message directly without the need to construct the struct yourself.
pub trait QueryMsgFns<
    Chain: ::cw_orch::core::environment::QueryHandler
        + ::cw_orch::core::environment::ChainState,
    CwOrchQueryMsgType,
    T,
>: ::cw_orch::core::contract::interface_traits::CwOrchQuery<
        Chain,
        QueryMsg = CwOrchQueryMsgType,
    >
where
    T: Serialize,
    QueryMsg<T>: Into<CwOrchQueryMsgType>,
{
    ///Automatically generated wrapper around QueryMsg::FirstQuery variant
    #[allow(clippy::too_many_arguments)]
    fn first_query(&self) -> Result<String, ::cw_orch::core::CwEnvError> {
        let msg = QueryMsg::FirstQuery {};
        <Self as ::cw_orch::core::contract::interface_traits::CwOrchQuery<
            Chain,
        >>::query(self, &msg.into())
    }
    ///Automatically generated wrapper around QueryMsg::SecondQuery variant
    #[allow(clippy::too_many_arguments)]
    fn second_query(
        &self,
        t: impl Into<T>,
    ) -> Result<String, ::cw_orch::core::CwEnvError> {
        let msg = QueryMsg::SecondQuery {
            t: t.into(),
        };
        <Self as ::cw_orch::core::contract::interface_traits::CwOrchQuery<
            Chain,
        >>::query(self, &msg.into())
    }
    ///Automatically generated wrapper around QueryMsg::ThirdQuery variant
    #[allow(clippy::too_many_arguments)]
    fn third_query(
        &self,
        t: impl Into<T>,
    ) -> Result<String, ::cw_orch::core::CwEnvError> {
        let msg = QueryMsg::ThirdQuery {
            t: t.into(),
        };
        <Self as ::cw_orch::core::contract::interface_traits::CwOrchQuery<
            Chain,
        >>::query(self, &msg.into())
    }
    ///Automatically generated wrapper around QueryMsg::FourthQuery variant
    #[allow(clippy::too_many_arguments)]
    fn fourth_query(
        &self,
        arg0: impl Into<u64>,
        arg1: impl Into<String>,
    ) -> Result<u64, ::cw_orch::core::CwEnvError> {
        let msg = QueryMsg::FourthQuery(arg0.into(), arg1.into());
        <Self as ::cw_orch::core::contract::interface_traits::CwOrchQuery<
            Chain,
        >>::query(self, &msg.into())
    }
}
#[cfg(not(target_arch = "wasm32"))]
#[automatically_derived]
impl<
    SupportedContract,
    Chain: ::cw_orch::core::environment::QueryHandler
        + ::cw_orch::core::environment::ChainState,
    CwOrchQueryMsgType,
    T,
> QueryMsgFns<Chain, CwOrchQueryMsgType, T> for SupportedContract
where
    T: Serialize,
    SupportedContract: ::cw_orch::core::contract::interface_traits::CwOrchQuery<
        Chain,
        QueryMsg = CwOrchQueryMsgType,
    >,
    QueryMsg<T>: Into<CwOrchQueryMsgType>,
{}
#[automatically_derived]
#[cfg(not(target_arch = "wasm32"))]
impl<T: ::cosmwasm_schema::schemars::JsonSchema> ::cosmwasm_schema::QueryResponses
for QueryMsg<T>
where
    T: Serialize,
{
    fn response_schemas_impl() -> ::std::collections::BTreeMap<
        String,
        ::cosmwasm_schema::schemars::schema::RootSchema,
    > {
        ::std::collections::BTreeMap::from([
            (
                "first_query".to_string(),
                ::cosmwasm_schema::schemars::gen::SchemaGenerator::new(
                        ::cosmwasm_schema::schemars::gen::SchemaSettings::draft07(),
                    )
                    .into_root_schema_for::<String>(),
            ),
            (
                "second_query".to_string(),
                ::cosmwasm_schema::schemars::gen::SchemaGenerator::new(
                        ::cosmwasm_schema::schemars::gen::SchemaSettings::draft07(),
                    )
                    .into_root_schema_for::<String>(),
            ),
            (
                "third_query".to_string(),
                ::cosmwasm_schema::schemars::gen::SchemaGenerator::new(
                        ::cosmwasm_schema::schemars::gen::SchemaSettings::draft07(),
                    )
                    .into_root_schema_for::<String>(),
            ),
            (
                "fourth_query".to_string(),
                ::cosmwasm_schema::schemars::gen::SchemaGenerator::new(
                        ::cosmwasm_schema::schemars::gen::SchemaSettings::draft07(),
                    )
                    .into_root_schema_for::<u64>(),
            ),
        ])
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use ::cosmwasm_schema::serde as _serde;
    #[automatically_derived]
    impl<T> ::cosmwasm_schema::serde::Serialize for QueryMsg<T>
    where
        T: Serialize,
        T: _serde::Serialize,
    {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> ::cosmwasm_schema::serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: ::cosmwasm_schema::serde::Serializer,
        {
            match *self {
                QueryMsg::FirstQuery {} => {
                    let __serde_state = _serde::Serializer::serialize_struct_variant(
                        __serializer,
                        "QueryMsg",
                        0u32,
                        "first_query",
                        0,
                    )?;
                    _serde::ser::SerializeStructVariant::end(__serde_state)
                }
                QueryMsg::SecondQuery { ref t } => {
                    let mut __serde_state = _serde::Serializer::serialize_struct_variant(
                        __serializer,
                        "QueryMsg",
                        1u32,
                        "second_query",
                        0 + 1,
                    )?;
                    _serde::ser::SerializeStructVariant::serialize_field(
                        &mut __serde_state,
                        "t",
                        t,
                    )?;
                    _serde::ser::SerializeStructVariant::end(__serde_state)
                }
                QueryMsg::ThirdQuery { ref t } => {
                    let mut __serde_state = _serde::Serializer::serialize_struct_variant(
                        __serializer,
                        "QueryMsg",
                        2u32,
                        "third_query",
                        0 + 1,
                    )?;
                    _serde::ser::SerializeStructVariant::serialize_field(
                        &mut __serde_state,
                        "t",
                        t,
                    )?;
                    _serde::ser::SerializeStructVariant::end(__serde_state)
                }
                QueryMsg::FourthQuery(ref __field0, ref __field1) => {
                    let mut __serde_state = _serde::Serializer::serialize_tuple_variant(
                        __serializer,
                        "QueryMsg",
                        3u32,
                        "fourth_query",
                        0 + 1 + 1,
                    )?;
                    _serde::ser::SerializeTupleVariant::serialize_field(
                        &mut __serde_state,
                        __field0,
                    )?;
                    _serde::ser::SerializeTupleVariant::serialize_field(
                        &mut __serde_state,
                        __field1,
                    )?;
                    _serde::ser::SerializeTupleVariant::end(__serde_state)
                }
            }
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use ::cosmwasm_schema::serde as _serde;
    #[automatically_derived]
    impl<'de, T> ::cosmwasm_schema::serde::Deserialize<'de> for QueryMsg<T>
    where
        T: Serialize,
        T: _serde::Deserialize<'de>,
    {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> ::cosmwasm_schema::serde::__private::Result<Self, __D::Error>
        where
            __D: ::cosmwasm_schema::serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __field1,
                __field2,
                __field3,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "variant identifier",
                    )
                }
                fn visit_u64<__E>(
                    self,
                    __value: u64,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        2u64 => _serde::__private::Ok(__Field::__field2),
                        3u64 => _serde::__private::Ok(__Field::__field3),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Unsigned(__value),
                                    &"variant index 0 <= i < 4",
                                ),
                            )
                        }
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "first_query" => _serde::__private::Ok(__Field::__field0),
                        "second_query" => _serde::__private::Ok(__Field::__field1),
                        "third_query" => _serde::__private::Ok(__Field::__field2),
                        "fourth_query" => _serde::__private::Ok(__Field::__field3),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::unknown_variant(__value, VARIANTS),
                            )
                        }
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"first_query" => _serde::__private::Ok(__Field::__field0),
                        b"second_query" => _serde::__private::Ok(__Field::__field1),
                        b"third_query" => _serde::__private::Ok(__Field::__field2),
                        b"fourth_query" => _serde::__private::Ok(__Field::__field3),
                        _ => {
                            let __value = &_serde::__private::from_utf8_lossy(__value);
                            _serde::__private::Err(
                                _serde::de::Error::unknown_variant(__value, VARIANTS),
                            )
                        }
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de, T>
            where
                T: Serialize,
                T: _serde::Deserialize<'de>,
            {
                marker: _serde::__private::PhantomData<QueryMsg<T>>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de, T> _serde::de::Visitor<'de> for __Visitor<'de, T>
            where
                T: Serialize,
                T: _serde::Deserialize<'de>,
            {
                type Value = QueryMsg<T>;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "enum QueryMsg")
                }
                fn visit_enum<__A>(
                    self,
                    __data: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::EnumAccess<'de>,
                {
                    match _serde::de::EnumAccess::variant(__data)? {
                        (__Field::__field0, __variant) => {
                            #[allow(non_camel_case_types)]
                            #[doc(hidden)]
                            enum __Field {}
                            #[doc(hidden)]
                            struct __FieldVisitor;
                            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                                type Value = __Field;
                                fn expecting(
                                    &self,
                                    __formatter: &mut _serde::__private::Formatter,
                                ) -> _serde::__private::fmt::Result {
                                    _serde::__private::Formatter::write_str(
                                        __formatter,
                                        "field identifier",
                                    )
                                }
                                fn visit_u64<__E>(
                                    self,
                                    __value: u64,
                                ) -> _serde::__private::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        _ => {
                                            _serde::__private::Err(
                                                _serde::de::Error::invalid_value(
                                                    _serde::de::Unexpected::Unsigned(__value),
                                                    &"field index 0 <= i < 0",
                                                ),
                                            )
                                        }
                                    }
                                }
                                fn visit_str<__E>(
                                    self,
                                    __value: &str,
                                ) -> _serde::__private::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        _ => {
                                            _serde::__private::Err(
                                                _serde::de::Error::unknown_field(__value, FIELDS),
                                            )
                                        }
                                    }
                                }
                                fn visit_bytes<__E>(
                                    self,
                                    __value: &[u8],
                                ) -> _serde::__private::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        _ => {
                                            let __value = &_serde::__private::from_utf8_lossy(__value);
                                            _serde::__private::Err(
                                                _serde::de::Error::unknown_field(__value, FIELDS),
                                            )
                                        }
                                    }
                                }
                            }
                            impl<'de> _serde::Deserialize<'de> for __Field {
                                #[inline]
                                fn deserialize<__D>(
                                    __deserializer: __D,
                                ) -> _serde::__private::Result<Self, __D::Error>
                                where
                                    __D: _serde::Deserializer<'de>,
                                {
                                    _serde::Deserializer::deserialize_identifier(
                                        __deserializer,
                                        __FieldVisitor,
                                    )
                                }
                            }
                            #[doc(hidden)]
                            struct __Visitor<'de, T>
                            where
                                T: Serialize,
                                T: _serde::Deserialize<'de>,
                            {
                                marker: _serde::__private::PhantomData<QueryMsg<T>>,
                                lifetime: _serde::__private::PhantomData<&'de ()>,
                            }
                            impl<'de, T> _serde::de::Visitor<'de> for __Visitor<'de, T>
                            where
                                T: Serialize,
                                T: _serde::Deserialize<'de>,
                            {
                                type Value = QueryMsg<T>;
                                fn expecting(
                                    &self,
                                    __formatter: &mut _serde::__private::Formatter,
                                ) -> _serde::__private::fmt::Result {
                                    _serde::__private::Formatter::write_str(
                                        __formatter,
                                        "struct variant QueryMsg::FirstQuery",
                                    )
                                }
                                #[inline]
                                fn visit_seq<__A>(
                                    self,
                                    _: __A,
                                ) -> _serde::__private::Result<Self::Value, __A::Error>
                                where
                                    __A: _serde::de::SeqAccess<'de>,
                                {
                                    _serde::__private::Ok(QueryMsg::FirstQuery {})
                                }
                                #[inline]
                                fn visit_map<__A>(
                                    self,
                                    mut __map: __A,
                                ) -> _serde::__private::Result<Self::Value, __A::Error>
                                where
                                    __A: _serde::de::MapAccess<'de>,
                                {
                                    _serde::__private::Option::map(
                                        _serde::de::MapAccess::next_key::<__Field>(&mut __map)?,
                                        |__impossible| match __impossible {},
                                    );
                                    _serde::__private::Ok(QueryMsg::FirstQuery {})
                                }
                            }
                            #[doc(hidden)]
                            const FIELDS: &'static [&'static str] = &[];
                            _serde::de::VariantAccess::struct_variant(
                                __variant,
                                FIELDS,
                                __Visitor {
                                    marker: _serde::__private::PhantomData::<QueryMsg<T>>,
                                    lifetime: _serde::__private::PhantomData,
                                },
                            )
                        }
                        (__Field::__field1, __variant) => {
                            #[allow(non_camel_case_types)]
                            #[doc(hidden)]
                            enum __Field {
                                __field0,
                            }
                            #[doc(hidden)]
                            struct __FieldVisitor;
                            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                                type Value = __Field;
                                fn expecting(
                                    &self,
                                    __formatter: &mut _serde::__private::Formatter,
                                ) -> _serde::__private::fmt::Result {
                                    _serde::__private::Formatter::write_str(
                                        __formatter,
                                        "field identifier",
                                    )
                                }
                                fn visit_u64<__E>(
                                    self,
                                    __value: u64,
                                ) -> _serde::__private::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        0u64 => _serde::__private::Ok(__Field::__field0),
                                        _ => {
                                            _serde::__private::Err(
                                                _serde::de::Error::invalid_value(
                                                    _serde::de::Unexpected::Unsigned(__value),
                                                    &"field index 0 <= i < 1",
                                                ),
                                            )
                                        }
                                    }
                                }
                                fn visit_str<__E>(
                                    self,
                                    __value: &str,
                                ) -> _serde::__private::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        "t" => _serde::__private::Ok(__Field::__field0),
                                        _ => {
                                            _serde::__private::Err(
                                                _serde::de::Error::unknown_field(__value, FIELDS),
                                            )
                                        }
                                    }
                                }
                                fn visit_bytes<__E>(
                                    self,
                                    __value: &[u8],
                                ) -> _serde::__private::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        b"t" => _serde::__private::Ok(__Field::__field0),
                                        _ => {
                                            let __value = &_serde::__private::from_utf8_lossy(__value);
                                            _serde::__private::Err(
                                                _serde::de::Error::unknown_field(__value, FIELDS),
                                            )
                                        }
                                    }
                                }
                            }
                            impl<'de> _serde::Deserialize<'de> for __Field {
                                #[inline]
                                fn deserialize<__D>(
                                    __deserializer: __D,
                                ) -> _serde::__private::Result<Self, __D::Error>
                                where
                                    __D: _serde::Deserializer<'de>,
                                {
                                    _serde::Deserializer::deserialize_identifier(
                                        __deserializer,
                                        __FieldVisitor,
                                    )
                                }
                            }
                            #[doc(hidden)]
                            struct __Visitor<'de, T>
                            where
                                T: Serialize,
                                T: _serde::Deserialize<'de>,
                            {
                                marker: _serde::__private::PhantomData<QueryMsg<T>>,
                                lifetime: _serde::__private::PhantomData<&'de ()>,
                            }
                            impl<'de, T> _serde::de::Visitor<'de> for __Visitor<'de, T>
                            where
                                T: Serialize,
                                T: _serde::Deserialize<'de>,
                            {
                                type Value = QueryMsg<T>;
                                fn expecting(
                                    &self,
                                    __formatter: &mut _serde::__private::Formatter,
                                ) -> _serde::__private::fmt::Result {
                                    _serde::__private::Formatter::write_str(
                                        __formatter,
                                        "struct variant QueryMsg::SecondQuery",
                                    )
                                }
                                #[inline]
                                fn visit_seq<__A>(
                                    self,
                                    mut __seq: __A,
                                ) -> _serde::__private::Result<Self::Value, __A::Error>
                                where
                                    __A: _serde::de::SeqAccess<'de>,
                                {
                                    let __field0 = match _serde::de::SeqAccess::next_element::<
                                        T,
                                    >(&mut __seq)? {
                                        _serde::__private::Some(__value) => __value,
                                        _serde::__private::None => {
                                            return _serde::__private::Err(
                                                _serde::de::Error::invalid_length(
                                                    0usize,
                                                    &"struct variant QueryMsg::SecondQuery with 1 element",
                                                ),
                                            );
                                        }
                                    };
                                    _serde::__private::Ok(QueryMsg::SecondQuery {
                                        t: __field0,
                                    })
                                }
                                #[inline]
                                fn visit_map<__A>(
                                    self,
                                    mut __map: __A,
                                ) -> _serde::__private::Result<Self::Value, __A::Error>
                                where
                                    __A: _serde::de::MapAccess<'de>,
                                {
                                    let mut __field0: _serde::__private::Option<T> = _serde::__private::None;
                                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                        __Field,
                                    >(&mut __map)? {
                                        match __key {
                                            __Field::__field0 => {
                                                if _serde::__private::Option::is_some(&__field0) {
                                                    return _serde::__private::Err(
                                                        <__A::Error as _serde::de::Error>::duplicate_field("t"),
                                                    );
                                                }
                                                __field0 = _serde::__private::Some(
                                                    _serde::de::MapAccess::next_value::<T>(&mut __map)?,
                                                );
                                            }
                                        }
                                    }
                                    let __field0 = match __field0 {
                                        _serde::__private::Some(__field0) => __field0,
                                        _serde::__private::None => {
                                            _serde::__private::de::missing_field("t")?
                                        }
                                    };
                                    _serde::__private::Ok(QueryMsg::SecondQuery {
                                        t: __field0,
                                    })
                                }
                            }
                            #[doc(hidden)]
                            const FIELDS: &'static [&'static str] = &["t"];
                            _serde::de::VariantAccess::struct_variant(
                                __variant,
                                FIELDS,
                                __Visitor {
                                    marker: _serde::__private::PhantomData::<QueryMsg<T>>,
                                    lifetime: _serde::__private::PhantomData,
                                },
                            )
                        }
                        (__Field::__field2, __variant) => {
                            #[allow(non_camel_case_types)]
                            #[doc(hidden)]
                            enum __Field {
                                __field0,
                            }
                            #[doc(hidden)]
                            struct __FieldVisitor;
                            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                                type Value = __Field;
                                fn expecting(
                                    &self,
                                    __formatter: &mut _serde::__private::Formatter,
                                ) -> _serde::__private::fmt::Result {
                                    _serde::__private::Formatter::write_str(
                                        __formatter,
                                        "field identifier",
                                    )
                                }
                                fn visit_u64<__E>(
                                    self,
                                    __value: u64,
                                ) -> _serde::__private::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        0u64 => _serde::__private::Ok(__Field::__field0),
                                        _ => {
                                            _serde::__private::Err(
                                                _serde::de::Error::invalid_value(
                                                    _serde::de::Unexpected::Unsigned(__value),
                                                    &"field index 0 <= i < 1",
                                                ),
                                            )
                                        }
                                    }
                                }
                                fn visit_str<__E>(
                                    self,
                                    __value: &str,
                                ) -> _serde::__private::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        "t" => _serde::__private::Ok(__Field::__field0),
                                        _ => {
                                            _serde::__private::Err(
                                                _serde::de::Error::unknown_field(__value, FIELDS),
                                            )
                                        }
                                    }
                                }
                                fn visit_bytes<__E>(
                                    self,
                                    __value: &[u8],
                                ) -> _serde::__private::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        b"t" => _serde::__private::Ok(__Field::__field0),
                                        _ => {
                                            let __value = &_serde::__private::from_utf8_lossy(__value);
                                            _serde::__private::Err(
                                                _serde::de::Error::unknown_field(__value, FIELDS),
                                            )
                                        }
                                    }
                                }
                            }
                            impl<'de> _serde::Deserialize<'de> for __Field {
                                #[inline]
                                fn deserialize<__D>(
                                    __deserializer: __D,
                                ) -> _serde::__private::Result<Self, __D::Error>
                                where
                                    __D: _serde::Deserializer<'de>,
                                {
                                    _serde::Deserializer::deserialize_identifier(
                                        __deserializer,
                                        __FieldVisitor,
                                    )
                                }
                            }
                            #[doc(hidden)]
                            struct __Visitor<'de, T>
                            where
                                T: Serialize,
                                T: _serde::Deserialize<'de>,
                            {
                                marker: _serde::__private::PhantomData<QueryMsg<T>>,
                                lifetime: _serde::__private::PhantomData<&'de ()>,
                            }
                            impl<'de, T> _serde::de::Visitor<'de> for __Visitor<'de, T>
                            where
                                T: Serialize,
                                T: _serde::Deserialize<'de>,
                            {
                                type Value = QueryMsg<T>;
                                fn expecting(
                                    &self,
                                    __formatter: &mut _serde::__private::Formatter,
                                ) -> _serde::__private::fmt::Result {
                                    _serde::__private::Formatter::write_str(
                                        __formatter,
                                        "struct variant QueryMsg::ThirdQuery",
                                    )
                                }
                                #[inline]
                                fn visit_seq<__A>(
                                    self,
                                    mut __seq: __A,
                                ) -> _serde::__private::Result<Self::Value, __A::Error>
                                where
                                    __A: _serde::de::SeqAccess<'de>,
                                {
                                    let __field0 = match _serde::de::SeqAccess::next_element::<
                                        T,
                                    >(&mut __seq)? {
                                        _serde::__private::Some(__value) => __value,
                                        _serde::__private::None => {
                                            return _serde::__private::Err(
                                                _serde::de::Error::invalid_length(
                                                    0usize,
                                                    &"struct variant QueryMsg::ThirdQuery with 1 element",
                                                ),
                                            );
                                        }
                                    };
                                    _serde::__private::Ok(QueryMsg::ThirdQuery {
                                        t: __field0,
                                    })
                                }
                                #[inline]
                                fn visit_map<__A>(
                                    self,
                                    mut __map: __A,
                                ) -> _serde::__private::Result<Self::Value, __A::Error>
                                where
                                    __A: _serde::de::MapAccess<'de>,
                                {
                                    let mut __field0: _serde::__private::Option<T> = _serde::__private::None;
                                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                        __Field,
                                    >(&mut __map)? {
                                        match __key {
                                            __Field::__field0 => {
                                                if _serde::__private::Option::is_some(&__field0) {
                                                    return _serde::__private::Err(
                                                        <__A::Error as _serde::de::Error>::duplicate_field("t"),
                                                    );
                                                }
                                                __field0 = _serde::__private::Some(
                                                    _serde::de::MapAccess::next_value::<T>(&mut __map)?,
                                                );
                                            }
                                        }
                                    }
                                    let __field0 = match __field0 {
                                        _serde::__private::Some(__field0) => __field0,
                                        _serde::__private::None => {
                                            _serde::__private::de::missing_field("t")?
                                        }
                                    };
                                    _serde::__private::Ok(QueryMsg::ThirdQuery {
                                        t: __field0,
                                    })
                                }
                            }
                            #[doc(hidden)]
                            const FIELDS: &'static [&'static str] = &["t"];
                            _serde::de::VariantAccess::struct_variant(
                                __variant,
                                FIELDS,
                                __Visitor {
                                    marker: _serde::__private::PhantomData::<QueryMsg<T>>,
                                    lifetime: _serde::__private::PhantomData,
                                },
                            )
                        }
                        (__Field::__field3, __variant) => {
                            #[doc(hidden)]
                            struct __Visitor<'de, T>
                            where
                                T: Serialize,
                                T: _serde::Deserialize<'de>,
                            {
                                marker: _serde::__private::PhantomData<QueryMsg<T>>,
                                lifetime: _serde::__private::PhantomData<&'de ()>,
                            }
                            impl<'de, T> _serde::de::Visitor<'de> for __Visitor<'de, T>
                            where
                                T: Serialize,
                                T: _serde::Deserialize<'de>,
                            {
                                type Value = QueryMsg<T>;
                                fn expecting(
                                    &self,
                                    __formatter: &mut _serde::__private::Formatter,
                                ) -> _serde::__private::fmt::Result {
                                    _serde::__private::Formatter::write_str(
                                        __formatter,
                                        "tuple variant QueryMsg::FourthQuery",
                                    )
                                }
                                #[inline]
                                fn visit_seq<__A>(
                                    self,
                                    mut __seq: __A,
                                ) -> _serde::__private::Result<Self::Value, __A::Error>
                                where
                                    __A: _serde::de::SeqAccess<'de>,
                                {
                                    let __field0 = match _serde::de::SeqAccess::next_element::<
                                        u64,
                                    >(&mut __seq)? {
                                        _serde::__private::Some(__value) => __value,
                                        _serde::__private::None => {
                                            return _serde::__private::Err(
                                                _serde::de::Error::invalid_length(
                                                    0usize,
                                                    &"tuple variant QueryMsg::FourthQuery with 2 elements",
                                                ),
                                            );
                                        }
                                    };
                                    let __field1 = match _serde::de::SeqAccess::next_element::<
                                        String,
                                    >(&mut __seq)? {
                                        _serde::__private::Some(__value) => __value,
                                        _serde::__private::None => {
                                            return _serde::__private::Err(
                                                _serde::de::Error::invalid_length(
                                                    1usize,
                                                    &"tuple variant QueryMsg::FourthQuery with 2 elements",
                                                ),
                                            );
                                        }
                                    };
                                    _serde::__private::Ok(
                                        QueryMsg::FourthQuery(__field0, __field1),
                                    )
                                }
                            }
                            _serde::de::VariantAccess::tuple_variant(
                                __variant,
                                2usize,
                                __Visitor {
                                    marker: _serde::__private::PhantomData::<QueryMsg<T>>,
                                    lifetime: _serde::__private::PhantomData,
                                },
                            )
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &[
                "first_query",
                "second_query",
                "third_query",
                "fourth_query",
            ];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "QueryMsg",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<QueryMsg<T>>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
#[automatically_derived]
#[allow(clippy::derive_partial_eq_without_eq)]
impl<T: ::core::clone::Clone> ::core::clone::Clone for QueryMsg<T>
where
    T: Serialize,
{
    #[inline]
    fn clone(&self) -> QueryMsg<T> {
        match self {
            QueryMsg::FirstQuery {} => QueryMsg::FirstQuery {},
            QueryMsg::SecondQuery { t: __self_0 } => {
                QueryMsg::SecondQuery {
                    t: ::core::clone::Clone::clone(__self_0),
                }
            }
            QueryMsg::ThirdQuery { t: __self_0 } => {
                QueryMsg::ThirdQuery {
                    t: ::core::clone::Clone::clone(__self_0),
                }
            }
            QueryMsg::FourthQuery(__self_0, __self_1) => {
                QueryMsg::FourthQuery(
                    ::core::clone::Clone::clone(__self_0),
                    ::core::clone::Clone::clone(__self_1),
                )
            }
        }
    }
}
#[automatically_derived]
#[allow(clippy::derive_partial_eq_without_eq)]
impl<T: ::core::fmt::Debug> ::core::fmt::Debug for QueryMsg<T>
where
    T: Serialize,
{
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            QueryMsg::FirstQuery {} => ::core::fmt::Formatter::write_str(f, "FirstQuery"),
            QueryMsg::SecondQuery { t: __self_0 } => {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "SecondQuery",
                    "t",
                    &__self_0,
                )
            }
            QueryMsg::ThirdQuery { t: __self_0 } => {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "ThirdQuery",
                    "t",
                    &__self_0,
                )
            }
            QueryMsg::FourthQuery(__self_0, __self_1) => {
                ::core::fmt::Formatter::debug_tuple_field2_finish(
                    f,
                    "FourthQuery",
                    __self_0,
                    &__self_1,
                )
            }
        }
    }
}
#[automatically_derived]
#[allow(clippy::derive_partial_eq_without_eq)]
impl<T> ::core::marker::StructuralPartialEq for QueryMsg<T>
where
    T: Serialize,
{}
#[automatically_derived]
#[allow(clippy::derive_partial_eq_without_eq)]
impl<T: ::core::cmp::PartialEq> ::core::cmp::PartialEq for QueryMsg<T>
where
    T: Serialize,
{
    #[inline]
    fn eq(&self, other: &QueryMsg<T>) -> bool {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        let __arg1_tag = ::core::intrinsics::discriminant_value(other);
        __self_tag == __arg1_tag
            && match (self, other) {
                (
                    QueryMsg::SecondQuery { t: __self_0 },
                    QueryMsg::SecondQuery { t: __arg1_0 },
                ) => *__self_0 == *__arg1_0,
                (
                    QueryMsg::ThirdQuery { t: __self_0 },
                    QueryMsg::ThirdQuery { t: __arg1_0 },
                ) => *__self_0 == *__arg1_0,
                (
                    QueryMsg::FourthQuery(__self_0, __self_1),
                    QueryMsg::FourthQuery(__arg1_0, __arg1_1),
                ) => *__self_0 == *__arg1_0 && *__self_1 == *__arg1_1,
                _ => true,
            }
    }
}
const _: () = {
    use ::cosmwasm_schema::schemars as schemars;
    #[automatically_derived]
    #[allow(unused_braces)]
    impl<T: schemars::JsonSchema> schemars::JsonSchema for QueryMsg<T>
    where
        T: Serialize,
    {
        fn schema_name() -> std::string::String {
            {
                let res = ::alloc::fmt::format(
                    format_args!("QueryMsg_for_{0}", T::schema_name()),
                );
                res
            }
        }
        fn schema_id() -> std::borrow::Cow<'static, str> {
            std::borrow::Cow::Owned({
                let res = ::alloc::fmt::format(
                    format_args!("mock_contract::QueryMsg_for_{0}", T::schema_id()),
                );
                res
            })
        }
        fn json_schema(
            gen: &mut schemars::gen::SchemaGenerator,
        ) -> schemars::schema::Schema {
            schemars::schema::Schema::Object(schemars::schema::SchemaObject {
                subschemas: Some(
                    Box::new(schemars::schema::SubschemaValidation {
                        one_of: Some(
                            <[_]>::into_vec(
                                #[rustc_box]
                                ::alloc::boxed::Box::new([
                                    schemars::_private::metadata::add_description(
                                        schemars::_private::new_externally_tagged_enum(
                                            "first_query",
                                            {
                                                let mut schema_object = schemars::schema::SchemaObject {
                                                    instance_type: Some(
                                                        schemars::schema::InstanceType::Object.into(),
                                                    ),
                                                    ..Default::default()
                                                };
                                                let object_validation = schema_object.object();
                                                object_validation
                                                    .additional_properties = Some(Box::new(false.into()));
                                                schemars::schema::Schema::Object(schema_object)
                                            },
                                        ),
                                        "test-doc-comment",
                                    ),
                                    schemars::_private::new_externally_tagged_enum(
                                        "second_query",
                                        {
                                            let mut schema_object = schemars::schema::SchemaObject {
                                                instance_type: Some(
                                                    schemars::schema::InstanceType::Object.into(),
                                                ),
                                                ..Default::default()
                                            };
                                            let object_validation = schema_object.object();
                                            object_validation
                                                .additional_properties = Some(Box::new(false.into()));
                                            {
                                                schemars::_private::insert_object_property::<
                                                    T,
                                                >(
                                                    object_validation,
                                                    "t",
                                                    false,
                                                    false,
                                                    schemars::_private::metadata::add_description(
                                                        gen.subschema_for::<T>(),
                                                        "test doc-comment",
                                                    ),
                                                );
                                            }
                                            schemars::schema::Schema::Object(schema_object)
                                        },
                                    ),
                                    schemars::_private::new_externally_tagged_enum(
                                        "third_query",
                                        {
                                            let mut schema_object = schemars::schema::SchemaObject {
                                                instance_type: Some(
                                                    schemars::schema::InstanceType::Object.into(),
                                                ),
                                                ..Default::default()
                                            };
                                            let object_validation = schema_object.object();
                                            object_validation
                                                .additional_properties = Some(Box::new(false.into()));
                                            {
                                                schemars::_private::insert_object_property::<
                                                    T,
                                                >(
                                                    object_validation,
                                                    "t",
                                                    false,
                                                    false,
                                                    schemars::_private::metadata::add_description(
                                                        gen.subschema_for::<T>(),
                                                        "test doc-comment",
                                                    ),
                                                );
                                            }
                                            schemars::schema::Schema::Object(schema_object)
                                        },
                                    ),
                                    schemars::_private::new_externally_tagged_enum(
                                        "fourth_query",
                                        schemars::schema::Schema::Object(schemars::schema::SchemaObject {
                                            instance_type: Some(
                                                schemars::schema::InstanceType::Array.into(),
                                            ),
                                            array: Some(
                                                Box::new(schemars::schema::ArrayValidation {
                                                    items: Some(
                                                        <[_]>::into_vec(
                                                                #[rustc_box]
                                                                ::alloc::boxed::Box::new([
                                                                    gen.subschema_for::<u64>(),
                                                                    gen.subschema_for::<String>(),
                                                                ]),
                                                            )
                                                            .into(),
                                                    ),
                                                    max_items: Some(2u32),
                                                    min_items: Some(2u32),
                                                    ..Default::default()
                                                }),
                                            ),
                                            ..Default::default()
                                        }),
                                    ),
                                ]),
                            ),
                        ),
                        ..Default::default()
                    }),
                ),
                ..Default::default()
            })
        }
    }
};
#[allow(clippy::derive_partial_eq_without_eq)]
#[serde(deny_unknown_fields, crate = "::cosmwasm_schema::serde")]
#[schemars(crate = "::cosmwasm_schema::schemars")]
pub struct MigrateMsg {
    pub t: String,
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use ::cosmwasm_schema::serde as _serde;
    #[automatically_derived]
    impl ::cosmwasm_schema::serde::Serialize for MigrateMsg {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> ::cosmwasm_schema::serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: ::cosmwasm_schema::serde::Serializer,
        {
            let mut __serde_state = _serde::Serializer::serialize_struct(
                __serializer,
                "MigrateMsg",
                false as usize + 1,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "t",
                &self.t,
            )?;
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use ::cosmwasm_schema::serde as _serde;
    #[automatically_derived]
    impl<'de> ::cosmwasm_schema::serde::Deserialize<'de> for MigrateMsg {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> ::cosmwasm_schema::serde::__private::Result<Self, __D::Error>
        where
            __D: ::cosmwasm_schema::serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "field identifier",
                    )
                }
                fn visit_u64<__E>(
                    self,
                    __value: u64,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Unsigned(__value),
                                    &"field index 0 <= i < 1",
                                ),
                            )
                        }
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "t" => _serde::__private::Ok(__Field::__field0),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::unknown_field(__value, FIELDS),
                            )
                        }
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"t" => _serde::__private::Ok(__Field::__field0),
                        _ => {
                            let __value = &_serde::__private::from_utf8_lossy(__value);
                            _serde::__private::Err(
                                _serde::de::Error::unknown_field(__value, FIELDS),
                            )
                        }
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<MigrateMsg>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = MigrateMsg;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "struct MigrateMsg",
                    )
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<
                        String,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct MigrateMsg with 1 element",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(MigrateMsg { t: __field0 })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                        __Field,
                    >(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("t"),
                                    );
                                }
                                __field0 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                );
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("t")?
                        }
                    };
                    _serde::__private::Ok(MigrateMsg { t: __field0 })
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["t"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "MigrateMsg",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<MigrateMsg>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
#[automatically_derived]
#[allow(clippy::derive_partial_eq_without_eq)]
impl ::core::clone::Clone for MigrateMsg {
    #[inline]
    fn clone(&self) -> MigrateMsg {
        MigrateMsg {
            t: ::core::clone::Clone::clone(&self.t),
        }
    }
}
#[automatically_derived]
#[allow(clippy::derive_partial_eq_without_eq)]
impl ::core::fmt::Debug for MigrateMsg {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(
            f,
            "MigrateMsg",
            "t",
            &&self.t,
        )
    }
}
#[automatically_derived]
#[allow(clippy::derive_partial_eq_without_eq)]
impl ::core::marker::StructuralPartialEq for MigrateMsg {}
#[automatically_derived]
#[allow(clippy::derive_partial_eq_without_eq)]
impl ::core::cmp::PartialEq for MigrateMsg {
    #[inline]
    fn eq(&self, other: &MigrateMsg) -> bool {
        self.t == other.t
    }
}
const _: () = {
    use ::cosmwasm_schema::schemars as schemars;
    #[automatically_derived]
    #[allow(unused_braces)]
    impl schemars::JsonSchema for MigrateMsg {
        fn schema_name() -> std::string::String {
            "MigrateMsg".to_owned()
        }
        fn schema_id() -> std::borrow::Cow<'static, str> {
            std::borrow::Cow::Borrowed("mock_contract::MigrateMsg")
        }
        fn json_schema(
            gen: &mut schemars::gen::SchemaGenerator,
        ) -> schemars::schema::Schema {
            {
                let mut schema_object = schemars::schema::SchemaObject {
                    instance_type: Some(schemars::schema::InstanceType::Object.into()),
                    ..Default::default()
                };
                let object_validation = schema_object.object();
                object_validation.additional_properties = Some(Box::new(false.into()));
                {
                    schemars::_private::insert_object_property::<
                        String,
                    >(
                        object_validation,
                        "t",
                        false,
                        false,
                        gen.subschema_for::<String>(),
                    );
                }
                schemars::schema::Schema::Object(schema_object)
            }
        }
    }
};
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    Ok(Response::new().add_attribute("action", "instantiate"))
}
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::FirstMessage {} => {
            Ok(Response::new().add_attribute("action", "first message passed"))
        }
        ExecuteMsg::SecondMessage { t: _ } => {
            Err(StdError::generic_err("Second Message Failed"))
        }
        ExecuteMsg::ThirdMessage { .. } => {
            Ok(Response::new().add_attribute("action", "third message passed"))
        }
        ExecuteMsg::FourthMessage => {
            Ok(Response::new().add_attribute("action", "fourth message passed"))
        }
        ExecuteMsg::FifthMessage => {
            if info.funds.is_empty() {
                return Err(StdError::generic_err("Coins missing"));
            }
            Ok(Response::new().add_attribute("action", "fourth message passed"))
        }
        ExecuteMsg::SixthMessage(_, _) => {
            Ok(Response::new().add_attribute("action", "sixth message passed"))
        }
        ExecuteMsg::SeventhMessage(amount, denom) => {
            let c = info.funds[0].clone();
            if c.amount != amount && c.denom.ne(&denom) {
                return Err(StdError::generic_err("Coins don't match message"));
            }
            Ok(Response::new().add_attribute("action", "fourth message passed"))
        }
    }
}
pub fn query(_deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::FirstQuery {} => to_json_binary("first query passed"),
        QueryMsg::SecondQuery { .. } => Err(StdError::generic_err("Query not available")),
        QueryMsg::ThirdQuery { .. } => to_json_binary("third query passed"),
        QueryMsg::FourthQuery(_, _) => to_json_binary(&4u64),
    }
}
pub fn migrate(_deps: DepsMut, _env: Env, msg: MigrateMsg) -> StdResult<Response> {
    if msg.t.eq("success") {
        Ok(Response::new())
    } else {
        Err(StdError::generic_err("migrate endpoint reached but no test implementation"))
    }
}
#[cfg(not(target_arch = "wasm32"))]
pub struct MockContract<Chain>(::cw_orch::core::contract::Contract<Chain>);
#[automatically_derived]
impl<Chain: ::core::clone::Clone> ::core::clone::Clone for MockContract<Chain> {
    #[inline]
    fn clone(&self) -> MockContract<Chain> {
        MockContract(::core::clone::Clone::clone(&self.0))
    }
}
#[cfg(not(target_arch = "wasm32"))]
impl<Chain> MockContract<Chain> {
    pub fn new(contract_id: impl ToString, chain: Chain) -> Self {
        Self(::cw_orch::core::contract::Contract::new(contract_id, chain))
    }
}
#[cfg(not(target_arch = "wasm32"))]
impl<
    Chain: ::cw_orch::core::environment::ChainState,
> ::cw_orch::core::contract::interface_traits::ContractInstance<Chain>
for MockContract<Chain> {
    fn as_instance(&self) -> &::cw_orch::core::contract::Contract<Chain> {
        &self.0
    }
    fn as_instance_mut(&mut self) -> &mut ::cw_orch::core::contract::Contract<Chain> {
        &mut self.0
    }
}
#[cfg(not(target_arch = "wasm32"))]
impl<Chain> ::cw_orch::core::contract::interface_traits::InstantiableContract
for MockContract<Chain> {
    type InstantiateMsg = InstantiateMsg;
}
#[cfg(not(target_arch = "wasm32"))]
impl<Chain> ::cw_orch::core::contract::interface_traits::ExecutableContract
for MockContract<Chain> {
    type ExecuteMsg = ExecuteMsg;
}
#[cfg(not(target_arch = "wasm32"))]
impl<Chain> ::cw_orch::core::contract::interface_traits::QueryableContract
for MockContract<Chain> {
    type QueryMsg = QueryMsg;
}
#[cfg(not(target_arch = "wasm32"))]
impl<Chain> ::cw_orch::core::contract::interface_traits::MigratableContract
for MockContract<Chain> {
    type MigrateMsg = MigrateMsg;
}
#[cfg(not(target_arch = "wasm32"))]
pub mod interface {
    use cw_orch::environment::ChainInfoOwned;
    use super::*;
    impl<Chain> cw_orch::prelude::Uploadable for MockContract<Chain> {
        fn wrapper() -> Box<
            dyn cw_orch::prelude::MockContract<cosmwasm_std::Empty, cosmwasm_std::Empty>,
        > {
            Box::new(
                cw_orch::prelude::ContractWrapper::new(execute, instantiate, query)
                    .with_migrate(migrate),
            )
        }
        fn wasm(_chain: &ChainInfoOwned) -> cw_orch::prelude::WasmPath {
            use cw_orch::prelude::*;
            ArtifactsDir::auto(
                    Some(
                        "/root/abstract/cw-orchestrator/contracts/mock_contract"
                            .to_string(),
                    ),
                )
                .find_wasm_path("mock_contract")
                .unwrap()
        }
    }
}
