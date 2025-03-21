import ChatBox from "../components/chat-box";
import { SidebarMenuItemProps } from "../components/sidebar-nav-item";
import Users from "../components/spacetime-components/users";

export const pagesThatDontNeedSidebar = [
  "/",
  "/components/",
  "/blog/",
  "/blog/*/",
  "/store/",
  "/store/*/",
  "/resources/*/",
  "/theme-generator/",
];

export const navItems: SidebarMenuItemProps[] = [
  {
    name: "Users",
    collapsible: true,

    renderChild: <Users />,
    icon: '<svg class="text-blue-500 size-5" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 48 48"><g fill="currentColor"><defs><path id="1733501816477-2951362_a" d="M0 0h48v48H0V0z"></path></defs><clipPath id="1733501816477-2951362_b"><use xlink:href="#1733501816477-2951362_a" overflow="visible"></use></clipPath><path clip-path="url(#1733501816477-2951362_b)" d="M40 8H8c-2.21 0-3.98 1.79-3.98 4L4 36c0 2.21 1.79 4 4 4h32c2.21 0 4-1.79 4-4V12c0-2.21-1.79-4-4-4zM17 30h-2.4l-5.1-7v7H7V18h2.5l5 7v-7H17v12zm10-9.49h-5v2.24h5v2.51h-5v2.23h5V30h-8V18h8v2.51zM41 28c0 1.1-.9 2-2 2h-8c-1.1 0-2-.9-2-2V18h2.5v9.01h2.25v-7.02h2.5v7.02h2.25V18H41v10z"></path></g></svg>',
  },
  {
    name: "Chat",
    collapsible: true,

    renderChild: <ChatBox />,
    icon: '<svg class="text-blue-500 size-5" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 48 48"><g fill="currentColor"><defs><path id="1733501816477-2951362_a" d="M0 0h48v48H0V0z"></path></defs><clipPath id="1733501816477-2951362_b"><use xlink:href="#1733501816477-2951362_a" overflow="visible"></use></clipPath><path clip-path="url(#1733501816477-2951362_b)" d="M40 8H8c-2.21 0-3.98 1.79-3.98 4L4 36c0 2.21 1.79 4 4 4h32c2.21 0 4-1.79 4-4V12c0-2.21-1.79-4-4-4zM17 30h-2.4l-5.1-7v7H7V18h2.5l5 7v-7H17v12zm10-9.49h-5v2.24h5v2.51h-5v2.23h5V30h-8V18h8v2.51zM41 28c0 1.1-.9 2-2 2h-8c-1.1 0-2-.9-2-2V18h2.5v9.01h2.25v-7.02h2.5v7.02h2.25V18H41v10z"></path></g></svg>',
  },
  {},
  {
    name: "Docs",
    icon: '<svg width="18" height="18" viewBox="0 0 48 48" class="text-orange-400 size-5" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M5 7H16C20.4183 7 24 10.5817 24 15V42C24 38.6863 21.3137 36 18 36H5V7Z" fill="none" stroke="currentColor" stroke-width="4" stroke-linejoin="bevel"/><path d="M43 7H32C27.5817 7 24 10.5817 24 15V42C24 38.6863 26.6863 36 30 36H43V7Z" fill="none" stroke="currentColor" stroke-width="4" stroke-linejoin="bevel"/></svg>',
    collapsible: true,
    items: [
      {
        name: "Introduction",
        href: "/docs/intro/",
      },
      {
        name: "Install",
        href: "/docs/install/",
      },
    ],
  },
  {},
  {
    name: "GitHub",
    href: "https://github.com/saadeghi/daisyui",
    icon: '<svg class="size-5" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g stroke-linejoin="round" stroke-linecap="round" stroke-width="2" fill="none" stroke="currentColor"><path d="M9 19c-4.3 1.4 -4.3 -2.5 -6 -3m12 5v-3.5c0 -1 .1 -1.4 -.5 -2c2.8 -.3 5.5 -1.4 5.5 -6a4.6 4.6 0 0 0 -1.3 -3.2a4.2 4.2 0 0 0 -.1 -3.2s-1.1 -.3 -3.5 1.3a12.3 12.3 0 0 0 -6.2 0c-2.4 -1.6 -3.5 -1.3 -3.5 -1.3a4.2 4.2 0 0 0 -.1 3.2a4.6 4.6 0 0 0 -1.3 3.2c0 4.6 2.7 5.7 5.5 6c-.6 .6 -.6 1.2 -.5 2v3.5"></path></g></svg>',
    target: "blank",
  },
];
