    const INTERNAL_ERR: &'static str = "`ethabi_derive` internal error";
    /// Contract's functions.
    #[allow(dead_code, unused_imports, unused_variables)]
    pub mod functions {
        use super::INTERNAL_ERR;
    }
    /// Contract's events.
    #[allow(dead_code, unused_imports, unused_variables)]
    pub mod events {
        use super::INTERNAL_ERR;
        #[derive(Debug, Clone, PartialEq)]
        pub struct EntryAdded {
            pub index: substreams::scalar::BigInt,
            pub uri: String,
            pub author: Vec<u8>,
        }
        impl EntryAdded {
            const TOPIC_ID: [u8; 32] = [
                3u8,
                153u8,
                100u8,
                198u8,
                110u8,
                4u8,
                129u8,
                86u8,
                244u8,
                32u8,
                226u8,
                134u8,
                143u8,
                179u8,
                212u8,
                85u8,
                241u8,
                151u8,
                72u8,
                110u8,
                195u8,
                26u8,
                51u8,
                206u8,
                217u8,
                200u8,
                105u8,
                76u8,
                209u8,
                200u8,
                169u8,
                167u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 1usize {
                    return false;
                }
                if log.data.len() < 128usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::String,
                            ethabi::ParamType::Address,
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    index: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                    uri: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_string()
                        .expect(INTERNAL_ERR),
                    author: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                })
            }
        }
        impl substreams_ethereum::Event for EntryAdded {
            const NAME: &'static str = "EntryAdded";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
    }