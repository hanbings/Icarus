export function randomBentoBox() {
    enum BoxType {
        base = "base",
        row = "row",
        col = "col",
    }

    const BoxSpan = [
        {
            key: BoxType.base,
            value: 'col-span-1 row-span-1'
        },
        {
            key: BoxType.row,
            value: 'col-span-2 row-span-1'
        },
        {
            key: BoxType.col,
            value: 'col-span-1 row-span-2'
        },
    ]

    const Box = [
        {type: BoxType.base, properties: {height: 1, width: 1}},
        {type: BoxType.row, properties: {height: 2, width: 1}},
        {type: BoxType.col, properties: {height: 1, width: 2}},
    ]

    const row = 4
    const col = 4

    const boxPosition: { box: BoxType, index: number }[][] = [[]]
    let sequence = 0;
    for (let i = 0; i < col; i++) {
        for (let j = 0; j < row; j++) {
            if (boxPosition[i] && boxPosition[i][j]) continue

            console.log(sequence, i, j)

            let box = Box[Math.floor(Math.random() * Box.length)]
            let width = box.properties.width
            let height = box.properties.height

            if (i + width >= col || j + height >= row) {
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

    const bento: { index: number, span: string | undefined }[] = []
    for (let index = 0; index < sequence; index++) {
        for (let i = 0; i < row; i++) {
            for (let j = 0; j < col; j++) {
                const box = boxPosition[i][j];
                if (box.index === index) {
                    bento[index] = {
                        index: index, span: BoxSpan.find(x => x.key === box.box)?.value
                    }
                }
            }
        }
    }

    return bento
}

export const profileBentoBoxTemplate = [
    {content: "Item 1", span: "col-span-1 row-span-2"},
    {content: "Item 2", span: "col-span-2 row-span-1"},
    {content: "Item 3", span: "col-span-1 row-span-1"},
    {content: "Item 4", span: "col-span-1 row-span-1"},
    {content: "Item 5", span: "col-span-1 row-span-1"},
    {content: "Item 6", span: "col-span-1 row-span-2"},
    {content: "Item 7", span: "col-span-1 row-span-1"},
    {content: "Item 8", span: "col-span-1 row-span-1"},
    {content: "Item 9", span: "col-span-1 row-span-2"},
    {content: "Item 10", span: "col-span-2 row-span-1"},
    {content: "Item 11", span: "col-span-1 row-span-1"},
]