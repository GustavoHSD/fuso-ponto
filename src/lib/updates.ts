import { getVersion } from "@tauri-apps/api/app";

// Public repo where releases are published. Tag a release vX.Y.Z to trigger the prompt.
const REPO = "GustavoHSD/fuso-ponto";

export interface UpdateInfo {
  version: string;
  url: string;
}

/** Numeric compare of dotted versions ("v0.2.1" vs "0.3"): -1 a<b, 0 equal, 1 a>b. */
export function compareVersions(a: string, b: string): number {
  const pa = a.replace(/^v/, "").split(".");
  const pb = b.replace(/^v/, "").split(".");
  for (let i = 0; i < Math.max(pa.length, pb.length); i++) {
    const na = parseInt(pa[i] ?? "0", 10);
    const nb = parseInt(pb[i] ?? "0", 10);
    if (na !== nb) return na < nb ? -1 : 1;
  }
  return 0;
}

/** Returns info about a newer release, or null when up to date / unreachable. */
export async function checkForUpdate(): Promise<UpdateInfo | null> {
  const current = await getVersion();
  const res = await fetch(`https://api.github.com/repos/${REPO}/releases/latest`);
  if (!res.ok) return null;
  const release: { tag_name: string; html_url: string } = await res.json();
  if (compareVersions(release.tag_name, current) <= 0) return null;
  return { version: release.tag_name, url: release.html_url };
}
