# Fridaylight Metadata

Located in the same directory as the mod's executable, mod creators have the ability to provide additional metadata to be used and displayed on its mod page by creating a folder with the name `.flight` and including any of these files:

| File | Purpose |
|------|---------|
|`banner.png`, `banner.webp`| Image file used for the mods' display banner|
|`logo.png`, `logo.webp`| Image file used for the mods' logo, displayed instead of its name|
|`metadata.json`| JSON Schema used for including mod metadata to be shown on the sidebar and mod page|

## metadata.json

### Schema

The following are the available fields for the metadata.json file. Only the `metadata_version` field is required, all others are optional. You can see a full example of a schema [here](/docs/example_metadata.json).

| Key | Value |
|-----|-------|
|`metadata_version`| **Required**<br>Integer, current version is `1`. |
|`name`| String, display name for the mod|
|`version`| String, displayed version for the mod |
| [`engine`](#engine) | Object containing engine-specific metadata|
|`description`| String, displayed text blurb under the mod's banner |
|`logo_position`| String, controls the position of the logo in the banner.<br>`left_bottom`, `left_middle`, `middle` |
| [`contributors`](#contributors) | Array of contributor groups |

### `engine`

| Key | Value |
|-----|-------|
| `engine_type` | String, this allows for mods folder mod management and small cosmetic changes. See more details below. |
| `engine_name` | String, shows a tooltip on engine icon hover |
| `engine_icon` | String, path to icon relative to the `.flight` folder |
| `mods_folder` | Boolean, tells if you want to show the mods folder mod management.<br> Default is `false`. |
| `mods_folder_path` | String, path to mods folder relative to the executable directory<br>Default is `mods`. |

Valid `engine_type` values include:

| Value | Features |
|-------|----------|
|`psych`, `fps-plus`, `vanilla` | Allows for mod enabling + disabling right from the mod page, shows an icon and other relevant metadata |
| `codename` | Shows a list of installed mods and some metadata |
| `kade`, `pre-vslice`, `other` | Cosmetic only, has no effect other than showing icon |

If anything else is entered in this field, it's not automatically set to anything, but it is treated as `unknown`, showing no cosmetic changes.

### `contributors`

Contributors are organized in groups, with each group containing an array of members.

Each contributor group has:

| Key | Value |
|-----|-------|
| `group` | String, the name of the group/team |
| `members` | Array of contributor objects |

Each contributor in the members array has:

| Key | Value |
|-----|-------|
| `name` | String, displayed name |
| `icon` | String, path to icon relative to the `.flight` folder |
| `role` | String, displayed role within the group |

Example:

```json
{
  "contributors": [
    {
      "group": "Epic Team",
      "members": [
        {
          "name": "Awesome Developer Man",
          "icon": "icons/adm.webp",
          "role": "Lead Developer"
        },
        {
          "name": "SomeoneElse",
          "icon": "icons/someone.png",
          "role": "Artist"
        }
      ]
    },
    {
      "group": "Music Team",
      "members": [
        {
          "name": "Composer1",
          "icon": "icons/composer1.jpg",
          "role": "Lead Composer"
        }
      ]
    }
  ]
}
```