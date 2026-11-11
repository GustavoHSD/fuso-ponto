import { useState, useEffect } from "react";
import { open as openDialog } from "@tauri-apps/plugin-dialog";
import { api } from "@/lib/api";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Copy, FileCheck2 } from "lucide-react";
import { toast } from "sonner";

export function Settings() {
  const [deviceId, setDeviceId] = useState("");
  const [paidUpTo, setPaidUpTo] = useState<string | null>(null);

  useEffect(() => {
    api.getDeviceId().then(setDeviceId).catch(console.error);
    api.getPaidUpTo().then(setPaidUpTo).catch(console.error);
  }, []);

  const handleCopy = async () => {
    try {
      await navigator.clipboard.writeText(deviceId);
      toast.success("ID copiado!");
    } catch (err) {
      console.error("Copy failed", err);
      toast.error("Falha ao copiar");
    }
  };

  const handleImport = async () => {
    const path = await openDialog({
      multiple: false,
      filters: [{ name: "Comprovante Ponto", extensions: ["ponto"] }],
    });
    if (typeof path !== "string") return;
    try {
      const newValue = await api.importPaidFile(path);
      setPaidUpTo(newValue);
      toast.success(`Pago até ${formatDate(newValue)}`);
    } catch (err) {
      console.error("Import failed", err);
      toast.error(String(err));
    }
  };

  const formatDate = (iso: string) =>
    new Date(iso + "T00:00:00").toLocaleDateString("pt-BR");

  return (
    <div className="flex flex-col gap-4 p-4">
      <h2 className="text-xl font-bold">Ajustes</h2>

      <Card>
        <CardHeader>
          <CardTitle className="text-base">Identidade deste app</CardTitle>
        </CardHeader>
        <CardContent className="flex flex-col gap-2">
          <p className="text-sm text-muted-foreground">
            Envie este ID para o administrador emitir seu comprovante de pagamento.
          </p>
          <div className="flex items-center gap-2">
            <code className="flex-1 text-xs bg-muted rounded px-2 py-2 break-all select-all">
              {deviceId || "…"}
            </code>
            <Button onClick={handleCopy} variant="outline" size="sm">
              <Copy className="w-4 h-4 mr-1" /> Copiar
            </Button>
          </div>
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle className="text-base">Comprovante de pagamento</CardTitle>
        </CardHeader>
        <CardContent className="flex flex-col gap-2">
          <p className="text-sm text-muted-foreground">
            {paidUpTo
              ? `Status atual: pago até ${formatDate(paidUpTo)}.`
              : "Nenhum comprovante importado."}
          </p>
          <Button onClick={handleImport} variant="outline" size="sm" className="self-start">
            <FileCheck2 className="w-4 h-4 mr-2" /> Importar comprovante…
          </Button>
        </CardContent>
      </Card>
    </div>
  );
}
