use {Reply, reply};

const REPLY_CODE: reply::Code = reply::code::STATUS_OR_HELP_REPLY;

/// A single feature supported by the server.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Feature
{
    pub name: String,
}

/// The response to a 'FEAT' command.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Features
{
    /// The list of supported features.
    pub features: Vec<Feature>,
}

impl Features
{
    pub fn new<F>(features: F) -> Self
        where F: IntoIterator<Item=Feature> {
        Features { features: features.into_iter().collect() }
    }
}

impl Default for Features
{
    fn default() -> Self { Features { features: Vec::new() }}
}

impl Into<Reply> for Features
{
    fn into(self) -> Reply {
        if self.features.is_empty() {
            Reply::single_line(REPLY_CODE, "no additional features supported")
        } else {
            let mut lines = Vec::new();
            lines.push("Extensions supported:".to_owned());
            lines.extend(self.features.iter().map(|feature| feature.name.to_string()));
            lines.push("END".to_owned());

            Reply::multi_line(REPLY_CODE, lines)
        }
    }
}

#[cfg(test)]
mod test
{
    use Reply;
    use super::*;

    #[test]
    fn generates_no_feature_responses() {
        let reply: Reply = Features::default().into();
        assert_eq!(reply, Reply::new(211, "no additional features supported"));
    }

    #[test]
    fn generates_single_feature_responses() {
        let reply: Reply = Features::new(vec![Feature { name: "foo".to_owned() }]).into();
        assert_eq!(reply, Reply::multi_line(211, vec![
            "Extensions supported:".to_owned(),
            "foo".to_owned(),
            "END".to_owned(),
        ]));
    }

    #[test]
    fn generates_multiple_feature_responses() {
        let reply: Reply = Features::new(vec![
            Feature { name: "foo".to_owned() },
            Feature { name: "bar".to_owned() },
            Feature { name: "baz".to_owned() },
        ]).into();

        assert_eq!(reply, Reply::multi_line(211, vec![
            "Extensions supported:".to_owned(),
            "foo".to_owned(),
            "bar".to_owned(),
            "baz".to_owned(),
            "END".to_owned(),
        ]));
    }
}
