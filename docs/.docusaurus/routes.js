import React from 'react';
import ComponentCreator from '@docusaurus/ComponentCreator';

export default [
  {
    path: '/pmdaemon/changelog',
    component: ComponentCreator('/pmdaemon/changelog', '75c'),
    exact: true
  },
  {
    path: '/pmdaemon/changelog/2025/05/29/v0.1.4',
    component: ComponentCreator('/pmdaemon/changelog/2025/05/29/v0.1.4', 'd79'),
    exact: true
  },
  {
    path: '/pmdaemon/changelog/archive',
    component: ComponentCreator('/pmdaemon/changelog/archive', '8cd'),
    exact: true
  },
  {
    path: '/pmdaemon/changelog/authors',
    component: ComponentCreator('/pmdaemon/changelog/authors', 'e68'),
    exact: true
  },
  {
    path: '/pmdaemon/changelog/tags',
    component: ComponentCreator('/pmdaemon/changelog/tags', 'f14'),
    exact: true
  },
  {
    path: '/pmdaemon/changelog/tags/accuracy',
    component: ComponentCreator('/pmdaemon/changelog/tags/accuracy', '78a'),
    exact: true
  },
  {
    path: '/pmdaemon/changelog/tags/bugfix',
    component: ComponentCreator('/pmdaemon/changelog/tags/bugfix', 'c20'),
    exact: true
  },
  {
    path: '/pmdaemon/changelog/tags/config-files',
    component: ComponentCreator('/pmdaemon/changelog/tags/config-files', 'e14'),
    exact: true
  },
  {
    path: '/pmdaemon/changelog/tags/cross-platform',
    component: ComponentCreator('/pmdaemon/changelog/tags/cross-platform', '6f7'),
    exact: true
  },
  {
    path: '/pmdaemon/changelog/tags/delete-operations',
    component: ComponentCreator('/pmdaemon/changelog/tags/delete-operations', 'f2c'),
    exact: true
  },
  {
    path: '/pmdaemon/changelog/tags/documentation',
    component: ComponentCreator('/pmdaemon/changelog/tags/documentation', '1cb'),
    exact: true
  },
  {
    path: '/pmdaemon/changelog/tags/ecosystem',
    component: ComponentCreator('/pmdaemon/changelog/tags/ecosystem', 'ced'),
    exact: true
  },
  {
    path: '/pmdaemon/changelog/tags/health-checks',
    component: ComponentCreator('/pmdaemon/changelog/tags/health-checks', '36a'),
    exact: true
  },
  {
    path: '/pmdaemon/changelog/tags/initial',
    component: ComponentCreator('/pmdaemon/changelog/tags/initial', '59f'),
    exact: true
  },
  {
    path: '/pmdaemon/changelog/tags/linux',
    component: ComponentCreator('/pmdaemon/changelog/tags/linux', '054'),
    exact: true
  },
  {
    path: '/pmdaemon/changelog/tags/macos',
    component: ComponentCreator('/pmdaemon/changelog/tags/macos', '387'),
    exact: true
  },
  {
    path: '/pmdaemon/changelog/tags/monitoring',
    component: ComponentCreator('/pmdaemon/changelog/tags/monitoring', 'b5e'),
    exact: true
  },
  {
    path: '/pmdaemon/changelog/tags/persistence',
    component: ComponentCreator('/pmdaemon/changelog/tags/persistence', '293'),
    exact: true
  },
  {
    path: '/pmdaemon/changelog/tags/pm-2',
    component: ComponentCreator('/pmdaemon/changelog/tags/pm-2', '447'),
    exact: true
  },
  {
    path: '/pmdaemon/changelog/tags/process-manager',
    component: ComponentCreator('/pmdaemon/changelog/tags/process-manager', 'dd6'),
    exact: true
  },
  {
    path: '/pmdaemon/changelog/tags/release',
    component: ComponentCreator('/pmdaemon/changelog/tags/release', 'b1d'),
    exact: true
  },
  {
    path: '/pmdaemon/changelog/tags/rust',
    component: ComponentCreator('/pmdaemon/changelog/tags/rust', '47e'),
    exact: true
  },
  {
    path: '/pmdaemon/changelog/tags/security',
    component: ComponentCreator('/pmdaemon/changelog/tags/security', '94d'),
    exact: true
  },
  {
    path: '/pmdaemon/changelog/tags/stability',
    component: ComponentCreator('/pmdaemon/changelog/tags/stability', '756'),
    exact: true
  },
  {
    path: '/pmdaemon/changelog/tags/testing',
    component: ComponentCreator('/pmdaemon/changelog/tags/testing', '28e'),
    exact: true
  },
  {
    path: '/pmdaemon/changelog/tags/windows',
    component: ComponentCreator('/pmdaemon/changelog/tags/windows', '7e5'),
    exact: true
  },
  {
    path: '/pmdaemon/changelog/v0.1.0',
    component: ComponentCreator('/pmdaemon/changelog/v0.1.0', 'd92'),
    exact: true
  },
  {
    path: '/pmdaemon/changelog/v0.1.1',
    component: ComponentCreator('/pmdaemon/changelog/v0.1.1', '975'),
    exact: true
  },
  {
    path: '/pmdaemon/changelog/v0.1.2-ecosystem-config-cross-platform',
    component: ComponentCreator('/pmdaemon/changelog/v0.1.2-ecosystem-config-cross-platform', '6f6'),
    exact: true
  },
  {
    path: '/pmdaemon/changelog/v0.1.3-critical-bug-fixes-state-persistence',
    component: ComponentCreator('/pmdaemon/changelog/v0.1.3-critical-bug-fixes-state-persistence', '002'),
    exact: true
  },
  {
    path: '/pmdaemon/search',
    component: ComponentCreator('/pmdaemon/search', 'a96'),
    exact: true
  },
  {
    path: '/pmdaemon/docs',
    component: ComponentCreator('/pmdaemon/docs', '574'),
    routes: [
      {
        path: '/pmdaemon/docs',
        component: ComponentCreator('/pmdaemon/docs', '12a'),
        routes: [
          {
            path: '/pmdaemon/docs',
            component: ComponentCreator('/pmdaemon/docs', '8b8'),
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
                path: '/pmdaemon/docs/api/authentication',
                component: ComponentCreator('/pmdaemon/docs/api/authentication', '09a'),
                exact: true
              },
              {
                path: '/pmdaemon/docs/api/error-handling',
                component: ComponentCreator('/pmdaemon/docs/api/error-handling', 'd2e'),
                exact: true
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
                path: '/pmdaemon/docs/architecture/overview',
                component: ComponentCreator('/pmdaemon/docs/architecture/overview', 'e86'),
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
                path: '/pmdaemon/docs/configuration/best-practices',
                component: ComponentCreator('/pmdaemon/docs/configuration/best-practices', '7ae'),
                exact: true
              },
              {
                path: '/pmdaemon/docs/configuration/ecosystem-files',
                component: ComponentCreator('/pmdaemon/docs/configuration/ecosystem-files', 'a76'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/pmdaemon/docs/configuration/environment-specific',
                component: ComponentCreator('/pmdaemon/docs/configuration/environment-specific', 'fa3'),
                exact: true
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
                path: '/pmdaemon/docs/examples/clustering',
                component: ComponentCreator('/pmdaemon/docs/examples/clustering', 'b35'),
                exact: true
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
                path: '/pmdaemon/docs/features/cross-platform',
                component: ComponentCreator('/pmdaemon/docs/features/cross-platform', '09f'),
                exact: true
              },
              {
                path: '/pmdaemon/docs/features/health-checks',
                component: ComponentCreator('/pmdaemon/docs/features/health-checks', '45b'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/pmdaemon/docs/features/load-balancing',
                component: ComponentCreator('/pmdaemon/docs/features/load-balancing', '2d2'),
                exact: true
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
              },
              {
                path: '/pmdaemon/docs/monitoring/overview',
                component: ComponentCreator('/pmdaemon/docs/monitoring/overview', '088'),
                exact: true
              },
              {
                path: '/pmdaemon/docs/performance/optimization',
                component: ComponentCreator('/pmdaemon/docs/performance/optimization', '8ce'),
                exact: true
              },
              {
                path: '/pmdaemon/docs/security/overview',
                component: ComponentCreator('/pmdaemon/docs/security/overview', 'd8b'),
                exact: true
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
