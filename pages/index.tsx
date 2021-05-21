import { commands, isDefaultCommand } from "../commands";
import Head from "next/head";
import { NamedCommand } from "../commands/type";

export default function Home() {
  return (
    <>
      <Head>
        {/* Inter Font */}
        <link
          rel="preload"
          href="/fonts/inter-var.woff2"
          as="font"
          type="font/woff2"
          crossOrigin="anonymous"
        />
      </Head>
      <div className="flex">
        <div className="mx-auto mt-8">
          <h1 className="font-bold text-8xl">Bunnylol</h1>

          {/* <div className="border border-gray-300 rounded-lg overflow-hidden">
          <div className="font-bold bg-blue-300 flex">
            <span>Tags</span>
            <span>Description</span>
          </div>
        </div> */}

          <div className="flex flex-col">
            <div className="-my-2 overflow-x-auto sm:-mx-6 lg:-mx-8">
              <div className="py-2 align-middle inline-block min-w-full sm:px-6 lg:px-8">
                <div className="shadow overflow-hidden border-b border-gray-200 sm:rounded-lg">
                  <table className="min-w-full divide-y divide-gray-200">
                    <thead className="bg-gray-50">
                      <tr>
                        <th
                          scope="col"
                          className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
                        >
                          Name
                        </th>
                        <th
                          scope="col"
                          className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
                        >
                          Description
                        </th>
                      </tr>
                    </thead>
                    <tbody className="bg-white divide-y divide-gray-200">
                      {commands
                        .filter((command) => !isDefaultCommand(command))
                        .map((command: NamedCommand, index) => (
                          <tr key={command.description}>
                            <td className="text-sm px-6 py-4 whitespace-nowrap">
                              {command.tags.join(", ")}
                            </td>
                            <td className="text-sm px-6 py-4 whitespace-nowrap">
                              {command.description}
                            </td>
                          </tr>
                        ))}

                      <tr>
                        <td className="px-6 py-4 whitespace-nowrap">
                          <div className="flex items-center">
                            <div className="flex-shrink-0 h-10 w-10"></div>
                            <div className="ml-4">
                              <div className="text-sm font-medium text-gray-900">
                                Jane Cooper
                              </div>
                              <div className="text-sm text-gray-500">
                                jane.cooper@example.com
                              </div>
                            </div>
                          </div>
                        </td>
                        <td className="px-6 py-4 whitespace-nowrap">
                          <div className="text-sm text-gray-900">
                            Regional Paradigm Technician
                          </div>
                          <div className="text-sm text-gray-500">
                            Optimization
                          </div>
                        </td>
                      </tr>
                    </tbody>
                  </table>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </>
  );
}
