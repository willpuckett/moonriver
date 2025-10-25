# Moonriver Documentation

This directory contains the VitePress documentation website for Moonriver.

## Development

### Prerequisites

- Node.js 18 or later
- npm

### Install Dependencies

```bash
cd docs
npm install
```

### Run Development Server

```bash
npm run docs:dev
```

The site will be available at http://localhost:5173 (note: this link won't work in markdown preview)

### Build for Production

```bash
npm run docs:build
```

The built site will be in `.vitepress/dist/`

### Preview Production Build

```bash
npm run docs:preview
```

## Structure

```
docs/
├── .vitepress/
│   └── config.mts          # VitePress configuration
├── public/
│   └── logo.svg            # Site logo
├── guide/
│   ├── what-is-moonriver.md
│   ├── getting-started.md
│   ├── quick-start.md
│   ├── interactive-mode.md
│   ├── scripting-mode.md
│   ├── configuration.md
│   └── multiple-printers.md
├── features/
│   ├── tab-completion.md
│   ├── syntax-highlighting.md
│   ├── command-history.md
│   └── emergency-stop.md
├── api/
│   └── index.md
├── contributing/
│   └── development.md
└── index.md                # Home page
```

## GitHub Pages Deployment

The documentation is automatically deployed to GitHub Pages when changes are pushed to the `main` branch.

### Setup GitHub Pages

1. Go to repository Settings → Pages
2. Set Source to "GitHub Actions"
3. The workflow in `.github/workflows/docs.yml` will handle deployment

### Custom Domain (Optional)

To use a custom domain:

1. Add a `docs/public/CNAME` file with your domain
2. Configure DNS to point to GitHub Pages
3. Update `base` in `.vitepress/config.mts` if needed

## Writing Documentation

### Markdown Extensions

VitePress supports extended Markdown features:

#### Code Groups

```markdown
::: code-group

```bash [Interactive]
moonriver --host printer.local
```

```bash [Scripting]
moonriver --host printer.local G28
```

:::
```

#### Tip/Warning/Danger Boxes

```markdown
::: tip
This is a helpful tip!
:::

::: warning
Be careful with this!
:::

::: danger
This is dangerous!
:::
```

#### Custom Containers

```markdown
::: details Click to expand
Hidden content here
:::
```

### Adding New Pages

1. Create a new `.md` file in the appropriate directory
2. Add frontmatter if needed
3. Update `.vitepress/config.mts` sidebar configuration
4. Write content using Markdown

Example:

```markdown
---
title: My New Page
description: Page description
---

# My New Page

Content goes here...
```

## Configuration

Edit `.vitepress/config.mts` to:

- Change site title and description
- Modify navigation menu
- Update sidebar structure
- Configure theme options
- Add social links

## Styling

VitePress uses a default theme that can be customized:

- Edit `.vitepress/theme/index.ts` for custom theme
- Add CSS in `.vitepress/theme/style.css`
- Override theme variables

## Testing

Before committing documentation changes:

1. Build locally: `npm run docs:build`
2. Preview: `npm run docs:preview`
3. Check all links work
4. Verify code examples are correct
5. Test on mobile viewport

## Resources

- [VitePress Documentation](https://vitepress.dev/)
- [Markdown Extensions](https://vitepress.dev/guide/markdown)
- [Theme Configuration](https://vitepress.dev/reference/default-theme-config)
