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
  const [paidUpTo, setPaidUpTo] = useState<string | null>(null);

  useEffect(() => {
    setIsLoading(true);
    Promise.all([api.getReports(), api.getPaidUpTo()])
      .then(([reports, paidUpTo]) => {
        setReports(reports);
        setPaidUpTo(paidUpTo);
      })
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

  const formatDate = (iso: string) =>
    new Date(iso + "T00:00:00").toLocaleDateString("pt-BR");

  return (
    <div className="flex flex-col gap-4 p-4">
      <div className="flex justify-between items-center">
        <h2 className="text-xl font-bold">Relatórios Diários</h2>
        <div className="flex items-center gap-2">
          {paidUpTo && (
            <span className="text-xs font-semibold text-green-700 bg-green-100 rounded-full px-2.5 py-1">
              Pago até {formatDate(paidUpTo)}
            </span>
          )}
          <Button onClick={handleExport} variant="outline" size="sm">
            <Share2 className="w-4 h-4 mr-2" /> Compartilhar
          </Button>
        </div>
      </div>

      <div className="rounded-md border">
        <Table>
          <TableHeader>
            <TableRow>
              <TableHead>Data</TableHead>
              <TableHead className="text-right">Duração</TableHead>
              <TableHead className="text-right">Status</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {isLoading ? (
              <TableRow>
                <TableCell colSpan={3} className="text-center text-muted-foreground h-24 italic animate-pulse">
                  Carregando relatórios...
                </TableCell>
              </TableRow>
            ) : reports.length === 0 ? (
              <TableRow>
                <TableCell colSpan={3} className="text-center text-muted-foreground h-24">
                  Nenhum registro encontrado
                </TableCell>
              </TableRow>
            ) : (
              reports.map((report) => (
                <TableRow key={report.date}>
                  <TableCell className="font-medium">
                    {formatDate(report.date)}
                  </TableCell>
                  <TableCell className="text-right">
                    {formatTime(report.total_seconds)}
                  </TableCell>
                  <TableCell className="text-right">
                    {paidUpTo && report.date <= paidUpTo ? (
                      <span className="text-xs font-semibold text-green-700 bg-green-100 rounded-full px-2 py-0.5">
                        Pago
                      </span>
                    ) : null}
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
