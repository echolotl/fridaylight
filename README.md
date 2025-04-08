# Fridaylight ![](https://img.shields.io/badge/Vue.js-35495E?style=for-the-badge&logo=vue.js&logoColor=4FC08D)
A mod manager for Friday Night Funkin mods! Built using Tauri and Vue.

## Features

- An organized UI, including a mod list sidebar and a dedicated mod page.
- Integrated GameBanana browsing + downloading
- UI Customization*
  - Currently limited to accent color
- Mod metadata support
- Mod launching

### WIP Features

- [ ] Engine mod management
- [ ] Folder organization
- [ ] Custom CSS/Themes
- [ ] More metadata support *(feel free to recommend ideas in the Issues tab)*
- [ ] Gamebanana download version picker
- [ ] Gamebanana deep linking
- [ ] Non-Gamebanana download links
- [ ] Import folder of FNF mod folders *(for migration)*
- [ ] Linux/Mac builds
- [ ] More unique UI, with same organization
- [ ] Psych engine achievement display

## Metadata Structure
Located in the same directory as the mod's executable, mod creators have the ability to provide additional metadata to be used and displayed on its mod page by creating a folder with the name `.flight` and including any of these files:
| File | Purpose |
|------|---------|
|`banner.png`, `banner.webp`| Image file used for the mods' display banner|
|`logo.png`, `logo.webp`| Image file used for the mods' logo, displayed instead of its name|
|`metadata.json`| JSON Schema used for including mod metadata to be shown on the sidebar and mod page|

### Example `metadata.json`
```json
{
  "name": "VS Impostor V4",
  "version": "4.1.0",
  "engine_type": "Psych Engine"
}
```
