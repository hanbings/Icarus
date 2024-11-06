import React from "react";

export interface BentoBlockProps {
    height: number
    width: number
    children?: React.ReactNode
    className?: string
    style?: React.CSSProperties
}

export default function BentoBlock(props: BentoBlockProps) {
    return (
        <div className={`h-[${props.height} * 24px] w-[${props.width} * 24px] ${props.className} col-span-${props.width} row-span-${props.height}`} style={props.style}>
            {props.children}
        </div>
    )
}