# MGS2 ASI Mod Config Editor

## Table of Contents
1. [Introduction](#introduction)
2. [Features](#features)
3. [Project Structure](#project-structure)
4. [Getting Started](#getting-started)
5. [Usage](#usage)
6. [Technology Stack](#technology-stack)
7. [Development Quickstart](#development-quickstart)
8. [Contributing](#contributing)


## Introduction

The MGS2 ASI Mod Config Editor is a desktop application tha provides a user-friendly interface for editing and managing configuration files for the [ASI plugin](https://mgs.w00ty.com/mgs2/asi/) created by [bmn](https://github.com/bmn) and used by speedrunners for the PC version of Metal Gear Solid 2: Substance. This tool simplifies the process of customizing various aspects of the game, making it easier for players to tailor their gaming experience.

## Features

- **Intuitive UI**: A clean, responsive interface for easy navigation and editing of INI files.
- **Real-time Editing**: Edit INI files with immediate visual feedback.
- **File Management**: Automatically manages and ensures the existence of necessary INI files.
- **Categorized Mods**: Organizes mods into categories like HUD, Turbo, Modifiers, and more for easy access.
- **Color Picker**: Built-in color picker for easy customization of color-related settings.
- **Boolean Toggles**: Checkbox toggles for easy enabling/disabling of features.
- **Sorted Display**: Alphabetically sorted sections and keys for consistent viewing.
- **Responsive Design**: Adapts to different screen sizes, including a collapsible sidebar for mobile devices.
- **Help Documentation**: Comprehensive in-app help section explaining mod configurations and shortcuts.

## Usage
- Download the latest release & launch the installer. When asked where to install the application, choose the `bin\scripts` folder where you have MGS2 installed.
  - For example: `C:\Program Files (x86)\GOG Galaxy\Games\Metal Gear Solid 2 Substance\bin\scripts`.
- Just to be safe, make a backup of your existing `.ini` files before using this application
- Use the sidebar to navigate between different mod categories.
- Click on a specific INI file to view and edit its contents.
- Make changes using the provided inputs (text fields, checkboxes, color pickers).
- Click "Save" to apply your changes or "Cancel" to discard them.
- Refer to the Help section for detailed information on configuring mods and using shortcuts.

## Technology Stack

- **Frontend**: [Vue.js 3](https://vuejs.org/) with [TypeScript](https://www.typescriptlang.org/)
- **Backend**: [Rust](https://www.rust-lang.org/) with [Tauri framework](https://tauri.app/)
- **Styling**: [SCSS](https://sass-lang.com/)
- **Icons**: [FontAwesome](https://fontawesome.com/)
- **UI Components**: Custom components and [vue-sidebar-menu](https://yaminncco.github.io/vue-sidebar-menu/#/)

## Development Quickstart
- Install the [prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites)
- Clone the repository
- Install frontend dependencies:
```shell
pnpm install
```
- Run the development server:
```shell
pnpm tauri dev
```
- Build for production:
```shell
pnpm tauri build
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

- Fork the repository
- Create your feature branch (`git checkout -b feature/AmazingFeature`)
- Commit your changes (`git commit -m 'Add some AmazingFeature'`)
- Push to the branch (`git push origin feature/AmazingFeature`)
- Open a Pull Request

---
## License

This project is licensed under the [GNU General Public License](LICENSE).
