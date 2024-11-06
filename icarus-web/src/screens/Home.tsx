import {Card, CardBody, Tab, Tabs} from "@nextui-org/react";
import Activity from "../pages/Activity.tsx";
import Config from "../pages/Config.tsx";
import Explore from "../pages/Explore.tsx";
import Message from "../pages/Message.tsx";

export function HomeScreen() {
    // const token = useSelector((state: AppStore) => state.token)
    // const navigate = useNavigate();

    // useEffect(() => { if (!token.token) navigate("/login") }, [navigate, token.token]);

    let tabs = [
        {
            id: "activity",
            label: "Activity",
            content: <Activity/>
        },
        {
            id: "config",
            label: "Config Center",
            content: <Config/>
        },
        {
            id: "explore",
            label: "Explore Service",
            content: <Explore/>
        },
        {
            id: "message",
            label: "Message Queue",
            content: <Message/>
        }
    ];

    return (
        <div className="bg-[#fff5e1] flex h-screen w-full flex-col justify-center items-center">
            <Tabs aria-label="Dynamic tabs" items={tabs}
                  className="py-4 top-0 left-0 right-0 z-50 items-center justify-center">
                {(item) => (
                    <Tab key={item.id} title={item.label} className="h-full w-full px-4">
                        <Card>
                            <CardBody>
                                {item.content}
                            </CardBody>
                        </Card>
                    </Tab>
                )}
            </Tabs>
        </div>
    )
}