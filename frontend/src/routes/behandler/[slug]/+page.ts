import type { PageLoad } from './$types';

export const load: PageLoad = ({ params }) => {
    return {
        name: params.slug
    }
}

export interface BehandlerData {
    name: string
}
