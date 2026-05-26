import { useState, useEffect, useCallback } from "react";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { api, TimeEntry, DailySummary, Report } from "@/lib/api";
import { Play, Square, Clock, BarChart3 } from "lucide-react";
import {
  BarChart,
  Bar,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  ResponsiveContainer,
  Cell,
} from "recharts";

export function Dashboard() {
  const [activeSession, setActiveSession] = useState<TimeEntry | null>(null);
  const [summary, setSummary] = useState<DailySummary | null>(null);
  const [weeklyData, setWeeklyData] = useState<any[]>([]);
  const [elapsed, setElapsed] = useState(0);
  const [isLoading, setIsLoading] = useState(true);

  const processWeeklyData = useCallback((reports: Report[]) => {
    const days = ["Dom", "Seg", "Ter", "Qua", "Qui", "Sex", "Sáb"];
    const now = new Date();
    const data = [];

    // Get the current week's Monday
    const currentDay = now.getDay(); // 0 is Sunday, 1 is Monday...
    const diffToMonday = currentDay === 0 ? -6 : 1 - currentDay;
    const monday = new Date(now);
    monday.setDate(now.getDate() + diffToMonday);

    // Show Monday to Friday (Seg to Sex)
    for (let i = 0; i < 5; i++) {
      const d = new Date(monday);
      d.setDate(monday.getDate() + i);
      const dateStr = d.toISOString().split("T")[0];
      const dayName = days[d.getDay()];

      const report = reports.find((r) => r.date === dateStr);
      data.push({
        name: dayName,
        date: dateStr,
        hours: report ? parseFloat((report.total_seconds / 3600).toFixed(2)) : 0,
      });
    }
    return data;
  }, []);

  const fetchData = useCallback(async () => {
    setIsLoading(true);
    try {
      const [session, daySummary, reports] = await Promise.all([
        api.getActiveSession(),
        api.getTodaySummary(),
        api.getReports(),
      ]);
      
      if (session) {
        const startTime = new Date(session.start_time).getTime();
        setElapsed(Math.floor((Date.now() - startTime) / 1000));
      }
      
      setActiveSession(session);
      setSummary(daySummary);
      setWeeklyData(processWeeklyData(reports));
    } catch (err) {
      console.error("Failed to fetch dashboard data", err);
    } finally {
      setIsLoading(false);
    }
  }, [processWeeklyData]);

  useEffect(() => {
    fetchData();
  }, [fetchData]);

  useEffect(() => {
    let interval: number;
    if (activeSession) {
      const startTime = new Date(activeSession.start_time).getTime();
      const updateElapsed = () => setElapsed(Math.floor((Date.now() - startTime) / 1000));
      
      updateElapsed(); // Run immediately
      interval = window.setInterval(updateElapsed, 1000);
    } else {
      setElapsed(0);
    }
    return () => clearInterval(interval);
  }, [activeSession]);

  const toggleTimer = async () => {
    try {
      if (activeSession?.id) {
        await api.stopTimer(activeSession.id);
      } else {
        await api.startTimer();
      }
      await fetchData();
    } catch (err) {
      console.error("Failed to toggle timer", err);
    }
  };

  const formatTime = (seconds: number) => {
    const h = Math.floor(seconds / 3600);
    const m = Math.floor((seconds % 3600) / 60);
    const s = seconds % 60;
    return `${h.toString().padStart(2, "0")}:${m
      .toString()
      .padStart(2, "0")}:${s.toString().padStart(2, "0")}`;
  };

  return (
    <div className="flex flex-col items-center gap-6 p-4 max-w-lg mx-auto">
      <Card className="w-full text-center border-primary/20 shadow-lg shadow-primary/5">
        <CardHeader>
          <CardTitle className="text-xl font-bold">Status Atual</CardTitle>
        </CardHeader>
        <CardContent>
          <div className="text-5xl font-extrabold tracking-tighter mb-4 text-primary">
            {isLoading ? (
              <span className="animate-pulse opacity-50">--:--:--</span>
            ) : activeSession ? (
              formatTime(elapsed)
            ) : (
              "00:00:00"
            )}
          </div>
          <div className="text-muted-foreground font-medium mb-8">
            {isLoading ? "Carregando..." : activeSession ? "Registrando" : "Pausado"}
          </div>
          <div className="flex justify-center">
            <Button
              size="lg"
              disabled={isLoading}
              className={`w-40 h-40 rounded-full text-white shadow-2xl transition-all duration-300 hover:scale-105 active:scale-95 ${
                activeSession 
                  ? "bg-destructive hover:bg-destructive/90 shadow-destructive/20" 
                  : "bg-primary hover:bg-primary/90 shadow-primary/20"
              }`}
              onClick={toggleTimer}
            >
              {isLoading ? (
                <Clock className="w-16 h-16 animate-spin opacity-20" />
              ) : activeSession ? (
                <Square className="w-16 h-16 fill-current" />
              ) : (
                <Play className="w-16 h-16 fill-current ml-2" />
              )}
            </Button>
          </div>
        </CardContent>
      </Card>

      <div className="grid grid-cols-1 w-full gap-4">
        <Card className="w-full">
          <CardHeader className="pb-2">
            <CardTitle className="text-sm font-semibold flex items-center gap-2 text-muted-foreground uppercase tracking-wider">
              <Clock className="w-4 h-4 text-primary" /> Resumo de Hoje
            </CardTitle>
          </CardHeader>
          <CardContent>
            <div className="text-3xl font-bold">
              {isLoading ? (
                <span className="animate-pulse opacity-50">--:--:--</span>
              ) : summary ? (
                formatTime(summary.total_seconds)
              ) : (
                "00:00:00"
              )}
            </div>
          </CardContent>
        </Card>


        <Card className="w-full overflow-hidden">
          <CardHeader className="pb-4">
            <CardTitle className="text-sm font-semibold flex items-center gap-2 text-muted-foreground uppercase tracking-wider">
              <BarChart3 className="w-4 h-4 text-primary" /> Atividade Semanal
            </CardTitle>
          </CardHeader>
          <CardContent className="px-2">
            <div className="h-[200px] w-full">
              <ResponsiveContainer width="100%" height="100%">
                <BarChart data={weeklyData} margin={{ top: 0, right: 0, left: -20, bottom: 0 }}>
                  <CartesianGrid strokeDasharray="3 3" vertical={false} stroke="hsl(var(--muted))" />
                  <XAxis 
                    dataKey="name" 
                    axisLine={false} 
                    tickLine={false} 
                    tick={{ fontSize: 12, fill: "hsl(var(--muted-foreground))" }}
                    dy={10}
                  />
                  <YAxis 
                    axisLine={false} 
                    tickLine={false} 
                    tick={{ fontSize: 12, fill: "hsl(var(--muted-foreground))" }}
                  />
                  <Tooltip 
                    cursor={{ fill: 'hsl(var(--muted))', opacity: 0.4 }}
                    contentStyle={{ 
                      borderRadius: 'var(--radius)', 
                      border: '1px solid var(--border)',
                      backgroundColor: 'var(--card)',
                      fontSize: '12px'
                    }}
                    formatter={(value: any) => [`${value}h`, 'Horas']}
                    labelStyle={{ fontWeight: 'bold', marginBottom: '4px' }}
                  />
                  <Bar 
                    dataKey="hours" 
                    radius={[4, 4, 0, 0]} 
                    barSize={30}
                  >
                    {weeklyData.map((entry, index) => (
                      <Cell 
                        key={`cell-${index}`} 
                        fill={entry.date === new Date().toISOString().split("T")[0] ? "var(--primary)" : "hsl(var(--primary) / 0.3)"} 
                      />
                    ))}
                  </Bar>
                </BarChart>
              </ResponsiveContainer>
            </div>
          </CardContent>
        </Card>
      </div>
    </div>
  );
}
