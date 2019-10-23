import { useRouter } from 'next/router'

const Server = () => {
    const router = useRouter()
    const { id } = router.query

    return <p>Showing server {id}</p>
}

export default Server
