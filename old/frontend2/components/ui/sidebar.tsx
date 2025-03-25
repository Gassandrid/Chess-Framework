"use client";

import * as React from "react";
import { cn } from "@/lib/utils";
import { useMobile } from "@/hooks/use-mobile";
import { Button } from "@/components/ui/button";
import { Menu } from "lucide-react";

// Context for the sidebar
const SidebarContext = React.createContext<{
  expanded: boolean;
  setExpanded: React.Dispatch<React.SetStateAction<boolean>>;
}>({
  expanded: true,
  setExpanded: () => undefined,
});

export function SidebarProvider({
  children,
  defaultExpanded = true,
}: {
  children: React.ReactNode;
  defaultExpanded?: boolean;
}) {
  const [expanded, setExpanded] = React.useState(defaultExpanded);
  const isMobile = useMobile();

  // Collapse sidebar on mobile by default
  React.useEffect(() => {
    if (isMobile) {
      setExpanded(false);
    } else {
      setExpanded(defaultExpanded);
    }
  }, [isMobile, defaultExpanded]);

  return (
    <SidebarContext.Provider value={{ expanded, setExpanded }}>
      {children}
    </SidebarContext.Provider>
  );
}

export function useSidebar() {
  return React.useContext(SidebarContext);
}

export function Sidebar({ children }: { children: React.ReactNode }) {
  const { expanded } = useSidebar();
  const isMobile = useMobile();

  return (
    <aside
      className={cn(
        "h-screen border-r bg-background transition-all duration-300 ease-in-out",
        expanded ? "w-64" : "w-16",
        isMobile && expanded && "fixed inset-0 z-50 w-64",
      )}
    >
      <div className="flex h-full flex-col">{children}</div>
    </aside>
  );
}

export function SidebarHeader({
  children,
  className,
}: {
  children: React.ReactNode;
  className?: string;
}) {
  return <div className={cn("px-4 py-2", className)}>{children}</div>;
}

export function SidebarContent({
  children,
  className,
}: {
  children: React.ReactNode;
  className?: string;
}) {
  return (
    <div className={cn("flex-1 overflow-auto", className)}>{children}</div>
  );
}

export function SidebarFooter({
  children,
  className,
}: {
  children: React.ReactNode;
  className?: string;
}) {
  return <div className={cn("border-t", className)}>{children}</div>;
}

export function SidebarTrigger() {
  const { expanded, setExpanded } = useSidebar();
  return (
    <Button variant="ghost" size="icon" onClick={() => setExpanded(!expanded)}>
      <Menu className="h-5 w-5" />
      <span className="sr-only">Toggle sidebar</span>
    </Button>
  );
}

export function SidebarMenu({ children }: { children: React.ReactNode }) {
  return <ul className="space-y-1 p-2">{children}</ul>;
}

export function SidebarMenuItem({ children }: { children: React.ReactNode }) {
  return <li>{children}</li>;
}

export function SidebarMenuButton({
  children,
  onClick,
  isActive,
}: {
  children: React.ReactNode;
  onClick?: () => void;
  isActive?: boolean;
}) {
  const { expanded } = useSidebar();
  return (
    <Button
      variant={isActive ? "secondary" : "ghost"}
      className={cn("w-full justify-start", !expanded && "justify-center px-0")}
      onClick={onClick}
    >
      {expanded
        ? children
        : React.Children.toArray(children).filter(
            (child) => React.isValidElement(child) && child.type !== "span",
          )}
    </Button>
  );
}

export function SidebarGroup({ children }: { children: React.ReactNode }) {
  return <div className="pb-4">{children}</div>;
}

export function SidebarGroupLabel({ children }: { children: React.ReactNode }) {
  const { expanded } = useSidebar();
  if (!expanded) return null;
  return (
    <div className="px-2 py-1 text-xs font-semibold text-muted-foreground">
      {children}
    </div>
  );
}

export function SidebarGroupContent({
  children,
}: {
  children: React.ReactNode;
}) {
  return <div>{children}</div>;
}
