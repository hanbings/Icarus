export interface ProfilePageProps {
    username: string
}

export default function ProfilePage(props: ProfilePageProps) {
    const profileBentoBox = [
        {index: "0", span: "col-span-2 row-span-1", height: 1, width: 2, heightGap: 0, widthGap: 0.5, content: <div>{props.username}‘s Profile</div>},
        {index: "1", span: "col-span-1 row-span-1", height: 1, width: 1, heightGap: 0, widthGap: 0, content: <div>Rank</div>},
        {index: "2", span: "col-span-1 row-span-2", height: 2, width: 1, heightGap: 0.5, widthGap: 0, content: <div>Blog</div>},
        {index: "3", span: "col-span-1 row-span-1", height: 1, width: 1, heightGap: 0, widthGap: 0, content: <div>LANG#0</div>},
        {index: "4", span: "col-span-1 row-span-1", height: 1, width: 1, heightGap: 0, widthGap: 0, content: <div>LANG#1 / 热门项目</div>},
        {index: "5", span: "col-span-1 row-span-2", height: 2, width: 1, heightGap: 0.5, widthGap: 0, content: <div>Github Hot 热力图</div>},
        {index: "6", span: "col-span-2 row-span-1", height: 1, width: 2, heightGap: 0, widthGap: 0.5, content: <div>AI Content</div>},
        {index: "7", span: "col-span-1 row-span-1", height: 1, width: 1, heightGap: 0, widthGap: 0, content: <div>LANG#3 / 热门项目</div>},
    ]

    return (
        <div className="grid grid-cols-4 gap-2">
            {profileBentoBox.map((item, index) => (
                <div
                    key={index}
                    className={`flex items-center justify-center bg-white rounded-3xl shadow ${item.span}`}
                    style={{
                        height: `${item.height * 24 + item.heightGap}vh`,
                        width: `${item.width * 12 + item.widthGap}vw`
                    }}
                >
                    {item.content}
                </div>
            ))}
        </div>
    )
}