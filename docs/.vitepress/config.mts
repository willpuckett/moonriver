import { defineConfig } from "vitepress";

export default defineConfig({
  title: "Moonriver",
  description:
    "A terminal-based console for connecting to Klipper via Moonraker WebSocket API",
  base: "/",
  ignoreDeadLinks: true,

  themeConfig: {
    logo: "/logo.svg",

    nav: [
      { text: "Home", link: "/" },
      { text: "Guide", link: "/guide/getting-started" },
      { text: "API", link: "/api/" },
      { text: "GitHub", link: "https://github.com/willpuckett/moonriver" },
    ],

    sidebar: [
      {
        text: "Introduction",
        items: [
          { text: "What is Moonriver?", link: "/guide/what-is-moonriver" },
          { text: "Getting Started", link: "/guide/getting-started" },
          { text: "Quick Start", link: "/guide/quick-start" },
        ],
      },
      {
        text: "Usage",
        items: [
          { text: "Interactive Mode", link: "/guide/interactive-mode" },
          { text: "Scripting Mode", link: "/guide/scripting-mode" },
          { text: "Configuration", link: "/guide/configuration" },
          { text: "Multiple Printers", link: "/guide/multiple-printers" },
        ],
      },
      {
        text: "Features",
        items: [
          { text: "Tab Completion", link: "/features/tab-completion" },
          {
            text: "Syntax Highlighting",
            link: "/features/syntax-highlighting",
          },
          { text: "Command History", link: "/features/command-history" },
          { text: "Emergency Stop", link: "/features/emergency-stop" },
        ],
      },
      {
        text: "API Reference",
        items: [
          { text: "Overview", link: "/api/" },
          { text: "Client", link: "/api/client" },
          { text: "REPL", link: "/api/repl" },
        ],
      },
      {
        text: "Contributing",
        items: [
          { text: "Development Guide", link: "/contributing/development" },
          { text: "Code Style", link: "/contributing/code-style" },
          { text: "Pull Requests", link: "/contributing/pull-requests" },
        ],
      },
    ],

    socialLinks: [
      { icon: "github", link: "https://github.com/willpuckett/moonriver" },
    ],

    footer: {
      message: "Released under the MIT License.",
      copyright: "Copyright © 2025 Moonriver Contributors",
    },

    search: {
      provider: "local",
    },
  },
});
