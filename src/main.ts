import { run, command, string } from "@drizzle-team/brocli";

const check = command({
  name: "check",
  desc: "check if your mods are updated",
  options: {
    file: string().required(),
    "mc-version": string().required(),
  },
  handler: async (options) => {
    //
  },
});

run([check], {
  //  this is required because brocli ignores the first 2 arguments "as they typically contain executable and executed file paths"
  argSource: ["", "", ...Deno.args],
  name: "are-we-there-yet",
});
