import { useEffect } from "react";
import { useRouter } from "next/router";
import { commandMap, commands, isDefaultCommand } from "../commands";

export interface SearchProps {}

const Search: React.FC<SearchProps> = () => {
  const router = useRouter();
  const { search } = router.query;

  useEffect(() => {
    if (search) {
      const [tag, ...options] = (search as string).split(" ");

      const command = commandMap.get(tag) ?? commandMap.get("default");

      if (typeof command.action === "string")
        window.location.assign(command.action);
      else window.location.assign(command.action(...options));
    }
  }, [search]);

  return null;
};

export default Search;
