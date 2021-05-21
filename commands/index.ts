import { Command, DefaultCommand } from "./type";

export const commands: Command[] = [
  {
    tags: ["c", "cal"],
    description: "Launches Google calendar",
    action: "https://calendar.google.com/calendar/u/0",
  },
  {
    tags: ["gm"],
    description: "Launches Gmail",
    action: "https://gmail.com/",
  },
  {
    tags: ["gh"],
    description: "Launches GitHub",
    action: "http://github.com/",
  },
  {
    tags: ["y"],
    description: "Launches Youtube",
    action: (...search) =>
      search.length > 0
        ? `https://www.youtube.com/results?search_query=${encodeURIComponent(
            search.join(" ")
          )}`
        : "http://youtube.com/",
  },
  {
    tags: ["n"],
    description: "Launches Netflix",
    action: "https://www.netflix.com/",
  },
  {
    isDefault: true,
    description: "Searches Google",
    action: (...search) =>
      `https://www.google.com/search?q=${encodeURIComponent(search.join(" "))}`,
  },
];

export function isDefaultCommand(command: unknown): command is DefaultCommand {
  return (
    typeof command === "object" &&
    command !== null &&
    (command as any).isDefault
  );
}

export const commandMap = new Map(
  commands.flatMap((command) => {
    if (isDefaultCommand(command)) {
      return [["default", command]];
    } else {
      return command.tags.map((tag) => [tag, command]) as [string, Command][];
    }
  }) as [string, Command][]
);
