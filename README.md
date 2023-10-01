# atntools
Several tools to convert Photoshop action files:

- display an .atn file as json

Example output:

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