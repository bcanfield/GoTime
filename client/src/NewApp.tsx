import { Drawer, DrawerButton } from "./components/daisy/drawer";

function NewApp() {
  return (
    <div className="flex h-screen bg-base-100">
      <div className="mx-auto max-w-6xl border w-full">
        <nav className="h-12 bg-green-500 flex items-center">
          <DrawerButton />
        </nav>

        <Drawer>
          <span>Main</span>
        </Drawer>
      </div>
    </div>
  );
}

export default NewApp;
