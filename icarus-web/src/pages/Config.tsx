import {Button, Chip, Table, TableBody, TableCell, TableColumn, TableHeader, TableRow} from "@nextui-org/react";
import {EyeIcon, PencilSquareIcon, TrashIcon} from "@heroicons/react/24/outline";

export default function Config() {
    const users = [
        {
            "key": "key1",
            "value": "value1"
        }
    ]

    const columns = [
        {name: "KEY", uid: "key"},
        {name: "VALUE", uid: "value"},
        {name: "STATUS", uid: "status"},
        {name: "ACTIONS", uid: "actions"},
    ]

    return (
        <div className="flex flex-col gap-4 md:p-4">
            <div className="flex flex-col gap-1">
                <h1 className="text-2xl font-bold">Aurora Config Center</h1>
                <p className="text-gray-500">A reliable distributed configuration management center where configuration
                    options are created and modified.</p>
            </div>
            <div className="flex flex-col gap-4">
                <div className="flex flex-row gap-2">
                    <Button color="primary">
                        Create Config Entry
                    </Button>
                    <Button color="danger">
                        Clear All
                    </Button>
                </div>
                <Table aria-label="Example table with custom cells">
                    <TableHeader columns={columns}>
                        {(column) => (
                            <TableColumn key={column.uid}
                                         align={column.uid === "actions" || column.uid === "status" ? "center" : "start"}>
                                {column.name}
                            </TableColumn>
                        )}
                    </TableHeader>
                    <TableBody items={users}>
                        {(item) => (
                            <TableRow key={item.key}>
                                <TableCell>{item.key}</TableCell>
                                <TableCell>
                                    <p className="line-clamp-3">
                                        {item.value}
                                    </p>
                                </TableCell>
                                <TableCell>
                                    <Chip className="bg-green-300">Active</Chip>
                                </TableCell>
                                <TableCell>
                                    <div className="flex flex-row gap-2 justify-center items-center">
                                        <EyeIcon className="size-5 text-gray-400"/>
                                        <PencilSquareIcon className="size-5 text-gray-400"/>
                                        <TrashIcon className="size-5 text-red-400"/>
                                    </div>
                                </TableCell>
                            </TableRow>
                        )}
                    </TableBody>
                </Table>
            </div>
        </div>
    )
}