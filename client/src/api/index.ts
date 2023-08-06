import axios, { AxiosResponse } from 'axios'

const API = import.meta.env.VITE_API || 'http://localhost:8000'

interface ReqBody {
  url: string
}

export async function addLinkAPI(
  reqBody: ReqBody,
): Promise<AxiosResponse<string>> {
  return axios.post(`${API}/`, reqBody)
}

export { API }
