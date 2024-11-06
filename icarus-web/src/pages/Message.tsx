import {Chip, Table, TableBody, TableCell, TableColumn, TableHeader, TableRow} from "@nextui-org/react";
import {MessageQueue} from "../types.ts";

export default function Message() {
    const columns = [
        {name: "VALUE", uid: "value"},
        {name: "STATUS", uid: "status"},
    ]

    const data: MessageQueue[] = [
        {channel: "Test 0", entries: ["1", "2", "3"]},
        {channel: "Test 1", entries: ["4", "5", "6"]},
    ]

    return (
        <div className="flex flex-col gap-4 md:p-4">
            <div className="flex flex-col gap-1">
                <h1 className="text-2xl font-bold">Makemake Message Queue</h1>
                <p className="text-gray-500">Rapidly deliver data across distributed application clusters.</p>
            </div>

            {data.map((item, index) => (
                <div key={index} className="flex flex-col gap-4">
                    <h2 className="text-xl font-bold">{item.channel}</h2>
                    <Table aria-label="Example table with custom cells">
                        <TableHeader columns={columns}>
                            {(column) => (
                                <TableColumn key={column.uid} align={column.uid === "status" ? "center" : "start"}>
                                    {column.name}
                                </TableColumn>
                            )}
                        </TableHeader>
                        <TableBody items={item.entries}>
                            {item.entries.map((entry, index) => {
                                const first: boolean = index == 0;
                                const end: boolean = (index === item.entries.length - 1)

                                return (
                                    <TableRow key={entry}>
                                        <TableCell>{entry}</TableCell>
                                        <TableCell>
                                            {
                                                first || end ?
                                                    (first ? <Chip className="bg-yellow-200">Head</Chip> :
                                                        <Chip className="bg-blue-200">End</Chip>) :
                                                    <Chip className="bg-green-300">Active</Chip>
                                            }
                                        </TableCell>
                                    </TableRow>
                                )
                            })}
                        </TableBody>
                    </Table>
                </div>
            ))}
        </div>
    )
}