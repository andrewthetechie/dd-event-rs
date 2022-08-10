# dd-event

A CLI app to send events to datadog via the API.

```shell
dd-event
Andrew Herrington <andrew@💻.kz>
Write an event to Datadog via the Datadog API.
Environment Variables:
* DD_API_KEY - Your Datadog API key
* DD_API_HOST - (Optional) The Datadog API host to use, defaults to DD's US API

USAGE:
    dd-event [OPTIONS] --text <TEXT> --title <TITLE>

OPTIONS:
    -a, --alert-type <ALERT_TYPE>
            If an alert event is enabled, set its type. Allowed values:
            error,warning,info,success,user_update,recommendation,snapshot

        --aggregation-key <AGGREGATION_KEY>
            An arbitrary string to use for aggregation. Limited to 100 characters. If you specify a
            key, all events using that key are grouped together in the Event Stream

    -d, --device-name <DEVICE_NAME>
            A device name

    -h, --host <HOST>
            Host name to associate with the event. Any tags associated with the host are also
            applied to this event

        --help
            Print help information

    -p, --priority <PRIORITY>
            The priority of the event. For example, normal or low. Allowed enum values: normal,low

    -r, --related-event-id <RELATED_EVENT_ID>
            Host name to associate with the event. Any tags associated with the host are also
            applied to this event

    -s, --source-type-name <SOURCE_TYPE_NAME>
            The type of event being posted. Option examples include nagios, hudson, jenkins,
            my_apps, chef, puppet, git, bitbucket, etc. A complete list of source attribute values
            available
            https://docs.datadoghq.com/integrations/faq/list-of-api-source-attribute-value/

    -t, --text <TEXT>
            The body of the event. Limited to 4000 characters. The text supports markdown. To use
            markdown in the event text, start the text block with %%% \n and end the text block with
            \n %%%

        --tags <TAGS>
            A list of tags to apply to the event. In the format key:value,key2:value2

        --title <TITLE>
            The event title

    -V, --version
            Print version information

        --verbose
            Will print verbose output to stdout
```
