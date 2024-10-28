export function randomBentoBox(row: number, col: number, height: number, width: number, gap: number) {
    enum BoxType {
        base = "base",
        row = "row",
        col = "col",
    }

    const BoxSpan = [
        {
            key: BoxType.base,
            value: `col-span-1 row-span-1`,
            height: height,
            width: width
        },
        {
            key: BoxType.row,
            value: `col-span-2 row-span-1`,
            height: height,
            width: width * 2 + gap
        },
        {
            key: BoxType.col,
            value: `col-span-1 row-span-2`,
            height: height * 2 + gap,
            width: width
        },
    ]

    const Box = [
        {type: BoxType.base, properties: {height: 1, width: 1}},
        {type: BoxType.row, properties: {height: 2, width: 1}},
        {type: BoxType.col, properties: {height: 1, width: 2}},
    ]

    const boxPosition: { box: BoxType, index: number }[][] = [[]]
    let sequence = 0;
    for (let i = 0; i < col; i++) {
        for (let j = 0; j < row; j++) {
            if (boxPosition[i] && boxPosition[i][j]) continue

            let box = Box[Math.floor(Math.random() * Box.length)]
            let width = box.properties.width
            let height = box.properties.height

            if (i + width > col || j + height > row) {
                box = Box[0]
                width = 1
                height = 1
            }

            if (
                (width > 1 || height > 1) &&
                (
                    (boxPosition[i] && boxPosition[i][j + 1]) ||
                    (boxPosition[i + 1] && boxPosition[i + 1][j])
                )
            ) {
                box = Box[0]
                width = 1
                height = 1
            }

            for (let k = 0; k < width; k++) {
                for (let l = 0; l < height; l++) {
                    if (!boxPosition[i + k]) boxPosition[i + k] = []

                    if (!boxPosition[i + k][j + l]) {
                        boxPosition[i + k][j + l] = {box: box.type, index: sequence}
                    }
                }
            }

            sequence = sequence + 1;
        }
    }

    const bento: {
        index: number,
        span: string | undefined,
        height: number | undefined,
        width: number | undefined
    }[] = []
    for (let index = 0; index < sequence; index++) {
        for (let i = 0; i < row; i++) {
            for (let j = 0; j < col; j++) {
                const box = boxPosition[i][j];
                if (box.index === index) {
                    const x = BoxSpan.find(x => x.key === box.box)

                    bento[index] = {
                        index: index,
                        span: x?.value,
                        height: x?.height,
                        width: x?.width
                    }
                }
            }
        }
    }

    return bento
}

export const profileBentoBoxTemplate = [
    {index: "0", span: "col-span-1 row-span-2"},
    {index: "1", span: "col-span-2 row-span-1"},
    {index: "2", span: "col-span-1 row-span-1"},
    {index: "3", span: "col-span-1 row-span-1"},
    {index: "4", span: "col-span-1 row-span-1"},
    {index: "5", span: "col-span-1 row-span-2"},
    {index: "6", span: "col-span-1 row-span-1"},
    {index: "7", span: "col-span-1 row-span-1"},
    {index: "8", span: "col-span-1 row-span-2"},
    {index: "9", span: "col-span-2 row-span-1"},
    {index: "10", span: "col-span-1 row-span-1"},
]

export const sloganBentoBoxTemplate = [
    {colorFrom: "#2fccba", colorTo: "#1abc9c", emoji: "❤️", title: "超级能力", text: undefined, background: undefined},
    {colorFrom: "#3498db", colorTo: "#2980b9", emoji: "✨", title: "超级开源", text: undefined, background: undefined},
    {colorFrom: "#9b59b6", colorTo: "#8e44ad", emoji: "💜", title: "超级好用", text: undefined, background: undefined},
    {colorFrom: "#e74c3c", colorTo: "#c0392b", emoji: "🎃", title: "超级可爱", text: undefined, background: undefined},
    {colorFrom: "#f1c40f", colorTo: "#f39c12", emoji: "😂", title: "超级开心", text: undefined, background: undefined},
    {colorFrom: "#1abc9c", colorTo: "#16a085", emoji: "👍", title: "超级好看", text: undefined, background: undefined},
    {colorFrom: "#e67e22", colorTo: "#d35400", emoji: "🍀", title: "超级幸运", text: undefined, background: undefined},
    {colorFrom: "#e74c3c", colorTo: "#c0392b", emoji: "🍭", title: "超级有趣", text: undefined, background: undefined},
    {colorFrom: "#3498db", colorTo: "#2980b9", emoji: "🚀", title: "超级快乐", text: undefined, background: undefined},
    {colorFrom: "#2ecc71", colorTo: "#27ae60", emoji: "🎉", title: "超级美好", text: undefined, background: undefined},
    {colorFrom: "#f39c12", colorTo: "#e67e22", emoji: "🍕", title: "超级好吃", text: undefined, background: undefined},
    {colorFrom: "#e67e22", colorTo: "#d35400", emoji: "🍀", title: "超级幸运", text: undefined, background: undefined},
    {colorFrom: "#3498db", colorTo: "#2980b9", emoji: "✨", title: "超级开源", text: undefined, background: undefined},
    {colorFrom: "#2fccba", colorTo: "#1abc9c", emoji: "❤️", title: "超级能力", text: undefined, background: undefined},
]