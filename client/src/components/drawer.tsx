import VersionDropdown from "./version-dropdown";

import { navItems } from "../lib/sidebar-items";
import SidebarMenuItem from "./sidebar-nav-item";
import ProfileSection from "./spacetime-components/profile";

export const Drawer = ({ children }: { children: React.ReactNode }) => {
  return (
    <div className="bg-base-100 drawer mx-auto max-w-[100rem] lg:drawer-open">
      <input id="drawer" type="checkbox" className="drawer-toggle"></input>

      <div className="drawer-content">
        <TopAlert />

        <NavBar />

        <div className="relative max-w-[100vw] px-6 pb-16 xl:pe-2">
          {children}
        </div>
      </div>

      <DrawerSide />
    </div>
  );
};

const TopAlert = () => {
  return (
    <div className="bg-base-100 flex justify-center rounded-sm p-2">
      <a
        href="/docs/upgrade/"
        className="btn btn-soft group btn-sm [width:clamp(20rem,100%,30rem)] rounded-full"
      >
        🎉 daisyUI 5.0 upgrade guide
      </a>
    </div>
  );
};
const NavBar = () => {
  return (
    <div className="bg-base-100/90 text-base-content sticky top-0 z-30 flex h-16 w-full [transform:translate3d(0,0,0)] justify-center backdrop-blur transition-shadow duration-100 print:hidden">
      <nav className="navbar w-full">
        <div className="flex flex-1 items-center md:gap-1 lg:gap-2">
          <span
            className="tooltip tooltip-bottom before:text-xs before:content-[attr(data-tip)]"
            data-tip="Menu"
          >
            <label
              aria-label="Open menu"
              htmlFor="drawer"
              className="btn btn-square btn-ghost drawer-button lg:hidden "
            >
              <svg
                width="20"
                height="20"
                xmlns="http://www.w3.org/2000/svg"
                fill="none"
                viewBox="0 0 24 24"
                className="inline-block h-5 w-5 stroke-current md:h-6 md:w-6"
              >
                <path
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  stroke-width="2"
                  d="M4 6h16M4 12h16M4 18h16"
                ></path>
              </svg>
            </label>
          </span>
          <div className="flex items-center gap-2 lg:hidden">
            <a className="btn btn-ghost flex-0 gap-1 px-2 md:gap-2" href="/">
              <svg
                className="h-5 w-5 md:h-6 md:w-6"
                width="28"
                height="28"
                viewBox="0 0 415 415"
                xmlns="http://www.w3.org/2000/svg"
              >
                <rect
                  x="82.5"
                  y="290"
                  width="250"
                  height="125"
                  rx="62.5"
                  fill="#1AD1A5"
                ></rect>
                <circle
                  cx="207.5"
                  cy="135"
                  r="130"
                  fill="black"
                  fill-opacity=".3"
                ></circle>
                <circle cx="207.5" cy="135" r="125" fill="white"></circle>
                <circle cx="207.5" cy="135" r="56" fill="#FF9903"></circle>
              </svg>
              <span className="font-title text-base-content text-lg md:text-xl">
                daisyUIasdf
              </span>
            </a>

            <VersionDropdown />
          </div>
        </div>
        <div className="flex">
          <div className="hidden flex-none items-center lg:inline-block">
            <a
              data-sveltekit-preload-data=""
              href="/components/"
              className="btn btn-sm btn-ghost drawer-button font-normal"
            >
              Components
            </a>
            <a
              data-sveltekit-preload-data=""
              href="/components/"
              className="btn btn-sm btn-ghost drawer-button font-normal"
            >
              Components
            </a>
          </div>
          <div className="dropdown dropdown-end block ">
            <div
              tabIndex={0}
              role="button"
              className="btn btn-sm gap-1 btn-ghost"
            >
              <div className="bg-base-100 border-base-content/10 grid shrink-0 grid-cols-2 gap-0.5 rounded-md border p-1">
                <div className="bg-base-content size-1 rounded-full"></div>{" "}
                <div className="bg-primary size-1 rounded-full"></div>{" "}
                <div className="bg-secondary size-1 rounded-full"></div>{" "}
                <div className="bg-accent size-1 rounded-full"></div>
              </div>{" "}
              <svg
                width="12px"
                height="12px"
                className="mt-px hidden h-2 w-2 fill-current opacity-60 sm:inline-block"
                xmlns="http://www.w3.org/2000/svg"
                viewBox="0 0 2048 2048"
              >
                <path d="M1799 349l242 241-1017 1017L7 590l242-241 775 775 775-775z"></path>
              </svg>
            </div>
            <div className="dropdown-content bg-base-200 text-base-content rounded-box top-px h-[30.5rem] max-h-[calc(100vh-8.6rem)] overflow-y-auto border border-white/5 shadow-2xl outline-1 outline-black/5 mt-16">
              <ul className="menu w-56">
                <li className="menu-title text-xs">Theme</li>
                <li>
                  <button className="gap-3 px-2" data-set-theme="dark">
                    <div
                      data-theme="dark"
                      className="bg-base-100 grid shrink-0 grid-cols-2 gap-0.5 rounded-md p-1 shadow-sm"
                    >
                      <div className="bg-base-content size-1 rounded-full"></div>{" "}
                      <div className="bg-primary size-1 rounded-full"></div>{" "}
                      <div className="bg-secondary size-1 rounded-full"></div>{" "}
                      <div className="bg-accent size-1 rounded-full"></div>
                    </div>{" "}
                    <div className="w-32 truncate">dark</div>{" "}
                    <svg
                      xmlns="http://www.w3.org/2000/svg"
                      width="16"
                      height="16"
                      viewBox="0 0 24 24"
                      fill="currentColor"
                      className="invisible h-3 w-3 shrink-0"
                    >
                      <path d="M20.285 2l-11.285 11.567-5.286-5.011-3.714 3.716 9 8.728 15-15.285z"></path>
                    </svg>
                  </button>
                </li>
              </ul>
            </div>
          </div>
        </div>
      </nav>
    </div>
  );
};

