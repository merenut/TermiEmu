# TermiEmu Themes

This directory contains example theme files for TermiEmu.

## Built-in Themes

The following themes are built into TermiEmu and don't require theme files:

- **Catppuccin Mocha** (`catppuccin-mocha`) - Soothing pastel dark theme
- **Tokyo Night** (`tokyo-night`) - Clean dark theme inspired by Tokyo's night skyline
- **Dracula** (`dracula`) - Dark theme with vibrant colors
- **Nord** (`nord`) - Arctic, north-bluish color palette

## Using Themes

To use a theme, edit your `config.toml` file and set the `theme` field:

```toml
theme = "catppuccin-mocha"
```

## Creating Custom Themes

You can create custom themes by creating TOML files in your themes directory:

- Linux/macOS: `~/.config/termiemu/themes/`
- Windows: `%APPDATA%\termiemu\themes\`

### Theme Format

See `catppuccin-mocha.toml` in this directory for an example theme file.

A theme file must include:

- `name`: Theme name
- `description`: Theme description (optional)
- `background`: Background color (RGB)
- `foreground`: Foreground color (RGB)
- `selection_background`: Selection background color (RGB)
- `ansi`: Array of 16 ANSI colors (standard 0-7, bright 8-15)

Optional fields:

- `cursor`: Cursor color (defaults to foreground)
- `selection_foreground`: Selection text color (defaults to foreground)

### Color Format

Colors are specified as RGB objects with `r`, `g`, and `b` values (0-255):

```toml
background = { r = 30, g = 30, b = 46 }
```

Or as hex strings:

```toml
background = "#1e1e2e"
```

## Installing Theme Files

1. Copy theme files to your themes directory
2. Edit `config.toml` to reference the theme name
3. Restart TermiEmu or reload configuration

## Contributing Themes

If you create a theme you'd like to share, please consider contributing it to the project!

## License

Themes in this directory are provided as examples and may be freely used and modified.
