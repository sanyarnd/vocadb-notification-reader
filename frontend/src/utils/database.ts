import type { Database } from "@/api/dto";

const databaseUrls: Record<Database, string> = {
  VocaDb: "https://vocadb.net",
  TouhouDb: "https://touhoudb.com",
  UtaiteDb: "https://utaitedb.net"
};

export function songUrl(database: Database, songId: number): string {
  return `${databaseUrls[database]}/S/${songId}`;
}
