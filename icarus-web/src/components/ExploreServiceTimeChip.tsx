import {Chip} from "@nextui-org/react";

interface ExploreServiceTimeChipProps {
    timestamp: number
}

export default function ExploreServiceTimeChip(props: ExploreServiceTimeChipProps) {
    const formatDate = (timestamp: number): string => {
        const date = new Date(timestamp)
        return date.toLocaleString()
    }

    return (
        <Chip className="bg-yellow-200">{formatDate(props.timestamp)}</Chip>
    )
}
