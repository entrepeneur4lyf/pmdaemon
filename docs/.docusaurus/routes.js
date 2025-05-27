import React from 'react';
import ComponentCreator from '@docusaurus/ComponentCreator';

export default [
  {
    path: '/pmdaemon/__docusaurus/debug',
    component: ComponentCreator('/pmdaemon/__docusaurus/debug', 'bf8'),
    exact: true
  },
  {
    path: '/pmdaemon/__docusaurus/debug/config',
    component: ComponentCreator('/pmdaemon/__docusaurus/debug/config', 'a20'),
    exact: true
  },
  {
    path: '/pmdaemon/__docusaurus/debug/content',
    component: ComponentCreator('/pmdaemon/__docusaurus/debug/content', '8d3'),
    exact: true
  },
  {
    path: '/pmdaemon/__docusaurus/debug/globalData',
    component: ComponentCreator('/pmdaemon/__docusaurus/debug/globalData', '1da'),
    exact: true
  },
  {
    path: '/pmdaemon/__docusaurus/debug/metadata',
    component: ComponentCreator('/pmdaemon/__docusaurus/debug/metadata', 'e60'),
    exact: true
  },
  {
    path: '/pmdaemon/__docusaurus/debug/registry',
    component: ComponentCreator('/pmdaemon/__docusaurus/debug/registry', 'f73'),
    exact: true
  },
  {
    path: '/pmdaemon/__docusaurus/debug/routes',
    component: ComponentCreator('/pmdaemon/__docusaurus/debug/routes', 'f71'),
    exact: true
  },
  {
    path: '/pmdaemon/search',
    component: ComponentCreator('/pmdaemon/search', 'a96'),
    exact: true
  },
  {
    path: '/pmdaemon/docs',
    component: ComponentCreator('/pmdaemon/docs', 'c3d'),
    routes: [
      {
        path: '/pmdaemon/docs',
        component: ComponentCreator('/pmdaemon/docs', '447'),
        routes: [
          {
            path: '/pmdaemon/docs',
            component: ComponentCreator('/pmdaemon/docs', '685'),
            routes: [
              {
                path: '/pmdaemon/docs/advanced/clustering',
                component: ComponentCreator('/pmdaemon/docs/advanced/clustering', '169'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/pmdaemon/docs/advanced/logging',
                component: ComponentCreator('/pmdaemon/docs/advanced/logging', '93e'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/pmdaemon/docs/advanced/performance-tuning',
                component: ComponentCreator('/pmdaemon/docs/advanced/performance-tuning', '389'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/pmdaemon/docs/advanced/security',
                component: ComponentCreator('/pmdaemon/docs/advanced/security', '90d'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/pmdaemon/docs/advanced/troubleshooting',
                component: ComponentCreator('/pmdaemon/docs/advanced/troubleshooting', '33b'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/pmdaemon/docs/api/api-examples',
                component: ComponentCreator('/pmdaemon/docs/api/api-examples', '87a'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/pmdaemon/docs/api/library-usage',
                component: ComponentCreator('/pmdaemon/docs/api/library-usage', 'a69'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/pmdaemon/docs/api/rest-api',
                component: ComponentCreator('/pmdaemon/docs/api/rest-api', '1d9'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/pmdaemon/docs/api/websocket-api',
                component: ComponentCreator('/pmdaemon/docs/api/websocket-api', '9c6'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/pmdaemon/docs/cli/commands',
                component: ComponentCreator('/pmdaemon/docs/cli/commands', '361'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/pmdaemon/docs/cli/configuration-options',
                component: ComponentCreator('/pmdaemon/docs/cli/configuration-options', '059'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/pmdaemon/docs/cli/environment-variables',
                component: ComponentCreator('/pmdaemon/docs/cli/environment-variables', '8da'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/pmdaemon/docs/cli/exit-codes',
                component: ComponentCreator('/pmdaemon/docs/cli/exit-codes', '8bf'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/pmdaemon/docs/comparison/pm2-vs-pmdaemon',
                component: ComponentCreator('/pmdaemon/docs/comparison/pm2-vs-pmdaemon', 'af8'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/pmdaemon/docs/configuration/advanced-configuration',
                component: ComponentCreator('/pmdaemon/docs/configuration/advanced-configuration', '52c'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/pmdaemon/docs/configuration/ecosystem-files',
                component: ComponentCreator('/pmdaemon/docs/configuration/ecosystem-files', 'a76'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/pmdaemon/docs/configuration/process-configuration',
                component: ComponentCreator('/pmdaemon/docs/configuration/process-configuration', '626'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/pmdaemon/docs/configuration/schema-validation',
                component: ComponentCreator('/pmdaemon/docs/configuration/schema-validation', '708'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/pmdaemon/docs/examples/deployment-examples',
                component: ComponentCreator('/pmdaemon/docs/examples/deployment-examples', '881'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/pmdaemon/docs/examples/integration-examples',
                component: ComponentCreator('/pmdaemon/docs/examples/integration-examples', '93f'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/pmdaemon/docs/examples/use-cases',
                component: ComponentCreator('/pmdaemon/docs/examples/use-cases', '8cf'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/pmdaemon/docs/features/configuration',
                component: ComponentCreator('/pmdaemon/docs/features/configuration', 'a50'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/pmdaemon/docs/features/health-checks',
                component: ComponentCreator('/pmdaemon/docs/features/health-checks', '45b'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/pmdaemon/docs/features/monitoring',
                component: ComponentCreator('/pmdaemon/docs/features/monitoring', '58e'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/pmdaemon/docs/features/port-management',
                component: ComponentCreator('/pmdaemon/docs/features/port-management', 'c88'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/pmdaemon/docs/features/process-management',
                component: ComponentCreator('/pmdaemon/docs/features/process-management', '36c'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/pmdaemon/docs/features/web-api',
                component: ComponentCreator('/pmdaemon/docs/features/web-api', 'eb6'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/pmdaemon/docs/getting-started/installation',
                component: ComponentCreator('/pmdaemon/docs/getting-started/installation', 'db6'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/pmdaemon/docs/getting-started/introduction',
                component: ComponentCreator('/pmdaemon/docs/getting-started/introduction', 'a5b'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/pmdaemon/docs/getting-started/migration-from-pm2',
                component: ComponentCreator('/pmdaemon/docs/getting-started/migration-from-pm2', '4d9'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/pmdaemon/docs/getting-started/quick-start',
                component: ComponentCreator('/pmdaemon/docs/getting-started/quick-start', 'f76'),
                exact: true,
                sidebar: "tutorialSidebar"
              }
            ]
          }
        ]
      }
    ]
  },
  {
    path: '/pmdaemon/',
    component: ComponentCreator('/pmdaemon/', 'abc'),
    exact: true
  },
  {
    path: '*',
    component: ComponentCreator('*'),
  },
];