const DrawerSide = () => {
  return (
    <div className="drawer-side z-40 scroll-smooth scroll-pt-20">
      <label
        htmlFor="drawer"
        className="drawer-overlay"
        aria-label="Close menu"
      ></label>
      <aside className="bg-base-100 min-h-screen w-80">
        <div className="bg-base-100/90 navbar sticky top-0 z-20 hidden items-center gap-2 px-4 py-2 backdrop-blur lg:flex ">
          <a
            href="/"
            aria-current="page"
            aria-label="Homepage"
            className="btn btn-ghost flex-0 px-2"
          >
            <svg
              width="32"
              height="32"
              viewBox="0 0 415 415"
              xmlns="http://www.w3.org/2000/svg"
            >
              <rect
                x="82.5"
                y="290"
                width="250"
                height="125"
                rx="62.5"
                fill="#1AD1A5"
              ></rect>
              <circle
                cx="207.5"
                cy="135"
                r="130"
                fill="black"
                fill-opacity=".3"
              ></circle>
              <circle cx="207.5" cy="135" r="125" fill="white"></circle>
              <circle cx="207.5" cy="135" r="56" fill="#FF9903"></circle>
            </svg>{" "}
            <div className="font-title inline-flex text-lg md:text-2xl">
              daisyUII
            </div>
          </a>
          <VersionDropdown />
        </div>
        <div className="h-4"></div>

        <ul className="menu w-full px-4 py-0 space-y-2">
          <ProfileSection />
          {navItems.map((item, index) => (
            <SidebarMenuItem key={index} {...item} />
          ))}
        </ul>
      </aside>
    </div>
  );
};

export const DrawerButton = () => {
  return (
    <label
      htmlFor="my-drawer-2"
      className="btn btn-primary drawer-button lg:hidden"
    >
      Open drawer
    </label>
  );
};
