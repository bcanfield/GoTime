import { Drawer, DrawerButton } from "./components/drawer";

function NewApp() {
  return (
    <div>
      <div className="bg-base-100 drawer mx-auto max-w-[100rem] lg:drawer-open">
        <input id="drawer" type="checkbox" className="drawer-toggle"></input>

        <div className="drawer-content">
          <div className="bg-base-100 flex justify-center rounded-sm p-2">
            <a
              href="/docs/upgrade/"
              className="btn btn-soft group btn-sm [width:clamp(20rem,100%,30rem)] rounded-full"
            >
              🎉 daisyUI 5.0 upgrade guide
            </a>
          </div>
        </div>
      </div>
    </div>

    // <div className="flex min-h-dvh bg-base-100">
    //   <div className="mx-auto max-w-6xl border w-full">
    //     <nav className="h-12 bg-green-500 flex items-center">
    //       <DrawerButton />
    //     </nav>

    //     <Drawer>
    //       <span>Main</span>
    //     </Drawer>
    //   </div>
    // </div>
  );
}

export default NewApp;
