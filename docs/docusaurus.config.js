// @ts-check
// `@type` JSDoc annotations allow editor autocompletion and type checking
// (when paired with `@ts-check`).
// There are various equivalent ways to declare your Docusaurus config.
// See: https://docusaurus.io/docs/api/docusaurus-config

import {themes as prismThemes} from 'prism-react-renderer';

/** @type {import('@docusaurus/types').Config} */
const config = {
  title: 'PMDaemon',
  tagline: 'Advanced Process Manager - PM2 evolved in Rust',
  favicon: 'img/favicon.ico',

  // Set the production url of your site here
  url: 'https://entrepeneur4lyf.github.io',
  // Set the /<baseUrl>/ pathname under which your site is served
  // For GitHub pages deployment, it is often '/<projectName>/'
  baseUrl: '/pmdaemon/',

  // GitHub pages deployment config.
  // If you aren't using GitHub pages, you don't need these.
  organizationName: 'entrepeneur4lyf', // Usually your GitHub org/user name.
  projectName: 'pmdaemon', // Usually your repo name.

  onBrokenLinks: 'throw',
  onBrokenMarkdownLinks: 'warn',

  // Even if you don't use internationalization, you can use this field to set
  // useful metadata like html lang. For example, if your site is Chinese, you
  // may want to replace "en" with "zh-Hans".
  i18n: {
    defaultLocale: 'en',
    locales: ['en'],
  },

  markdown: {
    mermaid: true,
  },
  themes: ['@docusaurus/theme-mermaid'],

  presets: [
    [
      'classic',
      /** @type {import('@docusaurus/preset-classic').Options} */
      ({
        docs: {
          sidebarPath: './sidebars.js',
          // Please change this to your repo.
          // Remove this to remove the "edit this page" links.
          editUrl:
            'https://github.com/entrepeneur4lyf/pmdaemon/tree/main/docs/',
        },
        blog: {
          path: 'changelog',
          routeBasePath: 'changelog',
          blogTitle: 'Changelog',
          blogDescription: 'PMDaemon release notes and changelog',
          blogSidebarTitle: 'Recent releases',
          blogSidebarCount: 10,
          postsPerPage: 'ALL',
          showReadingTime: false,
          feedOptions: {
            type: 'all',
            title: 'PMDaemon Changelog',
            description: 'PMDaemon release notes and changelog',
          },
        },
        theme: {
          customCss: './src/css/custom.css',
        },
      }),
    ],
  ],

  themeConfig:
    /** @type {import('@docusaurus/preset-classic').ThemeConfig} */
    ({
      // Replace with your project's social card
      image: 'img/pmdaemon-social-card.jpg',
      colorMode: {
        disableSwitch: true,
        defaultMode: 'dark',
        respectPrefersColorScheme: false,
      },
      navbar: {
        title: 'Home',
        logo: {
          alt: 'PMDaemon Logo',
          src: 'img/logo-small.png',
        },
        items: [
          {
            type: 'docSidebar',
            sidebarId: 'tutorialSidebar',
            position: 'left',
            label: 'Documentation',
          },
          {
            to: '/docs/api/rest-api',
            label: 'API',
            position: 'left'
          },
          {
            to: '/docs/examples/use-cases',
            label: 'Examples',
            position: 'left'
          },
          {to: '/changelog', label: 'Changelog', position: 'left'},
          {
            href: 'https://github.com/entrepeneur4lyf/pmdaemon',
            label: 'GitHub',
            position: 'right',
          },
          {
            href: 'https://crates.io/crates/pmdaemon',
            label: 'Crates.io',
            position: 'right',
          },
        ],
      },
      footer: {
        style: 'dark',
        links: [
          {
            title: 'Documentation',
            items: [
              {
                label: 'Getting Started',
                to: '/docs/getting-started/introduction',
              },
              {
                label: 'CLI Reference',
                to: '/docs/cli/commands',
              },
              {
                label: 'API Documentation',
                to: '/docs/api/rest-api',
              },
            ],
          },
          {
            title: 'Community',
            items: [
              {
                label: 'GitHub Issues',
                href: 'https://github.com/entrepeneur4lyf/pmdaemon/issues',
              },
              {
                label: 'GitHub Discussions',
                href: 'https://github.com/entrepeneur4lyf/pmdaemon/discussions',
              },
              {
                label: 'X',
                href: 'https://x.com/entrepeneur4lyf',
              },
            ],
          },
          {
            title: 'More',
            items: [
              {
                label: 'Changelog',
                to: '/changelog',
              },
              {
                label: 'GitHub',
                href: 'https://github.com/entrepeneur4lyf/pmdaemon',
              },
              {
                label: 'Rust Docs',
                href: 'https://docs.rs/pmdaemon',
              },
            ],
          },
        ],
        copyright: `Copyright © ${new Date().getFullYear()} PMDaemon. Built with Docusaurus.`,
      },
      prism: {
        theme: prismThemes.github,
        darkTheme: prismThemes.dracula,
        additionalLanguages: ['rust', 'toml', 'json', 'bash', 'powershell'],
      },
      algolia: {
        // The application ID provided by Algolia
        appId: 'O2R9OPMFS1',
        // Public API key: it is safe to commit it
        apiKey: 'e3b8bec8a3c350f252e460267f20312b',
        indexName: 'pmdaemon',
        // Optional: see doc section below
        contextualSearch: true,
        // Optional: Specify domains where the navigation should occur through window.location instead on history.push
        externalUrlRegex: 'external\\.com|domain\\.com',
        // Optional: Replace parts of the item URLs from Algolia
        replaceSearchResultPathname: {
          from: '/docs/', // or as RegExp: /\/docs\//
          to: '/',
        },
        // Optional: Algolia search parameters
        searchParameters: {},
        // Optional: path for search page that enabled by default (`false` to disable it)
        searchPagePath: 'search',
      },
    }),
};

export default config;
