import { ModrinthV2Client } from "@xmcl/modrinth";

export async function isOnVersion(
  client: ModrinthV2Client,
  projectId: string,
  loader: string,
  version: string,
) {
  const res = await client.getProjectVersions(projectId, {
    gameVersions: [version],
    loaders: [loader],
  });

  if (res.length === 0) return false;

  return true;
}

export async function getProjectTitle(
  client: ModrinthV2Client,
  projectId: string,
) {
  const res = await client.getProject(projectId);

  return res.title;
}
