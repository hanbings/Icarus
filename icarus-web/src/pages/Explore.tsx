import {Chip, Table, TableBody, TableCell, TableColumn, TableHeader, TableRow} from "@nextui-org/react";
import {ExploreServiceEntry} from "../types.ts";
import ExploreServiceTimeChip from "../components/ExploreServiceTimeChip.tsx";

interface ExploreService {
    instance_name: string,
    endpoint: string,
    created: number,
    last_updated: number,
}

export default function Explore() {
    const columns = [
        {name: "INSTANCE NAME", uid: "instance_name"},
        {name: "ENDPOINT", uid: "endpoint"},
        {name: "CREATED", uid: "created"},
        {name: "LAST UPDATE", uid: "last_update"},
        {name: "STATUS", uid: "status"},
    ]

    const data: ExploreServiceEntry[] = [
        {
            "endpoint": "http://127.0.0.1:63000",
            "created": 1730910000,
            "last_updated": 1730910853,
            "service_name": "Test Service 1",
            "instance_name": "Test 0"
        },
        {
            "endpoint": "http://127.0.0.1:63000",
            "created": 1730910000,
            "last_updated": 1730910853,
            "service_name": "Test Service 1",
            "instance_name": "Test 0"
        },
        {
            "endpoint": "http://127.0.0.1:63000",
            "created": 1730910000,
            "last_updated": 1730910853,
            "service_name": "Test Service 0",
            "instance_name": "Test 1"
        }
    ]

    const services = groupByServiceName(data)

    function groupByServiceName(entries: ExploreServiceEntry[]): Record<string, ExploreService[]> {
        return entries.reduce((acc, entry) => {
            const service: ExploreService = {
                instance_name: entry.instance_name,
                endpoint: entry.endpoint,
                created: entry.created,
                last_updated: entry.last_updated,
            };

            if (!acc[entry.service_name]) {
                acc[entry.service_name] = [];
            }

            acc[entry.service_name].push(service);
            return acc;
        }, {} as Record<string, ExploreService[]>);
    }

    return (
        <div className="flex flex-col gap-4 md:p-4">
            <div className="flex flex-col gap-1">
                <h1 className="text-2xl font-bold">Flora Explore Service</h1>
                <p className="text-gray-500">A service for fast online indexing of distributed applications.</p>
            </div>
            {Object.keys(services).map((serviceName) => {
                const service = services[serviceName];

                return (
                    <div className="flex flex-col gap-4" key={serviceName}>
                        <h2 className="text-xl font-bold">{serviceName}</h2>
                        <Table aria-label="Example table with custom cells">
                            <TableHeader columns={columns}>
                                {(column) => (
                                    <TableColumn key={column.uid} align={column.uid === "status" ? "center" : "start"}>
                                        {column.name}
                                    </TableColumn>
                                )}
                            </TableHeader>
                            <TableBody items={service}>
                                {(entry) => (
                                    <TableRow key={entry.endpoint}>
                                        <TableCell>{entry.instance_name}</TableCell>
                                        <TableCell>
                                            <p className="line-clamp-3">
                                                {entry.endpoint}
                                            </p>
                                        </TableCell>
                                        <TableCell>
                                            <ExploreServiceTimeChip timestamp={entry.created}/>
                                        </TableCell>
                                        <TableCell>
                                            <ExploreServiceTimeChip timestamp={entry.last_updated}/>
                                        </TableCell>
                                        <TableCell>
                                            <Chip className="bg-green-300">Active</Chip>
                                        </TableCell>
                                    </TableRow>
                                )}
                            </TableBody>
                        </Table>
                    </div>
                )
            })}
        </div>
    )
}