# Deploying PMDaemon Documentation to GitHub Pages

This guide will help you deploy the PMDaemon documentation to GitHub Pages using the automated workflow.

## Prerequisites

Before deploying, ensure you have:

1. **Repository Setup**: The repository is properly configured with GitHub Pages
2. **Documentation Built**: The documentation builds successfully locally
3. **GitHub Actions**: GitHub Actions are enabled for the repository
4. **Permissions**: Proper permissions for GitHub Pages deployment

## Automated Deployment (Recommended)

The repository includes a GitHub Actions workflow that automatically deploys documentation to GitHub Pages when changes are pushed.

### Setting Up Automated Deployment

1. **Enable GitHub Pages**:
   - Go to your repository settings on GitHub
   - Navigate to "Pages" section
   - Under "Source", select "GitHub Actions"
   - This allows the workflow to deploy to GitHub Pages

2. **Configure Repository Settings**:
   ```bash
   # Ensure the docusaurus.config.js has correct settings
   url: 'https://entrepeneur4lyf.github.io'
   baseUrl: '/pmdaemon/'
   organizationName: 'entrepeneur4lyf'
   projectName: 'pmdaemon'
   ```

3. **Trigger Deployment**:
   The deployment will automatically trigger when:
   - Changes are pushed to the `main` or `master` branch
   - Changes affect files in the `docs/` directory
   - README.md, CHANGELOG.md, or RELEASE*.md files are modified

### Manual Trigger

You can also manually trigger the deployment:

1. Go to the "Actions" tab in your GitHub repository
2. Select "Deploy Documentation to GitHub Pages"
3. Click "Run workflow"
4. Select the branch and click "Run workflow"

## Manual Deployment (Alternative)

If you prefer manual deployment or need to troubleshoot:

### Local Build and Deploy

1. **Navigate to docs directory**:
   ```bash
   cd docs/
   ```

2. **Install dependencies**:
   ```bash
   bun install
   ```

3. **Build the documentation**:
   ```bash
   bun run build
   ```

4. **Deploy to GitHub Pages**:
   ```bash
   # Set up deployment credentials
   git config --global user.email "action@github.com"
   git config --global user.name "GitHub Action"

   # Deploy using Docusaurus
   GIT_USER=entrepeneur4lyf bun run deploy
   ```

### Using npm instead of bun

If you prefer npm:

```bash
# Install dependencies
npm install

# Build documentation
npm run build

# Deploy
GIT_USER=entrepeneur4lyf npm run deploy
```

## Verifying Deployment

After deployment (either automated or manual):

1. **Check GitHub Pages URL**:
   - Visit: https://entrepeneur4lyf.github.io/pmdaemon/
   - The site should load with the latest documentation

2. **Verify Content**:
   - Check that the v0.1.2 release notes are visible
   - Verify ecosystem configuration documentation is present
   - Test navigation and search functionality

3. **Check Build Status**:
   - Go to repository "Actions" tab
   - Verify the "Deploy Documentation to GitHub Pages" workflow succeeded
   - Check for any error messages in the workflow logs

## Troubleshooting

### Common Issues

1. **Build Fails**:
   ```bash
   # Check for syntax errors in markdown files
   bun run build

   # Fix any broken links or invalid frontmatter
   ```

2. **Deployment Fails**:
   - Verify GitHub Pages is enabled in repository settings
   - Check that the workflow has proper permissions
   - Ensure the repository name matches the configuration

3. **Site Doesn't Update**:
   - GitHub Pages can take a few minutes to update
   - Check browser cache (try incognito/private browsing)
   - Verify the deployment actually succeeded

### Debug Build Locally

To debug issues:

```bash
cd docs/

# Clear cache
bun run clear

# Install dependencies fresh
rm -rf node_modules/
bun install

# Build and serve locally
bun run build
bun run serve
```

Visit `http://localhost:3000` to test locally.

## Updating Documentation

### For New Releases

1. **Update Version Numbers**:
   - Update `docs/package.json` version
   - Add new changelog entry in `docs/changelog/`
   - Update main documentation files

2. **Test Locally**:
   ```bash
   cd docs/
   bun run start
   ```

3. **Commit and Push**:
   ```bash
   git add .
   git commit -m "docs: update for v0.1.2 release"
   git push origin main
   ```

4. **Verify Deployment**:
   - Wait for GitHub Action to complete
   - Check the live site

### Content Updates

For regular content updates:

1. Edit files in `docs/docs/` directory
2. Test locally with `bun run start`
3. Commit and push changes
4. Automatic deployment will handle the rest

## File Structure

```
docs/
├── .docusaurus/           # Docusaurus build cache
├── build/                 # Built static files (generated)
├── docs/                  # Documentation content
├── src/                   # Custom React components
├── static/                # Static assets
├── changelog/             # Release notes
├── docusaurus.config.js   # Docusaurus configuration
├── package.json           # Dependencies and scripts
└── sidebars.js           # Sidebar configuration
```

## GitHub Actions Workflow

The deployment workflow (`.github/workflows/deploy-docs.yml`) handles:

- Installing Bun and dependencies
- Building the documentation
- Deploying to GitHub Pages
- Setting up proper permissions

The workflow runs on:
- Push to main/master branch (when docs files change)
- Manual trigger via GitHub Actions UI
- Pull requests (build-only, no deployment)

---

For additional help, see the [Docusaurus deployment documentation](https://docusaurus.io/docs/deployment#deploying-to-github-pages).
