/*!
routines used to serialize / deserialize a Cosmos / Tendermint / TerraD structure
*/
/// Convert a JSON date time into a rust one
pub mod terra_datetime_format {
    use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &str = "%Y-%m-%dT%H:%M:%S%.f";
    const FORMAT_TZ_SUPPLIED: &str = "%Y-%m-%dT%H:%M:%S.%f%:z";
    const FORMAT_SHORT_Z: &str = "%Y-%m-%dT%H:%M:%SZ";
    const FORMAT_SHORT_Z2: &str = "%Y-%m-%dT%H:%M:%S.%fZ";

    // The signature of a serialize_with function must follow the pattern:
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
    //    where
    //        S: Serializer
    //
    // although it may also be generic over the input types T.

    #[allow(missing_docs)]
    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    #[allow(missing_docs)]
    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        let len = s.len();
        let slice_len = if s.contains('.') {
            len.saturating_sub(4)
        } else {
            len
        };

        let sliced = &s[0..slice_len];

        match NaiveDateTime::parse_from_str(sliced, FORMAT) {
            Err(_e) => match NaiveDateTime::parse_from_str(&s, FORMAT_TZ_SUPPLIED) {
                Err(_e2) => match NaiveDateTime::parse_from_str(sliced, FORMAT_SHORT_Z) {
                    // block 6877827 has this
                    Err(_e3) => match NaiveDateTime::parse_from_str(&s, FORMAT_SHORT_Z2) {
                        Err(_e4) => {
                            eprintln!("DateTime Fail {} {:#?}", s, _e4);
                            Err(serde::de::Error::custom(_e4))
                        }
                        Ok(dt) => Ok(Utc.from_utc_datetime(&dt)),
                    },
                    Ok(dt) => Ok(Utc.from_utc_datetime(&dt)),
                },
                Ok(dt) => Ok(Utc.from_utc_datetime(&dt)),
            },
            Ok(dt) => Ok(Utc.from_utc_datetime(&dt)),
        }
    }
}

// /// Convert a JSON date time into a rust one
// pub mod terra_opt_datetime_format {
//     use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
//     use serde::{self, Deserialize, Deserializer, Serializer};

//     const FORMAT: &str = "%Y-%m-%dT%H:%M:%S%.f";
//     const FORMAT_TZ_SUPPLIED: &str = "%Y-%m-%dT%H:%M:%S.%f%:z";
//     const FORMAT_SHORT_Z: &str = "%Y-%m-%dT%H:%M:%SZ";
//     const FORMAT_SHORT_Z2: &str = "%Y-%m-%dT%H:%M:%S.%fZ";

//     // The signature of a serialize_with function must follow the pattern:
//     //
//     //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
//     //    where
//     //        S: Serializer
//     //
//     // although it may also be generic over the input types T.

//     #[allow(missing_docs)]
//     pub fn serialize<S>(date: &Option<DateTime<Utc>>, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         if let Some(val) = date {
//             let s = format!("{}", val.format(FORMAT));
//             serializer.serialize_str(&s)
//         } else {
//             serializer.serialize_none()
//         }
//     }

//     // The signature of a deserialize_with function must follow the pattern:
//     //
//     //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
//     //    where
//     //        D: Deserializer<'de>
//     //
//     // although it may also be generic over the output types T.
//     #[allow(missing_docs)]
//     pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         let value: Result<Option<String>, D::Error> = Option::deserialize(deserializer);
//         match value {
//             Ok(s_opt) => {
//                 if let Some(s) = s_opt {
//                     let len = s.len();
//                     let slice_len = if s.contains('.') {
//                         len.saturating_sub(4)
//                     } else {
//                         len
//                     };
//                     let sliced = &s[0..slice_len];
//                     match NaiveDateTime::parse_from_str(sliced, FORMAT) {
//                         Err(_e) => match NaiveDateTime::parse_from_str(&s, FORMAT_TZ_SUPPLIED) {
//                             Err(_e2) => match NaiveDateTime::parse_from_str(sliced, FORMAT_SHORT_Z)
//                             {
//                                 Err(_e3) => {
//                                     match NaiveDateTime::parse_from_str(&s, FORMAT_SHORT_Z2) {
//                                         Err(_e4) => {
//                                             eprintln!("DateTime Fail {} {:#?}", s, _e);
//                                             Err(serde::de::Error::custom(_e))
//                                         }
//                                         Ok(dt) => Ok(Some(Utc.from_utc_datetime(&dt))),
//                                     }
//                                 }
//                                 Ok(dt) => Ok(Some(Utc.from_utc_datetime(&dt))),
//                             },
//                             Ok(dt) => Ok(Some(Utc.from_utc_datetime(&dt))),
//                         },
//                         Ok(dt) => Ok(Some(Utc.from_utc_datetime(&dt))),
//                     }
//                 } else {
//                     Ok(None)
//                 }
//             }
//             Err(e) => {
//                 eprintln!("DateTimeOpt/Deserialization Fail - {:?}", e);
//                 Err(e)
//             }
//         }
//     }
// }

