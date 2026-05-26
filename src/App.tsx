import { useState } from "react";
import { Dashboard } from "@/components/Dashboard";
import { Reports } from "@/components/Reports";
import { Toaster } from "@/components/ui/sonner";
import { LayoutDashboard, ListRestart } from "lucide-react";

function App() {
  const [activeTab, setActiveTab] = useState<"dashboard" | "reports">("dashboard");

  return (
    <div className="h-screen bg-background flex flex-col overflow-hidden">
      <header className="sticky top-0 z-50 p-4 pt-safe border-b flex justify-between items-center bg-card/80 backdrop-blur-md">
        <div className="flex items-center gap-3">
          <img src="/fuso-ferrementas-logo.jpg" alt="Fuso" className="h-16 w-auto rounded-md" />
          <h1 className="text-2xl font-bold tracking-tight">Ponto</h1>
        </div>
        <div className="flex gap-2">
          <button
            onClick={() => setActiveTab("dashboard")}
            title="Painel"
            className={`p-2 rounded-md transition-colors ${
              activeTab === "dashboard" ? "bg-accent text-accent-foreground" : "text-muted-foreground hover:bg-accent/50"
            }`}
          >
            <LayoutDashboard className="w-6 h-6" />
          </button>
          <button
            onClick={() => setActiveTab("reports")}
            title="Relatórios"
            className={`p-2 rounded-md transition-colors ${
              activeTab === "reports" ? "bg-accent text-accent-foreground" : "text-muted-foreground hover:bg-accent/50"
            }`}
          >
            <ListRestart className="w-6 h-6" />
          </button>
        </div>
      </header>

      <main className="flex-1 overflow-y-auto">
        {activeTab === "dashboard" ? <Dashboard /> : <Reports />}
      </main>

      <footer className="sticky bottom-0 z-50 p-4 pb-safe border-t flex justify-center items-center gap-3 bg-background/80 backdrop-blur-md">
        <span className="text-[10px] text-muted-foreground uppercase font-bold tracking-widest">Fuso Ponto</span>
        <img src="/fuso-ferrementas-logo.jpg" alt="Fuso" className="h-4 w-auto rounded-sm opacity-60 grayscale hover:grayscale-0 hover:opacity-100 transition-all duration-300" />
      </footer>

      <Toaster />
    </div>
  );
}

export default App;
