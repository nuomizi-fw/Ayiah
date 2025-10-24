import { useNavigate, useParams } from "@solidjs/router";
import MediaDetail from "../components/MediaDetail";

export default function DetailPage() {
    const params = useParams();
    const navigate = useNavigate();

    const handleBack = () => {
        navigate(-1);
    };

    return <MediaDetail itemId={Number(params.id)} onBack={handleBack} />;
}
