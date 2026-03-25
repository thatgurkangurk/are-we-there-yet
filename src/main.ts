import { run, command, string } from "@drizzle-team/brocli";
import { getModSlugsFromFile } from "./lib/slugs.ts";
import ProgressBar from "@deno-library/progress";
import { ModrinthV2Client } from "@xmcl/modrinth";
import { getProjectTitle, isOnVersion } from "./lib/project.ts";

const check = command({
  name: "check",
  desc: "check if your mods are updated",
  options: {
    file: string().required(),
    "mc-version": string().required(),
    loader: string().required().enum("neoforge", "forge", "fabric"),
    out: string().required(),
  },
  handler: async (options) => {
    const modSlugs = await getModSlugsFromFile(options.file);

    const bar = new ProgressBar({
      title: "processing: ",
      total: modSlugs.ids.length,
    });

    const client = new ModrinthV2Client();

    const file = await Deno.open(options.out, {
      write: true,
      create: true,
      truncate: true,
    });

    const lines: string[] = [];

    for (const [index, id] of modSlugs.ids.entries()) {
      await bar.render(index);

      const title = await getProjectTitle(client, id);

      const isIt = await isOnVersion(
        client,
        id,
        options.loader,
        options["mc-version"],
      );

      lines.push(`${title} - ${isIt ? "✅" : "❌"}`);
    }

    await file.write(new TextEncoder().encode(lines.join("\n") + "\n"));

    file.close();
  },
});

run([check], {
  //  this is required because brocli ignores the first 2 arguments "as they typically contain executable and executed file paths"
  argSource: ["", "", ...Deno.args],
  name: "are-we-there-yet",
});
