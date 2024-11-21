# kumori-bar
A simple status bar, designed to be used with the [Komorebi tiling window manager](https://lgug2z.github.io/komorebi/index.html).

> [!WARNING]  
> Still very much WIP, use at your own risk. Currently the project very narrowly suits my own needs, at some point this might be more configurable and modular.

> [!WARNING]  
> The bar now includes an experimental media playback status/control widget. The [library](https://github.com/oplehtinen/WinPlayer-Rust) is kind of wonky, be ready to run into issues.
> This is based on the [Windows Media Control APIs](https://learn.microsoft.com/en-us/uwp/api/windows.media.control?view=winrt-26100) for cross-player support, but working with it is a pain. If anyone has a better implementation in mind, please do share! 

Thanks to developers of [Zebar](https://github.com/glzr-io/zebar) and [yasb](https://github.com/da-rth/yasb/tree/tauri-port) for making the navigation of the Komorebi api and Tauri much easier.

![preview](preview.png)

## Getting started / Download
- Experimental [builds ](https://github.com/oplehtinen/kumori-bar/releases)are provided for convenience. Please bear in mind they are very much untested.

## Features

- Displays the current workspace on all monitors.
No other widgets, yet.

## Developing

### Dependencies
1. Follow the [Tauri guide](https://tauri.app/v1/guides/getting-started/prerequisites) to download the required build tools.
2. Remember to run `npm i` after.

Use `npm run tauri dev` to start up the dev server.

Uses the following software/libraries:
- [Tauri](https://tauri.app/)
- [SvelteKit](https://kit.svelte.dev/)
- [Tailwind](https://tailwindcss.com/)
- [DaisyUI](https://daisyui.com/)


### Building

To create a production version of your app:

```bash
npm run tauri build
```

## Contribution

- Contributions are welcome. 
- If you want to add a bunch of new features, consider making a fork. The codebase is fairly small.
