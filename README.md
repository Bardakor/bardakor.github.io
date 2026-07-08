# Liam Abourousse Portfolio

Brutalist single-page portfolio for Liam Abourousse.

The site presents Liam's software/data engineering work around Python backends, data products, B2B SaaS tooling, maritime operations, and the Voyage Report mémoire: moving a data product from demonstrable prototype to operated production service with generative AI under human validation.

## Stack

- Rust compiled to WebAssembly
- Leptos 0.7, client-side rendered
- Tailwind CSS v4 plus scoped CSS
- GSAP, ScrollTrigger, and Lenis for motion
- Trunk for local development and static builds

## Content

Most portfolio copy lives in `content.json`.

Static assets live in `public/`:

- `liam-portrait-white.jpg`
- `liam-portrait-black.jpg`
- `CV_SOFT.pdf`
- `CV_SOFT_EN.pdf`
- `Memoire_Liam_Abourousse.pdf`
- `og-image.jpg`

SEO, social preview metadata, copied public files, and the Trunk Rust entry are configured in `index.html`.

## Local Development

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk
npm ci
trunk serve --open --public-url /
```

The existing GitHub Pages workflow builds with:

```bash
trunk build --release --public-url /
```

Production site: https://bardakor.github.io/

If the repository or Pages path changes, update these in the same pass:

- `index.html` canonical, Open Graph URL, Twitter URL, and `data-public-url`
- `.github/workflows/deploy.yml` public URL
- `public/sitemap.xml`
- `public/robots.txt`

## Verification

Useful local checks:

```bash
npm ci
npx @tailwindcss/cli -i styles/input.css -o styles/tailwind-output.css
python3 -m json.tool content.json >/dev/null
trunk build --release --public-url /
```

## License

MIT — see [LICENSE](LICENSE).
