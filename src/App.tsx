import { useState } from "react";
import { Dashboard } from "@/components/Dashboard";
import { Reports } from "@/components/Reports";
import { Toaster } from "@/components/ui/sonner";
import { LayoutDashboard, ListRestart } from "lucide-react";

function App() {
  const [activeTab, setActiveTab] = useState<"dashboard" | "reports">("dashboard");

  return (
    <div className="h-[100dvh] bg-background flex flex-col overflow-hidden">
      <header className="sticky top-0 z-50 px-4 py-3 border-b flex justify-between items-center bg-card/80 backdrop-blur-md pt-safe">
        <div className="flex items-center gap-2 sm:gap-3 min-w-0 flex-shrink">
          <img src="/fuso-ferrementas-logo.jpg" alt="Fuso" className="h-10 sm:h-14 w-auto rounded-md flex-shrink-0" />
          <h1 className="text-xl sm:text-2xl font-bold tracking-tight truncate">Ponto</h1>
        </div>
        <div className="flex gap-1 sm:gap-2 flex-shrink-0">
          <button
            onClick={() => setActiveTab("dashboard")}
            title="Painel"
            className={`p-2 rounded-md transition-colors ${
              activeTab === "dashboard" ? "bg-accent text-accent-foreground" : "text-muted-foreground hover:bg-accent/50"
            }`}
          >
            <LayoutDashboard className="w-5 h-5 sm:w-6 sm:h-6" />
          </button>
          <button
            onClick={() => setActiveTab("reports")}
            title="Relatórios"
            className={`p-2 rounded-md transition-colors ${
              activeTab === "reports" ? "bg-accent text-accent-foreground" : "text-muted-foreground hover:bg-accent/50"
            }`}
          >
            <ListRestart className="w-5 h-5 sm:w-6 sm:h-6" />
          </button>
        </div>
      </header>

      <main className="flex-1 overflow-y-auto">
        {activeTab === "dashboard" ? <Dashboard /> : <Reports />}
      </main>

      <footer className="sticky bottom-0 z-50 px-4 py-3 border-t flex justify-center items-center gap-3 bg-background/80 backdrop-blur-md pb-safe">
        <span className="text-[10px] text-muted-foreground uppercase font-bold tracking-widest">Fuso Ponto</span>
        <img src="/fuso-ferrementas-logo.jpg" alt="Fuso" className="h-4 w-auto rounded-sm opacity-60 grayscale" />
      </footer>

      <Toaster />
    </div>
  );
}

export default App;
