# PMDaemon Documentation

This directory contains the complete Docusaurus documentation for PMDaemon.

## Quick Start

### Prerequisites

- Node.js 18+ 
- npm or yarn

### Installation

```bash
cd docs
npm install
```

### Development

```bash
# Start development server
npm start

# Build for production
npm run build

# Serve production build
npm run serve
```

## Documentation Structure

```
docs/
├── docs/                          # Documentation content
│   ├── getting-started/           # Getting started guides
│   │   ├── introduction.md        # Project introduction
│   │   ├── installation.md        # Installation guide
│   │   ├── quick-start.md         # Quick start tutorial
│   │   └── migration-from-pm2.md  # PM2 migration guide
│   ├── features/                  # Core features
│   │   ├── process-management.md  # Process lifecycle
│   │   ├── port-management.md     # Advanced port features
│   │   ├── health-checks.md       # Health monitoring
│   │   ├── monitoring.md          # Real-time monitoring
│   │   ├── configuration.md       # Configuration options
│   │   └── web-api.md            # Web API features
│   ├── cli/                      # CLI reference
│   │   ├── commands.md           # Complete command reference
│   │   ├── configuration-options.md # Configuration options
│   │   ├── environment-variables.md # Environment variables
│   │   └── exit-codes.md         # Exit codes reference
│   ├── configuration/            # Configuration guides
│   │   ├── ecosystem-files.md    # Ecosystem configuration
│   │   ├── schema-validation.md  # Schema validation
│   │   ├── environment-specific.md # Environment configs
│   │   └── best-practices.md     # Configuration best practices
│   ├── api/                      # API documentation
│   │   ├── rest-api.md          # REST API reference
│   │   ├── websocket-api.md     # WebSocket API
│   │   ├── library-usage.md     # Rust library usage
│   │   └── error-handling.md    # Error handling guide
│   ├── examples/                 # Examples and tutorials
│   │   ├── use-cases.md         # Real-world use cases
│   │   ├── node-js-apps.md      # Node.js examples
│   │   ├── python-apps.md       # Python examples
│   │   ├── clustering.md        # Clustering examples
│   │   ├── health-checks.md     # Health check examples
│   │   └── docker-integration.md # Docker integration
│   ├── advanced/                 # Advanced topics
│   │   ├── performance-tuning.md # Performance optimization
│   │   ├── security.md          # Security considerations
│   │   ├── logging.md           # Logging configuration
│   │   ├── troubleshooting.md   # Troubleshooting guide
│   │   └── contributing.md      # Contributing guide
│   └── comparison/               # Comparisons
│       ├── pm2-vs-pmdaemon.md   # PM2 vs PMDaemon
│       ├── feature-matrix.md    # Feature comparison matrix
│       └── performance.md       # Performance benchmarks
├── blog/                         # Blog posts
├── src/                         # React components
│   ├── components/              # Custom components
│   ├── css/                     # Custom CSS
│   └── pages/                   # Custom pages
├── static/                      # Static assets
│   └── img/                     # Images and icons
├── docusaurus.config.js         # Docusaurus configuration
├── sidebars.js                  # Sidebar configuration
└── package.json                 # Dependencies
```

## Features

### Documentation Features

- **Comprehensive Coverage** - Complete documentation for all PMDaemon features
- **PM2 Migration Guide** - Step-by-step migration from PM2
- **API Reference** - Complete REST API and WebSocket documentation
- **Examples & Tutorials** - Real-world use cases and examples
- **CLI Reference** - Complete command documentation
- **Comparison Guides** - Detailed PM2 vs PMDaemon comparison

### Docusaurus Features

- **Modern UI** - Clean, responsive design
- **Search** - Full-text search with Algolia
- **Mermaid Diagrams** - Architecture and flow diagrams
- **Code Highlighting** - Syntax highlighting for multiple languages
- **Mobile Responsive** - Works on all devices
- **Dark Mode** - Built-in dark/light theme toggle

### Technical Features

- **Multiple Formats** - Supports Markdown, MDX, React components
- **Versioning** - Documentation versioning support
- **Internationalization** - Ready for multiple languages
- **SEO Optimized** - Meta tags, sitemap, structured data
- **Fast Loading** - Optimized build with code splitting

## Content Guidelines

### Writing Style

- **Clear and Concise** - Use simple, direct language
- **Example-Driven** - Include practical examples for all features
- **User-Focused** - Write from the user's perspective
- **Consistent Terminology** - Use consistent terms throughout

### Code Examples

- **Complete Examples** - Provide full, runnable examples
- **Multiple Languages** - Show examples in different languages when relevant
- **Real-World Scenarios** - Use realistic use cases
- **Error Handling** - Include error handling in examples

### Structure

- **Logical Flow** - Organize content in logical progression
- **Cross-References** - Link related topics
- **Table of Contents** - Use clear headings and structure
- **Visual Aids** - Include diagrams and screenshots where helpful

## Deployment

### GitHub Pages

The documentation is automatically deployed to GitHub Pages on push to main:

```bash
# Build and deploy
npm run build
npm run deploy
```

### Custom Domain

To use a custom domain:

1. Add `CNAME` file to `static/` directory
2. Configure DNS to point to GitHub Pages
3. Update `docusaurus.config.js` with custom URL

### Other Platforms

The documentation can be deployed to:
- **Netlify** - Connect GitHub repo for automatic deployment
- **Vercel** - Import project for automatic deployment  
- **AWS S3** - Upload build output to S3 bucket
- **Docker** - Use nginx to serve static files

## Contributing

### Adding New Documentation

1. **Create new file** in appropriate directory
2. **Add to sidebar** in `sidebars.js`
3. **Follow style guide** for consistency
4. **Test locally** before submitting
5. **Submit pull request** with description

### Updating Existing Documentation

1. **Check for accuracy** - Ensure information is current
2. **Maintain consistency** - Follow existing patterns
3. **Update cross-references** - Fix any broken links
4. **Test changes** - Verify locally before submitting

### Documentation Standards

- **Markdown Format** - Use standard Markdown syntax
- **Front Matter** - Include title and description
- **Code Blocks** - Use appropriate language tags
- **Links** - Use relative links for internal content
- **Images** - Optimize images and use descriptive alt text

## Maintenance

### Regular Updates

- **Version Updates** - Update version numbers when PMDaemon releases
- **Feature Updates** - Document new features as they're added
- **Link Checking** - Regularly check for broken links
- **Content Review** - Review content for accuracy and relevance

### Performance Monitoring

- **Build Times** - Monitor build performance
- **Bundle Size** - Keep bundle size optimized
- **Loading Speed** - Monitor page loading times
- **Search Quality** - Ensure search results are relevant

## Support

For documentation issues:

- **GitHub Issues** - Report documentation bugs or requests
- **Discussions** - Ask questions about documentation
- **Pull Requests** - Submit improvements directly

---

This documentation provides comprehensive coverage of PMDaemon's features and capabilities, making it easy for users to get started and make the most of the advanced process management features.
# Trigger rebuild
