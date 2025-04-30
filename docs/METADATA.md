# Fridaylight Metadata

Located in the same directory as the mod's executable, mod creators have the ability to provide additional metadata to be used and displayed on its mod page by creating a folder with the name `.flight` and including any of these files:

| File | Purpose |
|------|---------|
|`banner.png`, `banner.webp`| Image file used for the mods' display banner|
|`logo.png`, `logo.webp`| Image file used for the mods' logo, displayed instead of its name|
|`metadata.json`| JSON Schema used for including mod metadata to be shown on the sidebar and mod page|

## metadata.json

### Schema

None of these are required. You can you whichever ones you need!

| Key | Value |
|-----|-------|
|`name`| String, display name for the mod|
|`version`| String, displayed version for the mod |
|`engine`| Object, see more details below |
|`description`| String, displayed text blurb under the mod's banner |
|`logo_position`| String, controls the position of the logo in the banner.<br>`left_bottom`, `left_middle`, `middle` |


### `engine` Object

| Key | Value |
|-----|-------|
| `engine_type` | String, this allows for mods folder mod management and small cosmetic changes. See more details below. |
| `engine_name` | String, shows a tooltip on engine icon hover |
| `engine_icon` | String, path to icon relative to the `.flight` folder |
| `mods_folder` | Boolean, tells if you want to show the mods folder mod management.<br> Default is `false`. |
| `mods_folder_path` | String, path to mods folder relative to the executable directory<br>Default is `mods`. |

### Valid `engine_type` Values

| Value | Features |
|-------|----------|
|`psych`, `fps-plus`, `vanilla` | Allows for mod enabling + disabling right from the mod page, shows an icon and other relevant metadata |
| `codename` | Shows a list of installed mods and some metadata |
| `kade`, `pre-vslice`, `other` | Cosmetic only, has no effect other than showing icon |

### Example

```json
{
  "name": "VS Impostor",
  "version": "1.0.0",
  "description": "Codenamed UPDOG, this brand-new version is a full-on reimagining built from the ground up...",
  "logo_position": "middle",
  "contributors": [
    {
      "name": "Clowfoe",
      "icon": "path/to/icon/clowfoe.png",
      "title": "Director"
    }
  ],
  "engine": {
    "engine_type": "psych",
    "engine_name": "Nightmare Vision",
    "engine_icon": "path/to/icon",
    "mods_folder": false,
    "mods_folder_path": "",
  }
}
```