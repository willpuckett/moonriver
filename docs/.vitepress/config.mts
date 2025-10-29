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
          { text: "Temperature Bar", link: "/features/temperature-bar" },
          { text: "Position Bar", link: "/features/position-bar" },
          {
            text: "Job History Browser",
            link: "/features/job-history-browser",
          },
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
      {
        icon: {
          svg:
            '<svg viewBox="0 0 21.965297 25.299691" xmlns="http://www.w3.org/2000/svg"><path fill="currentColor" d="m -23.113281,213.26172 a 0.14410426,0.14410426 0 0 0 -0.08203,0.006 c -1.098124,0.39435 -2.161803,0.90487 -3.238282,1.32812 a 0.14410426,0.14410426 0 0 0 -0.0098,0.004 c -0.611707,0.28972 -1.328891,0.46627 -1.9375,0.85547 a 0.14410426,0.14410426 0 0 0 -0.06641,0.12109 c 0.0021,1.07514 0.0057,2.1517 0.0078,3.22656 l 0.0039,-0.0332 c -0.04972,0.21467 -3.28e-4,0.43392 0.01367,0.61914 0.007,0.0926 0.0054,0.17516 -0.0078,0.23633 -0.01323,0.0612 -0.03302,0.0981 -0.07422,0.13086 a 0.14410426,0.14410426 0 0 0 -0.05469,0.11328 v 2.31641 c -0.994711,0.42426 -1.989717,0.84919 -2.984375,1.27343 a 0.14410426,0.14410426 0 0 0 -0.0059,0.002 c -0.711136,0.33688 -1.498277,0.57799 -2.210938,0.97265 a 0.14410426,0.14410426 0 0 0 -0.07422,0.12696 v 5.80859 l 0.002,-0.0254 c -0.03894,0.22343 0.01376,0.44974 0.01172,0.62109 -0.001,0.0857 -0.01448,0.15211 -0.03906,0.19727 -0.02459,0.0452 -0.05672,0.079 -0.142578,0.10742 a 0.14410426,0.14410426 0 0 0 -0.09961,0.14063 c 0.01389,0.50873 0.02712,1.01642 0.04102,1.52539 a 0.14410426,0.14410426 0 0 0 0.04492,0.0996 c 0.477272,0.45327 1.13016,0.63171 1.625,0.93945 a 0.14410426,0.14410426 0 0 0 0.09766,0.0195 c 0.530375,-0.083 0.95055,-0.44054 1.369141,-0.62109 0.354461,0.0694 0.756192,0.32552 1.179687,0.5039 v 1.33789 a 0.14410426,0.14410426 0 0 0 0.08008,0.12891 c 0.600021,0.30052 1.198925,0.59994 1.798828,0.90039 a 0.14410426,0.14410426 0 0 0 0.128906,0 c 0.473359,-0.24075 0.941706,-0.49112 1.410156,-0.73633 0.392113,0.1866 0.783777,0.37402 1.175781,0.56055 v 1.22266 c -0.0077,0.14086 0.05065,0.26874 0.134766,0.35351 0.08411,0.0848 0.187738,0.13536 0.291016,0.17969 0.206554,0.0887 0.421817,0.15818 0.509765,0.23828 a 0.14410426,0.14410426 0 0 0 0.0332,0.0215 c 0.29858,0.14753 0.599457,0.31933 0.916016,0.46094 a 0.14410426,0.14410426 0 0 0 0.119141,0 c 3.53559,-1.66565 7.019681,-3.46206 10.539062,-5.1582 0.13587,-0.0228 0.250068,-0.0728 0.326172,-0.15625 0.08425,-0.0923 0.119048,-0.20689 0.132812,-0.31641 0.02753,-0.21904 -0.01191,-0.4466 0,-0.60937 a 0.14410426,0.14410426 0 0 0 -0.002,-0.0312 c -0.02404,-0.16046 0.01896,-0.39541 0.01367,-0.63086 -0.0026,-0.11773 -0.0183,-0.23983 -0.07813,-0.35352 -0.04534,-0.0862 -0.125161,-0.15542 -0.220703,-0.21289 l -0.05469,-12.25 a 0.14410426,0.14410426 0 0 0 -0.0039,-0.0273 c -0.02626,-0.13796 0.0018,-0.33185 0.01367,-0.53516 0.01186,-0.2033 0.0063,-0.43237 -0.146484,-0.61719 a 0.14410426,0.14410426 0 0 0 -0.05469,-0.041 c -2.749693,-1.16882 -5.502852,-2.32819 -8.255859,-3.48828 a 0.14410426,0.14410426 0 0 0 -0.0078,-0.002 c -0.704328,-0.25898 -1.399019,-0.67218 -2.167969,-0.85351 z" transform="translate(34.101648,-213.25836)"/></svg>',
        },
        link: "https://crates.io/crates/moonriver",
      },
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
