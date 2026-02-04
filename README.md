# Desktop Pet

A friendly penguin companion that floats on your desktop, helping you stay focused with Pomodoro timers.

## What is this?

A minimal, transparent pet that lives on your screen. Click to pat it, drag it anywhere. Use the control panel to manage timers, feed your pet, track goals, and watch it evolve.

## Features

- **Free-Floating Pet** - A transparent penguin companion that sits on top of all windows
- **Pomodoro Timer** - Three modes (15/5, 25/5, 50/10) with work/break cycling
- **Pet Care** - Feed, play, clean, and train your pet from the control panel
- **Pet Evolution** - Complete focus sessions to evolve through 3 stages
- **Quests & Events** - Random events and quests for bonus coins
- **Coin Shop** - Buy accessories: party hats, sunglasses, scarves, and snacks
- **Customization** - Change pet skins and scenes
- **Daily Goals & Tasks** - Track your productivity
- **System Tray** - Quick timer controls

## Tech Stack

Built with [Tauri 2](https://tauri.app/), React, TypeScript, and TailwindCSS. The pet is pure SVG. All data stored locally.

## Run It

```bash
npm install
npm run tauri dev
```

## Build

```bash
npm run tauri build
```

Outputs a native macOS `.app` bundle to `/Applications`.
