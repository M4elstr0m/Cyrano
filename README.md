# cyrano​​     <img src="gui/assets/logo-80.png" alt="Cyrano logo" width="64" height="64" align="middle">

Your favorite CV writer using HTML templates! Your CVs made with panache, using Cyrano!

![Stars Badge](https://img.shields.io/github/stars/M4elstr0m/cyrano?style=flat&color=yellow&label=Stars)
![License Badge](https://img.shields.io/badge/License-MIT-lightgrey)
![Rust Badge](https://img.shields.io/badge/Rust-v1.98.0-orange?logo=rust&logoColor=white)
![Dioxus Badge](https://img.shields.io/badge/Dioxus-v0.7-132135)

Please star & share this repository if you find it useful :star:

<p align="center">
  <img src="gui/assets/logo-192.png" alt="Cyrano" width="192" height="192">
</p>

## Table of Contents

- [Features](#features)
- [Installation](#installation-computer)
- [Usage](#usage-books)
- [Templates](#templates-jigsaw)
- [Roadmap](#roadmap-world_map)
- [Credits](#credits-newspaper)

## Features

- **Live preview**: edit your data in a form and see the rendered CV next to it.
- **PDF export**: HTML templates are converted to PDF using [Mestra](https://github.com/M4elstr0m/Mestra).
- **Extensible templates**: drop a folder with a `TEMPLATE.cyrano` TOML marker in your templates directory and it shows up in the app. Data is injected using [Tera](https://keats.github.io/tera/).
- **Encrypted profiles**: every profile is encrypted on disk as soon as it is written (Argon2 key derivation, XChaCha20-Poly1305).
- **Autosave**: changes are saved 700 ms after you stop typing, and flushed on exit.
- **Native renderer**: a Dioxus 0.7 desktop app on Blitz, Vello and WGPU, with no webview.

## Installation :computer:

> [!CAUTION]
> Cyrano requires a Chromium (Chrome) binary available on your `PATH` for PDF conversion.

Download an executable from the [release tab](https://github.com/M4elstr0m/cyrano/releases).

![wayland](https://img.shields.io/badge/Wayland-FFBC00?logo=wayland&logoColor=black&style=for-the-badge)

On Wayland the window icon comes from a `.desktop` file you must setup after your installation.
```sh
git clone https://github.com/M4elstr0m/cyrano.git
cd cyrano/gui/scripts/
chmod +x wayland-desktop-entry.sh
./wayland-desktop-entry.sh
```

## Usage :books:

1. Create a profile, with or without a password.
2. Pick a template from the home screen.
3. Fill in your data. The preview updates as you type.
4. Export. The PDF is named `[Person]-[Template].pdf` and written to your output directory (selected in the Settings panel).

## Templates :jigsaw:

A template is a folder containing an HTML entry file and a `TEMPLATE.cyrano` marker. Templates are looked up in `~/.local/share/cyrano/templates/`.

![Tera](https://img.shields.io/badge/Tera-132135?style=for-the-badge)
![HTML](https://img.shields.io/badge/HTML5-E34F26?logo=html5&logoColor=white&style=for-the-badge)

Available template fields: `name`, `title`, `description`, `age`, `location`, `email`, `phonenumber`, `links`, `projects`, `experience`, `education`, `hard_skills`, `soft_skills`, `languages`, `hobbies`.

Template creation remains really free, you can have a complete control over your design.

## Roadmap :world_map:

- [x] Encrypted profiles with password protection
- [x] Template system with PDF conversion and required assets
- [x] Native desktop GUI with live preview
- [x] Light and dark themes
- [ ] CvData does not save when switching from a template to another, which leads to data loss
- [ ] Fix slow memory growth during heavy preview use
- [ ] Default templates
- [ ] :crab: v1.0.0 

## Credits :newspaper:

- [Dioxus](https://dioxuslabs.com/) and [Blitz](https://github.com/DioxusLabs/blitz) for the native renderer
- [Tera](https://keats.github.io/tera/) for templating
- [hayro](https://github.com/LaurenzV/hayro) for PDF rasterisation
- [Mestra](https://github.com/M4elstr0m/Mestra) for PDF conversion
- [Raleway](https://fonts.google.com/specimen/Raleway) typeface
