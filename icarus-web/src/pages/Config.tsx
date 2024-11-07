import {
    Button,
    Chip,
    Input,
    Modal,
    ModalBody,
    ModalContent,
    ModalFooter,
    ModalHeader,
    Table,
    TableBody,
    TableCell,
    TableColumn,
    TableHeader,
    TableRow,
    Textarea,
    useDisclosure
} from "@nextui-org/react";
import {EyeIcon, PencilSquareIcon, TrashIcon} from "@heroicons/react/24/outline";
import {useState} from "react";
import {useQuery} from "@tanstack/react-query";
import {IcarusConfig} from "../config.ts";
import axios from "axios"

export default function Config() {
    const columns = [
        {name: "KEY", uid: "key"},
        {name: "VALUE", uid: "value"},
        {name: "STATUS", uid: "status"},
        {name: "ACTIONS", uid: "actions"},
    ]

    const {data} = useQuery({
        queryKey: ["config"],
        queryFn: (): Promise<Map<string, string>> =>
            axios.get(`${IcarusConfig.api}/config`, {
                headers: {
                    'Authorization': `Bearer a60e9151-62a9-12d5-f37f-83e2ce88b334`
                }
            }).then(data => data.data),
    });

    const {isOpen, onOpen, onOpenChange} = useDisclosure();
    const [inputValue, setInputValue] = useState("");
    const [textareaValue, setTextareaValue] = useState("");

    return (
        <div className="flex flex-col gap-4 md:p-4">
            <div className="flex flex-col gap-1">
                <h1 className="text-2xl font-bold">Aurora Config Center</h1>
                <p className="text-gray-500">A reliable distributed configuration management center where configuration
                    options are created and modified.</p>
            </div>
            <div className="flex flex-col gap-4">
                <div className="flex flex-row gap-2">
                    <Button color="primary" onPress={onOpen}>
                        Create Config Entry
                    </Button>
                    <Button color="danger">
                        Clear All
                    </Button>
                </div>
                {
                    data && (
                        <Table aria-label="Example table with custom cells">
                            <TableHeader columns={columns}>
                                {(column) => (
                                    <TableColumn key={column.uid}
                                                 align={column.uid === "actions" || column.uid === "status" ? "center" : "start"}>
                                        {column.name}
                                    </TableColumn>
                                )}
                            </TableHeader>
                            <TableBody items={data.entries()}>
                                {(item) => (
                                    <TableRow key={item[0]}>
                                        <TableCell>{item[0]}</TableCell>
                                        <TableCell>
                                            <p className="line-clamp-3">
                                                {item[1]}
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
                    )
                }
            </div>
            <Modal isOpen={isOpen} onOpenChange={onOpenChange}>
                <ModalContent>
                    {(onClose) => (
                        <>
                            <ModalHeader className="flex flex-col gap-1">Modal Title</ModalHeader>
                            <ModalBody>
                                <Input
                                    label="Key"
                                    placeholder="Key"
                                    value={inputValue}
                                    onValueChange={setInputValue}
                                />
                                <Textarea
                                    isRequired
                                    label="Value"
                                    labelPlacement="outside"
                                    placeholder="Enter config value"
                                    value={textareaValue}
                                    onValueChange={setTextareaValue}
                                />
                            </ModalBody>
                            <ModalFooter>
                                <Button color="danger" variant="light" onPress={onClose}>
                                    Close
                                </Button>
                                <Button color="primary" onPress={onClose}>
                                    Summit
                                </Button>
                            </ModalFooter>
                        </>
                    )}
                </ModalContent>
            </Modal>
        </div>
    )
}