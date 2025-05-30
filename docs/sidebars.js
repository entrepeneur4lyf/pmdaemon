/**
 * Creating a sidebar enables you to:
 - create an ordered group of docs
 - render a sidebar for each doc of that group
 - provide next/previous navigation

 The sidebars can be generated from the filesystem, or explicitly defined here.

 Create as many sidebars as you want.
 */

// @ts-check

/** @type {import('@docusaurus/plugin-content-docs').SidebarsConfig} */
const sidebars = {
  // By default, Docusaurus generates a sidebar from the docs folder structure
  tutorialSidebar: [
    {
      type: 'category',
      label: 'Getting Started',
      items: [
        'getting-started/introduction',
        'getting-started/installation',
        'getting-started/quick-start',
        'getting-started/migration-from-pm2',
      ],
    },
    {
      type: 'category',
      label: 'Architecture',
      items: [
        'architecture/overview',
      ],
    },
    {
      type: 'category',
      label: 'Core Features',
      items: [
        'features/process-management',
        'features/port-management',
        'features/health-checks',
        'features/monitoring',
        'features/web-api',
        'features/configuration',
      ],
    },
    {
      type: 'category',
      label: 'CLI Reference',
      items: [
        'cli/commands',
        'cli/configuration-options',
        'cli/environment-variables',
        'cli/exit-codes',
      ],
    },
    {
      type: 'category',
      label: 'Configuration',
      items: [
        'configuration/ecosystem-files',
        'configuration/process-configuration',
        'configuration/advanced-configuration',
        'configuration/schema-validation',
      ],
    },
    {
      type: 'category',
      label: 'API Documentation',
      items: [
        'api/rest-api',
        'api/websocket-api',
        'api/library-usage',
        'api/api-examples',
      ],
    },
    {
      type: 'category',
      label: 'Examples & Tutorials',
      items: [
        'examples/use-cases',
        'examples/deployment-examples',
        'examples/integration-examples',
      ],
    },
    {
      type: 'category',
      label: 'Advanced Topics',
      items: [
        'advanced/performance-tuning',
        'advanced/security',
        'advanced/clustering',
        'advanced/logging',
        'advanced/troubleshooting',
      ],
    },
    {
      type: 'category',
      label: 'Comparison',
      items: [
        'comparison/pm2-vs-pmdaemon',
      ],
    },
  ],
};

export default sidebars;
