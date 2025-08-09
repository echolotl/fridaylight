# ![](/docs/faqs.png)

## So... what even is the point of this?

I made this originally because I wanted a better way to manage my mods, as I was tired of having to find the one I wanted to play by sifting through folders. And then while making that, I wanted to tackle the problem of mod downloading, mostly just for convinience for myself. This definitely isn't for everyone, and I personally think that for mods that are installed to an engine, an in-game mod manager would be better.

## Why use this instead of [Funkin Launcher Legacy](https://gamebanana.com/tools/17526), [Funkin Launcher 2](https://gamebanana.com/wips/92420), or [FileDaddy](https://gamebanana.com/tools/7015)?

I mean, if you want to use FileDaddy or Funkin Launcher, go ahead I guess. Even though FileDaddy and Funkin Launcher Legacy aren't maintained anymore, they all have really cool people behind them, and are both good at a couple of their own respective things!

Fridaylight is more focused on the mod management side of things, so if you want to keep your standalone mods organized, this is probably the best option. It also combines a lot of the features from all of the launchers listed previously, so this might be a good all-in-one solution.

## Why is this Windows only? Can't Tauri be multiplatform?

Tauri can be multiplatform, but there's just a couple of things that I do that are very specific to Windows, like retreiving icons from the executables, some file management stuff, and the Mica theming. I also do not own a Mac to test this on at all. If you want to help port some of those features, please consider making a pull request.

## Why does this take so long to download things?

Most of the time, it's not the app's fault. Gamebanana is notorious for its slow downloading speeds, and its the main reason why most creators attach a Mediafire or Google Drive link on their mod pages. Sometimes, it might just be better to download the mods manually from those links, and then import them into Fridaylight.

## I just downloaded a modpack for an engine through Fridaylight, but it's not showing up in the list or in-game. Why?

This is most likely an issue with how the modpack creator packaged their mod. If they included a README or installation instructions outside of the base folder, Fridaylight will not automatically move it up a level. You can manually navigate to the folder and move it out to make it show up, though.

## How do I create a custom theme?

You can theme the app using css by placing a css file in the %APPDATA%/themes directory, and you can basically put anything you want. For a simple color theme, you can use this template:
```css
:root {
/* Used for the main app background */
--theme-bg: #000000;

/* Used for the mod cards */
--theme-card: #111111;

/* Used for other app backgrounds */
--theme-surface: #222222;

/* Used for text */
--theme-text: #ffffff;

/* Used for captions, etc */
--theme-text-secondary: #eeeeee;

/* Used for borders around the app */
--theme-border: #999999;

/* Background of modals */
--theme-solid: #111111;
}
```

## Where should I download this from?

The only places you should download this from are the [GitHub releases page](https://github.com/echolotl/fridaylight/releases), the Gamebanana page (when it releases), or eventually from [echolotl.lol/fridaylight](https://www.echolotl.lol/fridaylight) (also when it releases).

## Speaking of which, when is this releasing?

Sometime in September 2025, hopefully. There's still a few things I want to add or improve on before I feel okay with releasing a full version or beta or whatever. You can help test right now, though, by downloading the latest pre-release version from the [GitHub releases page](https://github.com/echolotl/fridaylight/releases)!

## Who is this guy???? <img align="right" src="https://i.imgur.com/DcEYLmb.png" alt="Light the Lamp" width="128" height="173" />

Well duh, that's Light! You didn't think I just randomly came up with "Fridaylight", maybe because it sounds like "Friday Night", on the spot, while submitting the original Gamebanana WIP, right? Wrong. It's entirely named on them and no other reason.

**No more questions**. FAQ over.
