import React, { useMemo, useState } from "react";

export type SidebarMenuItemProps = {
  closeDrawer?: () => void;
  name?: string;
  href?: string;
  icon?: string;
  badge?: string;
  badgeclass?: string;
  highlightAnotherItem?: string;
  deprecated?: boolean;
  items?: SidebarMenuItemProps[];
  collapsible?: boolean;
  highlight?: boolean;
  target?: string;
  /**
   * Custom children to render when the item is expanded (used instead of `items`)
   */
  renderChild?: React.ReactNode;
};

const SidebarMenuItem: React.FC<SidebarMenuItemProps> = ({
  closeDrawer,
  name = null,
  href = null,
  icon = null,
  badge = null,
  badgeclass = "",
  highlightAnotherItem = null,
  deprecated = false,
  items = null,
  collapsible = false,
  highlight = false,
  target = null,
  renderChild,
}) => {
  //   const location = useLocation()
  const pathname = location.pathname;

  const sanitize = (str: string) => str.toLowerCase().replace(/\W/g, "-");
  const disclosureId = useMemo(
    () => `disclosure-${sanitize(name || "")}`,
    [name]
  );

  const shouldDisclosureBeOpen = useMemo(() => {
    if (typeof window === "undefined") return false;
    if (localStorage.getItem(disclosureId) === "open") return true;

    const isChildActive = (items || []).some(
      (item) =>
        item.href === pathname ||
        (item.items || []).some((subitem) => subitem.href === pathname)
    );
    return isChildActive;
  }, [items, pathname, disclosureId]);

  const [open, setOpen] = useState(shouldDisclosureBeOpen);

  const handleToggle = (e: React.SyntheticEvent<HTMLDetailsElement>) => {
    const isOpen = e.currentTarget.open;
    setOpen(isOpen);
    if (typeof window !== "undefined") {
      localStorage.setItem(disclosureId, isOpen ? "open" : "close");
    }
  };

  if (!name && !items && !renderChild) return <li></li>;

  if (!items && !renderChild && href) {
    const isActive =
      pathname === href ||
      pathname === highlightAnotherItem ||
      pathname.startsWith(href);

    return (
      <li>
        <a
          href={href}
          target={target === "blank" ? "_blank" : undefined}
          rel={target === "blank" ? "noopener noreferrer" : undefined}
          onClick={closeDrawer}
          className={`group ${isActive ? "menu-active" : ""} ${
            highlight
              ? "from-primary to-primary/0 hover:to-primary/10 [background-image:linear-gradient(-35deg,var(--tw-gradient-stops))] from-[-200%] to-60%"
              : ""
          }`}
        >
          {icon && (
            <span
              className={
                highlight ? "group-hover:text-primary transition-colors" : ""
              }
              dangerouslySetInnerHTML={{ __html: icon }}
            />
          )}
          <span className={deprecated ? "line-through" : ""}>{name || ""}</span>
          {badge && (
            <span
              className={`badge badge-xs text-opacity-70 font-mono ${badgeclass}`}
            >
              {badge}
            </span>
          )}
          {target === "blank" && (
            <svg
              width="12"
              height="12"
              className="opacity-0 transition-opacity duration-300 ease-out group-hover:opacity-100"
              viewBox="0 0 48 48"
              fill="none"
              xmlns="http://www.w3.org/2000/svg"
            >
              <path
                d="M19 11H37V29"
                stroke="currentColor"
                strokeWidth="4"
                strokeLinecap="butt"
                strokeLinejoin="bevel"
              />
              <path
                d="M11.5439 36.4559L36.9997 11"
                stroke="currentColor"
                strokeWidth="4"
                strokeLinecap="butt"
                strokeLinejoin="bevel"
              />
            </svg>
          )}
        </a>
      </li>
    );
  }

  return (
    <li>
      {collapsible ? (
        <details id={disclosureId} open={open} onToggle={handleToggle}>
          <summary className="group cursor-pointer">
            {icon && <span dangerouslySetInnerHTML={{ __html: icon }} />}
            {name || ""}
          </summary>
          {open && (
            <div className="w-full overflow-x-hidden">
              {renderChild ? (
                renderChild
              ) : (
                <ul>
                  {items?.map((item, idx) => (
                    <SidebarMenuItem
                      key={idx}
                      {...item}
                      closeDrawer={closeDrawer}
                    />
                  ))}
                </ul>
              )}
            </div>
          )}
        </details>
      ) : (
        <>
          {!href && (
            <h2 className="menu-title flex items-center gap-4 px-1.5">
              {icon && (
                <span
                  className="text-base-content"
                  dangerouslySetInnerHTML={{ __html: icon }}
                />
              )}
              {name || ""}
            </h2>
          )}
          {items && (
            <ul>
              {items.map((item, idx) => (
                <SidebarMenuItem
                  key={idx}
                  {...item}
                  closeDrawer={closeDrawer}
                />
              ))}
            </ul>
          )}
        </>
      )}
    </li>
  );
};

export default SidebarMenuItem;
