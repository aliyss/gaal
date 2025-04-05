use indexmap::IndexMap;

use crate::gaal_core::provider::directory::GaalCoreDirectoryActions;

use super::{
    default::{GaalObject, GaalObjectAction},
    ObjectError,
};

#[derive(Clone, Debug, Default)]
pub struct Kvlm {
    pub fields: IndexMap<String, String>,
    pub message: String,
}

// KVLM is a key-value list message format
// Example:
// ```
// Subject Hello
// From Alice
// To Bob
// MultilineField This is a
//  multiline
//  message
//
// This is the message
// body
// ```

impl From<String> for Kvlm {
    fn from(data: String) -> Self {
        let mut fields = IndexMap::new();
        let mut message = String::new();

        let mut current_key = String::new();
        let mut is_message_body = false;
        for line in data.lines() {
            if is_message_body {
                // Append remaining lines to the message body
                message.push_str(line);
                message.push('\n');
                continue;
            }
            if line.is_empty() {
                // Empty line indicates the start of the message body
                is_message_body = true;
                continue;
            }

            if let Some((key, value)) = line.split_once(' ') {
                if !line.starts_with(' ') {
                    current_key = key.to_string();
                    fields.insert(current_key.clone(), value.to_string());
                } else {
                    // This is a continuation of the previous field
                    if let Some(value) = fields.get_mut(&current_key) {
                        value.push('\n');
                        value.push_str(&line[1..]);
                    }
                }
            }
        }

        message = message.trim_end_matches('\n').to_string();

        Self { fields, message }
    }
}

impl From<Kvlm> for String {
    fn from(kvlm: Kvlm) -> Self {
        let mut result = String::new();
        for (key, value) in kvlm.fields {
            result.push_str(&format!("{} {}\n", key, value.replace('\n', "\n ")));
        }
        result.push('\n');
        result.push_str(&kvlm.message);
        result
    }
}

impl<GCDA> GaalObjectAction<GCDA> for GaalObject<Kvlm>
where
    GCDA: GaalCoreDirectoryActions,
{
    type Data = Kvlm;
    type Serialized = String;

    fn new(fmt: String) -> Self {
        Self {
            fmt,
            data: Kvlm::default(),
        }
    }

    fn fmt(&self) -> Result<Self::Serialized, ObjectError> {
        Ok(self.fmt.clone())
    }

    fn from_serialized(fmt: &str, data: Self::Serialized) -> Self {
        let deserialized =
            <GaalObject<String> as GaalObjectAction<GCDA>>::deserialize_data(data).unwrap();
        Self {
            fmt: fmt.to_string(),
            data: deserialized.into(),
        }
    }

    fn from_data(fmt: &str, data: Self::Data) -> Self {
        Self {
            fmt: fmt.to_string(),
            data,
        }
    }

    fn serialize(&self) -> Result<Self::Serialized, ObjectError> {
        Ok(self.data.clone().into())
    }

    fn serialize_data(data: Self::Data) -> Result<Self::Serialized, ObjectError> {
        Ok(data.into())
    }

    fn deserialize(&self) -> Result<Self::Data, ObjectError> {
        Ok(self.data.clone())
    }

    fn deserialize_data(data: Self::Serialized) -> Result<Self::Data, ObjectError> {
        Ok(data.into())
    }
}

#[cfg(test)]
mod test_kvlm_from_string {
    #[test]
    fn kvlm_from_string() {
        let data = "Subject Hello\n\
            From Alice\n\
            To Bob\n\
            MultilineField This is a\n \
             mult\\niline\n \
             message\n\
            \n\
            This is the message \n\
            body";
        let kvlm = super::Kvlm::from(data.to_string());
        assert_eq!(kvlm.fields["Subject"], "Hello");
        assert_eq!(kvlm.fields["From"], "Alice");
        assert_eq!(kvlm.fields["To"], "Bob");
        assert_eq!(
            kvlm.fields["MultilineField"],
            "This is a\nmult\\niline\nmessage"
        );
        assert_eq!(kvlm.message, "This is the message \nbody");
    }
    #[test]
    fn kvlm_from_string_empty() {
        let data = "";
        let kvlm = super::Kvlm::from(data.to_string());
        assert_eq!(kvlm.fields.len(), 0);
        assert_eq!(kvlm.message, "");
    }
    #[test]
    fn kvlm_into_string() {
        let data = "Subject Hello\n\
            From Alice\n\
            To Bob\n\
            MultilineField This is a\n \
             mult\\niline\n \
             message\n\
            \n\
            This is the message \n\
            body";
        let kvlm = super::Kvlm::from(data.to_string());
        let result: String = kvlm.into();
        assert_eq!(result, data);
    }
}
