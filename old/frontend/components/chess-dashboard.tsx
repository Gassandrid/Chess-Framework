"use client";

import { useState } from "react";
import {
  Sidebar,
  SidebarContent,
  SidebarHeader,
  SidebarMenu,
  SidebarMenuItem,
  SidebarMenuButton,
  SidebarProvider,
  SidebarTrigger,
  SidebarGroup,
  SidebarGroupLabel,
  SidebarGroupContent,
  SidebarFooter,
} from "@/components/ui/sidebar";
import { Button } from "@/components/ui/button";
import {
  KeyboardIcon as ChessboardIcon,
  History,
  BarChart3,
  Cpu,
  Settings,
  BookOpen,
  Users,
  Sparkles,
  Moon,
  Sun,
} from "lucide-react";
import ChessGame from "@/components/chess-game";
import MoveHistoryPanel from "@/components/move-history-panel";
import AnalysisPanel from "@/components/analysis-panel";
import EngineSettingsPanel from "@/components/engine-settings-panel";
import { useTheme } from "@/components/theme-provider";

export default function ChessDashboard() {
  const [activePanel, setActivePanel] = useState<string>("game");
  const { theme, setTheme } = useTheme();

  const renderPanel = () => {
    switch (activePanel) {
      case "game":
        return <ChessGame />;
      case "history":
        return <MoveHistoryPanel />;
      case "analysis":
        return <AnalysisPanel />;
      case "engines":
        return <EngineSettingsPanel />;
      default:
        return <ChessGame />;
    }
  };

  return (
    <SidebarProvider>
      <div className="flex h-screen bg-background">
        <Sidebar>
          <SidebarHeader className="flex flex-col gap-0 py-4">
            <div className="flex items-center px-4">
              <ChessboardIcon className="h-6 w-6 mr-2" />
              <h1 className="text-xl font-bold">Chess Dashboard</h1>
            </div>
          </SidebarHeader>
          <SidebarContent>
            <SidebarGroup>
              <SidebarGroupLabel>Game</SidebarGroupLabel>
              <SidebarGroupContent>
                <SidebarMenu>
                  <SidebarMenuItem>
                    <SidebarMenuButton
                      onClick={() => setActivePanel("game")}
                      isActive={activePanel === "game"}
                    >
                      <ChessboardIcon className="h-4 w-4" />
                      <span>Chessboard</span>
                    </SidebarMenuButton>
                  </SidebarMenuItem>
                  <SidebarMenuItem>
                    <SidebarMenuButton
                      onClick={() => setActivePanel("history")}
                      isActive={activePanel === "history"}
                    >
                      <History className="h-4 w-4" />
                      <span>Move History</span>
                    </SidebarMenuButton>
                  </SidebarMenuItem>
                </SidebarMenu>
              </SidebarGroupContent>
            </SidebarGroup>

            <SidebarGroup>
              <SidebarGroupLabel>Analysis</SidebarGroupLabel>
              <SidebarGroupContent>
                <SidebarMenu>
                  <SidebarMenuItem>
                    <SidebarMenuButton
                      onClick={() => setActivePanel("analysis")}
                      isActive={activePanel === "analysis"}
                    >
                      <BarChart3 className="h-4 w-4" />
                      <span>Position Analysis</span>
                    </SidebarMenuButton>
                  </SidebarMenuItem>
                  <SidebarMenuItem>
                    <SidebarMenuButton
                      onClick={() => setActivePanel("engines")}
                      isActive={activePanel === "engines"}
                    >
                      <Cpu className="h-4 w-4" />
                      <span>Engine Settings</span>
                    </SidebarMenuButton>
                  </SidebarMenuItem>
                </SidebarMenu>
              </SidebarGroupContent>
            </SidebarGroup>
          </SidebarContent>
          <SidebarFooter>
            <div className="flex items-center justify-between p-4">
              <Button
                variant="outline"
                size="icon"
                onClick={() => setTheme(theme === "dark" ? "light" : "dark")}
              >
                {theme === "dark" ? (
                  <Sun className="h-4 w-4" />
                ) : (
                  <Moon className="h-4 w-4" />
                )}
                <span className="sr-only">Toggle theme</span>
              </Button>
              <Button variant="outline" size="icon">
                <Settings className="h-4 w-4" />
                <span className="sr-only">Settings</span>
              </Button>
            </div>
          </SidebarFooter>
        </Sidebar>
        <div className="flex-1 overflow-auto">
          <header className="sticky top-0 z-10 flex h-14 items-center gap-4 border-b bg-background px-4 sm:px-6">
            <SidebarTrigger />
            <div>
              <h1 className="text-lg font-semibold">
                {activePanel === "game"
                  ? "Chess Game"
                  : activePanel === "history"
                    ? "Move History"
                    : activePanel === "analysis"
                      ? "Position Analysis"
                      : "Engine Settings"}
              </h1>
            </div>
          </header>
          <main className="flex-1 p-4 md:p-6">{renderPanel()}</main>
        </div>
      </div>
    </SidebarProvider>
  );
}
