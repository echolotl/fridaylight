# Fridaylight Metadata
Located in the same directory as the mod's executable, mod creators have the ability to provide additional metadata to be used and displayed on its mod page by creating a folder with the name `.flight` and including any of these files:
| File | Purpose |
|------|---------|
|`banner.png`, `banner.webp`| Image file used for the mods' display banner|
|`logo.png`, `logo.webp`| Image file used for the mods' logo, displayed instead of its name|
|`metadata.json`| JSON Schema used for including mod metadata to be shown on the sidebar and mod page|

## metadata.json

### Schema

| Key | Value |
|-----|-------|
|`name`| String, display name for the mod|
|`version`| String, displayed version for the mod |
|`engine_type`| String, see valid values below. Used when searching for `/mods` folder mods, displays an icon |
|`description`| String, displayed text blurb under the mod's banner |

### Example 
```json
{
  "name": "VS Impostor V4",
  "version": "4.1.0",
  "engine_type": "Psych",
  "description": "A Friday Night Funkin' total conversion mod featuring 62 brand new songs spread out across 12 weeks..."
}
```

### `engine_type` Values
| Value | Features |
|-------|----------|
|`psych`, `fps-plus`, `vanilla` | Allows for mod enabling + disabling right from the mod page, shows an icon and other relevant metadata |
| `codename` | Shows a list of installed mods and some metadata |
| `kade`, `pre-vslice` | Cosmetic only, has no effect other than showing icon |
