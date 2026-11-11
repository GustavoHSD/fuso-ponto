import { useState, useEffect } from "react";
import { api, Report } from "@/lib/api";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import { Button } from "@/components/ui/button";
import { Share2 } from "lucide-react";
import { shareFile } from "@choochmeque/tauri-plugin-sharekit-api";
import { toast } from "sonner";

export function Reports() {
  const [reports, setReports] = useState<Report[]>([]);
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    setIsLoading(true);
    api.getReports()
      .then(setReports)
      .catch(console.error)
      .finally(() => setIsLoading(false));
  }, []);

  const handleExport = async () => {
    try {
      const path = await api.exportData();
      await shareFile(`file://${path}`, {
        mimeType: "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
        title: "ponto_export.xlsx",
      });
      toast.success("Planilha pronta para compartilhar!");
    } catch (err) {
      console.error("Export failed", err);
      toast.error("Falha na exportação");
    }
  };

  const formatTime = (seconds: number) => {
    const h = Math.floor(seconds / 3600);
    const m = Math.floor((seconds % 3600) / 60);
    return `${h}h ${m}m`;
  };

  return (
    <div className="flex flex-col gap-4 p-4">
      <div className="flex justify-between items-center">
        <h2 className="text-xl font-bold">Relatórios Diários</h2>
        <Button onClick={handleExport} variant="outline" size="sm">
          <Share2 className="w-4 h-4 mr-2" /> Compartilhar
        </Button>
      </div>

      <div className="rounded-md border">
        <Table>
          <TableHeader>
            <TableRow>
              <TableHead>Data</TableHead>
              <TableHead className="text-right">Duração</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {isLoading ? (
              <TableRow>
                <TableCell colSpan={2} className="text-center text-muted-foreground h-24 italic animate-pulse">
                  Carregando relatórios...
                </TableCell>
              </TableRow>
            ) : reports.length === 0 ? (
              <TableRow>
                <TableCell colSpan={2} className="text-center text-muted-foreground h-24">
                  Nenhum registro encontrado
                </TableCell>
              </TableRow>
            ) : (
              reports.map((report) => (
                <TableRow key={report.date}>
                  <TableCell className="font-medium">
                    {new Date(report.date + "T00:00:00").toLocaleDateString("pt-BR")}
                  </TableCell>
                  <TableCell className="text-right">
                    {formatTime(report.total_seconds)}
                  </TableCell>
                </TableRow>
              ))
            )}
          </TableBody>
        </Table>
      </div>
    </div>
  );
}
