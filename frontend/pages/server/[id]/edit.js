import { useRouter } from 'next/router'

const EditServer = () => {
    const router = useRouter()
    const { id } = router.query

    return <p>Now editing server {id}</p>
}

export default EditServer
