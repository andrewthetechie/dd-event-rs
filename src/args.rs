use clap::Parser;
use serde::ser::{Serialize, SerializeStruct, Serializer};
use std::str::FromStr;

#[derive(Debug)]
pub enum AlertType {
    Error,
    Warning,
    Info,
    Success,
    UserUpdate,
    Reccomendation,
    Snapshot,
}

impl FromStr for AlertType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "error" => Ok(AlertType::Error),
            "warning" => Ok(AlertType::Warning),
            "info" => Ok(AlertType::Info),
            "success" => Ok(AlertType::Success),
            "user_update" => Ok(AlertType::UserUpdate),
            "reccomendation" => Ok(AlertType::Reccomendation),
            "snapshot" => Ok(AlertType::Snapshot),
            _ => Err(String::from("Invalid alert type")),
        }
    }
}

#[derive(Debug)]
pub enum Priority {
    Normal,
    Low,
}

impl FromStr for Priority {
    type Err = String;
    fn from_str(s: &str) -> Result<Priority, Self::Err> {
        match s.to_lowercase().as_ref() {
            "normal" => Ok(Priority::Normal),
            "low" => Ok(Priority::Low),
            _ => Err(format!("Invalid priority: {}", s)),
        }
    }
}

#[derive(Debug)]
pub struct EventText {
    pub text: String,
}

impl FromStr for EventText {
    type Err = String;
    fn from_str(s: &str) -> Result<EventText, Self::Err> {
        match s.chars().count() > 4000 {
            true => Err(String::from(
                "Event text too long, limit to 4000 characters",
            )),
            false => Ok(EventText {
                text: s.to_string(),
            }),
        }
    }
}

#[derive(Debug)]
pub struct EventTitle {
    pub text: String,
}

impl FromStr for EventTitle {
    type Err = String;
    fn from_str(s: &str) -> Result<EventTitle, Self::Err> {
        match s.chars().count() > 256 {
            true => Err(String::from("Event title too long")),
            false => Ok(EventTitle {
                text: s.to_string(),
            }),
        }
    }
}

#[derive(Debug)]
pub struct AggregationKey {
    pub text: String,
}

impl FromStr for AggregationKey {
    type Err = String;
    fn from_str(s: &str) -> Result<AggregationKey, Self::Err> {
        match s.chars().count() > 300 {
            true => Err(String::from(
                "AggregationKey too long, limit to 300 characters",
            )),
            false => Ok(AggregationKey {
                text: s.to_string(),
            }),
        }
    }
}

#[derive(Debug)]
pub struct Tags {
    pub tags: Vec<String>,
}

impl FromStr for Tags {
    type Err = String;
    fn from_str(s: &str) -> Result<Tags, Self::Err> {
        let tags = s.split(',').map(|s| s.to_string()).collect();
        Ok(Tags { tags })
    }
}

#[derive(Parser, Debug)]
#[clap(
    author,
    version,
    about = "Write an event to Datadog via the Datadog API",
    long_about = "Write an event to Datadog via the Datadog API.\nEnvironment Variables:\n* DD_API_KEY - Your Datadog API key\n* DD_API_HOST - (Optional) The Datadog API host to use, defaults to DD's US API"
)]
pub struct Args {
    /// An arbitrary string to use for aggregation. Limited to 100 characters. If you specify a key, all events using that key are grouped together in the Event Stream.
    #[clap(long)]
    pub aggregation_key: Option<AggregationKey>,

    /// If an alert event is enabled, set its type. Allowed values: error,warning,info,success,user_update,recommendation,snapshot
    #[clap(short, long)]
    pub alert_type: Option<AlertType>,

    /// A device name.
    #[clap(short, long)]
    pub device_name: Option<String>,

    /// Host name to associate with the event. Any tags associated with the host are also applied to this event.
    #[clap(short, long)]
    pub host: Option<String>,

    /// The priority of the event. For example, normal or low. Allowed enum values: normal,low
    #[clap(short, long)]
    pub priority: Option<Priority>,

    /// Host name to associate with the event. Any tags associated with the host are also applied to this event.
    #[clap(short, long)]
    pub related_event_id: Option<u64>,

    /// The type of event being posted. Option examples include nagios, hudson, jenkins, my_apps, chef, puppet, git, bitbucket, etc. A complete list of source attribute values available https://docs.datadoghq.com/integrations/faq/list-of-api-source-attribute-value/.
    #[clap(short, long)]
    pub source_type_name: Option<String>,

    /// A list of tags to apply to the event. In the format key:value,key2:value2
    #[clap(long)]
    pub tags: Option<Tags>,

    /// The body of the event. Limited to 4000 characters. The text supports markdown. To use markdown in the event text, start the text block with %%% \n and end the text block with \n %%%.
    #[clap(short, long)]
    pub text: EventText,

    /// The event title.
    #[clap(long)]
    pub title: EventTitle,

    /// Will print verbose output to stdout
    #[clap(long)]
    pub verbose: bool,
}

impl Serialize for Args {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // 3 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("Color", 3)?;
        match &self.aggregation_key {
            Some(aggregation_key) => {
                state.serialize_field("aggregation_key", &aggregation_key.text)?;
            }
            None => {}
        }
        match &self.alert_type {
            Some(AlertType::Error) => {
                state.serialize_field("alert_type", &"error")?;
            }
            Some(AlertType::Warning) => {
                state.serialize_field("alert_type", &"warning")?;
            }
            Some(AlertType::Info) => {
                state.serialize_field("alert_type", &"info")?;
            }
            Some(AlertType::Success) => {
                state.serialize_field("alert_type", &"success")?;
            }
            Some(AlertType::UserUpdate) => {
                state.serialize_field("alert_type", &"user_update")?;
            }
            Some(AlertType::Reccomendation) => {
                state.serialize_field("alert_type", &"recommendation")?;
            }
            Some(AlertType::Snapshot) => {
                state.serialize_field("alert_type", &"snapshot")?;
            }
            None => {}
        }
        match &self.device_name {
            Some(device_name) => {
                state.serialize_field("device_name", &device_name)?;
            }
            None => {}
        }
        match &self.host {
            Some(host) => {
                state.serialize_field("host", &host)?;
            }
            None => {}
        }
        match &self.priority {
            Some(Priority::Normal) => {
                state.serialize_field("priority", &"normal")?;
            }
            Some(Priority::Low) => {
                state.serialize_field("priority", &"low")?;
            }
            None => {}
        }
        match &self.related_event_id {
            Some(related_event_id) => {
                state.serialize_field("related_event_id", &related_event_id)?;
            }
            None => {}
        }
        match &self.source_type_name {
            Some(source_type_name) => {
                state.serialize_field("source_type_name", &source_type_name)?;
            }
            None => {}
        }
        match &self.tags {
            Some(tags) => {
                state.serialize_field("tags", &tags.tags)?;
            }
            None => {}
        }

        state.serialize_field("text", &self.text.text)?;
        state.serialize_field("title", &self.title.text)?;
        state.end()
    }
}
