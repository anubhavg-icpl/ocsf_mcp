# OCSF MCP Server - Assets

**Maintainer:** Anubhav Gain (anubhavg-cipl) <anubhavg@infopercept.com>

This directory contains all visual assets for the OCSF MCP Server project.

## 📁 Directory Structure

```
assets/
├── icons/              # Logo and icon files
│   ├── icon.svg       # Primary SVG logo (recommended)
│   ├── icon-512.png   # Primary PNG logo (512x512)
│   ├── icon-256.png   # Medium size PNG
│   ├── icon-dark.svg  # Dark theme variant
│   └── favicon/       # Favicon sizes
│       ├── favicon-16x16.png
│       ├── favicon-32x32.png
│       ├── favicon-48x48.png
│       └── favicon.ico
│
├── banners/            # Header and banner images
│   ├── github-social-preview.png  # 1280x640
│   ├── readme-header.png          # 1200x300
│   └── docker-hub-header.png      # 1920x400
│
└── social/             # Social media images
    ├── twitter-card.png           # 1200x630
    └── linkedin-post.png          # 1200x627
```

## 🎨 Asset Guidelines

### Icons
- **Format:** SVG (preferred) or PNG with transparency
- **Primary Size:** 512x512px
- **Colors:** Blue-cyan gradient (#1E3A8A to #06B6D4)
- **Style:** Modern, minimal, professional

### Banners
- **Format:** PNG
- **Style:** High-tech cybersecurity theme
- **Background:** Dark with blue-cyan accents

### Favicons
- **Sizes:** 16x16, 32x32, 48x48, 64x64px
- **Format:** PNG or ICO
- **Style:** Simplified version of primary icon

## 🛠️ Generating Assets

See [ICON-PROMPTS.md](../ICON-PROMPTS.md) for detailed AI prompts and generation instructions.

### Quick Commands

```bash
# Create icons from SVG
convert icon.svg -resize 512x512 icon-512.png
convert icon.svg -resize 256x256 icon-256.png

# Create favicons
convert icon-512.png -resize 16x16 favicon/favicon-16x16.png
convert icon-512.png -resize 32x32 favicon/favicon-32x32.png

# Optimize PNGs
pngquant icon-*.png --quality=80-95
```

## 📝 Usage

### Docker Hub
Use `icon-512.png` as repository icon

### GitHub
1. Repository icon: `icon-512.png`
2. Social preview: `banners/github-social-preview.png`

### Documentation
- README header: `banners/readme-header.png`
- Favicon: Files in `favicon/` directory

### MCP Catalog
Use `icon-256.png` for catalog listing

## 🎯 Design Specifications

### Color Palette
```
Primary Blue:  #1E40AF
Cyan:          #06B6D4
Purple Accent: #8B5CF6
Dark Navy:     #0F172A
Light Cyan:    #E0F2FE
```

### Gradients
```css
/* Shield Gradient */
background: linear-gradient(135deg, #1E3A8A 0%, #06B6D4 100%);

/* Accent Gradient */
background: linear-gradient(135deg, #4F46E5 0%, #22D3EE 100%);
```

## 📞 Support

Questions about assets?
- Email: anubhavg@infopercept.com
- See: [ICON-PROMPTS.md](../ICON-PROMPTS.md)

---

**Note:** Currently, assets are placeholders. Generate using prompts in ICON-PROMPTS.md
