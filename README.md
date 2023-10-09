# atntools
Several tools to convert Photoshop action files:

- display an .atn file as json

## usage

To show available options:

```bash
$ atn2json -h
```

### Show detailled information of a action file:

```bash
$ atn2json ghosty.atn
```

Output:

```json
{
  "version": 16,
  "name": "Ghosty_Effect",
  "expanded": true,
  "actions": [
    {
      "index": 0,
      "shift_key": false,
      "command_key": false,
      "color_index": 0,
      "name": "ghosty",
      "expanded": false,
      "action_events": [
        {
          "expanded": false,
          "enabled": true,
          "with_dialog": false,
          "dialog_options": 0,
          "event_name": "gaussianBlur",
          "has_descriptor": true
        }
      ]
    }
  ]
}
```

### Show version only

This can be useful if you can't load the action file, but want to know what version it is:

```bash
$ atn2json -v ghosty.atn
```

Output:

```json
{
  "version": 16,
}
```

### Export to yaml

The default output format is json. However, you can also specify yaml as output format:

```bash
$ atn2json -f yaml ghosty.atn
```

Output:

```yaml
version: 16
name: MoveLayerOffset
expanded: true
actions:
- index: 0
  shift_key: false
  command_key: false
  color_index: 0
  name: Action 1
  expanded: true
  action_events:
  - expanded: true
    enabled: true
    with_dialog: false
    dialog_options: 2
    event_name: move
    dictionary: Move
    descriptor:
      class_id_1: ''
      class_id_2: 'null'
      nr_of_items: 2
      items:
      - !Item
        key: 'null'
      - !StringStructure
        value: ''
        foo: 666
```