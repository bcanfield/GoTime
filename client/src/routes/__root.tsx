import { Link, Outlet, createRootRoute } from "@tanstack/react-router";
import { TanStackRouterDevtools } from "@tanstack/react-router-devtools";
import { Drawer, DrawerButton } from "../components/daisy/drawer";
import { SpacetimeProvider } from "../providers/spacetime-provider";

export const Route = createRootRoute({
  component: RootComponent,
});

function RootComponent() {
  return (
    <SpacetimeProvider>
      <>
        <div className="flex h-screen bg-base-100">
          <div className="mx-auto max-w-6xl border w-full">
            <nav className="h-12 bg-green-500 flex items-center">
              <DrawerButton />
              <Link
                to="/"
                activeProps={{
                  className: "font-bold",
                }}
                activeOptions={{ exact: true }}
              >
                Home
              </Link>
              <Link
                to="/about"
                activeProps={{
                  className: "font-bold",
                }}
                activeOptions={{ exact: true }}
              >
                About
              </Link>
            </nav>

            <Drawer>
              <Outlet />
            </Drawer>
          </div>
        </div>
        <hr />
        <TanStackRouterDevtools position="bottom-right" />
      </>
    </SpacetimeProvider>
  );
}
