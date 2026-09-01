# FerrumWeave website

This directory contains the small React + Vite site published through GitHub Pages.

The website intentionally uses two visual modes:

- the **forge** — dark, metallic hero artwork for identity and storytelling;
- the **community** — clean, light, accessible surfaces for documentation, contribution, and trust.

Bootstrap provides grid and spacing utilities. Fluent UI 2 provides interactive controls and theme tokens. FerrumWeave's own CSS supplies the brand skin rather than replacing either system.

## Local development

From `docs/site`:

```bash
npm install
npm run dev
```

Production validation:

```bash
npm run build
```

The site consumes the repository's existing `assets/brand/` directory as Vite's public directory. Brand artwork therefore remains single-source rather than being copied into the website.

## Publishing

Pull requests to `dev` or `main` run the site build workflow. GitHub Pages deployment runs only from `main` (or manually), so integration work does not automatically become the public website.

The public URL is expected to be:

`https://chicodotnet.github.io/FerrumWeave/`
