import {
    Avatar,
    Card,
    Checkbox,
    CheckboxGroup,
    Chip,
    Link,
    Table,
    TableBody,
    TableCell,
    TableColumn,
    TableHeader,
    TableRow
} from "@nextui-org/react";
import {useState} from "react";

const rows = [
    {
        avatar: "https://avatars.githubusercontent.com/u/38599937",
        username: "hanbings",
        language: ["java", "typescript", "rust"],
        stars: 100,
        followers: 232,
        rank: 60,
    },
];

const columns = [
    {
        key: "avatar",
        label: "头像",
    },
    {
        key: "username",
        label: "Github 用户名",
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
            </Card>
            <Table aria-label="Rank Table">
                <TableHeader columns={columns}>
                    {(column) => <TableColumn key={column.key}>{column.label}</TableColumn>}
                </TableHeader>
                <TableBody items={rows}>
                    {(item) => (
                        <TableRow key={item.username}>
                            <TableCell><Avatar src={item.avatar}/></TableCell>
                            <TableCell>
                                <Link isBlock showAnchorIcon href={`https://github.com/${item.username}`}
                                      className="text-green-400">
                                    {`@${item.username}`}
                                </Link>
                            </TableCell>
                            <TableCell>{
                                item.language.map((language) => {
                                    return (
                                        <div key={language}>{language}</div>
                                    )
                                })
                            }</TableCell>
                            <TableCell>{item.stars}</TableCell>
                            <TableCell>{item.followers}</TableCell>
                            <TableCell><Chip className="bg-green-200">{item.rank}</Chip></TableCell>
                        </TableRow>
                    )}
                </TableBody>
            </Table>
        </div>
    )
}