// @ts-check
// Note: type annotations allow type checking and IDEs autocompletion

const lightCodeTheme = require('prism-react-renderer/themes/github');
const darkCodeTheme = require('prism-react-renderer/themes/dracula');

/** @type {import('@docusaurus/types').Config} */
const config = {
  title: 'Wasm Workers Server',
  tagline: 'A WebAssembly framework to develop serverless applications and run them anywhere',
  url: 'https://workers.wasmlabs.dev',
  baseUrl: '/',
  onBrokenLinks: 'throw',
  onBrokenMarkdownLinks: 'warn',
  favicon: 'img/favicon.svg',
  // Even if you don't use internalization, you can use this field to set useful
  // metadata like html lang. For example, if your site is Chinese, you may want
  // to replace "en" with "zh-Hans".
  i18n: {
    defaultLocale: 'en',
    locales: ['en'],
  },
  // Get the index metadata for `wws` language runtimes
  staticDirectories: ["static", "../metadata"],

  presets: [
    [
      'classic',
      /** @type {import('@docusaurus/preset-classic').Options} */
      ({
        docs: {
          sidebarPath: require.resolve('./sidebars.js'),
          // Please change this to your repo.
          // Remove this to remove the "edit this page" links.
          editUrl:
            'https://github.com/vmware-labs/wasm-workers-server/tree/main/docs',
        },
        theme: {
          customCss: require.resolve('./src/css/custom.css'),
        },
      }),
    ],
  ],

  themeConfig:
    /** @type {import('@docusaurus/preset-classic').ThemeConfig} */
    ({
      navbar: {
        title: 'VMware OCTO / Wasm Workers Server',
        items: [
          {
            type: 'doc',
            docId: 'get-started/introduction',
            position: 'left',
            label: 'Documentation',
          },
          {
            href: 'https://github.com/vmware-labs/wasm-workers-server',
            label: 'GitHub',
            position: 'right',
          },
          {
            href: 'https://wasmlabs.dev',
            label: 'Wasm Labs',
            position: 'right',
          },
        ],
      },
      footer: {
        style: 'dark',
        links: [
          {
            title: 'Docs',
            items: [
              {
                label: 'Tutorial',
                to: '/docs/get-started/quickstart',
              },
            ],
          },
          {
            title: 'Other Projects',
            items: [
              {
                label: 'WebAssembly Language Runtimes',
                href: 'https://github.com/vmware-labs/webassembly-language-runtimes',
              },
              {
                label: 'WordPress in the Browser',
                href: 'https://wordpress.wasmlabs.dev',
              },
              {
                label: 'Apache HTTP Server mod_wasm module',
                href: 'https://github.com/vmware-labs/mod_wasm',
              },
            ],
          },
          {
            title: 'More',
            items: [
              {
                href: 'https://wasmlabs.dev',
                label: 'Wasm Labs',
              },
              {
                label: 'GitHub',
                href: 'https://github.com/vmware-labs/wasm-workers-server',
              },
            ],
          },
        ],
        copyright: `Copyright © ${new Date().getFullYear()} VMware, Inc.`,
      },
      prism: {
        theme: lightCodeTheme,
        darkTheme: darkCodeTheme,
        additionalLanguages: ['rust', 'toml', 'ruby', 'python', 'zig'],
      },
    }),
};

module.exports = config;
