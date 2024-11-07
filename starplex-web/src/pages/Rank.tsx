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
import {StarplexConfig} from "../config.ts";
import {useQuery} from "@tanstack/react-query";
import axios from "axios";
import {SimpleRating} from "../types.ts";

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
        key: "profile",
        label: "个人页面",
    },
    {
        key: "blog",
        label: "博客",
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

interface Rank {
    rank: SimpleRating[]
}

export default function RankPage() {
    const [selected, setSelected] = useState([""]);

    const {data} = useQuery({
        queryKey: ["rank"],
        queryFn: (): Promise<Rank> => axios.get(`${StarplexConfig.api}/rank`).then(data => data.data),
    });

    const getRank = (rating: SimpleRating) => {
        let rank = 0
        rank = rank + rating.rating.backlinks_rating
        rank = rank + rating.rating.repositories_description_rating
        rank = rank + rating.rating.webpages_rating
        rank = rank + rating.rating.user_popularity
        rank = rank + rating.rating.repositories_popularity

        return rank / 5
    }

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
            {
                data && data.rank && <Table aria-label="Rank Table">
                    <TableHeader columns={columns}>
                        {(column) => <TableColumn key={column.key}>{column.label}</TableColumn>}
                    </TableHeader>
                    <TableBody items={data.rank}>
                        {(item) => (
                            <TableRow key={item.username}>
                                <TableCell><Avatar src={item.avatar}/></TableCell>
                                <TableCell>
                                    <Link isBlock showAnchorIcon href={`https://github.com/${item.username}`}
                                          className="text-green-400">
                                        {`@${item.username}`}
                                    </Link>
                                </TableCell>
                                <TableCell><Link isBlock showAnchorIcon href={`/profile/${item.username}`}
                                                 className="text-green-400">
                                    {`@${item.username}`}
                                </Link></TableCell>
                                <TableCell>
                                    {
                                        item.blog && <Link isBlock showAnchorIcon href={`/profile/${item.blog}`}
                                                           className="text-green-400">
                                            {`@${item.username}'s Blog`}
                                        </Link>
                                    }
                                </TableCell>
                                <TableCell>{item.star}</TableCell>
                                <TableCell>{item.followers}</TableCell>
                                <TableCell><Chip className="bg-green-200">{getRank(item)}</Chip></TableCell>
                            </TableRow>
                        )}
                    </TableBody>
                </Table>
            }
        </div>
    )
}