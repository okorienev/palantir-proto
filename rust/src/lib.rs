pub use prost;

pub mod palantir {
    pub mod apm {
        pub mod v1 {
            pub mod action {
                include!(concat!(env!("OUT_DIR"), "/palantir.apm.v1.action.rs"));
            }
        }
    }
    pub mod shared {
        pub mod measurement {
            include!(concat!(env!("OUT_DIR"), "/palantir.shared.measurement.rs"));
        }
        pub mod tag {
            include!(concat!(env!("OUT_DIR"), "/palantir.shared.tag.rs"));
        }
    }
    pub mod request {
        include!(concat!(env!("OUT_DIR"), "/palantir.request.rs"));
    }
}

#[cfg(test)]
mod tests {
    use super::palantir::apm::v1::action::ApmV1Action;
    use super::palantir::request::Request;
    use super::palantir::shared::measurement::Measurement;
    use super::palantir::shared::tag::Tag;
    use prost::bytes::BytesMut;
    use prost::Message;

    #[test]
    fn test_write_read_tag() {
        let tag = Tag {
            key: "key".to_string(),
            value: "value".to_string(),
        };
        let mut buf = BytesMut::with_capacity(tag.encoded_len());

        tag.encode(&mut buf).unwrap();
        let decoded = Tag::decode(buf).unwrap();

        assert_eq!(tag, decoded);
    }
}
