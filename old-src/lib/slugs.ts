import { ensureFile } from "@std/fs";

const modrinthProjectSlugRegex = /^[\w!@$()`.+,"\-']{3,64}$/;

async function getModSlugsFromFile(
  filename: string,
): Promise<{ ids: string[]; invalidLines: number }> {
  await ensureFile(filename);

  const text = await Deno.readTextFile(filename);
  const lines = text.split("\n");

  let invalidLines = 0;

  const ids = lines
    .map((line) => line.trim())
    .filter((line) => line.length > 0 && !line.startsWith("#"))
    .filter((line) => {
      const isValid = modrinthProjectSlugRegex.test(line);
      if (!isValid) invalidLines++;
      return isValid;
    });

  return { ids, invalidLines };
}

export { modrinthProjectSlugRegex, getModSlugsFromFile };
