import {
    Table,
    TableHeader,
    TableColumn,
    TableBody,
    TableRow,
    TableCell,
    getKeyValue,
    Checkbox, CheckboxGroup, Card
} from "@nextui-org/react";
import {useState} from "react";

const rows = [
    {
        key: "1",
        name: "Tony Reichert",
        role: "CEO",
        status: "Active",
    },
    {
        key: "2",
        name: "Zoey Lang",
        role: "Technical Lead",
        status: "Paused",
    },
    {
        key: "3",
        name: "Jane Fisher",
        role: "Senior Developer",
        status: "Active",
    },
    {
        key: "4",
        name: "William Howard",
        role: "Community Manager",
        status: "Vacation",
    },
];

const columns = [
    {
        key: "avatar",
        label: "头像",
    },
    {
        key: "username",
        label: "用户名",
    },
    {
        key: "language",
        label: "编程语言",
    },
    {
        key: "stars",
        label: "Star",
    },
    {
        key: "followers",
        label: "Follower",
    },
    {
        key: "rank",
        label: "分数",
    },
];

export default function RankPage() {
    const [selected, setSelected] = useState([""]);
    
    return (
        <div className="flex flex-col gap-4 w-full md:w-1/2">
            <Card className="p-4">
                <CheckboxGroup
                    label="筛选"
                    color="primary"
                    value={selected}
                    onValueChange={setSelected}
                >
                    <Checkbox value="follower">按 Follower 排序</Checkbox>
                    <Checkbox value="stars">按 Star 排序</Checkbox>
                    <Checkbox value="some-country">只看国内的开发者</Checkbox>
                    <Checkbox value="reverse">倒序</Checkbox>
                </CheckboxGroup>
                <p className="text-default-500 text-small">Selected: {selected.join(", ")}</p>
            </Card>
            <Table aria-label="Rank Table">
                <TableHeader columns={columns}>
                    {(column) => <TableColumn key={column.key}>{column.label}</TableColumn>}
                </TableHeader>
                <TableBody items={rows}>
                    {(item) => (
                        <TableRow key={item.key}>
                            {(columnKey) => <TableCell>{getKeyValue(item, columnKey)}</TableCell>}
                        </TableRow>
                    )}
                </TableBody>
            </Table>
        </div>
    )
}