/// Convert a u64 number (which is sent as a string) into a u64 rust structure
pub mod terra_u64_format {
    use serde::{self, Deserialize, Deserializer, Serializer};

    // convert a number in string format into a regular u64
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
    //    where
    //        S: Serializer
    //
    // although it may also be generic over the input types T.

    #[allow(missing_docs)]
    pub fn serialize<S>(val: &u64, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        //  let s = format!("{}", val);
        serializer.serialize_str(&val.to_string())
    }

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    #[allow(missing_docs)]
    pub fn deserialize<'de, D>(deserializer: D) -> Result<u64, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        match s.parse::<u64>() {
            Err(_e) => {
                eprintln!("u64 Fail {} {:#?}", s, _e);
                Err(serde::de::Error::custom(_e))
            }
            Ok(val) => Ok(val),
        }
    }
}

/// Convert a i64 number (which is sent as a string) into a u64 rust structure
pub mod terra_i64_format {
    use serde::{self, Deserialize, Deserializer, Serializer};

    // convert a number in string format into a regular u64
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
    //    where
    //        S: Serializer
    //
    // although it may also be generic over the input types T.

    #[allow(missing_docs)]
    pub fn serialize<S>(val: &i64, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        //  let s = format!("{}", val);
        serializer.serialize_str(&val.to_string())
    }

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    #[allow(missing_docs)]
    pub fn deserialize<'de, D>(deserializer: D) -> Result<i64, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        match s.parse::<i64>() {
            Err(_e) => {
                eprintln!("i64 Fail {} {:#?}", s, _e);
                Err(serde::de::Error::custom(_e))
            }
            Ok(val) => Ok(val),
        }
    }
}

/// Convert a f64 number (which is sent as a string) into a f64 rust structure
pub mod terra_f64_format {
    use serde::{self, Deserialize, Deserializer, Serializer};

    // convert a number in string format into a regular u64
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
    //    where
    //        S: Serializer
    //
    // although it may also be generic over the input types T.
    #[allow(missing_docs)]
    pub fn serialize<S>(val: &f64, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        //  let s = format!("{}", val);
        serializer.serialize_str(&val.to_string())
    }

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    #[allow(missing_docs)]
    pub fn deserialize<'de, D>(deserializer: D) -> Result<f64, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;

        match s.parse::<f64>() {
            Err(_e) => {
                eprintln!("f64 Fail {} {:#?}", s, _e);
                Err(serde::de::Error::custom(_e))
            }
            Ok(val) => Ok(val),
        }
    }
}

/// Convert a Decimal number (which is sent as a string) into a Decimal rust structure
pub mod terra_decimal_format {
    use rust_decimal::Decimal;
    use serde::{self, Deserialize, Deserializer, Serializer};

    // convert a number in string format into a regular u64
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
    //    where
    //        S: Serializer
    //
    // although it may also be generic over the input types T.
    #[allow(missing_docs)]
    pub fn serialize<S>(val: &Decimal, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        //  let s = format!("{}", val);
        serializer.serialize_str(&val.to_string())
    }

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    #[allow(missing_docs)]
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Decimal, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;

        match s.parse::<Decimal>() {
            Err(_e) => {
                eprintln!("Decimal Fail {} {:#?}", s, _e);
                Err(serde::de::Error::custom(_e))
            }
            Ok(val) => Ok(val),
        }
    }
}

/// Convert a Optional Decimal number (which is sent as a string) into a decimal rust structure
pub mod terra_opt_decimal_format {
    use rust_decimal::Decimal;
    use serde::{self, Deserialize, Deserializer, Serializer};

