export interface DefaultCommand {
  isDefault: true;
  action: string | ((...args: string[]) => string);
}

export interface NamedCommand {
  tags: string[];
  description: string;
  action: string | ((...args: string[]) => string);
}

export type Command = DefaultCommand | NamedCommand;
