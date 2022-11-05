# homeassistant cli

A small cli wrapper for the homeassistant rest api.

_Prerequisites:_
To set it up you should be using [`pass`](https://www.passwordstore.org/).

Generate a bearer token for the rest api ([docs](https://developers.home-assistant.io/docs/api/rest/)).

Insert it into the password store:
```sh
pass insert homeassistant/<hostname>_token
```

For each toggle you will need to obtain the entity_id from the service. This can be accessed from the "overview" tab on the home assistant ui in and the entity_id textfield on the desired switch.

![entity_id_setting](./assets/entity_id_settings_example.png)

You can then assign an alias to the entity in the config file. The config defaults to path `$HOME/.config/hcli/default-config.yml`

Here is an example config you can start with.
```yaml
---
api_base_url: "http://192.168.1.132:8123"
services:
  switch:
    kitchen_lamp:
      entity_id: switch.kitchen_sun_lamp
```

## Installation
Change to the project directory and run:
```
cargo install --path .
```

## Usage
Just call `hcli --help` and figure it out.