    // convert a number in string format into a regular u64
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
    //    where
    //        S: Serializer
    //
    // although it may also be generic over the input types T.
    #[allow(missing_docs)]
    pub fn serialize<S>(val: &Option<Decimal>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match val {
            None => serializer.serialize_none(),
            Some(d) => serializer.serialize_str(&d.to_string()),
        }
        //   serializer.serialize_str(&val.to_string())
    }

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    #[allow(missing_docs)]
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Decimal>, D::Error>
    where
        D: Deserializer<'de>,
    {
        match String::deserialize(deserializer) {
            Ok(s) => {
                if s.is_empty() {
                    Ok(None)
                } else {
                    match s.parse::<Decimal>() {
                        Err(_e) => {
                            eprintln!("Decimal Fail {} {:#?}", s, _e);
                            Err(serde::de::Error::custom(_e))
                        }
                        Ok(val) => Ok(Some(val)),
                    }
                }
            }
            Err(_) => Ok(None),
        }
    }
}

/// Convert a Optional u64 number (which is sent as a string) into a u64 rust structure
pub mod terra_opt_u64_format {
    use serde::{self, Deserialize, Deserializer, Serializer};

    // convert a number in string format into a regular u64
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
    //    where
    //        S: Serializer
    //
    // although it may also be generic over the input types T.
    #[allow(missing_docs)]
    pub fn serialize<S>(val: &Option<u64>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match val {
            None => serializer.serialize_none(),
            Some(d) => serializer.serialize_str(&d.to_string()),
        }
        //   serializer.serialize_str(&val.to_string())
    }

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    #[allow(missing_docs)]
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
    where
        D: Deserializer<'de>,
    {
        match String::deserialize(deserializer) {
            Ok(s) => {
                if s.is_empty() {
                    Ok(None)
                } else {
                    match s.parse::<u64>() {
                        Err(_e) => {
                            eprintln!("Decimal Fail {} {:#?}", s, _e);
                            Err(serde::de::Error::custom(_e))
                        }
                        Ok(val) => Ok(Some(val)),
                    }
                }
            }
            Err(_) => Ok(None),
        }
    }
}
/// serialize/deserialize a base64 encoded value
pub mod base64_encoded_format {
    use serde::{self, Deserialize, Deserializer, Serializer};

    //  convert a regular string into it's base64 representation
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
    //    where
    //        S: Serializer
    //
    // although it may also be generic over the input types T.
    #[allow(missing_docs)]
    pub fn serialize<S>(val: &str, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        //  let s = format!("{}", val);
        let encoded = base64::encode(val);
        serializer.serialize_str(&encoded)
    }

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    #[allow(missing_docs)]
    pub fn deserialize<'de, D>(deserializer: D) -> Result<String, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;

        match base64::decode(&s) {
            Err(_e) => {
                eprintln!("base64_encoded_format Fail {} {:#?}", s, _e);
                Err(serde::de::Error::custom(_e))
            }
            Ok(val) => Ok(String::from_utf8_lossy(&val).into()),
        }
    }
}

/// serialize/deserialize a base64 encoded value, with Option
pub mod base64_opt_encoded_format {
    use serde::{self, Deserialize, Deserializer, Serializer};

    // convert a regular Option<String> into it's base64 representation
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
    //    where
    //        S: Serializer
    //
    // although it may also be generic over the input types T.
    #[allow(missing_docs)]
    pub fn serialize<S>(v: &Option<String>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let Some(val) = v {
            let encoded = base64::encode(val);
            serializer.serialize_str(&encoded)
        } else {
            serializer.serialize_none()
        }
    }

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    #[allow(missing_docs)]
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: Result<Option<String>, D::Error> = Option::deserialize(deserializer);
        match value {
            Ok(Some(s)) => {
                if s.is_empty() {
                    Ok(None)
                } else {
                    match base64::decode(&s) {
                        Err(e) => {
                            log::error!("Base64-opt Fail {} {:#?}", s, e);
                            Err(serde::de::Error::custom(e))
                        }
                        Ok(val) => Ok(Some(String::from_utf8_lossy(&val).into())),
                    }
                }
            }
            Ok(None) => {
                //  log::error!("Deserializer {}", e);
                Ok(None)
            }
            Err(_e) => {
                eprintln!("base64_opt_encoded_format Fail {:#?}", _e);
                Err(serde::de::Error::custom(_e))
            }
        }
    }
}

pub mod cosm_denom_format {
    use std::str::FromStr;

    use cosmrs::Denom;
    use serde::{self, Deserialize, Deserializer, Serializer};

    #[allow(missing_docs)]
    pub fn serialize<S>(denom: &Denom, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = denom.to_string();
        serializer.serialize_str(&s)
    }

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    #[allow(missing_docs)]
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Denom, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Denom::from_str(&s).map_err(serde::de::Error::custom)
    }
}
