import { createSignal } from 'solid-js'
import { addLinkAPI, API } from '~/api'

interface ReqBody {
  url: string
}

export default function Form() {
  const [loading, setLoading] = createSignal(false)
  const [shortUrl, setShortUrl] = createSignal('')
  const [payload, setPayload] = createSignal<ReqBody>({ url: '' })

  const onSubmit = async (e: Event) => {
    e.preventDefault()
    if (!payload().url) return

    setLoading(true)
    const res = await addLinkAPI(payload())

    if (res?.data) {
      setShortUrl(`${API}/${res.data}`)
    }

    setLoading(false)
  }

  return (
    <section class="bg-white ">
      <div class="py-8 lg:py-16 px-4 mx-auto max-w-screen-md">
        {shortUrl() ? (
          shortUrl()
        ) : (
          <form onSubmit={onSubmit} class="space-y-8">
            <div class="sm:col-span-2">
              <textarea
                id="message"
                rows="6"
                class="block p-2.5 w-full text-sm text-gray-900 bg-gray-50 rounded-lg shadow-sm border border-gray-300 focus:ring-primary-500 focus:border-primary-500 "
                placeholder="https://example.com"
                value={payload().url}
                onInput={(e) => setPayload({ url: e.currentTarget.value })}
              ></textarea>
            </div>
            <button
              type="submit"
              class="py-3 px-5 text-sm font-medium text-center text-white rounded-lg bg-blue-700 sm:w-fit hover:bg-primary-800 focus:ring-4 focus:outline-none focus:ring-primary-300"
            >
              {loading() ? 'Loading 🚀' : 'Generate 🔨'}
            </button>
          </form>
        )}
      </div>
    </section>
  )
}
